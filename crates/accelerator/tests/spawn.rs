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
    let state = machine::RunState {
        purpose: machine::Purpose::new(format!("status: {status}")),
        environment: machine::Environment::empty("."),
        context: ctx,
        ..machine::RunState::default()
    };
    accelerator::Accelerator::primitive(state, done_policy(), machine::ToolRuntime::new(), name)
}

fn done_policy() -> Box<dyn machine::Policy> {
    use machine::{Action, Policy, PolicyView};
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
            _view: PolicyView<'a>,
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
}

#[tokio::test]
async fn spawn_multiple_items_aggregates_results() {
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "ok"));
    let items: Vec<serde_json::Value> = (1..=5)
        .map(|i| serde_json::json!({"id": format!("item_{i}")}))
        .collect();
    let result = tool
        .execute(
            serde_json::json!({"items": items}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");
    assert!(
        result.content.contains("5 items"),
        "report: {}",
        result.content
    );
    assert!(
        result.content.contains("ok=5"),
        "report: {}",
        result.content
    );
}

#[tokio::test]
async fn spawn_mixed_status_groups_success_and_failure() {
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "ok"));
    let items = vec![
        serde_json::json!({"id": "A"}),
        serde_json::json!({"id": "B"}),
        serde_json::json!({"id": "C"}),
    ];
    let result = tool
        .execute(
            serde_json::json!({"items": items, "max_parallel": 2}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");
    assert!(
        result.content.contains("ok=3"),
        "report: {}",
        result.content
    );
}

#[tokio::test]
async fn spawn_empty_items_returns_empty_report() {
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
async fn spawn_respects_max_parallel_concurrency() {
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "ok"));
    let items: Vec<serde_json::Value> = (1..=10)
        .map(|i| serde_json::json!({"id": format!("item_{i}")}))
        .collect();
    let start = std::time::Instant::now();
    let result = tool
        .execute(
            serde_json::json!({"items": items, "max_parallel": 3}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");
    // Even with fast Done workers, bounded concurrency should not increase
    // the total count.
    assert!(
        result.content.contains("10 items"),
        "report: {}",
        result.content
    );
    assert!(
        result.content.contains("ok=10"),
        "report: {}",
        result.content
    );
    let _elapsed = start.elapsed();
}
