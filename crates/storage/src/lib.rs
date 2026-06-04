pub use error::{WalError, WalResult};
pub use store::Store;

pub mod error;
pub mod index;
mod io;
pub mod manifest;
pub mod segment;
pub mod snapshot;
pub mod store;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use error::{CorruptManifest, CorruptSegment, IoOperation};
use index::SparseIndex;
use manifest::Manifest;
use segment::{SegmentReader, SegmentWriter};

pub struct Wal {
    manifest: Manifest,
    writer: Option<SegmentWriter>,
    readers: BTreeMap<u64, SegmentReader>,
    indexes: BTreeMap<u64, SparseIndex>,
    dir: PathBuf,
}

impl Wal {
    pub fn open(dir: &Path) -> WalResult<Self> {
        tracing::info!(dir = %dir.display(), "opening wal");

        let seg_base = dir.join("events").join("segments");
        std::fs::create_dir_all(&seg_base).map_err(WalError::at(seg_base, IoOperation::Create))?;
        let snap_dir = dir.join("snapshots");
        std::fs::create_dir_all(&snap_dir).map_err(WalError::at(snap_dir, IoOperation::Create))?;

        let mut manifest = Manifest::open(dir)?;

        let mut readers = BTreeMap::new();
        let mut indexes = BTreeMap::new();

        for seg_id in manifest.first_seg()..manifest.open_seg() {
            let path = seg_path(dir, seg_id);
            if !path.exists() {
                return Err(WalError::SegmentNotFound { segment_id: seg_id });
            }
            let reader = SegmentReader::open(&path)?;
            readers.insert(seg_id, reader);

            let idx = idx_path(dir, seg_id);
            if idx.exists() {
                let index = SparseIndex::load(&idx)?;
                indexes.insert(seg_id, index);
            }
        }

        let (writer, active_index) = if manifest.open_seg() == 0 {
            let seg_id = 1;
            let d = seg_dir(dir, seg_id);
            std::fs::create_dir_all(&d).map_err(WalError::at(d.clone(), IoOperation::Create))?;

            let writer = SegmentWriter::new(&d, seg_id, 0, manifest.seg_size())?;
            let index = SparseIndex::new(seg_id, manifest.idx_interval());
            manifest.set_open_seg(seg_id);
            manifest.set_next_offset(0);
            manifest.save()?;
            io::fsync_dir(&dir.join("events"))?;
            (writer, index)
        } else {
            let seg_id = manifest.open_seg();
            let path = seg_path(dir, seg_id);

            if path.exists() {
                match SegmentWriter::open(&path, manifest.seg_size(), manifest.idx_interval()) {
                    Ok((writer, index)) => {
                        if writer.truncated_items() > 0 {
                            tracing::warn!(
                                seg_id,
                                truncated = writer.truncated_items(),
                                "items truncated during recovery"
                            );
                        }
                        manifest.set_next_offset(writer.offset() + writer.count() as u64);
                        (writer, index)
                    }
                    Err(WalError::SegmentCorrupted {
                        reason: CorruptSegment::AlreadyClosed,
                        ..
                    }) => {
                        tracing::info!(
                            seg_id,
                            "active segment already closed, demoting to reader and creating new segment"
                        );
                        // Rotate committed manifest but crashed — demote to closed reader.
                        let reader = SegmentReader::open(&path)?;
                        readers.insert(seg_id, reader);

                        let idx = idx_path(dir, seg_id);
                        let index = if idx.exists() {
                            SparseIndex::load(&idx)?
                        } else {
                            SparseIndex::new(seg_id, manifest.idx_interval())
                        };
                        indexes.insert(seg_id, index);

                        let next_seg_id = seg_id + 1;
                        let d = seg_dir(dir, next_seg_id);
                        std::fs::create_dir_all(&d)
                            .map_err(WalError::at(d.clone(), IoOperation::Create))?;
                        let writer = SegmentWriter::new(
                            &d,
                            next_seg_id,
                            manifest.next_offset(),
                            manifest.seg_size(),
                        )?;
                        let next_index = SparseIndex::new(next_seg_id, manifest.idx_interval());
                        manifest.set_open_seg(next_seg_id);
                        manifest.save()?;
                        io::fsync_dir(&dir.join("events"))?;
                        (writer, next_index)
                    }
                    Err(e) => return Err(e),
                }
            } else {
                return Err(WalError::SegmentNotFound { segment_id: seg_id });
            }
        };

        let active_seg_id = writer.id();
        indexes.insert(active_seg_id, active_index);

        tracing::info!(
            active_seg = active_seg_id,
            closed_segments = readers.len(),
            next_offset = manifest.next_offset(),
            snap_offset = ?manifest.snap_offset(),
            "wal opened"
        );

        Ok(Self {
            manifest,
            writer: Some(writer),
            readers,
            indexes,
            dir: dir.to_path_buf(),
        })
    }

    /// Append an opaque byte item. Returns `(global_offset, byte_position)`.
    /// Data is written and fsynced before returning.
    pub fn append(&mut self, bytes: &[u8]) -> WalResult<(u64, u64)> {
        let writer = self.writer.as_mut().ok_or(WalError::ManifestError {
            reason: CorruptManifest::InvariantViolated {
                detail: "no active writer",
            },
        })?;

        let (offset, position) = writer.append(bytes)?;

        if let Some(index) = self.indexes.get_mut(&writer.id()) {
            index.insert(offset, position);
        }

        self.manifest.set_next_offset(offset + 1);

        if writer.is_full() {
            tracing::debug!(seg_id = writer.id(), offset, "segment full, rotating");
            self.rotate()?;
        }

        Ok((offset, position))
    }

    /// Append multiple items in one write + one fsync.
    /// Returns `(global_offset, byte_position)` per item.
    pub fn append_batch(&mut self, items: &[&[u8]]) -> WalResult<Vec<(u64, u64)>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        let writer = self.writer.as_mut().ok_or(WalError::ManifestError {
            reason: CorruptManifest::InvariantViolated {
                detail: "no active writer",
            },
        })?;

        let results = writer.append_batch(items)?;

        if let Some(index) = self.indexes.get_mut(&writer.id()) {
            for &(offset, position) in &results {
                index.insert(offset, position);
            }
        }

        if let Some(&(last_offset, _)) = results.last() {
            self.manifest.set_next_offset(last_offset + 1);
        }

        if writer.is_full() {
            tracing::debug!(
                seg_id = writer.id(),
                last_offset = results.last().map(|r| r.0),
                "segment full after batch, rotating"
            );
            self.rotate()?;
        }

        Ok(results)
    }

    /// Append a single item without fsync. Caller must call `sync()` later for durability.
    pub fn append_unsynced(&mut self, bytes: &[u8]) -> WalResult<(u64, u64)> {
        let writer = self.writer.as_mut().ok_or(WalError::ManifestError {
            reason: CorruptManifest::InvariantViolated {
                detail: "no active writer",
            },
        })?;

        let (offset, position) = writer.append_unsynced(bytes)?;

        if let Some(index) = self.indexes.get_mut(&writer.id()) {
            index.insert(offset, position);
        }

        self.manifest.set_next_offset(offset + 1);

        if writer.is_full() {
            tracing::debug!(
                seg_id = writer.id(),
                offset,
                "segment full after unsynced append, rotating"
            );
            self.rotate()?;
        }

        Ok((offset, position))
    }

    /// Flush pending writes to disk.
    pub fn sync(&mut self) -> WalResult<()> {
        let writer = self.writer.as_mut().ok_or(WalError::ManifestError {
            reason: CorruptManifest::InvariantViolated {
                detail: "no active writer",
            },
        })?;
        writer.sync()
    }

    /// Write a snapshot at the given offset. The payload is opaque to the WAL.
    pub fn checkpoint(&mut self, offset: u64, state: &[u8]) -> WalResult<()> {
        tracing::debug!(offset, size = state.len(), "writing checkpoint");
        if offset > self.manifest.next_offset() {
            return Err(WalError::OffsetOutOfRange {
                requested: offset,
                next: self.manifest.next_offset(),
            });
        }
        snapshot::save(&self.dir, offset, state)?;
        self.manifest.set_snap_offset(offset)?;
        self.trim_checkpoint(offset)?;
        snapshot::retain_latest(&self.dir, 2)?;
        Ok(())
    }

    pub fn load(&self) -> WalResult<Option<(u64, Vec<u8>)>> {
        snapshot::load(&self.dir, &self.manifest)
    }

    /// Replay items starting from `offset`. Yields `(global_offset, payload)`,
    /// crossing segment boundaries automatically.
    pub fn replay(&self, offset: u64) -> WalResult<ReplayIter<'_>> {
        if self.manifest.next_offset() > 0 && offset > self.manifest.next_offset() {
            return Err(WalError::OffsetOutOfRange {
                requested: offset,
                next: self.manifest.next_offset(),
            });
        }
        if let Some(snapshot_offset) = self.manifest.snap_offset()
            && offset < snapshot_offset
        {
            return Err(WalError::OffsetOutOfRange {
                requested: offset,
                next: snapshot_offset,
            });
        }

        // Build sorted (seg_id, first_offset) list for all segments.
        let mut segments: Vec<(u64, u64)> = Vec::new();
        for (&seg_id, reader) in &self.readers {
            segments.push((seg_id, reader.offset()));
        }
        if let Some(ref writer) = self.writer {
            segments.push((writer.id(), writer.offset()));
        }
        segments.sort_by_key(|&(_, first)| first);

        if let Some((_, first_available)) = segments.first()
            && offset < *first_available
        {
            return Err(WalError::OffsetOutOfRange {
                requested: offset,
                next: *first_available,
            });
        }

        // First segment that could contain `offset`.
        let start_idx = segments
            .iter()
            .rposition(|&(_, first)| first <= offset)
            .unwrap_or(0);

        Ok(ReplayIter {
            wal: self,
            segments,
            seg_idx: start_idx,
            current_reader: None,
            current_position: segment::HEADER_SIZE,
            item_offset: 0,
            next_offset: offset,
            exhausted: false,
        })
    }

    pub fn last_offset(&self) -> Option<u64> {
        let next = self.manifest.next_offset();
        if next == 0 { None } else { Some(next - 1) }
    }

    pub fn next_offset(&self) -> u64 {
        self.manifest.next_offset()
    }
}

/// Cross-segment iterator yielded by [`Wal::replay`].
pub struct ReplayIter<'a> {
    wal: &'a Wal,
    segments: Vec<(u64, u64)>,
    seg_idx: usize,
    current_reader: Option<SegmentReader>,
    current_position: u64,
    item_offset: u64,
    next_offset: u64,
    exhausted: bool,
}

impl<'a> Iterator for ReplayIter<'a> {
    type Item = WalResult<(u64, Vec<u8>)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        loop {
            if self.seg_idx >= self.segments.len() {
                self.exhausted = true;
                return None;
            }

            if self.current_reader.is_none() {
                let (seg_id, first_offset) = self.segments[self.seg_idx];

                // SparseIndex seek: find the nearest entry ≤ next_offset.
                let (start_pos, start_off) = self
                    .wal
                    .indexes
                    .get(&seg_id)
                    .and_then(|idx| idx.find(self.next_offset))
                    .map(|(off, byte_pos)| (byte_pos, off))
                    .unwrap_or((segment::HEADER_SIZE, first_offset));

                let path = seg_path(&self.wal.dir, seg_id);
                let reader = match SegmentReader::open(&path) {
                    Ok(r) => r,
                    Err(e) => {
                        tracing::warn!(seg_id, "failed to open segment during replay");
                        self.exhausted = true;
                        return Some(Err(e));
                    }
                };

                self.current_reader = Some(reader);
                self.current_position = start_pos;
                self.item_offset = start_off;
            }

            let reader = self.current_reader.as_ref().unwrap();

            match reader.read_at(self.current_position) {
                Ok(None) => {
                    self.current_reader = None;
                    self.seg_idx += 1;
                    continue;
                }
                Ok(Some((payload, next_position))) => {
                    let global_offset = self.item_offset;
                    self.current_position = next_position;
                    self.item_offset += 1;

                    if global_offset < self.next_offset {
                        continue;
                    }

                    self.next_offset = global_offset + 1;
                    return Some(Ok((global_offset, payload.to_vec())));
                }
                Err(e) => {
                    self.exhausted = true;
                    return Some(Err(e));
                }
            }
        }
    }
}

// -- private helpers --

impl Wal {
    fn trim_checkpoint(&mut self, offset: u64) -> WalResult<()> {
        let removable = self
            .readers
            .iter()
            .filter_map(|(&seg_id, reader)| {
                let end = reader.offset() + reader.count() as u64;
                (end <= offset).then_some(seg_id)
            })
            .collect::<Vec<_>>();

        if removable.is_empty() {
            return Ok(());
        }

        let new_first_seg = removable.last().copied().unwrap() + 1;
        self.manifest.set_first_seg(new_first_seg);
        self.manifest.save()?;

        let mut dirs_to_sync = BTreeSet::new();
        for seg_id in removable {
            self.readers.remove(&seg_id);
            self.indexes.remove(&seg_id);

            let seg_path = seg_path(&self.dir, seg_id);
            remove_file_if_exists(&seg_path)?;
            if let Some(parent) = seg_path.parent() {
                dirs_to_sync.insert(parent.to_path_buf());
            }

            let idx_path = idx_path(&self.dir, seg_id);
            remove_file_if_exists(&idx_path)?;
            if let Some(parent) = idx_path.parent() {
                dirs_to_sync.insert(parent.to_path_buf());
            }
        }

        for dir in dirs_to_sync {
            io::fsync_dir(&dir)?;
            remove_empty_dir(&dir)?;
        }

        Ok(())
    }

    fn rotate(&mut self) -> WalResult<()> {
        let writer = self.writer.take().ok_or(WalError::ManifestError {
            reason: CorruptManifest::InvariantViolated {
                detail: "no active writer to roll",
            },
        })?;

        let prev_seg_id = writer.id();
        let next_seg_id = prev_seg_id + 1;

        tracing::info!(from = prev_seg_id, to = next_seg_id, "rotating segment");

        // Phase 1: persist new segment + manifest before closing old one.
        let next_dir = seg_dir(&self.dir, next_seg_id);
        std::fs::create_dir_all(&next_dir)
            .map_err(WalError::at(next_dir.clone(), IoOperation::Create))?;

        let next_writer = SegmentWriter::new(
            &next_dir,
            next_seg_id,
            self.manifest.next_offset(),
            self.manifest.seg_size(),
        )?;

        self.indexes.insert(
            next_seg_id,
            SparseIndex::new(next_seg_id, self.manifest.idx_interval()),
        );

        self.manifest.set_open_seg(next_seg_id);
        self.manifest.save()?;
        io::fsync_dir(&self.dir.join("events"))?;

        // Phase 1 committed.
        self.writer = Some(next_writer);

        // Phase 2: close old segment, persist index, open reader.
        writer.close()?;

        if let Some(index) = self.indexes.remove(&prev_seg_id) {
            let idx_p = idx_path(&self.dir, prev_seg_id);
            index.save(&idx_p)?;
            io::fsync_dir(idx_p.parent().unwrap_or(Path::new(".")))?;
        }

        let seg_p = seg_path(&self.dir, prev_seg_id);
        let reader = SegmentReader::open(&seg_p)?;
        self.readers.insert(prev_seg_id, reader);

        Ok(())
    }
}

fn seg_dir(dir: &Path, seg_id: u64) -> PathBuf {
    let bucket = (seg_id / 1000) * 1000;
    let bucket_dir = format!("{:04}-{:04}", bucket, bucket + 999);
    dir.join("events").join("segments").join(bucket_dir)
}

fn seg_path(dir: &Path, seg_id: u64) -> PathBuf {
    seg_dir(dir, seg_id).join(format!("{:06}.seg", seg_id))
}

fn idx_path(dir: &Path, seg_id: u64) -> PathBuf {
    seg_dir(dir, seg_id).join(format!("{:06}.idx", seg_id))
}

fn remove_file_if_exists(path: &Path) -> WalResult<()> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(WalError::at(path.to_path_buf(), IoOperation::Remove)(e)),
    }
}

fn remove_empty_dir(path: &Path) -> WalResult<()> {
    match std::fs::remove_dir(path) {
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
