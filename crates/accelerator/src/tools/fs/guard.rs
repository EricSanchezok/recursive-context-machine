//! Read-before-write guard.
//!
//! Tracks which files have been read by this process and rejects write/edit
//! attempts on files that haven't been read, or that have been modified on
//! disk since they were last read.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::SystemTime;

/// Per-file read timestamps, keyed by (accelerator_name, absolute_path).
///
/// Grows with every unique (name, path) pair touched in this process.
/// Entries are freed when the process exits — no explicit eviction needed.
static READ_TIMES: LazyLock<Mutex<HashMap<(String, PathBuf), SystemTime>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Per-file write locks — serializes concurrent edits targeting the same file.
static WRITE_LOCKS: LazyLock<Mutex<HashMap<PathBuf, Arc<tokio::sync::Mutex<()>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Record that `path` was just read (or written) by the process named `name`.
pub fn mark_read(name: &str, path: &Path) {
    let key = (name.to_string(), path.to_path_buf());
    READ_TIMES.lock().unwrap().insert(key, SystemTime::now());
}

/// Assert that `path` has been read by this process and has not been modified
/// on disk since that read.
pub fn require_read(name: &str, path: &Path) -> Result<(), String> {
    let recorded = {
        let map = READ_TIMES.lock().unwrap();
        map.get(&(name.to_string(), path.to_path_buf())).copied()
    };

    let recorded = recorded.ok_or_else(|| {
        format!(
            "You must read the file '{}' before modifying it. Use the Read tool first",
            path.display()
        )
    })?;

    let disk_mtime = std::fs::metadata(path)
        .and_then(|m| m.modified())
        .map_err(|e| format!("cannot stat '{}': {e}", path.display()))?;

    if disk_mtime > recorded {
        return Err(format!(
            "File '{}' has been modified since it was last read.\n\
             Please read the file again before modifying it.",
            path.display(),
        ));
    }

    Ok(())
}

/// A per-file write lock guard. Released when dropped.
pub struct WriteLock {
    _guard: tokio::sync::OwnedMutexGuard<()>,
}

/// Acquire a per-file write lock so concurrent edits to the same file are
/// serialized. The std::sync::Mutex is only held for the HashMap lookup.
pub async fn acquire_write_lock(path: &Path) -> WriteLock {
    let arc = {
        let mut map = WRITE_LOCKS.lock().unwrap();
        map.entry(path.to_path_buf())
            .or_insert_with(|| Arc::new(tokio::sync::Mutex::new(())))
            .clone()
    };
    WriteLock {
        _guard: arc.lock_owned().await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn require_read_after_mark_read_ok() {
        let dir = std::env::temp_dir().join("guard_test_ok");
        let path = dir.join("a.txt");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::File::create(&path).unwrap();

        mark_read("test", &path);
        assert!(require_read("test", &path).is_ok());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn never_read_returns_error() {
        let dir = std::env::temp_dir().join("guard_test_never");
        let path = dir.join("b.txt");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(&path, b"data").unwrap();

        let err = require_read("test", &path).unwrap_err();
        assert!(err.contains("read the file"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn external_modification_detected() {
        let dir = std::env::temp_dir().join("guard_test_ext");
        let path = dir.join("c.txt");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(&path, b"v1").unwrap();

        mark_read("test", &path);

        std::thread::sleep(std::time::Duration::from_millis(15));
        std::fs::write(&path, b"v2").unwrap();

        let err = require_read("test", &path).unwrap_err();
        assert!(err.contains("modified"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn different_names_isolated() {
        let dir = std::env::temp_dir().join("guard_test_iso");
        let path = dir.join("d.txt");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(&path, b"data").unwrap();

        mark_read("alice", &path);
        assert!(require_read("alice", &path).is_ok());
        assert!(require_read("bob", &path).is_err());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[tokio::test]
    async fn write_lock_acquire_and_release() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};

        let dir = std::env::temp_dir().join("guard_test_lock");
        let path = dir.join("e.txt");
        std::fs::create_dir_all(&dir).unwrap();

        let locked = Arc::new(AtomicBool::new(false));
        let unlocked = Arc::new(AtomicBool::new(false));

        let lock = locked.clone();
        let unlock = unlocked.clone();
        let path2 = path.clone();

        let guard = acquire_write_lock(&path).await;
        let race = tokio::spawn(async move {
            let _g = acquire_write_lock(&path2).await;
            lock.store(true, Ordering::SeqCst);
            drop(_g);
            unlock.store(true, Ordering::SeqCst);
        });

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        assert!(
            !locked.load(Ordering::SeqCst),
            "second acquire should block"
        );
        drop(guard);
        race.await.unwrap();
        assert!(unlocked.load(Ordering::SeqCst));

        std::fs::remove_dir_all(&dir).ok();
    }
}
