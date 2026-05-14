use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Tool — an executable capability.
///
/// Implementations may wrap rig tools, pure functions, WASM plugins, remote
/// RPCs, or any other execution mechanism — the interface is uniform.
pub trait Tool: Send + Sync {
    /// Unique tool name (e.g. `"read_file"`).
    fn name(&self) -> &str;

    /// Natural-language description for the language model.
    fn description(&self) -> &str;

    /// JSON Schema describing the tool's input parameters.
    fn parameters(&self) -> Value;

    /// Execute the tool with the given arguments.
    fn execute<'a>(
        &'a self,
        args: Value,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>>;
}

/// The outcome of a tool execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolResult {
    /// Unique ID matching the tool call that produced this result.
    pub call_id: String,

    /// Textual content returned to the model.
    pub content: String,

    /// Optional short title for display/logging.
    pub title: Option<String>,
}
