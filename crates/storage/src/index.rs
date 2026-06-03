use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use crate::error::{CorruptIndex, IoOperation, WalError, WalResult};
use crate::io;

const MAGIC: &[u8; 4] = b"SIDX";
const HEADER_SIZE: usize = 4096;
const ENTRY_SIZE: usize = 16;

#[derive(Debug)]
pub struct SparseIndex {
    pub items: Vec<(u64, u64)>,
    pub seg_id: u64,
    pub interval: u32,
    pub count: u64,
}

impl SparseIndex {
    pub fn new(seg_id: u64, interval: u32) -> Self {
        Self {
            items: Vec::new(),
            seg_id,
            interval,
            count: 0,
        }
    }

    pub fn insert(&mut self, offset: u64, byte_off: u64) {
        if self.interval == 0 || self.count.is_multiple_of(self.interval as u64) {
            self.items.push((offset, byte_off));
        }
        self.count += 1;
    }

    pub fn find(&self, target_off: u64) -> Option<(u64, u64)> {
        if self.items.is_empty() {
            return None;
        }
        match self.items.binary_search_by_key(&target_off, |(o, _)| *o) {
            Ok(i) => Some(self.items[i]),
            Err(0) => None,
            Err(i) => Some(self.items[i - 1]),
        }
    }

    pub fn save(&self, path: &Path) -> WalResult<()> {
        tracing::debug!(
            seg_id = self.seg_id,
            entries = self.items.len(),
            "saving sparse index"
        );
        let mut items_buf = Vec::with_capacity(self.items.len() * ENTRY_SIZE);
        for &(offset, byte_off) in &self.items {
            items_buf.extend_from_slice(&offset.to_le_bytes());
            items_buf.extend_from_slice(&byte_off.to_le_bytes());
        }
        let items_crc = crc32c::crc32c(&items_buf);

        let mut buf = Vec::with_capacity(HEADER_SIZE + items_buf.len());
        buf.extend_from_slice(MAGIC);
        buf.extend_from_slice(&self.seg_id.to_le_bytes());
        buf.extend_from_slice(&self.interval.to_le_bytes());
        buf.extend_from_slice(&(self.items.len() as u32).to_le_bytes());
        buf.extend_from_slice(&items_crc.to_le_bytes());
        buf.extend_from_slice(&self.count.to_le_bytes());
        buf.resize(HEADER_SIZE, 0);
        buf.extend_from_slice(&items_buf);

        let temp_path = path.with_extension("idx.tmp");
        {
            let mut file = fs::File::create(&temp_path)
                .map_err(WalError::at(temp_path.clone(), IoOperation::Create))?;
            file.write_all(&buf)
                .map_err(WalError::at(temp_path.clone(), IoOperation::Write))?;
            file.sync_all()
                .map_err(WalError::at(temp_path.clone(), IoOperation::Sync))?;
        }
        fs::rename(&temp_path, path)
            .map_err(WalError::at(path.to_path_buf(), IoOperation::Rename))?;

        if let Some(parent) = path.parent() {
            io::fsync_dir(parent)?;
        }

        Ok(())
    }

    pub fn load(path: &Path) -> WalResult<Self> {
        let mut data = Vec::new();
        let mut file =
            fs::File::open(path).map_err(WalError::at(path.to_path_buf(), IoOperation::Read))?;
        file.read_to_end(&mut data)
            .map_err(WalError::at(path.to_path_buf(), IoOperation::Read))?;

        if data.len() < HEADER_SIZE {
            return Err(WalError::IndexCorrupted {
                segment_id: 0,
                reason: CorruptIndex::FileTooSmall {
                    actual_len: data.len() as u64,
                },
            });
        }

        let header = &data[..HEADER_SIZE];

        if &header[0..4] != MAGIC {
            return Err(WalError::IndexCorrupted {
                segment_id: 0,
                reason: CorruptIndex::BadMagic {
                    expected: *MAGIC,
                    actual: header[0..4].try_into().unwrap(),
                },
            });
        }

        let seg_id = le_u64(header, 4);
        let interval = le_u32(header, 12);
        let item_count = le_u32(header, 16) as usize;
        let stored_crc = le_u32(header, 20);
        let count = le_u64(header, 24);

        let expected_len = HEADER_SIZE + item_count * ENTRY_SIZE;
        if data.len() < expected_len {
            return Err(WalError::IndexCorrupted {
                segment_id: seg_id,
                reason: CorruptIndex::ItemTruncated {
                    expected_len: item_count * ENTRY_SIZE,
                    actual_len: data.len() - HEADER_SIZE,
                },
            });
        }

        let items_data = &data[HEADER_SIZE..expected_len];
        let computed_crc = crc32c::crc32c(items_data);
        if computed_crc != stored_crc {
            return Err(WalError::IndexCorrupted {
                segment_id: seg_id,
                reason: CorruptIndex::ChecksumMismatch {
                    expected: stored_crc,
                    actual: computed_crc,
                },
            });
        }

        let mut items = Vec::with_capacity(item_count);
        for chunk in items_data.chunks_exact(ENTRY_SIZE) {
            let offset = le_u64(chunk, 0);
            let byte_off = le_u64(chunk, 8);
            items.push((offset, byte_off));
        }

        tracing::debug!(seg_id, entries = items.len(), count, "sparse index loaded");

        Ok(Self {
            items,
            seg_id,
            interval,
            count,
        })
    }

    pub fn set_count(&mut self, count: u64) {
        self.count = count;
    }
}

pub fn le_u64(buf: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(
        buf[offset..offset + 8]
            .try_into()
            .expect("slice is 8 bytes"),
    )
}

pub fn le_u32(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        buf[offset..offset + 4]
            .try_into()
            .expect("slice is 4 bytes"),
    )
}
