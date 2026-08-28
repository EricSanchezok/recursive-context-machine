use accelerator::tools::{LedgerTool, ledger_digest_for};
use machine::{Environment, Tool};

fn run_dir_env() -> (tempfile::TempDir, Environment) {
    let dir = tempfile::tempdir().unwrap();
    let mut env = Environment::empty(dir.path());
    env.run_dir = Some(dir.path().to_path_buf());
    (dir, env)
}

async fn call(
    tool: &LedgerTool,
    args: serde_json::Value,
    env: &Environment,
) -> Result<String, String> {
    tool.execute(args, env).await.map(|result| result.content)
}

fn transitions_of(content: &str) -> Vec<(String, String, String)> {
    let value: serde_json::Value = serde_json::from_str(content).unwrap();
    value["transitions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            (
                entry["id"].as_str().unwrap().to_string(),
                entry["from"].as_str().unwrap().to_string(),
                entry["to"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

#[tokio::test]
async fn legal_lifecycle_pending_running_completed() {
    let (_dir, env) = run_dir_env();
    let tool = LedgerTool;

    call(
        &tool,
        serde_json::json!({"op": "add", "id": "scan", "title": "scan repo"}),
        &env,
    )
    .await
    .unwrap();
    let started = call(
        &tool,
        serde_json::json!({"op": "start", "id": "scan"}),
        &env,
    )
    .await
    .unwrap();
    assert_eq!(
        transitions_of(&started),
        vec![("scan".into(), "pending".into(), "running".into())]
    );

    let done = call(
        &tool,
        serde_json::json!({"op": "complete", "id": "scan", "result": "3 modules"}),
        &env,
    )
    .await
    .unwrap();
    assert_eq!(
        transitions_of(&done),
        vec![("scan".into(), "running".into(), "completed".into())]
    );
}

#[tokio::test]
async fn illegal_transition_completed_to_running_is_rejected() {
    let (_dir, env) = run_dir_env();
    let tool = LedgerTool;

    call(
        &tool,
        serde_json::json!({"op": "add", "id": "one", "title": "work"}),
        &env,
    )
    .await
    .unwrap();
    call(&tool, serde_json::json!({"op": "start", "id": "one"}), &env)
        .await
        .unwrap();
    call(
        &tool,
        serde_json::json!({"op": "complete", "id": "one"}),
        &env,
    )
    .await
    .unwrap();

    let error = call(&tool, serde_json::json!({"op": "start", "id": "one"}), &env)
        .await
        .unwrap_err();
    assert!(error.contains("illegal ledger transition"), "got: {error}");
}

#[tokio::test]
async fn start_with_uncompleted_deps_is_rejected() {
    let (_dir, env) = run_dir_env();
    let tool = LedgerTool;

    call(
        &tool,
        serde_json::json!({"op": "add", "id": "dep", "title": "dependency"}),
        &env,
    )
    .await
    .unwrap();
    call(
        &tool,
        serde_json::json!({"op": "add", "id": "child", "title": "dependent", "deps": ["dep"]}),
        &env,
    )
    .await
    .unwrap();

    let error = call(
        &tool,
        serde_json::json!({"op": "start", "id": "child"}),
        &env,
    )
    .await
    .unwrap_err();
    assert!(error.contains("uncompleted deps"), "got: {error}");
}

#[tokio::test]
async fn completing_dependency_promotes_ready_dependent() {
    let (_dir, env) = run_dir_env();
    let tool = LedgerTool;

    call(
        &tool,
        serde_json::json!({"op": "add", "id": "dep", "title": "dependency"}),
        &env,
    )
    .await
    .unwrap();
    call(
        &tool,
        serde_json::json!({"op": "add", "id": "child", "title": "dependent", "deps": ["dep"]}),
        &env,
    )
    .await
    .unwrap();
    call(&tool, serde_json::json!({"op": "start", "id": "dep"}), &env)
        .await
        .unwrap();

    let done = call(
        &tool,
        serde_json::json!({"op": "complete", "id": "dep"}),
        &env,
    )
    .await
    .unwrap();
    let transitions = transitions_of(&done);
    assert_eq!(
        transitions,
        vec![
            ("dep".into(), "running".into(), "completed".into()),
            // promotion fired by code, not by the model
            ("child".into(), "pending".into(), "running".into()),
        ]
    );
}

#[tokio::test]
async fn digest_reflects_status_counts_and_current_entry() {
    let (dir, env) = run_dir_env();
    let tool = LedgerTool;

    call(
        &tool,
        serde_json::json!({"op": "add", "id": "a", "title": "first"}),
        &env,
    )
    .await
    .unwrap();
    call(
        &tool,
        serde_json::json!({"op": "add", "id": "b", "title": "second"}),
        &env,
    )
    .await
    .unwrap();
    call(&tool, serde_json::json!({"op": "start", "id": "b"}), &env)
        .await
        .unwrap();

    let digest = ledger_digest_for(dir.path()).expect("digest after writes");
    assert_eq!(digest.total, 2);
    assert_eq!(digest.by_status.get("pending"), Some(&1));
    assert_eq!(digest.by_status.get("running"), Some(&1));
    let current = digest.current_entry.expect("current entry");
    assert_eq!(current.id, "b");
    assert_eq!(current.status, "running");
}

#[tokio::test]
async fn digest_is_none_for_untouched_run_dir() {
    let dir = tempfile::tempdir().unwrap();
    assert!(ledger_digest_for(dir.path()).is_none());
}

#[tokio::test]
async fn ledger_persists_to_run_dir_json() {
    let (dir, env) = run_dir_env();
    let tool = LedgerTool;

    call(
        &tool,
        serde_json::json!({"op": "add", "id": "kept", "title": "survives"}),
        &env,
    )
    .await
    .unwrap();

    let persisted = dir.path().join("ledger.json");
    assert!(persisted.is_file(), "ledger.json must exist under run_dir");
    let raw = std::fs::read_to_string(&persisted).unwrap();
    let book: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(book["kept"]["status"], "pending");
}

#[tokio::test]
async fn no_run_dir_leaves_no_file_behind() {
    let env = Environment::empty(".");
    let tool = LedgerTool;

    call(
        &tool,
        serde_json::json!({"op": "add", "id": "ephemeral", "title": "no file"}),
        &env,
    )
    .await
    .unwrap();

    assert!(
        !std::path::Path::new("ledger.json").exists(),
        "no run_dir means no ledger file in the working directory"
    );
}

#[tokio::test]
async fn tool_result_transitions_lift_into_machine_parser() {
    let (_dir, env) = run_dir_env();
    let tool = LedgerTool;

    call(
        &tool,
        serde_json::json!({"op": "add", "id": "x", "title": "lift"}),
        &env,
    )
    .await
    .unwrap();
    let started = call(&tool, serde_json::json!({"op": "start", "id": "x"}), &env)
        .await
        .unwrap();

    // The tool result is a machine-side parseable tool-result fragment body.
    let fragment = machine::Fragment::tool_result("call-1", started, None);
    let effect = machine::Effect::InboxPushed {
        item: machine::InboxItem::new(fragment, None),
    };
    let lifted = machine::ledger_transitions_in(&[effect]);
    assert_eq!(lifted.len(), 1);
    assert_eq!(lifted[0].entry_id, "x");
    assert_eq!(lifted[0].from_status, "pending");
    assert_eq!(lifted[0].to_status, "running");
}
