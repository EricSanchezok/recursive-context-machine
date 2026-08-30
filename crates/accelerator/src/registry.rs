//! Runtime resource registry — the Resource-OS table (design doc
//! `docs/design/resource-os-v0.md`, R1).
//!
//! Everything except the model is a resource: tools, prompts, MCP servers.
//! The registry is the source of truth; `RunState.resources` and the
//! `ToolRuntime` are per-step projections of it. Mutations arrive through
//! the `resources` tool (the model writes structure, code guards the
//! invariants) and land in a pending queue; the fire loop drains the queue
//! after every apply, so a registered tool is callable on the next step.
//!
//! Persistence follows the ledger's pattern: a process-global table keyed
//! by run directory, written through to `<run_dir>/resources/registry.json`
//! when a run directory exists, ephemeral otherwise. Single-process v0
//! semantics (same caveat as the ledger); multi-process deployments need a
//! single-writer convention on the registry file.

use std::collections::HashMap;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Arc, Mutex, OnceLock};

use machine::{RegistryEvent, ToolDefinition, ToolResult, ToolRuntime};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::warn;

const REGISTRY_FILE_NAME: &str = "registry.json";
/// Prefix required for agent-registered resources, keeping the namespace
/// of hand-written (seed) resources untouched.
pub const GENERATED_PREFIX: &str = "gen/";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRecord {
    pub kind: String,
    pub name: String,
    pub manifest: Value,
    /// seed (hand-written) | generated (agent-created) | evolved (promoted).
    pub provenance: String,
    pub created_at: String,
}

/// One queued mutation, applied by the fire loop's drain pass.
pub enum RegistryMutation {
    InstallTool {
        definition: ToolDefinition,
        executor: Arc<dyn machine::Tool>,
    },
    InstallPrompt {
        name: String,
        body: String,
    },
    RetireTool {
        name: String,
    },
    RetirePrompt {
        name: String,
    },
}

struct RegistryState {
    records: HashMap<String, ResourceRecord>,
    pending: Vec<RegistryMutation>,
}

fn registries() -> &'static Mutex<HashMap<PathBuf, RegistryState>> {
    static REGISTRIES: OnceLock<Mutex<HashMap<PathBuf, RegistryState>>> = OnceLock::new();
    REGISTRIES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn registry_path(run_dir: Option<&Path>) -> PathBuf {
    run_dir
        .map(|dir| dir.join("resources").join(REGISTRY_FILE_NAME))
        .unwrap_or_else(|| PathBuf::from(REGISTRY_FILE_NAME))
}

fn cache_key(run_dir: Option<&Path>) -> PathBuf {
    run_dir.map(Path::to_path_buf).unwrap_or_default()
}

fn load_records(run_dir: Option<&Path>) -> HashMap<String, ResourceRecord> {
    let Some(dir) = run_dir else {
        return HashMap::new();
    };
    let path = registry_path(Some(dir));
    match std::fs::read_to_string(&path) {
        Ok(raw) => serde_json::from_str(&raw).unwrap_or_else(|error| {
            warn!(
                path = %path.display(),
                ?error,
                "resource registry unreadable; starting empty"
            );
            HashMap::new()
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => HashMap::new(),
        Err(error) => {
            warn!(
                path = %path.display(),
                ?error,
                "resource registry access failed; starting empty"
            );
            HashMap::new()
        }
    }
}

fn persist_records(run_dir: Option<&Path>, records: &HashMap<String, ResourceRecord>) {
    let Some(dir) = run_dir else {
        return;
    };
    let resources_dir = dir.join("resources");
    if let Err(error) = std::fs::create_dir_all(&resources_dir) {
        warn!(
            dir = %resources_dir.display(),
            ?error,
            "resource registry directory creation failed"
        );
        return;
    }
    let path = resources_dir.join(REGISTRY_FILE_NAME);
    match serde_json::to_string_pretty(records) {
        Ok(raw) => {
            if let Err(error) = std::fs::write(&path, raw) {
                warn!(
                    path = %path.display(),
                    ?error,
                    "resource registry persist failed; memory stays authoritative"
                );
            }
        }
        Err(error) => warn!(?error, "resource registry serialization failed"),
    }
}

fn ensure_state<'a>(
    registries: &'a mut HashMap<PathBuf, RegistryState>,
    run_dir: Option<&Path>,
) -> &'a mut RegistryState {
    registries
        .entry(cache_key(run_dir))
        .or_insert_with(|| RegistryState {
            records: load_records(run_dir),
            pending: Vec::new(),
        })
}

/// Registry digest for the observation channel. `None` until the first
/// resource exists — no phantom "empty registry" in every observation.
pub fn digest_for(run_dir: Option<&Path>) -> Option<machine::ResourceDigest> {
    let mut registries = registries().lock().expect("registry lock poisoned");
    let state = registries.get_mut(&cache_key(run_dir))?;
    if state.records.is_empty() {
        return None;
    }
    let mut by_kind: HashMap<String, u64> = HashMap::new();
    for record in state.records.values() {
        *by_kind.entry(record.kind.clone()).or_default() += 1;
    }
    let mut names: Vec<String> = state.records.keys().cloned().collect();
    names.sort();
    Some(machine::ResourceDigest {
        total: state.records.len() as u64,
        by_kind,
        names,
    })
}

/// Apply every queued mutation to the run's resource projections. Called by
/// the fire loop after each apply. Retiring a tool that is still active is
/// refused — visibility is the policy's decision (Deactivate first).
pub fn drain(
    run_dir: Option<&Path>,
    resources: &mut machine::Resources,
    tool_runtime: &mut ToolRuntime,
) -> Vec<RegistryEvent> {
    let mut events = Vec::new();
    let mutations = {
        let mut registries = registries().lock().expect("registry lock poisoned");
        match registries.get_mut(&cache_key(run_dir)) {
            Some(state) => std::mem::take(&mut state.pending),
            None => return events,
        }
    };
    for mutation in mutations {
        match mutation {
            RegistryMutation::InstallTool {
                definition,
                executor,
            } => {
                events.push(RegistryEvent {
                    op: "install".into(),
                    kind: "tool".into(),
                    name: definition.name.clone(),
                });
                resources
                    .tool_definitions
                    .insert(definition.name.clone(), definition);
                tool_runtime.insert(executor);
            }
            RegistryMutation::InstallPrompt { name, body } => {
                events.push(RegistryEvent {
                    op: "install".into(),
                    kind: "prompt".into(),
                    name: name.clone(),
                });
                resources.prompts.insert(name, body);
            }
            RegistryMutation::RetireTool { name } => {
                if resources.active_tools.contains(&name) {
                    // Record the refusal so the trajectory shows it; keep the
                    // definition so Deactivate remains meaningful.
                    events.push(RegistryEvent {
                        op: "retire_refused_active".into(),
                        kind: "tool".into(),
                        name,
                    });
                    continue;
                }
                events.push(RegistryEvent {
                    op: "retire".into(),
                    kind: "tool".into(),
                    name: name.clone(),
                });
                resources.tool_definitions.remove(&name);
                tool_runtime.remove(&name);
            }
            RegistryMutation::RetirePrompt { name } => {
                events.push(RegistryEvent {
                    op: "retire".into(),
                    kind: "prompt".into(),
                    name: name.clone(),
                });
                resources.prompts.remove(&name);
            }
        }
    }
    events
}

/// The `resources` tool: runtime CRUD over the registry. The model writes
/// structure; the guards hold the invariants (gen/ namespace, duplicate
/// refusal, seed resources are never retired — deactivate instead).
pub struct ResourcesTool;

impl machine::Tool for ResourcesTool {
    fn name(&self) -> &str {
        "resources"
    }

    fn description(&self) -> &str {
        "Runtime resource registry: inspect and extend your own harness. Ops: \
         list (all resources), describe (one manifest), attach_mcp (connect an \
         MCP server at runtime; its tools become activatable), register_prompt \
         (add a prompt slot usable by Replace/Insert), retire (remove an unused \
         generated resource). Agent-created names must start with gen/."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "op": {
                    "type": "string",
                    "enum": ["list", "describe", "attach_mcp", "register_prompt", "retire"]
                },
                "name": {"type": "string", "description": "Resource name (describe/retire, or the new name for register_prompt)."},
                "body": {"type": "string", "description": "Prompt body (register_prompt)."},
                "label": {"type": "string", "description": "MCP server label (attach_mcp); tools are named <label>__<tool>."},
                "transport": {
                    "type": "object",
                    "description": "MCP transport (attach_mcp): {stdio: {command, args, env}} or {http: {url}}.",
                    "properties": {
                        "stdio": {
                            "type": "object",
                            "properties": {
                                "command": {"type": "string"},
                                "args": {"type": "array", "items": {"type": "string"}},
                                "env": {"type": "object", "additionalProperties": {"type": "string"}}
                            },
                            "required": ["command"]
                        },
                        "http": {
                            "type": "object",
                            "properties": {"url": {"type": "string"}},
                            "required": ["url"]
                        }
                    }
                }
            },
            "required": ["op"]
        })
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a machine::Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        let args = args.clone();
        let env = env.clone();
        Box::pin(async move {
            let run_dir = env.run_dir.clone();
            let operation = args["op"]
                .as_str()
                .ok_or("missing required parameter 'op'")?;

            let (events, payload) = match operation {
                "list" => {
                    let listing = list_records(run_dir.as_deref());
                    (Vec::new(), json!({"op": "list", "resources": listing}))
                }
                "describe" => {
                    let name = args["name"].as_str().ok_or("describe requires 'name'")?;
                    let record = find_record(run_dir.as_deref(), name)?;
                    (Vec::new(), json!({"op": "describe", "resource": record}))
                }
                "attach_mcp" => {
                    let label = args["label"]
                        .as_str()
                        .ok_or("attach_mcp requires 'label'")?
                        .to_string();
                    let config = mcp_config_from_args(&args)?;
                    let (events, tools) = attach_mcp(run_dir.as_deref(), &label, config).await?;
                    (
                        events,
                        json!({"op": "attach_mcp", "label": label, "tools": tools}),
                    )
                }
                "register_prompt" => {
                    let name = args["name"]
                        .as_str()
                        .ok_or("register_prompt requires 'name'")?
                        .to_string();
                    let body = args["body"]
                        .as_str()
                        .ok_or("register_prompt requires 'body'")?
                        .to_string();
                    let event = register_prompt(run_dir.as_deref(), &name, body)?;
                    (vec![event], json!({"op": "register_prompt", "name": name}))
                }
                "retire" => {
                    let name = args["name"]
                        .as_str()
                        .ok_or("retire requires 'name'")?
                        .to_string();
                    let event = retire(run_dir.as_deref(), &name)?;
                    (vec![event], json!({"op": "retire", "name": name}))
                }
                other => return Err(format!("unknown resources op '{other}'")),
            };

            let content = json!({
                "tool": "resources",
                "op": operation,
                "events": events,
                "result": payload,
            })
            .to_string();
            Ok(ToolResult {
                call_id: String::new(),
                content,
                title: Some(format!("resources {operation}")),
            })
        })
    }
}

fn list_records(run_dir: Option<&Path>) -> Vec<ResourceRecord> {
    let registries = registries().lock().expect("registry lock poisoned");
    match registries.get(&cache_key(run_dir)) {
        Some(state) => {
            let mut records: Vec<ResourceRecord> = state.records.values().cloned().collect();
            records.sort_by(|left, right| left.name.cmp(&right.name));
            records
        }
        None => Vec::new(),
    }
}

fn find_record(run_dir: Option<&Path>, name: &str) -> Result<ResourceRecord, String> {
    let registries = registries().lock().expect("registry lock poisoned");
    registries
        .get(&cache_key(run_dir))
        .and_then(|state| state.records.get(name))
        .cloned()
        .ok_or_else(|| format!("resource '{name}' not found"))
}

fn now_timestamp() -> String {
    chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
}

fn mcp_config_from_args(args: &Value) -> Result<crate::mcp::McpServerConfig, String> {
    use crate::mcp::{McpServerConfig, McpTransportConfig};

    let transport = args
        .get("transport")
        .ok_or("attach_mcp requires 'transport'")?;
    if let Some(stdio) = transport.get("stdio") {
        let command = stdio
            .get("command")
            .and_then(Value::as_str)
            .ok_or("stdio transport requires 'command'")?
            .to_string();
        let args_list = stdio
            .get("args")
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(Value::as_str)
                    .map(String::from)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let mut env_map = HashMap::new();
        if let Some(vars) = stdio.get("env").and_then(Value::as_object) {
            for (key, value) in vars {
                if let Some(text) = value.as_str() {
                    env_map.insert(key.clone(), text.to_string());
                }
            }
        }
        return Ok(McpServerConfig {
            label: String::new(),
            transport: McpTransportConfig::Stdio {
                command,
                args: args_list,
                env: env_map,
                cwd: None,
            },
        });
    }
    if let Some(http) = transport.get("http") {
        let url = http
            .get("url")
            .and_then(Value::as_str)
            .ok_or("http transport requires 'url'")?
            .to_string();
        return Ok(McpServerConfig {
            label: String::new(),
            transport: McpTransportConfig::Http {
                url,
                headers: Vec::new(),
            },
        });
    }
    Err("transport must be {stdio: {...}} or {http: {url}}".into())
}

/// Connect an MCP server at runtime and queue its tools. The capability
/// probe IS the connection: a server that cannot initialize or list tools
/// never reaches the registry. Duplicate labels are refused before any
/// connection attempt.
async fn attach_mcp(
    run_dir: Option<&Path>,
    label: &str,
    mut config: crate::mcp::McpServerConfig,
) -> Result<(Vec<RegistryEvent>, Vec<String>), String> {
    config.label = label.to_string();
    let server_record_name = format!("{GENERATED_PREFIX}mcp/{label}");

    // Refuse duplicates before spawning anything.
    {
        let mut registries = registries().lock().expect("registry lock poisoned");
        let state = ensure_state(&mut registries, run_dir);
        if state.records.contains_key(&server_record_name) {
            return Err(format!("MCP server '{label}' already attached"));
        }
    }

    let registry = crate::mcp::McpRegistry::start(&[config]).await?;
    let tools = registry
        .tools_for(label)
        .ok_or_else(|| format!("MCP server '{label}' produced no tools"))?;

    let mut events = Vec::new();
    let mut tool_names = Vec::new();
    let mut registries = registries().lock().expect("registry lock poisoned");
    let state = ensure_state(&mut registries, run_dir);
    for tool in tools {
        let definition = ToolDefinition::from_tool(tool.as_ref());
        tool_names.push(definition.name.clone());
        state.pending.push(RegistryMutation::InstallTool {
            definition,
            executor: tool,
        });
    }
    let record = ResourceRecord {
        kind: "mcp-server".into(),
        name: server_record_name.clone(),
        manifest: json!({
            "label": label,
            "tools": tool_names,
        }),
        provenance: "generated".into(),
        created_at: now_timestamp(),
    };
    state
        .records
        .insert(server_record_name.clone(), record.clone());
    persist_records(run_dir, &state.records);
    events.push(RegistryEvent {
        op: "register".into(),
        kind: record.kind,
        name: record.name,
    });
    Ok((events, tool_names))
}

fn register_prompt(
    run_dir: Option<&Path>,
    name: &str,
    body: String,
) -> Result<RegistryEvent, String> {
    if !name.starts_with(GENERATED_PREFIX) {
        return Err(format!(
            "agent-registered resources must be named with the '{GENERATED_PREFIX}' prefix: {name}"
        ));
    }
    let record = ResourceRecord {
        kind: "prompt".into(),
        name: name.to_string(),
        manifest: json!({
            "body_chars": body.len(),
            "token_estimate": body.len() / 4,
        }),
        provenance: "generated".into(),
        created_at: now_timestamp(),
    };
    let mut registries = registries().lock().expect("registry lock poisoned");
    let state = ensure_state(&mut registries, run_dir);
    if state.records.contains_key(&record.name) {
        return Err(format!("resource '{}' already registered", record.name));
    }
    state.records.insert(record.name.clone(), record.clone());
    // The registry file stores only the manifest; the body rides the
    // pending queue to the fire loop, which projects it into
    // resources.prompts (usable by Replace/Insert on the next step).
    state.pending.push(RegistryMutation::InstallPrompt {
        name: record.name.clone(),
        body,
    });
    persist_records(run_dir, &state.records);
    Ok(RegistryEvent {
        op: "register".into(),
        kind: record.kind,
        name: record.name,
    })
}

fn retire(run_dir: Option<&Path>, name: &str) -> Result<RegistryEvent, String> {
    let mut registries = registries().lock().expect("registry lock poisoned");
    let key = cache_key(run_dir);
    let state = registries
        .get_mut(&key)
        .ok_or_else(|| format!("resource '{name}' not found"))?;
    let record = state
        .records
        .get(name)
        .ok_or_else(|| format!("resource '{name}' not found"))?
        .clone();
    if record.provenance == "seed" {
        return Err(format!(
            "refusing to retire seed resource '{name}'; deactivate it instead"
        ));
    }
    state.records.remove(name);
    if record.kind == "mcp-server" {
        // Retiring the server record retires its tools by name; each is
        // refused individually at drain time if still active.
        if let Some(tool_names) =
            record
                .manifest
                .get("tools")
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(Value::as_str)
                        .map(String::from)
                        .collect::<Vec<_>>()
                })
        {
            for tool_name in tool_names {
                state
                    .pending
                    .push(RegistryMutation::RetireTool { name: tool_name });
            }
        }
    }
    if record.kind == "prompt" {
        state.pending.push(RegistryMutation::RetirePrompt {
            name: record.name.clone(),
        });
    }
    persist_records(run_dir, &state.records);
    Ok(RegistryEvent {
        op: "retire".into(),
        kind: record.kind,
        name: record.name,
    })
}
