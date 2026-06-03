use std::fs;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

use crate::error::{CorruptManifest, IoOperation, WalError, WalResult};
use crate::io;

pub const DEFAULT_SEG_SIZE: u64 = 128 * 1024 * 1024;
pub const DEFAULT_IDX_INTERVAL: u32 = 256;

const MANIFEST_VERSION: u32 = 1;

pub struct Manifest {
    pub version: u32,
    open_segment: u64,
    first_segment: u64,
    next_offset: u64,
    snapshot_offset: Option<u64>,
    segment_size: u64,
    index_interval: u32,
    dir: PathBuf,
}

impl Manifest {
    pub fn open(dir: &Path) -> WalResult<Self> {
        let p = path(dir);
        if p.exists() {
            let file = fs::File::open(&p).map_err(WalError::at(p.clone(), IoOperation::Read))?;

            let mut version: u32 = 0;
            let mut active_segment: u64 = 0;
            let mut first_segment: u64 = 1;
            let mut next_event_seq: u64 = 0;
            let mut latest_snapshot_seq: Option<u64> = None;
            let mut segment_size: u64 = DEFAULT_SEG_SIZE;
            let mut index_interval: u32 = DEFAULT_IDX_INTERVAL;

            for line in std::io::BufReader::new(file).lines() {
                let line = line.map_err(WalError::at(p.clone(), IoOperation::Read))?;
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((key, value)) = line.split_once('=') {
                    match key.trim() {
                        "version" => {
                            version = value.trim().parse().map_err(|_| WalError::ManifestError {
                                reason: CorruptManifest::InvalidField { key: "version" },
                            })?
                        }
                        "active_segment" => {
                            active_segment =
                                value.trim().parse().map_err(|_| WalError::ManifestError {
                                    reason: CorruptManifest::InvalidField {
                                        key: "active_segment",
                                    },
                                })?
                        }
                        "first_segment" => {
                            first_segment =
                                value.trim().parse().map_err(|_| WalError::ManifestError {
                                    reason: CorruptManifest::InvalidField {
                                        key: "first_segment",
                                    },
                                })?
                        }
                        "next_event_seq" => {
                            next_event_seq =
                                value.trim().parse().map_err(|_| WalError::ManifestError {
                                    reason: CorruptManifest::InvalidField {
                                        key: "next_event_seq",
                                    },
                                })?
                        }
                        "latest_snapshot_seq" => {
                            let v = value.trim();
                            latest_snapshot_seq = if v.is_empty() {
                                None
                            } else {
                                Some(v.parse().map_err(|_| WalError::ManifestError {
                                    reason: CorruptManifest::InvalidField {
                                        key: "latest_snapshot_seq",
                                    },
                                })?)
                            }
                        }
                        "segment_size" => {
                            segment_size =
                                value.trim().parse().map_err(|_| WalError::ManifestError {
                                    reason: CorruptManifest::InvalidField {
                                        key: "segment_size",
                                    },
                                })?
                        }
                        "index_interval" => {
                            index_interval =
                                value.trim().parse().map_err(|_| WalError::ManifestError {
                                    reason: CorruptManifest::InvalidField {
                                        key: "index_interval",
                                    },
                                })?
                        }
                        _ => {}
                    }
                }
            }

            if version == 0 {
                return Err(WalError::ManifestError {
                    reason: CorruptManifest::MissingVersion,
                });
            }

            tracing::debug!(
                version,
                active_segment,
                first_segment,
                next_event_seq,
                latest_snapshot_seq = ?latest_snapshot_seq,
                segment_size,
                index_interval,
                "manifest loaded"
            );

            Ok(Manifest {
                version,
                open_segment: active_segment,
                first_segment,
                next_offset: next_event_seq,
                snapshot_offset: latest_snapshot_seq,
                segment_size,
                index_interval,
                dir: dir.to_path_buf(),
            })
        } else {
            tracing::debug!("no manifest found, creating new");
            let manifest = Manifest {
                version: MANIFEST_VERSION,
                open_segment: 0,
                first_segment: 1,
                next_offset: 0,
                snapshot_offset: None,
                segment_size: DEFAULT_SEG_SIZE,
                index_interval: DEFAULT_IDX_INTERVAL,
                dir: dir.to_path_buf(),
            };
            manifest.save()?;
            Ok(manifest)
        }
    }

    pub fn save(&self) -> WalResult<()> {
        let events_dir = self.dir.join("events");
        fs::create_dir_all(&events_dir)
            .map_err(WalError::at(events_dir.clone(), IoOperation::Create))?;

        let temp_path = events_dir.join("manifest.dat.tmp");
        let final_path = events_dir.join("manifest.dat");

        let mut file = fs::File::create(&temp_path)
            .map_err(WalError::at(temp_path.clone(), IoOperation::Create))?;
        writeln!(file, "# axiom manifest")
            .map_err(WalError::at(temp_path.clone(), IoOperation::Write))?;
        writeln!(file, "version={}", self.version)
            .map_err(WalError::at(temp_path.clone(), IoOperation::Write))?;
        writeln!(file, "active_segment={}", self.open_segment)
            .map_err(WalError::at(temp_path.clone(), IoOperation::Write))?;
        writeln!(file, "first_segment={}", self.first_segment)
            .map_err(WalError::at(temp_path.clone(), IoOperation::Write))?;
        writeln!(file, "next_event_seq={}", self.next_offset)
            .map_err(WalError::at(temp_path.clone(), IoOperation::Write))?;
        writeln!(
            file,
            "latest_snapshot_seq={}",
            self.snapshot_offset
                .map(|s| s.to_string())
                .unwrap_or_default()
        )
        .map_err(WalError::at(temp_path.clone(), IoOperation::Write))?;
        writeln!(file, "segment_size={}", self.segment_size)
            .map_err(WalError::at(temp_path.clone(), IoOperation::Write))?;
        writeln!(file, "index_interval={}", self.index_interval)
            .map_err(WalError::at(temp_path.clone(), IoOperation::Write))?;
        file.sync_all()
            .map_err(WalError::at(temp_path.clone(), IoOperation::Sync))?;

        fs::rename(&temp_path, &final_path)
            .map_err(WalError::at(final_path.clone(), IoOperation::Rename))?;
        io::fsync_dir(&events_dir)?;
        Ok(())
    }

    pub fn set_snap_offset(&mut self, offset: u64) -> WalResult<()> {
        if let Some(current) = self.snapshot_offset {
            if offset < current {
                return Err(WalError::ManifestError {
                    reason: CorruptManifest::InvariantViolated {
                        detail: "checkpoint offset must not decrease",
                    },
                });
            }
        }
        self.snapshot_offset = Some(offset);
        self.save()
    }

    pub fn open_seg(&self) -> u64 {
        self.open_segment
    }

    pub fn first_seg(&self) -> u64 {
        self.first_segment
    }

    pub fn next_offset(&self) -> u64 {
        self.next_offset
    }

    pub fn snap_offset(&self) -> Option<u64> {
        self.snapshot_offset
    }

    pub fn seg_size(&self) -> u64 {
        self.segment_size
    }

    pub fn idx_interval(&self) -> u32 {
        self.index_interval
    }

    pub fn set_open_seg(&mut self, seg_id: u64) {
        self.open_segment = seg_id;
    }

    pub fn set_first_seg(&mut self, seg_id: u64) {
        self.first_segment = seg_id;
    }

    pub fn set_next_offset(&mut self, offset: u64) {
        self.next_offset = offset;
    }
}

pub fn path(dir: &Path) -> PathBuf {
    dir.join("events").join("manifest.dat")
}
