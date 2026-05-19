use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;
use tokio::sync::Mutex;

use super::transport::Transport;

/// A tool discovered from an MCP server.
///
/// Wraps the server's tool description and delegates `execute` to the
/// `tools/call` JSON-RPC method. The transport is shared — all tools
/// from the same server use the same connection.
pub(crate) struct McpTool {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) parameters: Value,
    pub(crate) transport: Arc<Mutex<Transport>>,
}

impl Tool for McpTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn parameters(&self) -> Value {
        self.parameters.clone()
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        _env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let transport = self.transport.lock().await;
            let result = transport
                .call(
                    "tools/call",
                    serde_json::json!({
                        "name": self.name,
                        "arguments": args
                    }),
                )
                .await?;

            let mut output = String::new();
            if let Some(contents) = result.get("content").and_then(|c| c.as_array()) {
                for entry in contents {
                    if let Some(text) = entry.get("text").and_then(|t| t.as_str()) {
                        if !output.is_empty() {
                            output.push('\n');
                        }
                        output.push_str(text);
                    }
                    match entry.get("type").and_then(|t| t.as_str()) {
                        Some("image") => output.push_str("[MCP image result]"),
                        Some("resource") => {
                            if let Some(uri) = entry.get("uri").and_then(|u| u.as_str()) {
                                output.push_str(&format!("[MCP resource: {uri}]"));
                            } else {
                                output.push_str("[MCP resource result]");
                            }
                        }
                        _ => {}
                    }
                }
            }

            if let Some(is_error) = result.get("isError").and_then(|e| e.as_bool()) {
                if is_error {
                    return Err(if output.is_empty() {
                        "MCP tool returned an error with no text content".to_string()
                    } else {
                        output
                    });
                }
            }

            if output.is_empty() {
                output = "[tool returned no content]".to_string();
            }

            Ok(ToolResult {
                call_id: String::new(),
                content: output,
                title: Some(self.name.clone()),
            })
        })
    }
}
