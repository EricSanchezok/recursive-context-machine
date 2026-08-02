use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::env::Environment;
use crate::fragment::ToolResult;

pub const DEFAULT_TOOL_TIMEOUT_SECS: u64 = 1_800;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

impl ToolDefinition {
    pub fn from_tool(tool: &dyn Tool) -> Self {
        Self {
            name: tool.name().to_string(),
            description: tool.description().to_string(),
            parameters: tool.parameters(),
        }
    }
}

pub trait Tool: Send + Sync {
    fn name(&self) -> &str;

    fn description(&self) -> &str;

    fn parameters(&self) -> Value;

    fn timeout(&self) -> Duration {
        Duration::from_secs(DEFAULT_TOOL_TIMEOUT_SECS)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>>;
}

#[cfg(test)]
mod tests {
    use super::DEFAULT_TOOL_TIMEOUT_SECS;

    #[test]
    fn tool_default_timeout_allows_thirty_minute_calls() {
        assert_eq!(DEFAULT_TOOL_TIMEOUT_SECS, 1_800);
    }
}

#[derive(Clone, Default)]
pub struct ToolRuntime {
    executors: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, tool: Arc<dyn Tool>) {
        self.executors.insert(tool.name().to_string(), tool);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.executors.get(name).map(|tool| tool.as_ref())
    }

    pub fn get_arc(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.executors.get(name).cloned()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.executors.contains_key(name)
    }

    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.executors.keys().cloned().collect();
        names.sort();
        names
    }

    pub fn merge(&mut self, other: &ToolRuntime) {
        for (name, tool) in &other.executors {
            self.executors
                .entry(name.clone())
                .or_insert_with(|| tool.clone());
        }
    }
}
