pub const HEADER_SIZE: u64 = 4096;
pub const HEADER_MAGIC: [u8; 4] = [0x45, 0x56, 0x4E, 0x54];
pub const TAIL_MAGIC: [u8; 4] = [0x45, 0x4E, 0x44, 0x21];
pub const FORMAT_VERSION: u16 = 1;
pub const ITEM_HEADER_SIZE: u32 = 8;

mod reader;
mod writer;

pub use reader::SegmentReader;
pub use writer::SegmentWriter;
