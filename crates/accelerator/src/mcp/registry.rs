use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use tracing::{debug, info};

use super::http::HttpTransport;
use super::sse::SseTransport;
use super::stdio::StdioTransport;
use super::tool::McpTool;
use super::transport::Transport;

pub struct McpRegistry {
    tools_by_server: HashMap<String, Vec<Arc<dyn machine::Tool>>>,
}

impl McpRegistry {
    pub async fn start(configs: &[McpServerConfig]) -> Result<Self, String> {
        let mut tools_by_server = HashMap::new();

        for config in configs {
            info!(target: "mcp", server = %config.label, "starting MCP server");

            let transport = config.create_transport().await?;
            let transport = Arc::new(tokio::sync::Mutex::new(transport));

            {
                let server = transport.lock().await;
                server.initialize().await?;
            }

            let tool_defs = {
                let server = transport.lock().await;
                server.list_tools().await?
            };

            let mut server_tools: Vec<Arc<dyn machine::Tool>> = Vec::new();
            let mut public_names = HashSet::new();
            for def in &tool_defs {
                let server_name = def["name"]
                    .as_str()
                    .ok_or_else(|| format!("MCP tool missing 'name': {def}"))?
                    .to_string();
                let public_name = public_tool_name(&config.label, &server_name);
                if !public_names.insert(public_name.clone()) {
                    return Err(format!(
                        "duplicate MCP tool name after namespacing: {public_name}"
                    ));
                }
                let description = def["description"].as_str().unwrap_or("").to_string();
                let input_schema = def["inputSchema"].clone();
                let parameters = if input_schema.is_null() {
                    serde_json::json!({ "type": "object", "properties": {} })
                } else {
                    input_schema
                };

                debug!(target: "mcp", server = %config.label, tool = %public_name, "discovered tool");

                server_tools.push(Arc::new(McpTool {
                    public_name,
                    server_name,
                    description,
                    parameters,
                    transport: Arc::clone(&transport),
                }));
            }

            info!(target: "mcp", server = %config.label, tool_count = server_tools.len(), "MCP server ready");

            if tools_by_server
                .insert(config.label.clone(), server_tools)
                .is_some()
            {
                return Err(format!("duplicate MCP server label: {}", config.label));
            }
        }

        Ok(Self { tools_by_server })
    }

    pub fn tools_for(&self, label: &str) -> Option<Vec<Arc<dyn machine::Tool>>> {
        self.tools_by_server.get(label).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct McpServerConfig {
    pub label: String,
    pub transport: McpTransportConfig,
}

#[derive(Debug, Clone)]
pub enum McpTransportConfig {
    Stdio {
        command: String,
        args: Vec<String>,
        env: HashMap<String, String>,
        cwd: Option<PathBuf>,
    },
    Http {
        url: String,
        headers: Vec<(String, String)>,
    },
    Sse {
        url: String,
        headers: Vec<(String, String)>,
    },
}

fn public_tool_name(label: &str, server_name: &str) -> String {
    format!(
        "{}__{}",
        sanitize_tool_name(label),
        sanitize_tool_name(server_name)
    )
}

fn sanitize_tool_name(name: &str) -> String {
    let mut sanitized = name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' || character == '-' {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    if sanitized.is_empty() {
        sanitized.push_str("tool");
    }
    if sanitized.len() > 64 {
        sanitized.truncate(64);
    }
    sanitized
}

impl McpServerConfig {
    async fn create_transport(&self) -> Result<Transport, String> {
        match &self.transport {
            McpTransportConfig::Stdio {
                command,
                args,
                env,
                cwd,
            } => {
                let transport = StdioTransport::spawn(command, args, env, cwd.as_deref()).await?;
                Ok(Transport::Stdio(Box::new(transport)))
            }
            McpTransportConfig::Http { url, headers } => Ok(Transport::Http(HttpTransport::new(
                url.clone(),
                headers.clone(),
            )?)),
            McpTransportConfig::Sse { url, headers } => {
                let transport = SseTransport::connect(url.clone(), headers.clone()).await?;
                Ok(Transport::Sse(Box::new(transport)))
            }
        }
    }
}
