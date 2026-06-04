use std::fs;
use std::io::Write;

use storage::error::{CorruptIndex, WalError};
use storage::index::SparseIndex;
use tempfile::TempDir;

#[test]
fn new_creates_empty_index() {
    let idx = SparseIndex::new(42, 256);
    assert_eq!(idx.seg_id, 42);
    assert_eq!(idx.interval, 256);
    assert_eq!(idx.items.len(), 0);
    assert!(idx.items.is_empty());
    assert_eq!(idx.count, 0);
}

#[test]
fn insert_adds_on_interval_boundary() {
    let mut idx = SparseIndex::new(1, 256);
    idx.insert(100, 0);
    assert_eq!(idx.items.len(), 1);
    for i in 1..256 {
        idx.insert(100 + i, 1024 + i * 4);
    }
    idx.insert(356, 2048);
    assert_eq!(idx.items.len(), 2);
    for i in 1..256 {
        idx.insert(356 + i, 3000 + i * 4);
    }
    idx.insert(612, 4096);
    assert_eq!(idx.items.len(), 3);
    assert_eq!(idx.count, 513);
}

#[test]
fn insert_skips_non_boundary() {
    let mut idx = SparseIndex::new(1, 256);
    idx.insert(1, 100);
    idx.insert(255, 200);
    idx.insert(257, 300);
    idx.insert(500, 400);
    assert_eq!(idx.items.len(), 1);
    assert_eq!(idx.count, 4);
}

#[test]
fn insert_interval_of_one_always_records() {
    let mut idx = SparseIndex::new(1, 1);
    idx.insert(0, 0);
    idx.insert(1, 10);
    idx.insert(2, 20);
    assert_eq!(idx.items.len(), 3);
    assert_eq!(idx.count, 3);
}

#[test]
fn insert_zero_interval_always_records() {
    let mut idx = SparseIndex::new(1, 0);
    idx.insert(7, 100);
    idx.insert(99, 200);
    assert_eq!(idx.items.len(), 2);
    assert_eq!(idx.count, 2);
}

#[test]
fn insert_mixed_sequence() {
    let mut idx = SparseIndex::new(1, 4);
    idx.insert(10, 0);
    idx.insert(11, 10);
    idx.insert(12, 20);
    idx.insert(13, 30);
    idx.insert(14, 40);
    idx.insert(15, 50);
    idx.insert(16, 60);
    assert_eq!(idx.items.len(), 2);
    assert_eq!(idx.count, 7);
}

#[test]
fn set_count_resumes_counting() {
    let mut idx = SparseIndex::new(1, 4);
    idx.set_count(3);
    idx.insert(50, 500);
    assert_eq!(idx.items.len(), 0);
    idx.insert(51, 510);
    assert_eq!(idx.count, 5);
    assert_eq!(idx.items.len(), 1);
}

#[test]
fn find_returns_none_for_empty_index() {
    let idx = SparseIndex::new(1, 256);
    assert!(idx.find(0).is_none());
    assert!(idx.find(1000).is_none());
}

#[test]
fn find_returns_exact_match() {
    let mut idx = SparseIndex::new(1, 1);
    idx.insert(0, 0);
    idx.insert(4, 40);
    idx.insert(8, 80);

    assert_eq!(idx.find(0), Some((0, 0)));
    assert_eq!(idx.find(4), Some((4, 40)));
    assert_eq!(idx.find(8), Some((8, 80)));
}

#[test]
fn find_returns_closest_entry_before_target() {
    let mut idx = SparseIndex::new(1, 1);
    idx.insert(0, 0);
    idx.insert(256, 1024);
    idx.insert(512, 2048);

    assert_eq!(idx.find(300), Some((256, 1024)));
    assert_eq!(idx.find(511), Some((256, 1024)));
    assert_eq!(idx.find(1000), Some((512, 2048)));
}

#[test]
fn find_returns_none_when_target_before_first_entry() {
    let mut idx = SparseIndex::new(1, 1);
    idx.insert(256, 1024);
    assert!(idx.find(100).is_none());
    assert!(idx.find(255).is_none());
}

#[test]
fn find_with_single_entry() {
    let mut idx = SparseIndex::new(1, 1);
    idx.insert(256, 1024);
    assert_eq!(idx.find(256), Some((256, 1024)));
    assert_eq!(idx.find(500), Some((256, 1024)));
    assert!(idx.find(100).is_none());
}

#[test]
fn save_load_round_trip() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("000001.idx");

    let mut idx = SparseIndex::new(7, 256);
    idx.insert(0, 0);
    for i in 1..256 {
        idx.insert(i, i * 16);
    }
    idx.insert(256, 4096);
    for i in 1..256 {
        idx.insert(256 + i, 4096 + i * 16);
    }
    idx.insert(512, 8192);
    assert_eq!(idx.count, 513);

    idx.save(&path).unwrap();
    let loaded = SparseIndex::load(&path).unwrap();

    assert_eq!(loaded.seg_id, 7);
    assert_eq!(loaded.interval, 256);
    assert_eq!(loaded.items.len(), 3);
    assert_eq!(loaded.items[0], (0, 0));
    assert_eq!(loaded.items[1], (256, 4096));
    assert_eq!(loaded.items[2], (512, 8192));
    assert_eq!(loaded.count, 513);
}

#[test]
fn save_load_empty_index() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("000002.idx");

    let idx = SparseIndex::new(99, 128);
    idx.save(&path).unwrap();
    let loaded = SparseIndex::load(&path).unwrap();

    assert_eq!(loaded.seg_id, 99);
    assert_eq!(loaded.interval, 128);
    assert_eq!(loaded.items.len(), 0);
    assert!(loaded.items.is_empty());
    assert_eq!(loaded.count, 0);
}

#[test]
fn save_load_large_entries() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("000003.idx");

    let mut idx = SparseIndex::new(1, 10);
    for i in 0..10_000u64 {
        idx.insert(i, (i / 10) * 64);
    }

    assert_eq!(idx.items.len(), 1000);
    idx.save(&path).unwrap();
    let loaded = SparseIndex::load(&path).unwrap();

    assert_eq!(loaded.items.len(), 1000);
    assert_eq!(loaded.items[0], (0, 0));
    assert_eq!(loaded.items[999], (9990, 999 * 64));

    assert_eq!(loaded.find(5000), Some((5000, 500 * 64)));
    assert_eq!(loaded.find(5005), Some((5000, 500 * 64)));
}

#[test]
fn load_rejects_bad_magic() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("bad.idx");

    let mut file = fs::File::create(&path).unwrap();
    let mut header = vec![0u8; 4096];
    header[0..4].copy_from_slice(b"XXXX");
    file.write_all(&header).unwrap();

    let result = SparseIndex::load(&path);
    assert!(result.is_err());
    assert!(
        matches!(
            result.unwrap_err(),
            WalError::IndexCorrupted {
                reason: CorruptIndex::BadMagic { .. },
                ..
            }
        ),
        "expected BadMagic error"
    );
}

#[test]
fn load_rejects_truncated_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("short.idx");

    fs::write(&path, b"SIDX").unwrap();

    let result = SparseIndex::load(&path);
    assert!(result.is_err());
    assert!(
        matches!(
            result.unwrap_err(),
            WalError::IndexCorrupted {
                reason: CorruptIndex::FileTooSmall { .. },
                ..
            }
        ),
        "expected FileTooSmall error"
    );
}

#[test]
fn load_rejects_corrupted_entries() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("corrupt.idx");

    let mut idx = SparseIndex::new(1, 1);
    idx.insert(0, 0);
    idx.insert(256, 1024);
    idx.save(&path).unwrap();

    let mut data = fs::read(&path).unwrap();
    let entry_offset = 4096 + 4;
    data[entry_offset] ^= 0xFF;
    fs::write(&path, &data).unwrap();

    let result = SparseIndex::load(&path);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("checksum") || err.contains("mismatch"),
        "unexpected error: {err}"
    );
}

#[test]
fn load_rejects_truncated_entries() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("trunc.idx");

    let mut idx = SparseIndex::new(1, 1);
    idx.insert(0, 0);
    idx.insert(256, 1024);
    idx.save(&path).unwrap();

    let mut data = fs::read(&path).unwrap();
    data.truncate(4096 + 16);
    fs::write(&path, &data).unwrap();

    let result = SparseIndex::load(&path);
    assert!(result.is_err());
    assert!(
        matches!(
            result.unwrap_err(),
            WalError::IndexCorrupted {
                reason: CorruptIndex::ItemTruncated { .. },
                ..
            }
        ),
        "expected ItemTruncated error"
    );
}

#[test]
fn simulate_segment_indexing() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("000010.idx");

    let mut idx = SparseIndex::new(10, 256);
    let mut byte_off: u64 = 0;

    for call in 0u64..1024 {
        let offset = call + 500;
        idx.insert(offset, byte_off);
        byte_off += 48;
    }

    assert_eq!(idx.items.len(), 4);
    assert_eq!(idx.count, 1024);

    idx.save(&path).unwrap();
    let loaded = SparseIndex::load(&path).unwrap();
    assert_eq!(loaded.items.len(), 4);
    assert_eq!(loaded.count, 4 * 256);

    assert_eq!(loaded.find(500), Some((500, 0)));
    assert_eq!(loaded.find(800), Some((756, 256 * 48)));
    assert_eq!(loaded.find(1100), Some((1012, 512 * 48)));
    assert_eq!(loaded.find(1400), Some((1268, 768 * 48)));
    assert_eq!(loaded.find(1523), Some((1268, 768 * 48)));
}
