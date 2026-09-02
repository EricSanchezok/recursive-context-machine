use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum CorruptSegment {
    #[error("segment file is too small to contain a valid header (got {actual_len} bytes)")]
    FileTooSmall { actual_len: u64 },
    #[error("segment header magic mismatch: expected {expected:?}, got {actual:?}")]
    BadMagic { expected: [u8; 4], actual: [u8; 4] },
    #[error("invalid item length {len} at offset {offset}: item header is incomplete")]
    InvalidItemLen { len: u32, offset: u64 },
    #[error("item at offset {offset} extends past end of segment file")]
    ItemTruncated { offset: u64 },
    #[error("checksum mismatch at offset {offset}: expected {expected:#010x}, got {actual:#010x}")]
    ChecksumMismatch {
        offset: u64,
        expected: u32,
        actual: u32,
    },
    #[error("segment is already closed: cannot open for writing")]
    AlreadyClosed,
}

#[derive(Debug, thiserror::Error)]
pub enum CorruptSnapshot {
    #[error("snapshot file header is too short to read: {source}")]
    HeaderTooShort { source: std::io::Error },
    #[error("snapshot header magic mismatch: expected {expected:?}, got {actual:?}")]
    BadMagic { expected: [u8; 4], actual: [u8; 4] },
    #[error("unsupported snapshot format version {version}")]
    UnsupportedVersion { version: u16 },
    #[error("snapshot payload is shorter than declared in header: {source}")]
    PayloadTruncated { source: std::io::Error },
    #[error("snapshot checksum mismatch: expected {expected:#010x}, got {actual:#010x}")]
    ChecksumMismatch { expected: u32, actual: u32 },
}

#[derive(Debug, thiserror::Error)]
pub enum CorruptIndex {
    #[error("index file is too small to contain a valid header (got {actual_len} bytes)")]
    FileTooSmall { actual_len: u64 },
    #[error("index header magic mismatch: expected {expected:?}, got {actual:?}")]
    BadMagic { expected: [u8; 4], actual: [u8; 4] },
    #[error("index item data is truncated: expected {expected_len} bytes, got {actual_len}")]
    ItemTruncated {
        expected_len: usize,
        actual_len: usize,
    },
    #[error("index checksum mismatch: expected {expected:#010x}, got {actual:#010x}")]
    ChecksumMismatch { expected: u32, actual: u32 },
}

#[derive(Debug, thiserror::Error)]
pub enum CorruptManifest {
    #[error("manifest field '{key}' has an invalid value that cannot be parsed")]
    InvalidField { key: &'static str },
    #[error("manifest is missing the required 'version' field")]
    MissingVersion,
    #[error("manifest invariant violated: {detail}")]
    InvariantViolated { detail: &'static str },
}

#[derive(Debug, Clone, Copy)]
pub enum IoOperation {
    Create,
    Read,
    Write,
    Sync,
    Rename,
    Remove,
}

impl std::fmt::Display for IoOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoOperation::Create => write!(f, "create"),
            IoOperation::Read => write!(f, "read"),
            IoOperation::Write => write!(f, "write"),
            IoOperation::Sync => write!(f, "sync"),
            IoOperation::Rename => write!(f, "rename"),
            IoOperation::Remove => write!(f, "remove"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Operational,
    Integrity,
    Fatal,
}

pub mod codes {
    pub const IO: u16 = 1000;
    pub const INVALID_INPUT: u16 = 1001;
    pub const SEGMENT_CORRUPTED: u16 = 2002;
    pub const SNAPSHOT_CORRUPTED: u16 = 2003;
    pub const INDEX_CORRUPTED: u16 = 2004;
    pub const SNAPSHOT_NOT_FOUND: u16 = 3001;
    pub const OFFSET_OUT_OF_RANGE: u16 = 3002;
    pub const SEGMENT_NOT_FOUND: u16 = 3003;
    pub const MANIFEST_ERROR: u16 = 4001;
    pub const CODEC: u16 = 5001;
    pub const REPLAY_FAILED: u16 = 5002;
}

#[derive(Debug, thiserror::Error)]
pub enum WalError {
    #[error("segment {segment_id} corrupted: {reason}")]
    SegmentCorrupted {
        segment_id: u64,
        reason: CorruptSegment,
    },
    #[error("snapshot corrupted at {path}: {reason}")]
    SnapshotCorrupted {
        path: PathBuf,
        reason: CorruptSnapshot,
    },
    #[error("index corrupted for segment {segment_id}: {reason}")]
    IndexCorrupted {
        segment_id: u64,
        reason: CorruptIndex,
    },
    #[error("no snapshot found: the WAL has never been checkpointed")]
    SnapshotNotFound,
    #[error("offset {requested} out of range (next available is {next})")]
    OffsetOutOfRange { requested: u64, next: u64 },
    #[error("segment {segment_id} not found")]
    SegmentNotFound { segment_id: u64 },
    #[error("manifest error: {reason}")]
    ManifestError { reason: CorruptManifest },
    #[error("{operation} {path}: {source}")]
    Io {
        path: PathBuf,
        operation: IoOperation,
        source: std::io::Error,
    },
    #[error("invalid input: {detail}")]
    InvalidInput { detail: &'static str },
    #[error("codec error: {detail}")]
    Codec { detail: String },
    #[error("replay failed at offset {offset}: {detail}")]
    ReplayFailed { offset: u64, detail: String },
}

impl WalError {
    pub fn at(path: PathBuf, operation: IoOperation) -> impl FnOnce(std::io::Error) -> Self {
        move |source| Self::Io {
            path,
            operation,
            source,
        }
    }

    pub fn code(&self) -> u16 {
        match self {
            Self::Io { .. } => codes::IO,
            Self::InvalidInput { .. } => codes::INVALID_INPUT,
            Self::SegmentCorrupted { .. } => codes::SEGMENT_CORRUPTED,
            Self::SnapshotCorrupted { .. } => codes::SNAPSHOT_CORRUPTED,
            Self::IndexCorrupted { .. } => codes::INDEX_CORRUPTED,
            Self::SnapshotNotFound => codes::SNAPSHOT_NOT_FOUND,
            Self::OffsetOutOfRange { .. } => codes::OFFSET_OUT_OF_RANGE,
            Self::SegmentNotFound { .. } => codes::SEGMENT_NOT_FOUND,
            Self::ManifestError { .. } => codes::MANIFEST_ERROR,
            Self::Codec { .. } => codes::CODEC,
            Self::ReplayFailed { .. } => codes::REPLAY_FAILED,
        }
    }

    pub fn severity(&self) -> Severity {
        match self {
            Self::SnapshotNotFound
            | Self::OffsetOutOfRange { .. }
            | Self::SegmentNotFound { .. } => Severity::Operational,
            Self::SegmentCorrupted { .. }
            | Self::SnapshotCorrupted { .. }
            | Self::IndexCorrupted { .. } => Severity::Integrity,
            Self::ManifestError { .. }
            | Self::Io { .. }
            | Self::InvalidInput { .. }
            | Self::Codec { .. }
            | Self::ReplayFailed { .. } => Severity::Fatal,
        }
    }

    pub fn hint(&self) -> &'static str {
        match self {
            Self::SegmentCorrupted { .. } => {
                "Segment file is damaged. Run `axiom recover` to repair or remove the affected segment."
            }
            Self::SnapshotCorrupted { .. } => {
                "Snapshot file is damaged. Delete the corrupted snapshot and recover from an earlier one."
            }
            Self::IndexCorrupted { .. } => {
                "Index file is damaged. Run `axiom recover` to rebuild the sparse index from the segment."
            }
            Self::SnapshotNotFound => {
                "No snapshot exists yet. The engine will replay the full WAL on startup."
            }
            Self::OffsetOutOfRange { .. } => {
                "The requested offset does not exist. Check that the offset is valid."
            }
            Self::SegmentNotFound { .. } => {
                "The requested segment does not exist. The segment ID may be incorrect."
            }
            Self::ManifestError { .. } => {
                "The manifest is corrupted or missing. Check the manifest file or restore from backup."
            }
            Self::Io { .. } => {
                "A file system operation failed. Check disk space, permissions, and hardware health."
            }
            Self::InvalidInput { .. } => {
                "The input is too large or malformed. Reduce the payload size and retry."
            }
            Self::Codec { .. } => {
                "Stored data could not be encoded or decoded. Check the storage format version."
            }
            Self::ReplayFailed { .. } => {
                "A recorded machine event could not be applied to the restored state."
            }
        }
    }
}

pub type WalResult<T> = std::result::Result<T, WalError>;
