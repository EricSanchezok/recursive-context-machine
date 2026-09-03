use accelerator::registry::{GENERATED_PREFIX, ResourcesTool};
use machine::{Environment, Tool, ToolRuntime};

/// Minimal MCP stdio server: one tool `echo` that returns its input.
/// Speaks the newline-delimited JSON-RPC 2.0 subset McpRegistry uses
/// (initialize / tools/list / tools/call).
const MCP_ECHO_SERVER: &str = r#"
import json, sys

def respond(req_id, result):
    sys.stdout.write(json.dumps({"jsonrpc": "2.0", "id": req_id, "result": result}) + "\n")
    sys.stdout.flush()

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    req = json.loads(line)
    method, req_id = req.get("method"), req.get("id")
    if method == "initialize":
        respond(req_id, {"protocolVersion": "2025-06-18", "capabilities": {}, "serverInfo": {"name": "echo", "version": "0"}})
    elif method == "tools/list":
        respond(req_id, {"tools": [{
            "name": "echo",
            "description": "echo the input back",
            "inputSchema": {"type": "object", "properties": {"text": {"type": "string"}}, "required": ["text"]},
        }]})
    elif method == "tools/call":
        text = req["params"]["arguments"].get("text", "")
        respond(req_id, {"content": [{"type": "text", "text": f"echo: {text}"}]})
"#;

fn run_dir_env(dir: &std::path::Path) -> Environment {
    let mut env = Environment::empty(dir);
    env.run_dir = Some(dir.to_path_buf());
    env
}

async fn call(
    tool: &ResourcesTool,
    args: serde_json::Value,
    env: &Environment,
) -> Result<String, String> {
    tool.execute(args, env).await.map(|result| result.content)
}

fn payload_of(content: &str) -> serde_json::Value {
    let value: serde_json::Value = serde_json::from_str(content).unwrap();
    value["result"].clone()
}

fn events_of(content: &str) -> Vec<(String, String, String)> {
    let value: serde_json::Value = serde_json::from_str(content).unwrap();
    value["events"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            (
                entry["op"].as_str().unwrap().to_string(),
                entry["kind"].as_str().unwrap().to_string(),
                entry["name"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

#[tokio::test]
async fn attach_mcp_installs_tools_through_drain() {
    let dir = tempfile::tempdir().unwrap();
    // Unique registry per test: same temp dir is both run_dir and registry
    // key, so parallel tests never share the process-global table.
    let env = run_dir_env(dir.path());
    let tool = ResourcesTool;

    // Write the echo server script into the run dir.
    let server_path = dir.path().join("echo_server.py");
    std::fs::write(&server_path, MCP_ECHO_SERVER).unwrap();

    let attached = call(
        &tool,
        serde_json::json!({
            "op": "attach_mcp",
            "label": "echo",
            "transport": {"stdio": {"command": "python3", "args": [server_path.to_string_lossy()]}}
        }),
        &env,
    )
    .await
    .unwrap();

    let events = events_of(&attached);
    assert_eq!(
        events,
        vec![(
            "register".into(),
            "mcp-server".into(),
            format!("{GENERATED_PREFIX}mcp/echo")
        )],
        "attach must register exactly one server record"
    );
    let payload = payload_of(&attached);
    let installed: Vec<&str> = payload["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| name.as_str().unwrap())
        .collect();
    assert_eq!(installed, vec!["echo__echo"]);

    // Drain projects the queued tools into Resources + ToolRuntime.
    let mut resources = machine::Resources::new();
    let mut tool_runtime = ToolRuntime::new();
    let drain_events =
        accelerator::registry::drain(Some(dir.path()), &mut resources, &mut tool_runtime);
    assert!(resources.tool_definitions.contains_key("echo__echo"));
    assert!(tool_runtime.contains("echo__echo"));
    assert_eq!(drain_events.len(), 1);
    assert_eq!(drain_events[0].op, "install");

    // The installed tool is actually callable through the runtime.
    let echo = tool_runtime.get_arc("echo__echo").unwrap();
    let result = echo
        .execute(serde_json::json!({"text": "self-evolution"}), &env)
        .await
        .unwrap();
    assert!(result.content.contains("echo: self-evolution"));
}

#[tokio::test]
async fn attach_mcp_refuses_duplicate_labels() {
    let dir = tempfile::tempdir().unwrap();
    let env = run_dir_env(dir.path());
    let tool = ResourcesTool;
    let server_path = dir.path().join("echo_server.py");
    std::fs::write(&server_path, MCP_ECHO_SERVER).unwrap();

    let args = serde_json::json!({
        "op": "attach_mcp",
        "label": "dupe",
        "transport": {"stdio": {"command": "python3", "args": [server_path.to_string_lossy()]}}
    });
    call(&tool, args.clone(), &env).await.unwrap();
    let error = call(&tool, args, &env).await.unwrap_err();
    assert!(error.contains("already attached"), "got: {error}");
}

#[tokio::test]
async fn register_prompt_requires_gen_prefix_and_projects_on_drain() {
    let dir = tempfile::tempdir().unwrap();
    let env = run_dir_env(dir.path());
    let tool = ResourcesTool;

    let error = call(
        &tool,
        serde_json::json!({"op": "register_prompt", "name": "style", "body": "be terse"}),
        &env,
    )
    .await
    .unwrap_err();
    assert!(
        error.contains("gen/"),
        "guard must demand the prefix: {error}"
    );

    let registered = call(
        &tool,
        serde_json::json!({"op": "register_prompt", "name": "gen/style", "body": "be terse"}),
        &env,
    )
    .await
    .unwrap();
    assert_eq!(
        events_of(&registered),
        vec![("register".into(), "prompt".into(), "gen/style".into())]
    );

    let mut resources = machine::Resources::new();
    let mut tool_runtime = ToolRuntime::new();
    accelerator::registry::drain(Some(dir.path()), &mut resources, &mut tool_runtime);
    assert_eq!(
        resources.prompts.get("gen/style").map(String::as_str),
        Some("be terse")
    );
}

#[tokio::test]
async fn retire_generated_prompt_and_refuse_unknown_names() {
    let dir = tempfile::tempdir().unwrap();
    let env = run_dir_env(dir.path());

    // A prompt resource (generated) can be retired.
    call(
        &ResourcesTool,
        serde_json::json!({"op": "register_prompt", "name": "gen/tmp", "body": "x"}),
        &env,
    )
    .await
    .unwrap();
    let retired = call(
        &ResourcesTool,
        serde_json::json!({"op": "retire", "name": "gen/tmp"}),
        &env,
    )
    .await
    .unwrap();
    assert_eq!(
        events_of(&retired),
        vec![("retire".into(), "prompt".into(), "gen/tmp".into())]
    );

    // Retiring an unknown name errors.
    let error = call(
        &ResourcesTool,
        serde_json::json!({"op": "retire", "name": "gen/ghost"}),
        &env,
    )
    .await
    .unwrap_err();
    assert!(error.contains("not found"), "got: {error}");
}

#[tokio::test]
async fn retire_refused_for_active_tool_records_the_refusal() {
    let dir = tempfile::tempdir().unwrap();
    let env = run_dir_env(dir.path());
    let server_path = dir.path().join("echo_server.py");
    std::fs::write(&server_path, MCP_ECHO_SERVER).unwrap();

    call(
        &ResourcesTool,
        serde_json::json!({
            "op": "attach_mcp",
            "label": "live",
            "transport": {"stdio": {"command": "python3", "args": [server_path.to_string_lossy()]}}
        }),
        &env,
    )
    .await
    .unwrap();

    // Drain #1 projects the queued InstallTool mutations; only then can
    // the tool be activated — definitions exist.
    let mut resources = machine::Resources::new();
    let mut tool_runtime = ToolRuntime::new();
    accelerator::registry::drain(Some(dir.path()), &mut resources, &mut tool_runtime);
    resources.enable("live__echo").unwrap();

    call(
        &ResourcesTool,
        serde_json::json!({"op": "retire", "name": "gen/mcp/live"}),
        &env,
    )
    .await
    .unwrap();

    // The tool is active at drain time: retire must be refused and the
    // refusal recorded as an event (trajectory-visible), definition kept.
    let events = accelerator::registry::drain(Some(dir.path()), &mut resources, &mut tool_runtime);
    assert!(resources.tool_definitions.contains_key("live__echo"));
    assert!(tool_runtime.contains("live__echo"));
    let ops: Vec<&str> = events.iter().map(|event| event.op.as_str()).collect();
    assert_eq!(
        ops,
        vec!["retire_refused_active"],
        "active retire must be refused: {ops:?}"
    );
}

#[tokio::test]
async fn registry_persists_to_run_dir_and_lists_back() {
    let dir = tempfile::tempdir().unwrap();
    let env = run_dir_env(dir.path());

    call(
        &ResourcesTool,
        serde_json::json!({"op": "register_prompt", "name": "gen/persist", "body": "keep"}),
        &env,
    )
    .await
    .unwrap();

    let registry_file = dir.path().join("resources").join("registry.json");
    assert!(
        registry_file.is_file(),
        "registry must persist under run_dir"
    );
    let raw = std::fs::read_to_string(&registry_file).unwrap();
    let persisted: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(persisted["gen/persist"]["kind"], "prompt");

    let listed = call(&ResourcesTool, serde_json::json!({"op": "list"}), &env)
        .await
        .unwrap();
    let payload = payload_of(&listed);
    let names: Vec<&str> = payload["resources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|record| record["name"].as_str().unwrap())
        .collect();
    assert!(names.contains(&"gen/persist"));
}

#[tokio::test]
async fn registry_events_lift_into_machine_parser() {
    // A resources tool result body lifted through the shared parser.
    let fragment = machine::Fragment::tool_result(
        "call-9",
        serde_json::json!({
            "tool": "resources",
            "op": "attach_mcp",
            "events": [{"op": "register", "kind": "mcp-server", "name": "gen/mcp/x"}],
            "result": {},
        })
        .to_string(),
        None,
    );
    let effect = machine::Effect::InboxPushed {
        item: machine::InboxItem::new(fragment, None),
    };
    let lifted = machine::registry_events_in(&[effect]);
    assert_eq!(lifted.len(), 1);
    assert_eq!(lifted[0].op, "register");
    assert_eq!(lifted[0].kind, "mcp-server");
    assert_eq!(lifted[0].name, "gen/mcp/x");
}

#[tokio::test]
async fn no_run_dir_registry_is_ephemeral_but_functional() {
    let env = Environment::empty(".");
    let registered = call(
        &ResourcesTool,
        serde_json::json!({"op": "register_prompt", "name": "gen/ephemeral", "body": "x"}),
        &env,
    )
    .await
    .unwrap();
    assert_eq!(events_of(&registered).len(), 1);
    // cwd is not a run dir: nothing may be written there.
    assert!(!std::path::Path::new("resources").exists());
}
