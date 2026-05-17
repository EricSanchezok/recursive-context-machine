use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Environment — the external world accessible to the machine.
///
/// Carries the working directory and environment variables.
/// The Policy observes but does not modify the environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub cwd: PathBuf,
    pub vars: HashMap<String, String>,
    /// Filesystem boundary — tools may only access paths within this root.
    /// When `None`, no boundary is enforced.
    pub root: Option<PathBuf>,
    /// Operating system platform (e.g. "macos", "linux", "windows").
    pub platform: String,
}

impl Environment {
    pub fn new(cwd: impl Into<PathBuf>) -> Self {
        Self {
            cwd: cwd.into(),
            vars: HashMap::new(),
            root: None,
            platform: std::env::consts::OS.to_string(),
        }
    }

    /// Generate a human-readable snapshot of the environment.
    ///
    /// Format:
    /// ```text
    /// cwd: /path/to/dir
    /// platform: macos
    /// time: 2026-05-17T13:17:12+08:00
    /// ```
    pub fn snapshot(&self) -> String {
        let now = Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
        format!(
            "cwd: {}\nplatform: {}\ntime: {}",
            self.cwd.display(),
            self.platform,
            now,
        )
    }
}
