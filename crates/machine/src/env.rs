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
}

impl Environment {
    pub fn new(cwd: impl Into<PathBuf>) -> Self {
        Self {
            cwd: cwd.into(),
            vars: HashMap::new(),
            root: None,
        }
    }
}
