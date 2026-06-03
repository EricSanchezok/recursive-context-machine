use std::fs;

use storage::manifest::Manifest;
use tempfile::TempDir;

#[test]
fn create_and_load() {
    let dir = TempDir::new().unwrap();
    let manifest = Manifest::open(dir.path()).unwrap();
    assert_eq!(manifest.version, 1);
    assert_eq!(manifest.open_seg(), 0);
    assert_eq!(manifest.first_seg(), 1);
    assert_eq!(manifest.next_offset(), 0);
    assert!(manifest.snap_offset().is_none());

    let loaded = Manifest::open(dir.path()).unwrap();
    assert_eq!(loaded.version, manifest.version);
    assert_eq!(loaded.open_seg(), manifest.open_seg());
    assert_eq!(loaded.next_offset(), manifest.next_offset());
}

#[test]
fn update_persists() {
    let dir = TempDir::new().unwrap();
    let mut manifest = Manifest::open(dir.path()).unwrap();

    manifest.set_open_seg(3);
    manifest.set_first_seg(2);
    manifest.set_next_offset(500001);
    manifest.set_snap_offset(450000).unwrap();

    let loaded = Manifest::open(dir.path()).unwrap();
    assert_eq!(loaded.open_seg(), 3);
    assert_eq!(loaded.first_seg(), 2);
    assert_eq!(loaded.next_offset(), 500001);
    assert_eq!(loaded.snap_offset(), Some(450000));
}

#[test]
fn human_readable() {
    let dir = TempDir::new().unwrap();
    let mut manifest = Manifest::open(dir.path()).unwrap();
    manifest.set_next_offset(42);
    manifest.save().unwrap();

    let content = fs::read_to_string(dir.path().join("events/manifest.dat")).unwrap();
    assert!(content.contains("version=1"));
    assert!(content.contains("next_event_seq=42"));
}
