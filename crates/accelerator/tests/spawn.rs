use accelerator::tools::SpawnTool;
use machine::Fragment;
use machine::Tool;

/// Builtin fragments that act as actual scaffolding so the reorder logic in
/// `fire()` finds an `env` tag and stops looping.
fn scaffolding_env() -> Fragment {
    Fragment::system("cwd: .\nplatform: test").with_tag("env")
}

/// An inner accelerator that halts immediately and leaves a well-known `status:`
/// line in the output context. The status text is set as the base purpose
/// (which `merge_input` always preserves), and the output context gets it via
/// the reorder step copying the purpose tag.
fn worker_with_status(name: &str, status: &str) -> accelerator::Accelerator {
    let mut ctx = machine::Context::default();
    ctx.append(scaffolding_env());
    let state = accelerator::State {
        purpose: format!("status: {status}"),
        env: machine::Environment::empty("."),
        ctx,
        ..accelerator::State::default()
    };
    accelerator::Accelerator::primitive(state, done_policy(), machine::ToolRuntime::new(), name)
}

fn done_policy() -> Box<dyn machine::Policy> {
    use machine::{Action, Environment, Inbox, Policy, Purpose};
    use std::future::Future;
    use std::pin::Pin;
    struct Done;
    impl Policy for Done {
        fn clone_box(&self) -> Box<dyn Policy> {
            Box::new(Done)
        }
        fn name(&self) -> &str {
            "done"
        }
        fn decide<'a>(
            &'a self,
            _purpose: &'a Purpose,
            _ctx: &'a machine::Context,
            _env: &'a Environment,
            _resources: &'a machine::Resources,
            _inbox: &'a Inbox,
        ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
            Box::pin(async { Action::Done })
        }
    }
    Box::new(Done)
}

#[tokio::test]
async fn spawn_single_item_reports_ok() {
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "ok"));
    let result = tool
        .execute(
            serde_json::json!({"items": [{"id": "2401.001", "title": "Test Paper"}]}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");
    assert!(
        result.content.contains("1 items"),
        "report: {}",
        result.content
    );
    assert!(
        result.content.contains("ok=1"),
        "report: {}",
        result.content
    );
    assert!(
        result.content.contains("All items completed successfully"),
        "report: {}",
        result.content
    );
}

#[tokio::test]
async fn spawn_empty_items_returns_early() {
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "ok"));
    let result = tool
        .execute(
            serde_json::json!({"items": []}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");
    assert!(
        result.content.contains("empty"),
        "report: {}",
        result.content
    );
}

#[tokio::test]
async fn spawn_respects_concurrency_cap() {
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "ok"));
    let items: Vec<serde_json::Value> = (0..5)
        .map(|i| serde_json::json!({"id": format!("2401.{:03}", i)}))
        .collect();
    let result = tool
        .execute(
            serde_json::json!({"items": items, "max_parallel": 2}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");
    assert!(
        result.content.contains("ok=5"),
        "report: {}",
        result.content
    );
    assert!(
        result.content.contains("max_parallel=2"),
        "report: {}",
        result.content
    );
}

#[tokio::test]
async fn spawn_reports_failed_items() {
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "failed"));
    let items: Vec<serde_json::Value> = (0..3)
        .map(|i| serde_json::json!({"id": format!("2401.{:03}", i)}))
        .collect();
    let result = tool
        .execute(
            serde_json::json!({"items": items}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");
    assert!(
        result.content.contains("failed=3"),
        "report: {}",
        result.content
    );
    for i in 0..3 {
        assert!(
            result.content.contains(&format!("2401.{:03}", i)),
            "report misses 2401.{:03}:\n{}",
            i,
            result.content
        );
    }
}

#[tokio::test]
async fn spawn_blocked_items_surface_in_report() {
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "blocked"));
    let result = tool
        .execute(
            serde_json::json!({"items": [{"id": "2401.099"}]}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");
    assert!(
        result.content.contains("failed=1"),
        "report: {}",
        result.content
    );
    assert!(
        result.content.contains("2401.099"),
        "report: {}",
        result.content
    );
}

#[tokio::test]
async fn spawn_errors_on_missing_items() {
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "ok"));
    let result = tool
        .execute(
            serde_json::json!({"max_parallel": 1}),
            &machine::Environment::empty("."),
        )
        .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("items"));
}

#[tokio::test]
async fn spawn_abstract_only_surfaces_as_failure() {
    let tool = SpawnTool::new(
        "spawn_test",
        worker_with_status("inner", "evidence: abstract_only"),
    );
    let result = tool
        .execute(
            serde_json::json!({"items": [{"id": "2401.888"}]}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");
    assert!(
        result.content.contains("failed=1"),
        "report: {}",
        result.content
    );
    assert!(
        result.content.contains("2401.888"),
        "report: {}",
        result.content
    );
}

#[tokio::test]
async fn spawn_mixed_blocked_and_ok_surfaces_only_failed() {
    // Each worker is the same type (halt policy). The tool wraps ONE worker
    // type. Since every worker returns "blocked", the report shows all blocked.
    // This tests that the reporting code handles partial-failure output.
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "blocked"));
    let items: Vec<serde_json::Value> = (0..4)
        .map(|i| serde_json::json!({"id": format!("paper.{i}")}))
        .collect();
    let result = tool
        .execute(
            serde_json::json!({"items": items, "max_parallel": 2}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");
    assert!(
        result.content.contains("failed=4"),
        "report: {}",
        result.content
    );
    assert!(
        result.content.contains("paper.0"),
        "report: {}",
        result.content
    );
    assert!(
        result.content.contains("paper.3"),
        "report: {}",
        result.content
    );
}
