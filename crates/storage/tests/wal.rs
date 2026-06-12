use std::path::Path;

use storage::{Wal, WalError};
use tempfile::TempDir;

fn patch_segment_size(dir: &Path, size: u64) {
    let manifest_path = dir.join("events/manifest.dat");
    let content = std::fs::read_to_string(&manifest_path).unwrap();
    let patched = content.replace("segment_size=134217728", &format!("segment_size={size}"));
    std::fs::write(&manifest_path, patched).unwrap();
}

fn count_files_with_ext(dir: &Path, ext: &str) -> usize {
    let mut count = 0;
    if !dir.exists() {
        return 0;
    }
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            count += count_files_with_ext(&path, ext);
        } else if path.extension().and_then(|s| s.to_str()) == Some(ext) {
            count += 1;
        }
    }
    count
}

#[test]
fn open_creates_new_workspace() {
    let dir = TempDir::new().unwrap();
    let wal = Wal::open(dir.path()).unwrap();
    assert_eq!(wal.next_offset(), 0);
}

#[test]
fn append_returns_correct_offset() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    let (off0, _) = wal.append(b"hello").unwrap();
    let (off1, _) = wal.append(b"world").unwrap();
    assert_eq!(off0, 0);
    assert_eq!(off1, 1);
    assert_eq!(wal.next_offset(), 2);
}

#[test]
fn checkpoint_and_load() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    wal.append(b"e1").unwrap();
    wal.append(b"e2").unwrap();
    wal.checkpoint(2, b"state-at-2").unwrap();

    let (offset, state) = wal.load().unwrap().unwrap();
    assert_eq!(offset, 2);
    assert_eq!(state, b"state-at-2");
}

#[test]
fn checkpoint_trims_segments_covered_by_snapshot() {
    let dir = TempDir::new().unwrap();
    Wal::open(dir.path()).unwrap();
    patch_segment_size(dir.path(), 4196);

    let mut wal = Wal::open(dir.path()).unwrap();
    let payload = vec![0u8; 60];
    for _ in 0..5 {
        wal.append(&payload).unwrap();
    }

    let seg_dir = dir.path().join("events/segments/0000-0999");
    assert!(count_files_with_ext(&seg_dir, "seg") > 1);

    wal.checkpoint(5, b"state-at-5").unwrap();

    assert!(!seg_dir.join("000001.seg").exists());
    assert!(!seg_dir.join("000001.idx").exists());
    let err = match wal.replay(0) {
        Ok(_) => panic!("expected compacted replay offset to fail"),
        Err(err) => err,
    };
    match err {
        WalError::OffsetOutOfRange { requested, next } => {
            assert_eq!(requested, 0);
            assert_eq!(next, 5);
        }
        other => panic!("expected OffsetOutOfRange, got: {other}"),
    }

    let items: Vec<_> = wal
        .replay(5)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert!(items.is_empty());

    drop(wal);
    let wal = Wal::open(dir.path()).unwrap();
    let (offset, state) = wal.load().unwrap().unwrap();
    assert_eq!(offset, 5);
    assert_eq!(state, b"state-at-5");
}

#[test]
fn checkpoint_keeps_only_two_latest_snapshots() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    for i in 0..4 {
        wal.append(format!("e{i}").as_bytes()).unwrap();
        wal.checkpoint(i + 1, format!("state-{i}").as_bytes())
            .unwrap();
    }

    let snapshot_dir = dir.path().join("snapshots");
    assert_eq!(count_files_with_ext(&snapshot_dir, "dat"), 2);
    assert!(!snapshot_dir.join("0000-0999/ckpt-000001.dat").exists());
    assert!(!snapshot_dir.join("0000-0999/ckpt-000002.dat").exists());
    assert!(snapshot_dir.join("0000-0999/ckpt-000003.dat").exists());
    assert!(snapshot_dir.join("0000-0999/ckpt-000004.dat").exists());

    let (offset, state) = wal.load().unwrap().unwrap();
    assert_eq!(offset, 4);
    assert_eq!(state, b"state-3");
}

#[test]
fn no_checkpoint_returns_none() {
    let dir = TempDir::new().unwrap();
    let wal = Wal::open(dir.path()).unwrap();
    assert!(wal.load().unwrap().is_none());
}

#[test]
fn reopen_preserves_data() {
    let dir = TempDir::new().unwrap();

    {
        let mut wal = Wal::open(dir.path()).unwrap();
        wal.append(b"before close").unwrap();
    }

    let wal = Wal::open(dir.path()).unwrap();
    assert_eq!(wal.next_offset(), 1);
}

#[test]
fn segment_roll() {
    let dir = TempDir::new().unwrap();

    // Create workspace with default segment size, then patch manifest.
    {
        let wal = Wal::open(dir.path()).unwrap();
        drop(wal);
    }

    // Patch manifest on disk to use a tiny segment size.
    let manifest_path = dir.path().join("events/manifest.dat");
    let content = std::fs::read_to_string(&manifest_path).unwrap();
    let patched = content.replace("segment_size=134217728", "segment_size=4196");
    std::fs::write(&manifest_path, patched).unwrap();

    let mut wal = Wal::open(dir.path()).unwrap();
    let payload = vec![0u8; 60];
    for _ in 0..5 {
        wal.append(&payload).unwrap();
    }

    // After 5 appends with tiny segments, at least one roll should have happened.
    // next_offset should be 5.
    assert_eq!(wal.next_offset(), 5);
}

#[test]
fn recovery_with_snapshot() {
    let dir = TempDir::new().unwrap();

    {
        let mut wal = Wal::open(dir.path()).unwrap();
        wal.append(b"e0").unwrap();
        wal.append(b"e1").unwrap();
        wal.checkpoint(2, b"state-at-2").unwrap();
        wal.append(b"e2").unwrap();
        wal.append(b"e3").unwrap();
    }

    let wal = Wal::open(dir.path()).unwrap();

    let (snap_offset, snap_state) = wal.load().unwrap().unwrap();
    assert_eq!(snap_offset, 2);
    assert_eq!(snap_state, b"state-at-2");
}

#[test]
fn last_offset() {
    let dir = TempDir::new().unwrap();
    let wal = Wal::open(dir.path()).unwrap();
    assert_eq!(wal.last_offset(), None);

    let mut wal = Wal::open(dir.path()).unwrap();
    wal.append(b"x").unwrap();
    assert_eq!(wal.last_offset(), Some(0));
    assert_eq!(wal.next_offset(), 1);
}

#[test]
fn next_offset() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    assert_eq!(wal.next_offset(), 0);
    wal.append(b"a").unwrap();
    assert_eq!(wal.next_offset(), 1);
    wal.append(b"b").unwrap();
    assert_eq!(wal.next_offset(), 2);
}

// -- replay tests --

#[test]
fn replay_from_zero() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    wal.append(b"a").unwrap();
    wal.append(b"b").unwrap();
    wal.append(b"c").unwrap();

    let items: Vec<_> = wal
        .replay(0)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(items.len(), 3);
    assert_eq!(items[0], (0, b"a".to_vec()));
    assert_eq!(items[1], (1, b"b".to_vec()));
    assert_eq!(items[2], (2, b"c".to_vec()));
}

#[test]
fn replay_from_middle() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    wal.append(b"a").unwrap();
    wal.append(b"b").unwrap();
    wal.append(b"c").unwrap();

    let items: Vec<_> = wal
        .replay(1)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(items.len(), 2);
    assert_eq!(items[0], (1, b"b".to_vec()));
    assert_eq!(items[1], (2, b"c".to_vec()));
}

#[test]
fn replay_across_segments() {
    let dir = TempDir::new().unwrap();

    // Create workspace with tiny segments to force rolls.
    {
        let wal = Wal::open(dir.path()).unwrap();
        drop(wal);
    }

    let manifest_path = dir.path().join("events/manifest.dat");
    let content = std::fs::read_to_string(&manifest_path).unwrap();
    let patched = content.replace("segment_size=134217728", "segment_size=4196");
    std::fs::write(&manifest_path, patched).unwrap();

    let mut wal = Wal::open(dir.path()).unwrap();
    let payload = vec![0u8; 60];
    for _ in 0..5 {
        wal.append(&payload).unwrap();
    }

    let items: Vec<_> = wal
        .replay(0)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(items.len(), 5);
    for (i, (offset, data)) in items.iter().enumerate() {
        assert_eq!(*offset, i as u64);
        assert_eq!(data, &payload);
    }
}

#[test]
fn replay_with_snapshot_recovery() {
    let dir = TempDir::new().unwrap();

    {
        let mut wal = Wal::open(dir.path()).unwrap();
        wal.append(b"e0").unwrap();
        wal.append(b"e1").unwrap();
        wal.checkpoint(2, b"state-at-2").unwrap();
        wal.append(b"e2").unwrap();
        wal.append(b"e3").unwrap();
    }

    let wal = Wal::open(dir.path()).unwrap();

    let (snap_offset, snap_state) = wal.load().unwrap().unwrap();
    assert_eq!(snap_offset, 2);
    assert_eq!(snap_state, b"state-at-2");

    // Replay from the snapshot offset to recover post-snapshot items.
    let items: Vec<_> = wal
        .replay(snap_offset)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0], (2, b"e2".to_vec()));
    assert_eq!(items[1], (3, b"e3".to_vec()));
}

#[test]
fn replay_empty_wal() {
    let dir = TempDir::new().unwrap();
    let wal = Wal::open(dir.path()).unwrap();

    let items: Vec<_> = wal
        .replay(0)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert!(items.is_empty());
}

// -- checkpoint monotonicity --

#[test]
fn checkpoint_offset_must_not_decrease() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    wal.append(b"e0").unwrap();
    wal.append(b"e1").unwrap();
    wal.checkpoint(2, b"state-v1").unwrap();

    // Checkpoint at a lower offset should fail.
    let err = wal.checkpoint(1, b"state-v0").unwrap_err();
    assert!(
        err.to_string().contains("must not decrease"),
        "expected monotonicity error, got: {err}"
    );
}

#[test]
fn checkpoint_same_offset_is_ok() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    wal.append(b"e0").unwrap();
    wal.checkpoint(1, b"state-v1").unwrap();

    // Re-checkpoint at the same offset is allowed (overwrite).
    wal.checkpoint(1, b"state-v1-updated").unwrap();

    let (_, state) = wal.load().unwrap().unwrap();
    assert_eq!(state, b"state-v1-updated");
}

// -- crash safety / recovery --

#[test]
fn missing_segment_returns_error() {
    let dir = TempDir::new().unwrap();

    // Create a workspace with data.
    {
        let mut wal = Wal::open(dir.path()).unwrap();
        wal.append(b"data").unwrap();
    }

    // Delete the segment file to simulate a crash that lost it.
    let seg_path = dir
        .path()
        .join("events")
        .join("segments")
        .join("0000-0999")
        .join("000001.seg");
    assert!(seg_path.exists());
    std::fs::remove_file(&seg_path).unwrap();

    let result = Wal::open(dir.path());
    let err = result.err().expect("expected SegmentNotFound error");
    match err {
        WalError::SegmentNotFound { segment_id } => assert_eq!(segment_id, 1),
        other => panic!("expected SegmentNotFound, got: {other}"),
    }
}

#[test]
fn recovery_from_already_closed_segment() {
    let dir = TempDir::new().unwrap();

    // Simulate the crash scenario: manifest points at segment N,
    // but segment N is already closed (has TAIL_MAGIC).
    // This happens when rotate() committed the manifest but
    // crashed before fully completing.
    {
        let mut wal = Wal::open(dir.path()).unwrap();
        wal.append(b"item-0").unwrap();
        wal.append(b"item-1").unwrap();

        // Force a close of the segment by dropping, then manually
        // add TAIL_MAGIC + update count to simulate a closed segment.
        // Actually, let's use the real rotate path with tiny segments.
    }

    // Use tiny segments to force rotation.
    let manifest_path = dir.path().join("events/manifest.dat");
    let content = std::fs::read_to_string(&manifest_path).unwrap();
    let patched = content.replace("segment_size=134217728", "segment_size=4196");
    std::fs::write(&manifest_path, patched).unwrap();

    {
        let mut wal = Wal::open(dir.path()).unwrap();
        let payload = vec![0u8; 60];
        // Force at least one rotation.
        for _ in 0..5 {
            wal.append(&payload).unwrap();
        }
        // Now segment 1 is closed and segment 2 is active.
        // Simulate: crash after manifest committed to seg 2 but
        // before the old segment's index was saved.
        // (This is already the normal state after rotate.)
    }

    // Reopening should work — segment 1 is a closed segment,
    // segment 2 is the active one.
    let wal = Wal::open(dir.path()).unwrap();
    assert_eq!(wal.next_offset(), 7); // 2 from seg 1 + 5 from seg 2
}

#[test]
fn checkpoint_future_offset_rejected() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    wal.append(b"e0").unwrap();
    // next_offset is 1; checkpoint at 5 should fail.
    let err = wal.checkpoint(5, b"future-state").unwrap_err();
    match err {
        WalError::OffsetOutOfRange { requested, next } => {
            assert_eq!(requested, 5);
            assert_eq!(next, 1);
        }
        other => panic!("expected OffsetOutOfRange, got: {other}"),
    }
}

#[test]
fn load_missing_snapshot_returns_error() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    wal.append(b"e0").unwrap();
    wal.checkpoint(1, b"state").unwrap();

    // Delete the snapshot file to simulate data loss.
    let snap_path = dir
        .path()
        .join("snapshots")
        .join("0000-0999")
        .join("ckpt-000001.dat");
    assert!(snap_path.exists());
    std::fs::remove_file(&snap_path).unwrap();

    // Re-open: manifest says snapshot at offset 1 but file is missing.
    let wal = Wal::open(dir.path()).unwrap();
    let result = wal.load();
    assert!(result.is_err());
    match result.unwrap_err() {
        WalError::SnapshotNotFound => {}
        other => panic!("expected SnapshotNotFound, got: {other}"),
    }
}

#[test]
fn open_creates_dir_fsync() {
    // Smoke test: open creates workspace and all dir fsyncs succeed.
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();
    wal.append(b"data").unwrap();
    wal.checkpoint(1, b"state").unwrap();
    drop(wal);

    // Reopen to verify durability.
    let wal = Wal::open(dir.path()).unwrap();
    assert_eq!(wal.next_offset(), 1);
    assert_eq!(wal.last_offset(), Some(0));
}

// -- append_batch tests --

#[test]
fn append_batch_basic() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    let results = wal.append_batch(&[b"one", b"two", b"three"]).unwrap();
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].0, 0);
    assert_eq!(results[1].0, 1);
    assert_eq!(results[2].0, 2);
    assert_eq!(wal.next_offset(), 3);

    let items: Vec<_> = wal
        .replay(0)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(items.len(), 3);
    assert_eq!(items[0], (0, b"one".to_vec()));
    assert_eq!(items[1], (1, b"two".to_vec()));
    assert_eq!(items[2], (2, b"three".to_vec()));
}

#[test]
fn append_batch_empty() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    let results = wal.append_batch(&[]).unwrap();
    assert!(results.is_empty());
    assert_eq!(wal.next_offset(), 0);
}

#[test]
fn append_batch_persists_across_reopen() {
    let dir = TempDir::new().unwrap();

    {
        let mut wal = Wal::open(dir.path()).unwrap();
        wal.append_batch(&[b"a", b"b", b"c", b"d"]).unwrap();
    }

    let wal = Wal::open(dir.path()).unwrap();
    assert_eq!(wal.next_offset(), 4);

    let items: Vec<_> = wal
        .replay(0)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(items.len(), 4);
}

#[test]
fn append_batch_interleaved_with_append() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    wal.append(b"solo1").unwrap();
    wal.append_batch(&[b"batch1", b"batch2"]).unwrap();
    wal.append(b"solo2").unwrap();

    let items: Vec<_> = wal
        .replay(0)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(items.len(), 4);
    assert_eq!(items[0].1, b"solo1");
    assert_eq!(items[1].1, b"batch1");
    assert_eq!(items[2].1, b"batch2");
    assert_eq!(items[3].1, b"solo2");
}

// -- append_unsynced + sync tests --

#[test]
fn append_unsynced_then_sync() {
    let dir = TempDir::new().unwrap();
    let mut wal = Wal::open(dir.path()).unwrap();

    wal.append_unsynced(b"unsynced1").unwrap();
    wal.append_unsynced(b"unsynced2").unwrap();
    wal.sync().unwrap();

    let items: Vec<_> = wal
        .replay(0)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].1, b"unsynced1");
    assert_eq!(items[1].1, b"unsynced2");
}

#[test]
fn append_unsynced_persists_after_sync_and_reopen() {
    let dir = TempDir::new().unwrap();

    {
        let mut wal = Wal::open(dir.path()).unwrap();
        wal.append_unsynced(b"x").unwrap();
        wal.append_unsynced(b"y").unwrap();
        wal.sync().unwrap();
    }

    let wal = Wal::open(dir.path()).unwrap();
    assert_eq!(wal.next_offset(), 2);
}
