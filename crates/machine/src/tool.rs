use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use serde_json::Value;

use crate::fragment::ToolResult;

/// Default tool execution timeout in seconds (3 minutes).
const DEFAULT_TOOL_TIMEOUT_SECS: u64 = 180;

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

    /// Maximum time the tool is allowed to run before cancellation.
    ///
    /// Defaults to [`DEFAULT_TOOL_TIMEOUT_SECS`] (3 minutes).
    fn timeout(&self) -> Duration {
        Duration::from_secs(DEFAULT_TOOL_TIMEOUT_SECS)
    }

    /// Execute the tool with the given arguments.
    fn execute<'a>(
        &'a self,
        args: Value,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>>;
}
