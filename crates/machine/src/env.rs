use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use utils::{EnvironmentId, Name};

use crate::assistant::CompletionAssistant;

fn default_environment_id() -> EnvironmentId {
    EnvironmentId::new()
}

fn default_environment_name() -> Name {
    Name::from_static("environment")
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Environment {
    #[serde(default = "default_environment_id")]
    id: EnvironmentId,
    #[serde(default = "default_environment_name")]
    pub name: Name,
    pub cwd: PathBuf,
    pub vars: HashMap<String, String>,
    pub root: Option<PathBuf>,
    pub platform: String,
    /// Override for the run directory. When set, injected into the env context
    /// fragment and exported as `RCM_RUN_DIR` for subprocess/shell tools.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub run_dir: Option<PathBuf>,
    /// Read-only snapshot of the context directory (cell metadata rows),
    /// refreshed by the machine before every tool execution. Tools use it
    /// for staleness/range decisions without seeing cell contents. Never
    /// serialized into WAL checkpoints — it is a derived observation.
    #[serde(skip, default)]
    pub context_directory: Vec<crate::obs::CellDirEntry>,
    /// Handle to the metered completion gateway generative tools use
    /// (context.compact, …). The machine only carries the handle; the
    /// accelerator publishes the live document/model into it each step.
    /// Never serialized — restored runs re-inject at fire time.
    #[serde(skip, default)]
    pub assistant: Option<Arc<dyn CompletionAssistant>>,
}

impl Environment {
    pub fn id(&self) -> &EnvironmentId {
        &self.id
    }

    /// An honest snapshot of the host the agent is currently running on:
    /// inherits process env vars and the host platform tag.
    ///
    /// For sandboxed scenarios where the agent should not see host state, use
    /// [`Environment::empty`].
    pub fn new(cwd: impl Into<PathBuf>) -> Self {
        Self::named("environment", cwd)
    }

    /// Same as [`new`] but with an explicit name. Inherits host env vars.
    pub fn named(name: impl Into<String>, cwd: impl Into<PathBuf>) -> Self {
        Self {
            id: EnvironmentId::new(),
            name: Name::new(name).expect("environment name must be valid"),
            cwd: cwd.into(),
            vars: std::env::vars().collect(),
            root: None,
            platform: std::env::consts::OS.to_string(),
            run_dir: None,
            context_directory: Vec::new(),
            assistant: None,
        }
    }
    /// A deliberately empty environment for sandbox scenarios — no inherited
    /// env vars, no platform tag (defaults to the host OS string for
    /// compatibility but env vars stay empty).
    ///
    /// Callers that want to lie about the platform too should set the field
    /// directly after construction.
    pub fn empty(cwd: impl Into<PathBuf>) -> Self {
        Self::empty_named("environment", cwd)
    }

    /// Same as [`empty`] but with an explicit name.
    pub fn empty_named(name: impl Into<String>, cwd: impl Into<PathBuf>) -> Self {
        Self {
            id: EnvironmentId::new(),
            name: Name::new(name).expect("environment name must be valid"),
            cwd: cwd.into(),
            vars: HashMap::new(),
            root: None,
            platform: std::env::consts::OS.to_string(),
            run_dir: None,
            context_directory: Vec::new(),
            assistant: None,
        }
    }
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The assistant handle and directory snapshot are runtime carriers,
        // not identity; everything else is plain data.
        formatter
            .debug_struct("Environment")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("cwd", &self.cwd)
            .field("vars", &self.vars.len())
            .field("root", &self.root)
            .field("platform", &self.platform)
            .field("run_dir", &self.run_dir)
            .finish_non_exhaustive()
    }
}
