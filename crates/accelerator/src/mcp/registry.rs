use std::sync::Arc;

use tracing::{debug, info};

use super::http::HttpTransport;
use super::stdio::StdioTransport;
use super::tool::McpTool;
use super::transport::Transport;

/// Lifecycle manager for MCP servers.
pub struct McpRegistry {
    tools: Vec<Arc<dyn machine::Tool>>,
}

impl McpRegistry {
    /// Start all configured MCP servers and discover their tools.
    ///
    /// Detects the transport type automatically:
    /// - URLs starting with `http://` or `https://` → Streamable HTTP
    /// - Anything else → stdio subprocess
    pub async fn start(configs: &[McpServerConfig]) -> Result<Self, String> {
        let mut all_tools: Vec<Arc<dyn machine::Tool>> = Vec::new();

        for config in configs {
            info!(target: "mcp", server = %config.label, "starting MCP server");

            let transport = config.create_transport().await?;
            let transport = Arc::new(tokio::sync::Mutex::new(transport));

            {
                let t = transport.lock().await;
                t.initialize().await?;
            }

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
/// Supports two transport types, auto-detected from the value:
/// - `label=command arg1 arg2` → stdio subprocess
/// - `label=https://example.com/path` → Streamable HTTP
///
/// For HTTP servers, use `--mcp-header` to add custom headers.
#[derive(Debug, Clone)]
pub struct McpServerConfig {
    pub label: String,
    pub command: Option<String>,
    pub args: Vec<String>,
    pub url: Option<String>,
    pub headers: Vec<(String, String)>,
}

impl McpServerConfig {
    /// Parse a `label=value` string where value is either a command line
    /// or an HTTP(S) URL.
    pub fn parse(raw: &str) -> Result<Self, String> {
        let (label, rest) = raw.split_once('=').ok_or_else(|| {
            format!(
                "invalid MCP server format '{raw}' — expected 'label=command' or 'label=https://...'"
            )
        })?;

        if label.is_empty() {
            return Err("MCP server label cannot be empty".to_string());
        }
        if rest.is_empty() {
            return Err(format!("MCP server '{label}' has no value"));
        }

        let trimmed = rest.trim();
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            // Format: url|HeaderName:Value|HeaderName:Value
            // `|` is used instead of `,` because neither URLs nor
            // typical header values contain pipe characters.
            let mut headers = Vec::new();
            let url = if let Some((url_part, header_part)) = trimmed.split_once('|') {
                for h in header_part.split('|') {
                    let h = h.trim();
                    if let Some((key, value)) = h.split_once(':') {
                        headers.push((key.trim().to_string(), value.trim().to_string()));
                    }
                }
                url_part.to_string()
            } else {
                trimmed.to_string()
            };

            Ok(Self {
                label: label.to_string(),
                command: None,
                args: Vec::new(),
                url: Some(url),
                headers,
            })
        } else {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.is_empty() {
                return Err(format!("MCP server '{label}' has no command or URL"));
            }
            Ok(Self {
                label: label.to_string(),
                command: Some(parts[0].to_string()),
                args: parts[1..].iter().map(|s| s.to_string()).collect(),
                url: None,
                headers: Vec::new(),
            })
        }
    }

    /// Create the transport for this configuration.
    async fn create_transport(&self) -> Result<Transport, String> {
        match (&self.command, &self.url) {
            (Some(cmd), None) => {
                let t = StdioTransport::spawn(cmd, &self.args).await?;
                Ok(Transport::Stdio(t))
            }
            (None, Some(url)) => {
                let t = HttpTransport::new(url.clone(), self.headers.clone());
                Ok(Transport::Http(t))
            }
            _ => Err("McpServerConfig has neither command nor url".to_string()),
        }
    }
}
