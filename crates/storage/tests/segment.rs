use storage::error::{CorruptSegment, WalError, WalResult};
use storage::segment::{HEADER_MAGIC, HEADER_SIZE, SegmentReader, SegmentWriter, TAIL_MAGIC};
use tempfile::TempDir;

fn seg_path(dir: &std::path::Path, id: u64) -> std::path::PathBuf {
    dir.join(format!("{:06}.seg", id))
}

fn make_writer(dir: &std::path::Path, seg_id: u64, offset: u64) -> SegmentWriter {
    SegmentWriter::new(dir, seg_id, offset, 64 * 1024 * 1024)
        .expect("failed to create SegmentWriter")
}

#[test]
fn header_magic_and_version() {
    let dir = TempDir::new().unwrap();
    let writer = make_writer(dir.path(), 1, 100);
    let path = seg_path(dir.path(), 1);

    let data = std::fs::read(&path).unwrap();
    assert_eq!(&data[0..4], &HEADER_MAGIC);
    assert_eq!(u16::from_le_bytes([data[4], data[5]]), 1);
    assert_eq!(u64::from_le_bytes(data[8..16].try_into().unwrap()), 1);
    assert_eq!(u64::from_le_bytes(data[16..24].try_into().unwrap()), 100);
    assert_eq!(u32::from_le_bytes(data[24..28].try_into().unwrap()), 0);
    drop(writer);
}

#[test]
fn header_is_4096_bytes() {
    let dir = TempDir::new().unwrap();
    let writer = make_writer(dir.path(), 1, 0);
    let path = seg_path(dir.path(), 1);
    let data = std::fs::read(&path).unwrap();
    assert_eq!(data.len(), HEADER_SIZE as usize);
    drop(writer);
}

#[test]
fn append_returns_correct_offset_and_position() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 5, 1000);

    let (off0, pos0) = writer.append(b"hello").unwrap();
    assert_eq!(off0, 1000);
    assert_eq!(pos0, HEADER_SIZE);

    let (off1, pos1) = writer.append(b"world").unwrap();
    assert_eq!(off1, 1001);
    assert_eq!(pos1, HEADER_SIZE + 8 + 5);
}

#[test]
fn iter_after_close() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 0);

    let (_, pos0) = writer.append(b"alpha").unwrap();
    let (_, pos1) = writer.append(b"beta").unwrap();
    writer.close().unwrap();

    let reader = SegmentReader::open(&seg_path(dir.path(), 1)).unwrap();
    let items: Vec<_> = reader.iter().collect::<WalResult<Vec<_>>>().unwrap();

    assert_eq!(items.len(), 2);
    assert_eq!(items[0].0, pos0);
    assert_eq!(items[0].1, b"alpha");
    assert_eq!(items[1].0, pos1);
    assert_eq!(items[1].1, b"beta");
}

#[test]
fn iter_without_close() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 0);

    let (_, pos0) = writer.append(b"gamma").unwrap();
    drop(writer);

    let reader = SegmentReader::open(&seg_path(dir.path(), 1)).unwrap();
    let items: Vec<_> = reader.iter().collect::<WalResult<Vec<_>>>().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].0, pos0);
    assert_eq!(items[0].1, b"gamma");
}

#[test]
fn empty_payload() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 0);

    let (_, pos) = writer.append(b"").unwrap();
    writer.close().unwrap();

    let reader = SegmentReader::open(&seg_path(dir.path(), 1)).unwrap();
    let items: Vec<_> = reader.iter().collect::<WalResult<Vec<_>>>().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].0, pos);
    assert_eq!(items[0].1, b"");
}

#[test]
fn large_payload() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 0);

    let payload: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
    let (_, pos) = writer.append(&payload).unwrap();
    writer.close().unwrap();

    let reader = SegmentReader::open(&seg_path(dir.path(), 1)).unwrap();
    let items: Vec<_> = reader.iter().collect::<WalResult<Vec<_>>>().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].0, pos);
    assert_eq!(items[0].1, &payload[..]);
}

#[test]
fn detect_corruption() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 0);
    let (_, pos) = writer.append(b"clean data").unwrap();
    writer.close().unwrap();

    let path = seg_path(dir.path(), 1);
    let mut data = std::fs::read(&path).unwrap();

    let payload_start = pos as usize + 8;
    data[payload_start] ^= 0xFF;
    std::fs::write(&path, &data).unwrap();

    let reader = SegmentReader::open(&path).unwrap();
    let collected: Result<Vec<_>, _> = reader.iter().collect();
    let err = collected.unwrap_err();
    match err {
        WalError::SegmentCorrupted {
            reason: CorruptSegment::ChecksumMismatch { .. },
            ..
        } => {}
        other => panic!("expected SegmentCorrupted with ChecksumMismatch, got {other}"),
    }
}

#[test]
fn total_crc_in_tail() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 0);

    let payload_a = b"hello";
    let payload_b = b"world";
    writer.append(payload_a).unwrap();
    writer.append(payload_b).unwrap();
    writer.close().unwrap();

    let path = seg_path(dir.path(), 1);
    let data = std::fs::read(&path).unwrap();

    let tail_start = data.len() - 8;
    assert_eq!(&data[tail_start..tail_start + 4], &TAIL_MAGIC);

    let stored_total_crc =
        u32::from_le_bytes(data[tail_start + 4..tail_start + 8].try_into().unwrap());
    let mut concatenated = Vec::new();
    concatenated.extend_from_slice(payload_a);
    concatenated.extend_from_slice(payload_b);
    assert_eq!(stored_total_crc, crc32c::crc32c(&concatenated));
}

#[test]
fn close_updates_count() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 0);
    writer.append(b"a").unwrap();
    writer.append(b"b").unwrap();
    writer.append(b"c").unwrap();
    writer.close().unwrap();

    let reader = SegmentReader::open(&seg_path(dir.path(), 1)).unwrap();
    assert_eq!(reader.count(), 3);
}

#[test]
fn iter_all_items() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 10);

    writer.append(b"one").unwrap();
    writer.append(b"two").unwrap();
    writer.append(b"three").unwrap();
    writer.close().unwrap();

    let reader = SegmentReader::open(&seg_path(dir.path(), 1)).unwrap();
    let items: Vec<_> = reader.iter().collect::<WalResult<Vec<_>>>().unwrap();

    assert_eq!(items.len(), 3);
    assert_eq!(items[0].1, b"one");
    assert_eq!(items[1].1, b"two");
    assert_eq!(items[2].1, b"three");

    assert_eq!(items[0].0, HEADER_SIZE);
    assert_eq!(items[1].0, HEADER_SIZE + 8 + 3);
    assert_eq!(items[2].0, HEADER_SIZE + 8 + 3 + 8 + 3);
}

#[test]
fn iter_without_tail() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 0);
    writer.append(b"x").unwrap();
    writer.append(b"y").unwrap();
    drop(writer);

    let reader = SegmentReader::open(&seg_path(dir.path(), 1)).unwrap();
    let items: Vec<_> = reader.iter().collect::<WalResult<Vec<_>>>().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].1, b"x");
    assert_eq!(items[1].1, b"y");
}

#[test]
fn iter_empty_segment() {
    let dir = TempDir::new().unwrap();
    let writer = make_writer(dir.path(), 1, 0);
    writer.close().unwrap();

    let reader = SegmentReader::open(&seg_path(dir.path(), 1)).unwrap();
    let items: Vec<_> = reader.iter().collect::<WalResult<Vec<_>>>().unwrap();
    assert!(items.is_empty());
}

#[test]
fn is_full_when_full() {
    let dir = TempDir::new().unwrap();
    let max_size = HEADER_SIZE + 8 + 5;
    let mut writer = SegmentWriter::new(dir.path(), 1, 0, max_size).unwrap();

    assert!(!writer.is_full());
    writer.append(b"hello").unwrap();
    assert!(writer.is_full());
}

#[test]
fn open_rejects_bad_magic() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("bad.seg");
    let mut data = vec![0u8; 4096];
    data[0..4].copy_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);
    std::fs::write(&path, &data).unwrap();

    let result = SegmentReader::open(&path);
    assert!(result.is_err());
    match result.unwrap_err() {
        WalError::SegmentCorrupted {
            reason: CorruptSegment::BadMagic { .. },
            ..
        } => {}
        other => panic!("expected SegmentCorrupted with BadMagic, got {other}"),
    }
}

#[test]
fn open_rejects_too_small() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("tiny.seg");
    std::fs::write(&path, b"EVNT").unwrap();

    let result = SegmentReader::open(&path);
    assert!(result.is_err());
}

#[test]
fn multiple_segments_independent() {
    let dir = TempDir::new().unwrap();

    let mut w1 = make_writer(dir.path(), 1, 0);
    let mut w2 = make_writer(dir.path(), 2, 100);

    w1.append(b"seg1").unwrap();
    w2.append(b"seg2").unwrap();

    w1.close().unwrap();
    w2.close().unwrap();

    let r1 = SegmentReader::open(&seg_path(dir.path(), 1)).unwrap();
    let r2 = SegmentReader::open(&seg_path(dir.path(), 2)).unwrap();

    assert_eq!(r1.id(), 1);
    assert_eq!(r2.id(), 2);

    let items1: Vec<_> = r1.iter().collect::<WalResult<Vec<_>>>().unwrap();
    let items2: Vec<_> = r2.iter().collect::<WalResult<Vec<_>>>().unwrap();
    assert_eq!(items1[0].1, b"seg1");
    assert_eq!(items2[0].1, b"seg2");
}

#[test]
fn writer_properties() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 7, 200);

    assert_eq!(writer.id(), 7);
    assert_eq!(writer.offset(), 200);
    assert_eq!(writer.count(), 0);

    writer.append(b"test").unwrap();
    assert_eq!(writer.count(), 1);
}

#[test]
fn open_empty_segment() {
    let dir = TempDir::new().unwrap();
    let writer = make_writer(dir.path(), 1, 100);
    drop(writer);

    let path = seg_path(dir.path(), 1);
    let (resumed, index) = SegmentWriter::open(&path, 64 * 1024 * 1024, 256).unwrap();
    assert_eq!(resumed.id(), 1);
    assert_eq!(resumed.offset(), 100);
    assert_eq!(resumed.count(), 0);
    assert!(index.items.is_empty());
    drop(resumed);
}

#[test]
fn open_with_records() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 0);
    writer.append(b"one").unwrap();
    writer.append(b"two").unwrap();
    writer.append(b"three").unwrap();
    drop(writer);

    let path = seg_path(dir.path(), 1);
    let (resumed, _index) = SegmentWriter::open(&path, 64 * 1024 * 1024, 256).unwrap();
    assert_eq!(resumed.count(), 3);
    assert_eq!(resumed.offset(), 0);
}

#[test]
fn open_and_continue_writing() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 10);
    writer.append(b"before").unwrap();
    drop(writer);

    let path = seg_path(dir.path(), 1);
    let (mut resumed, _) = SegmentWriter::open(&path, 64 * 1024 * 1024, 256).unwrap();
    let (off, _pos) = resumed.append(b"after").unwrap();
    assert_eq!(off, 11);
    assert_eq!(resumed.count(), 2);
    resumed.close().unwrap();

    let reader = SegmentReader::open(&path).unwrap();
    assert_eq!(reader.count(), 2);
    let items: Vec<_> = reader.iter().collect::<WalResult<Vec<_>>>().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].1, b"before");
    assert_eq!(items[1].1, b"after");
}

#[test]
fn open_closed_segment_returns_error() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 0);
    writer.append(b"data").unwrap();
    writer.close().unwrap();

    let path = seg_path(dir.path(), 1);
    let result = SegmentWriter::open(&path, 64 * 1024 * 1024, 256);
    assert!(result.is_err(), "expected error for closed segment");
    let err = result.err().unwrap();
    assert!(
        matches!(
            err,
            WalError::SegmentCorrupted {
                reason: CorruptSegment::AlreadyClosed,
                ..
            }
        ),
        "expected AlreadyClosed, got: {err}"
    );
}

#[test]
fn open_with_partial_write() {
    let dir = TempDir::new().unwrap();
    let mut writer = make_writer(dir.path(), 1, 0);
    writer.append(b"good").unwrap();
    drop(writer);

    let path = seg_path(dir.path(), 1);
    let mut data = std::fs::read(&path).unwrap();
    data.extend_from_slice(&[0xFF; 5]);
    std::fs::write(&path, &data).unwrap();

    let (resumed, _) = SegmentWriter::open(&path, 64 * 1024 * 1024, 256).unwrap();
    assert_eq!(resumed.count(), 1);

    let file_len = std::fs::metadata(&path).unwrap().len();
    assert_eq!(file_len, HEADER_SIZE + 8 + 4);
}
