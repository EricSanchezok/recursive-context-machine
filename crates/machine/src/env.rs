use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub cwd: PathBuf,
    pub vars: HashMap<String, String>,
    pub root: Option<PathBuf>,
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
}
