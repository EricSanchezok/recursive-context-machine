use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;
use tokio::time::sleep;

pub struct WaitTool;

impl Tool for WaitTool {
    fn name(&self) -> &str {
        "wait"
    }

    fn description(&self) -> &str {
        "Wait for a number of seconds. Use after launching a long-running command in tmux to give it time before reading output."
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "seconds": {
                    "type": "integer",
                    "description": "Seconds to wait (max 3600)."
                }
            },
            "required": ["seconds"]
        })
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(3600)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        _env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let seconds = args["seconds"]
                .as_u64()
                .ok_or("missing required parameter 'seconds'")?
                .min(3600);

            sleep(Duration::from_secs(seconds)).await;

            Ok(ToolResult {
                call_id: String::new(),
                content: format!("waited {seconds}s"),
                title: Some(format!("⏱ {seconds}s")),
            })
        })
    }
}
