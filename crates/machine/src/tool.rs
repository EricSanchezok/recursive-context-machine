use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use serde_json::Value;

use crate::env::Environment;
use crate::fragment::ToolResult;

const DEFAULT_TIMEOUT_SECS: u64 = 180;

/// Tool — an executable capability.
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
