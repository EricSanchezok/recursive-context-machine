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
    worker_with_handoff(name, &format!("status: {status}"))
}

fn worker_with_handoff(name: &str, handoff: &str) -> accelerator::Accelerator {
    let mut ctx = machine::Context::default();
    ctx.append(scaffolding_env());
    let state = machine::RunState {
        purpose: machine::Purpose::new(handoff),
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

#[tokio::test]
async fn spawn_partial_status_counts_as_completed_not_failed() {
    // `partial` is a success per schema/handoff.md (the worker produced its
    // artifact). It must not be surfaced as a failure for retry.
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "partial"));
    let items = vec![
        serde_json::json!({"id": "P1"}),
        serde_json::json!({"id": "P2"}),
    ];
    let result = tool
        .execute(
            serde_json::json!({"items": items}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");
    assert!(
        result.content.contains("partial=2"),
        "report: {}",
        result.content
    );
    assert!(
        result.content.contains("All items completed successfully."),
        "report: {}",
        result.content
    );
    assert!(
        !result.content.contains("failed="),
        "report: {}",
        result.content
    );
}

#[tokio::test]
async fn spawn_lenient_status_parsing_tolerates_trailing_note() {
    // A worker that annotates its status (`status: ok (image-only PDF)`) is
    // still classified as ok, not an unknown failure.
    let tool = SpawnTool::new(
        "spawn_test",
        worker_with_status("inner", "ok (image-only PDF)"),
    );
    let result = tool
        .execute(
            serde_json::json!({"items": [{"id": "L1"}]}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");
    assert!(
        result.content.contains("ok=1"),
        "report: {}",
        result.content
    );
    assert!(
        !result.content.contains("failed="),
        "report: {}",
        result.content
    );
}

#[tokio::test]
async fn spawn_accepts_markdown_emphasis_around_status_key() {
    let tool = SpawnTool::new(
        "spawn_test",
        worker_with_handoff("inner", "- **status**: **ok** (full text)"),
    );
    let result = tool
        .execute(
            serde_json::json!({"items": [{"id": "M1"}]}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should succeed");

    assert!(
        result.content.contains("ok=1"),
        "report: {}",
        result.content
    );
    assert!(
        !result.content.contains("failed="),
        "report: {}",
        result.content
    );
}

#[tokio::test]
async fn spawn_failure_report_identifies_section_items_without_id() {
    let tool = SpawnTool::new("spawn_section", worker_with_status("inner", "blocked"));
    let result = tool
        .execute(
            serde_json::json!({
                "items": [{"n": "07", "slug": "open-problems"}]
            }),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should return a failure report");

    assert!(
        result.content.contains("failed=1"),
        "report: {}",
        result.content
    );
    assert!(
        result.content.contains("✗ 07"),
        "report: {}",
        result.content
    );
    assert!(
        !result.content.contains("✗ ?"),
        "report: {}",
        result.content
    );
}

#[tokio::test]
async fn spawn_caps_model_requested_parallelism() {
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "ok"));
    let result = tool
        .execute(
            serde_json::json!({
                "items": [{"id": "A"}, {"id": "B"}],
                "max_parallel": 100_000
            }),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should cap concurrency rather than reject the batch");

    assert!(
        result.content.contains("max_parallel=32"),
        "report: {}",
        result.content
    );
}

#[tokio::test]
async fn spawn_rejects_unbounded_item_batches() {
    let tool = SpawnTool::new("spawn_test", worker_with_status("inner", "ok"));
    let items: Vec<serde_json::Value> = (0..1_025)
        .map(|index| serde_json::json!({"id": format!("item_{index}")}))
        .collect();

    let error = tool
        .execute(
            serde_json::json!({"items": items}),
            &machine::Environment::empty("."),
        )
        .await
        .expect_err("oversized spawn batches must be rejected before execution");

    assert!(error.contains("1024"), "error: {error}");
}

#[tokio::test]
async fn spawn_uses_the_last_handoff_status() {
    let tool = SpawnTool::new(
        "spawn_test",
        worker_with_handoff("inner", "status: blocked\nstatus: ok"),
    );

    let result = tool
        .execute(
            serde_json::json!({"items": [{"id": "recovered"}]}),
            &machine::Environment::empty("."),
        )
        .await
        .expect("spawn should use the final worker status");

    assert!(
        result.content.contains("ok=1"),
        "report: {}",
        result.content
    );
}
