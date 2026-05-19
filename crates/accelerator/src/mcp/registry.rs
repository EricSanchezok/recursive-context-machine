use std::sync::Arc;

use tracing::{debug, info};

use super::tool::McpTool;
use super::transport::Transport;

/// Lifecycle manager for MCP servers.
///
/// Starts each server, discovers tools via `tools/list`, and wraps each
/// discovered tool into an `McpTool` that implements `machine::Tool`.
/// The transport is kept alive by `Arc` references from each `McpTool`.
pub struct McpRegistry {
    tools: Vec<Arc<dyn machine::Tool>>,
}

impl McpRegistry {
    /// Start zero or more MCP servers and discover their tools.
    pub async fn start(configs: &[McpServerConfig]) -> Result<Self, String> {
        let mut all_tools: Vec<Arc<dyn machine::Tool>> = Vec::new();

        for config in configs {
            info!(target: "mcp", server = %config.label, "starting MCP server");

            let transport = Transport::spawn(&config.command, &config.args).await?;
            let transport = Arc::new(tokio::sync::Mutex::new(transport));

            // Handshake.
            {
                let t = transport.lock().await;
                t.initialize().await?;
            }

            // Discover tools.
            let tool_defs = {
                let t = transport.lock().await;
                t.list_tools().await?
            };

            for def in &tool_defs {
                let name = def["name"]
                    .as_str()
                    .ok_or_else(|| format!("MCP tool missing 'name': {def}"))?
                    .to_string();
                let description = def["description"].as_str().unwrap_or("").to_string();
                let input_schema = def["inputSchema"].clone();
                let parameters = if input_schema.is_null() {
                    serde_json::json!({
                        "type": "object",
                        "properties": {}
                    })
                } else {
                    input_schema
                };

                debug!(
                    target: "mcp",
                    server = %config.label,
                    tool = %name,
                    "discovered tool"
                );

                all_tools.push(Arc::new(McpTool {
                    name,
                    description,
                    parameters,
                    transport: Arc::clone(&transport),
                }));
            }

            info!(
                target: "mcp",
                server = %config.label,
                tool_count = tool_defs.len(),
                "MCP server ready"
            );
        }

        Ok(Self { tools: all_tools })
    }

    /// All discovered MCP tools, flattened across all servers.
    pub fn tools(&self) -> Vec<Arc<dyn machine::Tool>> {
        self.tools.clone()
    }
}

/// Configuration for launching an MCP server.
///
/// Parsed from CLI `--mcp-server` flags in the form `label=command arg1 arg2`.
#[derive(Debug, Clone)]
pub struct McpServerConfig {
    pub label: String,
    pub command: String,
    pub args: Vec<String>,
}

impl McpServerConfig {
    /// Parse a `label=command arg1 arg2` string.
    ///
    /// Returns `None` if the string doesn't contain a `=` separating
    /// the label from the command line.
    pub fn parse(raw: &str) -> Result<Self, String> {
        let (label, rest) = raw.split_once('=').ok_or_else(|| {
            format!("invalid MCP server format '{raw}' — expected 'label=command arg1 arg2'")
        })?;

        if label.is_empty() {
            return Err("MCP server label cannot be empty".to_string());
        }

        let parts: Vec<&str> = rest.split_whitespace().collect();
        if parts.is_empty() {
            return Err(format!(
                "MCP server '{label}' has no command — expected 'label=command arg1 arg2'"
            ));
        }

        let command = parts[0].to_string();
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

        Ok(Self {
            label: label.to_string(),
            command,
            args,
        })
    }
}
