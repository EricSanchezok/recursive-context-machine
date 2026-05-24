use accelerator::mcp::{McpServerConfig, McpTransportConfig};
use std::collections::HashMap;
use tonic::Status;

pub fn build_mcp_config(spec: &crate::rcm::McpServerSpec) -> Result<McpServerConfig, Status> {
    let transport = spec
        .transport
        .as_ref()
        .ok_or(Status::invalid_argument("mcp requires transport"))?;
    use crate::rcm::mcp_transport_spec::Kind;
    let transport = match &transport.kind {
        Some(Kind::Stdio(stdio)) => McpTransportConfig::Stdio {
            command: stdio.command.clone(),
            args: stdio.args.clone(),
            env: resolve_mcp_values(&stdio.env)?,
            cwd: stdio.cwd.clone().map(std::path::PathBuf::from),
        },
        Some(Kind::Http(http)) => McpTransportConfig::Http {
            url: http.url.clone(),
            headers: resolve_mcp_pairs(&http.headers)?,
        },
        Some(Kind::Sse(sse)) => McpTransportConfig::Sse {
            url: sse.url.clone(),
            headers: resolve_mcp_pairs(&sse.headers)?,
        },
        None => return Err(Status::invalid_argument("mcp transport kind required")),
    };
    Ok(McpServerConfig {
        label: spec.label.clone(),
        transport,
    })
}

fn resolve_mcp_values(
    values: &HashMap<String, crate::rcm::McpValueSpec>,
) -> Result<HashMap<String, String>, Status> {
    values
        .iter()
        .map(|(k, v)| Ok((k.clone(), resolve_mcp_value(v)?)))
        .collect()
}

fn resolve_mcp_pairs(
    values: &HashMap<String, crate::rcm::McpValueSpec>,
) -> Result<Vec<(String, String)>, Status> {
    values
        .iter()
        .map(|(k, v)| Ok((k.clone(), resolve_mcp_value(v)?)))
        .collect()
}

fn resolve_mcp_value(value: &crate::rcm::McpValueSpec) -> Result<String, Status> {
    use crate::rcm::mcp_value_spec::Source;
    match &value.source {
        Some(Source::Literal(v)) => Ok(v.clone()),
        Some(Source::Env(v)) => std::env::var(v)
            .map_err(|e| Status::invalid_argument(format!("env var '{}' not set: {}", v, e))),
        None => Ok(String::new()),
    }
}
