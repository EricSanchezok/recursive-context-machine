use std::path::Path;

use accelerator::Catalog;

use crate::args::InventoryArgs;

pub fn run(args: InventoryArgs) -> anyhow::Result<()> {
    let catalog = Catalog::new();
    let project = args.project;

    let policies = catalog.policy_names();
    let builtin_tools = catalog.tool_names();
    let builtin_prompts = catalog.prompt_names();

    let mcp_servers = discover_mcp_servers(&project);
    let external_models = discover_models(&project);
    let external_prompts = discover_prompts(&project);

    let inventory = serde_json::json!({
        "policies": policies,
        "tools": {
            "builtin": builtin_tools,
            "mcp_servers": mcp_servers
        },
        "prompts": {
            "builtin": builtin_prompts,
            "external": external_prompts
        },
        "models": {
            "external": external_models
        }
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&inventory).map_err(anyhow::Error::msg)?
    );
    Ok(())
}

fn discover_mcp_servers(project: &Path) -> Vec<serde_json::Value> {
    let mut servers = Vec::new();
    let rcm_dir = project.join("rcm");
    if !rcm_dir.is_dir() {
        return servers;
    }
    let Ok(entries) = std::fs::read_dir(&rcm_dir) else {
        return servers;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rcm") {
            continue;
        }
        let Ok(source) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(file) = crate::rcm::parse(&source) else {
            continue;
        };
        for mcp in file.mcps {
            servers.push(serde_json::json!({
                "label": mcp.label,
                "transport": mcp_transport_json(&mcp.transport)
            }));
        }
    }
    servers
}

fn mcp_transport_json(transport: &crate::rcm::McpTransportDef) -> serde_json::Value {
    match transport {
        crate::rcm::McpTransportDef::Stdio { command, args, .. } => {
            serde_json::json!({
                "type": "stdio",
                "command": command,
                "args": args,
            })
        }
        crate::rcm::McpTransportDef::Http { url, .. } => {
            serde_json::json!({
                "type": "http",
                "url": url,
            })
        }
        crate::rcm::McpTransportDef::Sse { url, .. } => {
            serde_json::json!({
                "type": "sse",
                "url": url,
            })
        }
    }
}

fn discover_models(project: &Path) -> Vec<serde_json::Value> {
    let mut models = Vec::new();
    let rcm_dir = project.join("rcm");
    if !rcm_dir.is_dir() {
        return models;
    }
    let Ok(entries) = std::fs::read_dir(&rcm_dir) else {
        return models;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rcm") {
            continue;
        }
        let Ok(source) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(file) = crate::rcm::parse(&source) else {
            continue;
        };
        for model in file.models {
            models.push(serde_json::json!({
                "name": model.id,
                "protocol": model.protocol,
                "endpoint": model.endpoint,
            }));
        }
    }
    models
}

fn discover_prompts(project: &Path) -> Vec<serde_json::Value> {
    let mut prompts = Vec::new();
    let prompts_dir = project.join("prompts");
    if !prompts_dir.is_dir() {
        return prompts;
    }
    let Ok(entries) = std::fs::read_dir(&prompts_dir) else {
        return prompts;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_stem().and_then(|n| n.to_str()) else {
            continue;
        };
        let Ok(content) = std::fs::read_to_string(&path) else {
            continue;
        };
        let preview: String = content.chars().take(80).collect();
        prompts.push(serde_json::json!({
            "name": name,
            "path": path.to_string_lossy(),
            "preview": preview,
        }));
    }
    prompts
}
