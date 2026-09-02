use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;
use tokio::sync::Mutex;

use super::transport::Transport;

pub(crate) struct McpTool {
    pub(crate) public_name: String,
    pub(crate) server_name: String,
    pub(crate) description: String,
    pub(crate) parameters: Value,
    pub(crate) transport: Arc<Mutex<Transport>>,
}

impl Tool for McpTool {
    fn name(&self) -> &str {
        &self.public_name
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
                        "name": self.server_name,
                        "arguments": args
                    }),
                )
                .await?;

            let output = format_tool_result(&result)?;
            Ok(ToolResult {
                call_id: String::new(),
                content: output,
                title: Some(self.public_name.clone()),
            })
        })
    }
}

fn format_tool_result(result: &Value) -> Result<String, String> {
    let mut output = String::new();
    if let Some(contents) = result.get("content").and_then(|content| content.as_array()) {
        for entry in contents {
            append_content(&mut output, entry);
        }
    }
    if let Some(structured) = result.get("structuredContent") {
        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str(&structured.to_string());
    }
    if result
        .get("isError")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
    {
        return Err(if output.is_empty() {
            "MCP tool returned an error with no content".to_string()
        } else {
            output
        });
    }
    if output.is_empty() {
        output = "[tool returned no content]".to_string();
    }
    Ok(output)
}

fn append_content(output: &mut String, entry: &Value) {
    if !output.is_empty() {
        output.push('\n');
    }
    match entry.get("type").and_then(|value| value.as_str()) {
        Some("text") => output.push_str(
            entry
                .get("text")
                .and_then(|value| value.as_str())
                .unwrap_or(""),
        ),
        Some("image") | Some("audio") => {
            // Emit as data URL when base64 data is available.
            let mime = entry
                .get("mimeType")
                .and_then(|v| v.as_str())
                .unwrap_or("application/octet-stream");
            // Only allow image/* and audio/* MIME types to prevent injection.
            let is_valid_mime = mime.starts_with("image/") || mime.starts_with("audio/");
            if let Some(data) = entry.get("data").and_then(|v| v.as_str()) {
                if is_valid_mime {
                    output.push_str(&format!("data:{mime};base64,{data}"));
                } else {
                    output.push_str(&format!(
                        "[MCP {} result: invalid mime type]",
                        entry.get("type").and_then(|v| v.as_str()).unwrap_or("")
                    ));
                }
            } else if let Some(url) = entry.get("url").and_then(|v| v.as_str()) {
                output.push_str(url);
            } else {
                output.push_str(&format!(
                    "[MCP {} result]",
                    entry.get("type").and_then(|v| v.as_str()).unwrap_or("")
                ));
            }
        }
        Some("resource_link") => {
            let uri = entry
                .get("uri")
                .and_then(|value| value.as_str())
                .unwrap_or("unknown");
            output.push_str(&format!("[MCP resource link: {uri}]"));
        }
        Some("resource") => output.push_str("[MCP embedded resource]"),
        Some(other) => output.push_str(&format!("[MCP {other} result]")),
        None => output.push_str("[MCP result]"),
    }
}
