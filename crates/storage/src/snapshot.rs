use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::error::{CorruptSnapshot, IoOperation, WalError, WalResult};
use crate::io;
use crate::manifest::Manifest;

const MAGIC: &[u8; 4] = b"SNAP";
const VERSION: u16 = 1;
const HEADER_SIZE: usize = 4096;

pub fn save(dir: &Path, offset: u64, state: &[u8]) -> WalResult<()> {
    tracing::debug!(offset, size = state.len(), "saving snapshot");
    let final_path = snap_path(dir, offset);

    if let Some(parent) = final_path.parent() {
        fs::create_dir_all(parent)
            .map_err(WalError::at(final_path.clone(), IoOperation::Create))?;
    }

    let temp_path = final_path.with_extension("dat.tmp");

    let mut file = fs::File::create(&temp_path)
        .map_err(WalError::at(temp_path.clone(), IoOperation::Create))?;

    let checksum = crc32c::crc32c(state);
    let mut header = [0u8; HEADER_SIZE];
    header[0..4].copy_from_slice(MAGIC);
    header[4..6].copy_from_slice(&VERSION.to_le_bytes());
    header[6..14].copy_from_slice(&offset.to_le_bytes());
    header[14..22].copy_from_slice(&(state.len() as u64).to_le_bytes());
    header[22..26].copy_from_slice(&checksum.to_le_bytes());

    file.write_all(&header)
        .map_err(WalError::at(temp_path.clone(), IoOperation::Write))?;
    file.write_all(state)
        .map_err(WalError::at(temp_path.clone(), IoOperation::Write))?;
    file.sync_all()
        .map_err(WalError::at(temp_path.clone(), IoOperation::Sync))?;

    fs::rename(&temp_path, &final_path)
        .map_err(WalError::at(final_path.clone(), IoOperation::Rename))?;

    if let Some(parent) = final_path.parent() {
        io::fsync_dir(parent)?;
    }
    Ok(())
}

pub fn retain_latest(dir: &Path, keep: usize) -> WalResult<()> {
    let mut snapshots = list_snapshots(dir)?;
    if snapshots.len() <= keep {
        return Ok(());
    }

    snapshots.sort_by_key(|(offset, _)| *offset);
    let remove_count = snapshots.len() - keep;
    for (_, path) in snapshots.into_iter().take(remove_count) {
        fs::remove_file(&path).map_err(WalError::at(path.clone(), IoOperation::Remove))?;
        if let Some(parent) = path.parent() {
            io::fsync_dir(parent)?;
            remove_empty_dir(parent)?;
        }
    }
    Ok(())
}

pub fn load(dir: &Path, manifest: &Manifest) -> WalResult<Option<(u64, Vec<u8>)>> {
    match manifest.snap_offset() {
        Some(offset) => {
            tracing::debug!(offset, "loading snapshot");
            let path = snap_path(dir, offset);
            if !path.exists() {
                return Err(WalError::SnapshotNotFound);
            }
            let (event_offset, payload) = read(&path)?;
            Ok(Some((event_offset, payload)))
        }
        None => {
            tracing::debug!("no snapshot to load");
            Ok(None)
        }
    }
}

pub fn read(path: &Path) -> WalResult<(u64, Vec<u8>)> {
    let mut file =
        fs::File::open(path).map_err(WalError::at(path.to_path_buf(), IoOperation::Read))?;

    let mut header = [0u8; HEADER_SIZE];
    file.read_exact(&mut header)
        .map_err(|e| WalError::SnapshotCorrupted {
            path: path.to_path_buf(),
            reason: CorruptSnapshot::HeaderTooShort { source: e },
        })?;

    if &header[0..4] != MAGIC {
        return Err(WalError::SnapshotCorrupted {
            path: path.to_path_buf(),
            reason: CorruptSnapshot::BadMagic {
                expected: *MAGIC,
                actual: header[0..4].try_into().unwrap(),
            },
        });
    }

    let version = u16::from_le_bytes([header[4], header[5]]);
    if version != VERSION {
        return Err(WalError::SnapshotCorrupted {
            path: path.to_path_buf(),
            reason: CorruptSnapshot::UnsupportedVersion { version },
        });
    }

    let offset = u64::from_le_bytes(header[6..14].try_into().unwrap());
    let payload_len = u64::from_le_bytes(header[14..22].try_into().unwrap());
    let expected_crc = u32::from_le_bytes(header[22..26].try_into().unwrap());

    let mut payload = vec![0u8; payload_len as usize];
    file.read_exact(&mut payload)
        .map_err(|e| WalError::SnapshotCorrupted {
            path: path.to_path_buf(),
            reason: CorruptSnapshot::PayloadTruncated { source: e },
        })?;

    let actual_crc = crc32c::crc32c(&payload);
    if actual_crc != expected_crc {
        return Err(WalError::SnapshotCorrupted {
            path: path.to_path_buf(),
            reason: CorruptSnapshot::ChecksumMismatch {
                expected: expected_crc,
                actual: actual_crc,
            },
        });
    }

    Ok((offset, payload))
}

pub fn snap_path(dir: &Path, offset: u64) -> PathBuf {
    let bucket = (offset / 1000) * 1000;
    let bucket_dir = format!("{:04}-{:04}", bucket, bucket + 999);
    dir.join("snapshots")
        .join(bucket_dir)
        .join(format!("ckpt-{:06}.dat", offset))
}

pub fn list_snapshots(dir: &Path) -> WalResult<Vec<(u64, PathBuf)>> {
    let snapshots_dir = dir.join("snapshots");
    if !snapshots_dir.exists() {
        return Ok(Vec::new());
    }

    let mut snapshots = Vec::new();
    let buckets = fs::read_dir(&snapshots_dir)
        .map_err(WalError::at(snapshots_dir.clone(), IoOperation::Read))?;
    for bucket in buckets {
        let bucket = bucket.map_err(WalError::at(snapshots_dir.clone(), IoOperation::Read))?;
        let bucket_path = bucket.path();
        if !bucket_path.is_dir() {
            continue;
        }

        let files = fs::read_dir(&bucket_path)
            .map_err(WalError::at(bucket_path.clone(), IoOperation::Read))?;
        for file in files {
            let file = file.map_err(WalError::at(bucket_path.clone(), IoOperation::Read))?;
            let path = file.path();
            if !path.is_file() {
                continue;
            }
            if let Some(offset) = parse_snapshot_offset(&path) {
                snapshots.push((offset, path));
            }
        }
    }
    Ok(snapshots)
}

fn parse_snapshot_offset(path: &Path) -> Option<u64> {
    let filename = path.file_name()?.to_str()?;
    let offset = filename.strip_prefix("ckpt-")?.strip_suffix(".dat")?;
    offset.parse().ok()
}

pub fn remove_empty_dir(path: &Path) -> WalResult<()> {
    match fs::remove_dir(path) {
        Ok(()) => Ok(()),
        Err(e)
            if matches!(
                e.kind(),
                std::io::ErrorKind::NotFound | std::io::ErrorKind::DirectoryNotEmpty
            ) =>
        {
            Ok(())
        }
        Err(e) => Err(WalError::at(path.to_path_buf(), IoOperation::Remove)(e)),
    }
}
