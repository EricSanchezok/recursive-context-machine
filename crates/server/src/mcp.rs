use accelerator::mcp::{McpServerConfig, McpTransportConfig};
use std::collections::HashMap;
use tonic::Status;

pub fn mcp_config_from_spec(spec: &crate::rcm::McpServerSpec) -> Result<McpServerConfig, Status> {
    let transport = spec
        .transport
        .as_ref()
        .ok_or(Status::invalid_argument("mcp requires transport"))?;
    use crate::rcm::mcp_transport_spec::Kind;
    let transport = match &transport.kind {
        Some(Kind::Stdio(stdio)) => McpTransportConfig::Stdio {
            command: stdio.command.clone(),
            args: stdio.args.clone(),
            env: mcp_env_from_spec(&stdio.env)?,
            cwd: stdio.cwd.clone().map(std::path::PathBuf::from),
        },
        Some(Kind::Http(http)) => McpTransportConfig::Http {
            url: http.url.clone(),
            headers: mcp_headers_from_spec(&http.headers)?,
        },
        Some(Kind::Sse(sse)) => McpTransportConfig::Sse {
            url: sse.url.clone(),
            headers: mcp_headers_from_spec(&sse.headers)?,
        },
        None => return Err(Status::invalid_argument("mcp transport kind required")),
    };
    Ok(McpServerConfig {
        label: spec.label.clone(),
        transport,
    })
}

fn mcp_env_from_spec(
    values: &HashMap<String, crate::rcm::McpValueSpec>,
) -> Result<HashMap<String, String>, Status> {
    values
        .iter()
        .map(|(name, value)| Ok((name.clone(), mcp_value_text(value)?)))
        .collect()
}

fn mcp_headers_from_spec(
    values: &HashMap<String, crate::rcm::McpValueSpec>,
) -> Result<Vec<(String, String)>, Status> {
    values
        .iter()
        .map(|(name, value)| Ok((name.clone(), mcp_value_text(value)?)))
        .collect()
}

fn mcp_value_text(value: &crate::rcm::McpValueSpec) -> Result<String, Status> {
    use crate::rcm::mcp_value_spec::Source;
    match &value.source {
        Some(Source::Literal(value)) => Ok(value.clone()),
        Some(Source::Env(name)) => std::env::var(name).map_err(|error| {
            Status::invalid_argument(format!("env var '{name}' not set: {error}"))
        }),
        None => Ok(String::new()),
    }
}
