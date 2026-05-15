use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use serde_json::Value;

use crate::fragment::ToolResult;

const DEFAULT_TIMEOUT_SECS: u64 = 180;

/// Tool — an executable capability.
pub trait Tool: Send + Sync {
    /// Unique tool name (e.g. `"read_file"`).
    fn name(&self) -> &str;

    /// Natural-language description for the language model.
    fn description(&self) -> &str;

    /// JSON Schema describing the tool's input parameters.
    fn parameters(&self) -> Value;

    /// Maximum execution time before cancellation.
    fn timeout(&self) -> Duration {
        Duration::from_secs(DEFAULT_TIMEOUT_SECS)
    }

    /// Execute the tool.
    fn execute<'a>(
        &'a self,
        args: Value,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>>;
}
