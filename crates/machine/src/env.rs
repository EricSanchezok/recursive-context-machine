use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use utils::{EnvironmentId, Name};

fn default_environment_id() -> EnvironmentId {
    EnvironmentId::new()
}

fn default_environment_name() -> Name {
    Name::from_static("environment")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    #[serde(default = "default_environment_id")]
    id: EnvironmentId,
    #[serde(default = "default_environment_name")]
    pub name: Name,
    pub cwd: PathBuf,
    pub vars: HashMap<String, String>,
    pub root: Option<PathBuf>,
    pub platform: String,
}

impl Environment {
    pub fn id(&self) -> &EnvironmentId {
        &self.id
    }

    pub fn new(cwd: impl Into<PathBuf>) -> Self {
        Self::named("environment", cwd)
    }

    pub fn named(name: impl Into<String>, cwd: impl Into<PathBuf>) -> Self {
        Self {
            id: EnvironmentId::new(),
            name: Name::new(name).expect("environment name must be valid"),
            cwd: cwd.into(),
            vars: HashMap::new(),
            root: None,
            platform: std::env::consts::OS.to_string(),
        }
    }
}
