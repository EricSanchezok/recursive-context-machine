use std::future::Future;
use std::pin::Pin;

use serde_json::Value;

use crate::fragment::ToolDef;

/// Tool — an executable capability.
///
/// A tool has a schema (`name`, `description`, `parameters`) that is exposed
/// to the language model as a [`ToolDef`] fragment, and an `execute` method
/// that performs the actual work when the model requests a tool call.
///
/// Implementations may wrap rig tools, pure functions, WASM plugins, remote
/// RPCs, or any other execution mechanism — the interface is uniform.
pub trait Tool: Send + Sync {
    /// Unique tool identifier (e.g. `"read_file"`).
    fn name(&self) -> &str;

    /// Natural-language description for the language model.
    fn description(&self) -> &str;

    /// JSON Schema describing the tool's input parameters.
    fn parameters(&self) -> Value;

    /// Execute the tool with the given arguments.
    fn execute<'a>(
        &'a self,
        args: Value,
    ) -> Pin<Box<dyn Future<Output = Result<ToolOutput, String>> + Send + 'a>>;
}

/// The outcome of a tool execution.
#[derive(Debug)]
pub struct ToolOutput {
    /// Textual content returned to the model.
    pub content: String,

    /// Optional short title for display/logging.
    pub title: Option<String>,
}

/// Convert a tool into its definition fragment.
///
/// This is what the Policy inserts into the context so the model
/// knows the tool exists and how to call it.
pub fn tool_def(tool: &dyn Tool) -> ToolDef {
    ToolDef {
        name: tool.name().to_string(),
        description: tool.description().to_string(),
        parameters: tool.parameters(),
    }
}
