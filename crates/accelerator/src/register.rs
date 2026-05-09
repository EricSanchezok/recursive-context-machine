use crate::tool::Tool;
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

/// The state register of a Rica — holds the machine's current configuration.
///
/// Tools are assembled here (not on the tape). Environment isolation
/// happens by constructing a child Register with different cwd / env / tools.
#[derive(Debug, Clone)]
pub struct Register {
    pub cwd: PathBuf,
    pub env: HashMap<String, String>,
    pub tools: Vec<Tool>,
    pub state: HashMap<String, Value>,
}

impl Register {
    pub fn new(cwd: PathBuf) -> Self {
        Self {
            cwd,
            env: HashMap::new(),
            tools: Vec::new(),
            state: HashMap::new(),
        }
    }

    /// Create a child register that inherits cwd, env, and tools
    /// but starts with a fresh state map.
    pub fn child(&self) -> Self {
        Self {
            cwd: self.cwd.clone(),
            env: self.env.clone(),
            tools: self.tools.clone(),
            state: HashMap::new(),
        }
    }

    pub fn with_tools(mut self, names: &[&str]) -> Self {
        self.tools.retain(|t| names.contains(&t.name.as_str()));
        self
    }

    pub fn with_cwd(mut self, cwd: PathBuf) -> Self {
        self.cwd = cwd;
        self
    }

    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(key.into(), value.into());
        self
    }

    pub fn with_state(mut self, key: impl Into<String>, value: Value) -> Self {
        self.state.insert(key.into(), value);
        self
    }
}
