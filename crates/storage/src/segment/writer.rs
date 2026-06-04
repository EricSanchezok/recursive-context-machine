use std::fs::{File, OpenOptions};
use std::io::{Read as _, Seek as _, Write as _};
use std::path::{Path, PathBuf};

use crc32c::crc32c;
use memmap2::Mmap;

use super::{FORMAT_VERSION, HEADER_MAGIC, HEADER_SIZE, ITEM_HEADER_SIZE, TAIL_MAGIC};
use crate::error::{CorruptSegment, IoOperation, WalError, WalResult};
use crate::index::SparseIndex;
use crate::io;

#[derive(Copy, Clone)]
struct ItemMeta {
    offset: u64,
    position: u64,
    item_crc: u32,
    payload_len: usize,
}

pub struct SegmentWriter {
    file: File,
    path: PathBuf,
    id: u64,
    offset: u64,
    count: u32,
    position: u64,
    max_size: u64,
    checksum: u32,
    truncated_items: u32,
}

impl SegmentWriter {
    pub fn new(dir: &Path, seg_id: u64, offset: u64, max_size: u64) -> WalResult<Self> {
        tracing::debug!(seg_id, offset, max_size, "creating new segment");
        let path = dir.join(format!("{:06}.seg", seg_id));
        let mut file =
            File::create(&path).map_err(WalError::at(path.clone(), IoOperation::Create))?;

        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let mut header = [0u8; HEADER_SIZE as usize];
        header[0..4].copy_from_slice(&HEADER_MAGIC);
        header[4..6].copy_from_slice(&FORMAT_VERSION.to_le_bytes());
        // bytes 6..8: padding for u64 alignment
        header[8..16].copy_from_slice(&seg_id.to_le_bytes());
        header[16..24].copy_from_slice(&offset.to_le_bytes());
        // bytes 24..28: count — written on close
        header[28..36].copy_from_slice(&created_at.to_le_bytes());

        file.write_all(&header)
            .map_err(WalError::at(path.clone(), IoOperation::Write))?;
        file.sync_all()
            .map_err(WalError::at(path.clone(), IoOperation::Sync))?;

        io::fsync_dir(dir)?;

        Ok(Self {
            file,
            path,
            id: seg_id,
            offset,
            count: 0,
            position: HEADER_SIZE,
            max_size,
            checksum: 0,
            truncated_items: 0,
        })
    }

    pub fn open(path: &Path, max_size: u64, idx_interval: u32) -> WalResult<(Self, SparseIndex)> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .map_err(WalError::at(path.to_path_buf(), IoOperation::Read))?;

        let file_len = file
            .metadata()
            .map_err(WalError::at(path.to_path_buf(), IoOperation::Read))?
            .len();

        if file_len < HEADER_SIZE {
            return Err(WalError::SegmentCorrupted {
                segment_id: 0,
                reason: CorruptSegment::FileTooSmall {
                    actual_len: file_len,
                },
            });
        }

        let mut header = [0u8; HEADER_SIZE as usize];
        file.read_exact(&mut header)
            .map_err(WalError::at(path.to_path_buf(), IoOperation::Read))?;

        if header[0..4] != HEADER_MAGIC {
            return Err(WalError::SegmentCorrupted {
                segment_id: 0,
                reason: CorruptSegment::BadMagic {
                    expected: HEADER_MAGIC,
                    actual: header[0..4].try_into().unwrap(),
                },
            });
        }

        let seg_id = u64::from_le_bytes(header[8..16].try_into().unwrap());
        let first_offset = u64::from_le_bytes(header[16..24].try_into().unwrap());
        let header_count = u32::from_le_bytes(header[24..28].try_into().unwrap());

        // Fast recovery via mmap: zero-syscall, zero-alloc item scanning.
        // For closed segments with valid footer CRC, skip per-item CRC checks.
        if let Some(result) = Self::mmap_recover(
            path,
            file_len,
            seg_id,
            first_offset,
            header_count,
            idx_interval,
        ) {
            let (position, count, checksum, truncated_items, index) = result;

            file.set_len(position)
                .map_err(WalError::at(path.to_path_buf(), IoOperation::Write))?;
            file.seek(std::io::SeekFrom::Start(position))
                .map_err(WalError::at(path.to_path_buf(), IoOperation::Write))?;
            file.sync_all()
                .map_err(WalError::at(path.to_path_buf(), IoOperation::Sync))?;

            if truncated_items > 0 {
                tracing::warn!(
                    seg_id,
                    recovered = count,
                    truncated = truncated_items,
                    "segment recovery completed with truncation"
                );
            } else {
                tracing::debug!(seg_id, items = count, "segment recovered cleanly via mmap");
            }

            return Ok((
                Self {
                    file,
                    path: path.to_path_buf(),
                    id: seg_id,
                    offset: first_offset,
                    count,
                    position,
                    max_size,
                    checksum,
                    truncated_items,
                },
                index,
            ));
        }

        // Fallback: slow path via read syscalls.
        let mut position = HEADER_SIZE;
        let mut count: u32 = 0;
        let mut checksum: u32 = 0;
        let mut index = SparseIndex::new(seg_id, idx_interval);
        let mut truncated = false;

        file.seek(std::io::SeekFrom::Start(HEADER_SIZE))
            .map_err(WalError::at(path.to_path_buf(), IoOperation::Read))?;

        loop {
            if position + ITEM_HEADER_SIZE as u64 > file_len {
                break;
            }

            let mut len_buf = [0u8; 4];
            if file.read_exact(&mut len_buf).is_err() {
                truncated = true;
                break;
            }

            let item_len = u32::from_le_bytes(len_buf);

            if len_buf == TAIL_MAGIC && position + 8 == file_len {
                return Err(WalError::SegmentCorrupted {
                    segment_id: seg_id,
                    reason: CorruptSegment::AlreadyClosed,
                });
            }

            if item_len < ITEM_HEADER_SIZE || position + item_len as u64 > file_len {
                truncated = true;
                break;
            }

            let mut crc_buf = [0u8; 4];
            if file.read_exact(&mut crc_buf).is_err() {
                truncated = true;
                break;
            }
            let stored_crc = u32::from_le_bytes(crc_buf);

            let payload_len = (item_len - ITEM_HEADER_SIZE) as usize;
            let mut payload = vec![0u8; payload_len];
            if file.read_exact(&mut payload).is_err() {
                truncated = true;
                break;
            }

            let item_crc = crc32c(&payload);
            if item_crc != stored_crc {
                tracing::warn!(
                    seg_id,
                    item = count,
                    position,
                    "CRC mismatch at item during recovery, truncating"
                );
                truncated = true;
                break;
            }

            let global_offset = first_offset + count as u64;
            index.insert(global_offset, position);

            checksum = crc32c::crc32c_combine(checksum, item_crc, payload_len);
            position += item_len as u64;
            count += 1;
        }

        file.set_len(position)
            .map_err(WalError::at(path.to_path_buf(), IoOperation::Write))?;
        file.seek(std::io::SeekFrom::Start(position))
            .map_err(WalError::at(path.to_path_buf(), IoOperation::Write))?;
        file.sync_all()
            .map_err(WalError::at(path.to_path_buf(), IoOperation::Sync))?;

        let truncated_items = if truncated {
            header_count.saturating_sub(count)
        } else {
            0
        };

        if truncated_items > 0 {
            tracing::warn!(
                seg_id,
                recovered = count,
                truncated = truncated_items,
                "segment recovery completed with truncation"
            );
        } else {
            tracing::debug!(
                seg_id,
                items = count,
                "segment recovered cleanly via read path"
            );
        }

        Ok((
            Self {
                file,
                path: path.to_path_buf(),
                id: seg_id,
                offset: first_offset,
                count,
                position,
                max_size,
                checksum,
                truncated_items,
            },
            index,
        ))
    }

    /// Read segment-level CRC from footer if TAIL_MAGIC is present.
    /// mmap-based recovery: scan items with zero syscalls and zero heap allocs.
    /// Returns None for closed segments (caller should return AlreadyClosed)
    /// or if mmap fails (caller falls back to read-syscall path).
    fn mmap_recover(
        path: &Path,
        file_len: u64,
        seg_id: u64,
        first_offset: u64,
        header_count: u32,
        idx_interval: u32,
    ) -> Option<(u64, u32, u32, u32, SparseIndex)> {
        let file = std::fs::File::open(path).ok()?;
        let mmap = unsafe { Mmap::map(&file) }.ok()?;

        // Closed segment — open() should not recover these.
        if file_len >= HEADER_SIZE + 8 {
            let tail_start = file_len as usize - 8;
            if mmap[tail_start..tail_start + 4] == TAIL_MAGIC {
                return None;
            }
        }

        let mut pos = HEADER_SIZE as usize;
        let mut count: u32 = 0;
        let mut checksum: u32 = 0;
        let mut index = SparseIndex::new(seg_id, idx_interval);
        let mut truncated = false;

        while pos + ITEM_HEADER_SIZE as usize <= file_len as usize {
            let item_len_bytes: [u8; 4] = mmap[pos..pos + 4].try_into().ok()?;
            let item_len = u32::from_le_bytes(item_len_bytes);

            if item_len < ITEM_HEADER_SIZE || pos + item_len as usize > file_len as usize {
                truncated = true;
                break;
            }

            let stored_crc = u32::from_le_bytes(mmap[pos + 4..pos + 8].try_into().ok()?);
            let payload = &mmap[pos + 8..pos + item_len as usize];
            let item_crc = crc32c(payload);

            if item_crc != stored_crc {
                tracing::warn!(
                    seg_id,
                    item = count,
                    position = pos,
                    "CRC mismatch at item during mmap recovery, truncating"
                );
                truncated = true;
                break;
            }

            let global_offset = first_offset + count as u64;
            index.insert(global_offset, pos as u64);

            checksum = crc32c::crc32c_combine(checksum, item_crc, payload.len());
            pos += item_len as usize;
            count += 1;
        }

        let truncated_items = if truncated {
            header_count.saturating_sub(count)
        } else {
            0
        };

        Some((pos as u64, count, checksum, truncated_items, index))
    }

    pub fn append(&mut self, bytes: &[u8]) -> WalResult<(u64, u64)> {
        let mut buf = Vec::new();
        let meta = self.prepare_item(bytes, &mut buf)?;
        self.write_buffer(&buf)?;
        self.sync()?;
        self.commit_item(meta);
        Ok((meta.offset, meta.position))
    }

    /// Append multiple items in one write + one fsync.
    pub(crate) fn append_batch(&mut self, items: &[&[u8]]) -> WalResult<Vec<(u64, u64)>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        let saved_position = self.position;
        let saved_count = self.count;

        let mut buf = Vec::new();
        let mut metas = Vec::with_capacity(items.len());

        for &bytes in items {
            let meta = self.prepare_item(bytes, &mut buf)?;
            metas.push(meta);
        }

        if let Err(e) = self.write_buffer(&buf).and_then(|_| self.sync()) {
            self.position = saved_position;
            self.count = saved_count;
            return Err(e);
        }

        for meta in &metas {
            self.commit_item(*meta);
        }

        Ok(metas.iter().map(|m| (m.offset, m.position)).collect())
    }

    /// Append a single item without fsync. Caller must call `sync()` later.
    pub(crate) fn append_unsynced(&mut self, bytes: &[u8]) -> WalResult<(u64, u64)> {
        let saved_position = self.position;
        let saved_count = self.count;

        let mut buf = Vec::new();
        let meta = self.prepare_item(bytes, &mut buf)?;

        if let Err(e) = self.write_buffer(&buf) {
            self.position = saved_position;
            self.count = saved_count;
            return Err(e);
        }

        self.commit_item(meta);
        Ok((meta.offset, meta.position))
    }

    pub(crate) fn sync(&mut self) -> WalResult<()> {
        self.file
            .sync_all()
            .map_err(WalError::at(self.path.clone(), IoOperation::Sync))
    }

    /// Prepare an item: validate, compute CRC, encode into buffer.
    /// Advances position and count so consecutive calls produce correct offsets.
    fn prepare_item(&mut self, bytes: &[u8], buf: &mut Vec<u8>) -> WalResult<ItemMeta> {
        let payload_len: u32 = bytes.len().try_into().map_err(|_| WalError::InvalidInput {
            detail: "payload too large",
        })?;
        let item_len = ITEM_HEADER_SIZE
            .checked_add(payload_len)
            .ok_or(WalError::InvalidInput {
                detail: "item too large",
            })?;

        let item_crc = crc32c(bytes);

        buf.extend_from_slice(&item_len.to_le_bytes());
        buf.extend_from_slice(&item_crc.to_le_bytes());
        buf.extend_from_slice(bytes);

        let byte_pos = self.position;
        let global_offset =
            self.offset
                .checked_add(self.count as u64)
                .ok_or(WalError::InvalidInput {
                    detail: "offset overflow",
                })?;

        self.position += item_len as u64;
        self.count += 1;

        Ok(ItemMeta {
            offset: global_offset,
            position: byte_pos,
            item_crc,
            payload_len: bytes.len(),
        })
    }

    fn write_buffer(&mut self, buf: &[u8]) -> WalResult<()> {
        self.file
            .write_all(buf)
            .map_err(WalError::at(self.path.clone(), IoOperation::Write))
    }

    /// Update checksum after an item has been written to the file.
    fn commit_item(&mut self, meta: ItemMeta) {
        self.checksum = crc32c::crc32c_combine(self.checksum, meta.item_crc, meta.payload_len);
    }

    pub fn is_full(&self) -> bool {
        self.position >= self.max_size
    }

    pub fn close(mut self) -> WalResult<()> {
        tracing::debug!(seg_id = self.id, items = self.count, "closing segment");
        self.file
            .write_all(&TAIL_MAGIC)
            .map_err(WalError::at(self.path.clone(), IoOperation::Write))?;
        self.file
            .write_all(&self.checksum.to_le_bytes())
            .map_err(WalError::at(self.path.clone(), IoOperation::Write))?;

        self.file
            .seek(std::io::SeekFrom::Start(24))
            .map_err(WalError::at(self.path.clone(), IoOperation::Write))?;
        self.file
            .write_all(&self.count.to_le_bytes())
            .map_err(WalError::at(self.path.clone(), IoOperation::Write))?;
        self.file
            .sync_all()
            .map_err(WalError::at(self.path.clone(), IoOperation::Sync))?;

        Ok(())
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn truncated_items(&self) -> u32 {
        self.truncated_items
    }
}
