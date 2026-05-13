use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::fragment::ToolDef;

/// Environment — the external world accessible to the machine.
///
/// Carries the working directory, environment variables, runtime
/// configuration (provider, model, parameters), and the set of
/// available tools. The Policy can modify the environment via
/// [`Action::Set`] to switch models or adjust parameters mid-computation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub cwd: String,
    pub vars: HashMap<String, String>,
    pub config: Config,
    pub tools: Vec<ToolDef>,
}

impl Environment {
    pub fn new(cwd: impl Into<String>) -> Self {
        Self {
            cwd: cwd.into(),
            vars: HashMap::new(),
            config: Config::default(),
            tools: Vec::new(),
        }
    }

    /// Set a nested key (e.g. `"config.model"`) to a value.
    pub fn set(&mut self, key: &str, value: Value) {
        match key {
            "cwd" => {
                if let Some(s) = value.as_str() {
                    self.cwd = s.to_string();
                }
            }
            "config.provider" => {
                if let Some(s) = value.as_str() {
                    self.config.provider = s.to_string();
                }
            }
            "config.model" => {
                if let Some(s) = value.as_str() {
                    self.config.model = s.to_string();
                }
            }
            _ => {
                self.config.params.insert(key.to_string(), value);
            }
        }
    }
}

/// Runtime configuration for the language model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub provider: String,
    pub model: String,
    pub params: HashMap<String, Value>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: String::new(),
            model: String::new(),
            params: HashMap::new(),
        }
    }
}
