use std::path::Path;

use crc32c::crc32c;
use memmap2::Mmap;

use super::{HEADER_MAGIC, HEADER_SIZE, ITEM_HEADER_SIZE, TAIL_MAGIC};
use crate::error::{CorruptSegment, IoOperation, WalError, WalResult};

#[derive(Debug)]
pub struct SegmentReader {
    mmap: Mmap,
    id: u64,
    offset: u64,
    count: u32,
}

impl SegmentReader {
    pub fn open(path: &Path) -> WalResult<Self> {
        let file = std::fs::File::open(path)
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

        let mmap = unsafe { Mmap::map(&file) }
            .map_err(WalError::at(path.to_path_buf(), IoOperation::Read))?;

        if mmap[0..4] != HEADER_MAGIC {
            return Err(WalError::SegmentCorrupted {
                segment_id: 0,
                reason: CorruptSegment::BadMagic {
                    expected: HEADER_MAGIC,
                    actual: mmap[0..4].try_into().unwrap(),
                },
            });
        }

        let id = u64::from_le_bytes(mmap[8..16].try_into().unwrap());
        let offset = u64::from_le_bytes(mmap[16..24].try_into().unwrap());
        let count = u32::from_le_bytes(mmap[24..28].try_into().unwrap());

        Ok(Self {
            mmap,
            id,
            offset,
            count,
        })
    }

    pub(crate) fn read_at(&self, position: u64) -> WalResult<Option<(&[u8], u64)>> {
        let pos = position as usize;
        let map_len = self.mmap.len();

        if pos + (ITEM_HEADER_SIZE as usize) > map_len {
            return Ok(None);
        }

        let item_len = u32::from_le_bytes(self.mmap[pos..pos + 4].try_into().unwrap());

        if item_len < ITEM_HEADER_SIZE {
            if self.mmap[pos..pos + 4] == TAIL_MAGIC {
                return Ok(None);
            }
            return Err(WalError::SegmentCorrupted {
                segment_id: self.id,
                reason: CorruptSegment::InvalidItemLen {
                    len: item_len,
                    offset: position,
                },
            });
        }

        let total = item_len as usize;
        if pos + total > map_len {
            let tail_magic_value: u32 = u32::from_le_bytes(TAIL_MAGIC);
            if item_len == tail_magic_value && map_len < tail_magic_value as usize {
                return Ok(None);
            }
            return Err(WalError::SegmentCorrupted {
                segment_id: self.id,
                reason: CorruptSegment::ItemTruncated { offset: position },
            });
        }

        let stored_crc = u32::from_le_bytes(self.mmap[pos + 4..pos + 8].try_into().unwrap());
        let payload = &self.mmap[pos + 8..pos + total];
        let actual_crc = crc32c(payload);

        if stored_crc != actual_crc {
            return Err(WalError::SegmentCorrupted {
                segment_id: self.id,
                reason: CorruptSegment::ChecksumMismatch {
                    offset: position,
                    expected: stored_crc,
                    actual: actual_crc,
                },
            });
        }

        Ok(Some((payload, position + total as u64)))
    }

    pub fn iter(&self) -> SegmentIter<'_> {
        SegmentIter {
            reader: self,
            position: HEADER_SIZE,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn count(&self) -> u32 {
        self.count
    }
}

pub struct SegmentIter<'a> {
    reader: &'a SegmentReader,
    position: u64,
}

impl<'a> Iterator for SegmentIter<'a> {
    type Item = WalResult<(u64, &'a [u8])>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_at(self.position) {
            Ok(None) => None,
            Ok(Some((payload, next_position))) => {
                let item_offset = self.position;
                self.position = next_position;
                Some(Ok((item_offset, payload)))
            }
            Err(e) => {
                self.position = u64::MAX;
                Some(Err(e))
            }
        }
    }
}
