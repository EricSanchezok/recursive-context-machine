use crate::tool_def::ToolDef;

/// A pool of available tools.
///
/// Cells link to tools from the pool at creation time. Each cell gets its own
/// subset — no global visibility, no cross-cell tool leakage.
///
/// In the future, ToolPool can be backed by an external service (gRPC, Unix
/// socket, WASM interface). For now, it's an in-process registry.
#[derive(Debug, Default)]
pub struct ToolPool {
    tools: Vec<ToolDef>,
}

impl ToolPool {
    pub fn new() -> Self {
        Self { tools: vec![] }
    }

    /// Register a tool in the pool.
    pub fn register(&mut self, tool: ToolDef) {
        self.tools.push(tool);
    }

    /// List all available tools.
    pub fn list(&self) -> &[ToolDef] {
        &self.tools
    }

    /// Get a tool by name.
    pub fn get(&self, name: &str) -> Option<&ToolDef> {
        self.tools.iter().find(|t| t.name == name)
    }

    /// Link a subset of tools to a cell. Returns the selected ToolDefs.
    ///
    /// `names` specifies which tools the cell needs. If empty, links all tools.
    pub fn link(&self, names: &[&str]) -> Vec<ToolDef> {
        if names.is_empty() {
            self.tools.clone()
        } else {
            self.tools
                .iter()
                .filter(|t| names.contains(&t.name.as_str()))
                .cloned()
                .collect()
        }
    }
}

