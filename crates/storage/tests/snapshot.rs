use std::fs;
use std::path::Path;

use storage::error::{CorruptSnapshot, WalError};
use storage::manifest::Manifest;
use storage::snapshot;
use tempfile::TempDir;

fn make_manifest(dir: &Path, snap_offset: Option<u64>) -> Manifest {
    let mut m = Manifest::open(dir).unwrap();
    if let Some(offset) = snap_offset {
        m.set_snap_offset(offset).unwrap();
    }
    m
}

#[test]
fn save_and_read_round_trip() {
    let dir = TempDir::new().unwrap();
    let state = b"hello snapshot world";
    let offset = 450u64;

    snapshot::save(dir.path(), offset, state).unwrap();

    let path = snapshot::snap_path(dir.path(), offset);
    assert!(path.exists());

    let (read_offset, read_state) = snapshot::read(&path).unwrap();
    assert_eq!(read_offset, offset);
    assert_eq!(read_state, state);
}

#[test]
fn path_generation_bucket_0() {
    let dir = TempDir::new().unwrap();
    let path = snapshot::snap_path(dir.path(), 0);
    assert_eq!(path, dir.path().join("snapshots/0000-0999/ckpt-000000.dat"));
}

#[test]
fn path_generation_bucket_middle() {
    let dir = TempDir::new().unwrap();
    let path = snapshot::snap_path(dir.path(), 450);
    assert_eq!(path, dir.path().join("snapshots/0000-0999/ckpt-000450.dat"));
}

#[test]
fn path_generation_bucket_boundary() {
    let dir = TempDir::new().unwrap();
    let path = snapshot::snap_path(dir.path(), 1000);
    assert_eq!(path, dir.path().join("snapshots/1000-1999/ckpt-001000.dat"));
}

#[test]
fn path_generation_bucket_large() {
    let dir = TempDir::new().unwrap();
    let path = snapshot::snap_path(dir.path(), 500001);
    assert_eq!(
        path,
        dir.path().join("snapshots/500000-500999/ckpt-500001.dat")
    );
}

#[test]
fn load_with_manifest() {
    let dir = TempDir::new().unwrap();
    let manifest = make_manifest(dir.path(), None);

    let result = snapshot::load(dir.path(), &manifest).unwrap();
    assert!(result.is_none());

    snapshot::save(dir.path(), 200, b"state-v1").unwrap();
    let manifest = make_manifest(dir.path(), Some(200));

    let (offset, state) = snapshot::load(dir.path(), &manifest).unwrap().unwrap();
    assert_eq!(offset, 200);
    assert_eq!(state, b"state-v1");
}

#[test]
fn load_manifest_points_to_missing_file() {
    let dir = TempDir::new().unwrap();
    let manifest = make_manifest(dir.path(), Some(9999));

    let result = snapshot::load(dir.path(), &manifest);
    assert!(result.is_err());
    assert!(
        matches!(result.unwrap_err(), WalError::SnapshotNotFound),
        "expected SnapshotNotFound"
    );
}

#[test]
fn atomic_save_no_temp_file_left() {
    let dir = TempDir::new().unwrap();
    snapshot::save(dir.path(), 100, b"data").unwrap();

    let path = snapshot::snap_path(dir.path(), 100);
    assert!(path.exists());

    let temp_path = path.with_extension("dat.tmp");
    assert!(!temp_path.exists());
}

#[test]
fn save_creates_parent_directories() {
    let dir = TempDir::new().unwrap();
    let offset = 5000u64;

    snapshot::save(dir.path(), offset, b"deep write").unwrap();

    let path = snapshot::snap_path(dir.path(), offset);
    assert!(path.exists());
    assert!(dir.path().join("snapshots/5000-5999").exists());
}

#[test]
fn empty_payload_round_trip() {
    let dir = TempDir::new().unwrap();
    snapshot::save(dir.path(), 0, b"").unwrap();

    let path = snapshot::snap_path(dir.path(), 0);
    let (offset, state) = snapshot::read(&path).unwrap();
    assert_eq!(offset, 0);
    assert!(state.is_empty());
}

#[test]
fn large_payload_round_trip() {
    let dir = TempDir::new().unwrap();
    let state: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();

    snapshot::save(dir.path(), 9999, &state).unwrap();

    let path = snapshot::snap_path(dir.path(), 9999);
    let (offset, read_state) = snapshot::read(&path).unwrap();
    assert_eq!(offset, 9999);
    assert_eq!(read_state, state);
}

#[test]
fn bad_magic_rejected() {
    let dir = TempDir::new().unwrap();
    snapshot::save(dir.path(), 42, b"good data").unwrap();

    let path = snapshot::snap_path(dir.path(), 42);
    let mut data = fs::read(&path).unwrap();
    data[0] = b'X';
    fs::write(&path, &data).unwrap();

    let err = snapshot::read(&path).unwrap_err();
    match err {
        WalError::SnapshotCorrupted { reason, .. } => {
            assert!(matches!(reason, CorruptSnapshot::BadMagic { .. }));
        }
        other => panic!("expected SnapshotCorrupted, got {other}"),
    }
}

#[test]
fn bad_checksum_rejected() {
    let dir = TempDir::new().unwrap();
    snapshot::save(dir.path(), 42, b"good data").unwrap();

    let path = snapshot::snap_path(dir.path(), 42);
    let mut data = fs::read(&path).unwrap();
    let payload_start = 4096;
    data[payload_start] ^= 0xFF;
    fs::write(&path, &data).unwrap();

    let err = snapshot::read(&path).unwrap_err();
    match err {
        WalError::SnapshotCorrupted { reason, .. } => {
            assert!(matches!(reason, CorruptSnapshot::ChecksumMismatch { .. }));
        }
        other => panic!("expected SnapshotCorrupted with ChecksumMismatch, got {other}"),
    }
}

#[test]
fn truncated_header_rejected() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("snapshots/bad.dat");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, b"SNAP").unwrap();

    let err = snapshot::read(&path).unwrap_err();
    match err {
        WalError::SnapshotCorrupted { reason, .. } => {
            assert!(matches!(reason, CorruptSnapshot::HeaderTooShort { .. }));
        }
        other => panic!("expected SnapshotCorrupted, got {other}"),
    }
}

#[test]
fn unsupported_version_rejected() {
    let dir = TempDir::new().unwrap();
    snapshot::save(dir.path(), 10, b"data").unwrap();

    let path = snapshot::snap_path(dir.path(), 10);
    let mut data = fs::read(&path).unwrap();
    data[4..6].copy_from_slice(&99u16.to_le_bytes());
    fs::write(&path, &data).unwrap();

    let err = snapshot::read(&path).unwrap_err();
    match err {
        WalError::SnapshotCorrupted { reason, .. } => {
            assert!(matches!(reason, CorruptSnapshot::UnsupportedVersion { .. }));
        }
        other => panic!("expected SnapshotCorrupted, got {other}"),
    }
}

#[test]
fn truncated_payload_rejected() {
    let dir = TempDir::new().unwrap();
    snapshot::save(dir.path(), 10, b"some payload").unwrap();

    let path = snapshot::snap_path(dir.path(), 10);
    let data = fs::read(&path).unwrap();
    fs::write(&path, &data[..data.len() - 3]).unwrap();

    let err = snapshot::read(&path).unwrap_err();
    match err {
        WalError::SnapshotCorrupted { reason, .. } => {
            assert!(matches!(reason, CorruptSnapshot::PayloadTruncated { .. }));
        }
        other => panic!("expected SnapshotCorrupted, got {other}"),
    }
}

#[test]
fn multiple_snapshots_coexist() {
    let dir = TempDir::new().unwrap();
    snapshot::save(dir.path(), 100, b"first").unwrap();
    snapshot::save(dir.path(), 200, b"second").unwrap();
    snapshot::save(dir.path(), 1000, b"third").unwrap();

    let (off1, p1) = snapshot::read(&snapshot::snap_path(dir.path(), 100)).unwrap();
    let (off2, p2) = snapshot::read(&snapshot::snap_path(dir.path(), 200)).unwrap();
    let (off3, p3) = snapshot::read(&snapshot::snap_path(dir.path(), 1000)).unwrap();

    assert_eq!(off1, 100);
    assert_eq!(p1, b"first");
    assert_eq!(off2, 200);
    assert_eq!(p2, b"second");
    assert_eq!(off3, 1000);
    assert_eq!(p3, b"third");
}

#[test]
fn overwrite_existing_snapshot() {
    let dir = TempDir::new().unwrap();
    snapshot::save(dir.path(), 50, b"old").unwrap();
    snapshot::save(dir.path(), 50, b"new").unwrap();

    let (offset, state) = snapshot::read(&snapshot::snap_path(dir.path(), 50)).unwrap();
    assert_eq!(offset, 50);
    assert_eq!(state, b"new");
}
