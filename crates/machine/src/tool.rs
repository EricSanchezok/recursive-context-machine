use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::env::Environment;
use crate::fragment::ToolResult;

const DEFAULT_TIMEOUT_SECS: u64 = 180;

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
        Duration::from_secs(DEFAULT_TIMEOUT_SECS)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>>;
}

#[derive(Clone, Default)]
pub struct ToolRuntime {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tool(mut self, tool: Arc<dyn Tool>) -> Self {
        self.insert(tool);
        self
    }

    pub fn insert(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|tool| tool.as_ref())
    }

    pub fn get_arc(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.tools.keys().cloned().collect();
        names.sort();
        names
    }

    pub fn merge(&mut self, other: &ToolRuntime) {
        for (name, tool) in &other.tools {
            self.tools
                .entry(name.clone())
                .or_insert_with(|| tool.clone());
        }
    }
}
