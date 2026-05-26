use machine::fragment::{Content, Fragment};
use machine::{Environment, Resources, Tool, ToolResult};
use std::sync::Arc;
use std::time::Instant;
use tokio::time::{Duration, sleep};

/// A tool that simply sleeps for the specified milliseconds.
struct DelayTool {
    name: String,
    ms: u64,
}

impl DelayTool {
    fn new(name: &str, ms: u64) -> Self {
        Self {
            name: name.into(),
            ms,
        }
    }
}

impl Tool for DelayTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        "delays for N ms"
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "ms": { "type": "integer" }
            }
        })
    }

    fn execute<'a>(
        &'a self,
        _args: serde_json::Value,
        _env: &'a Environment,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ToolResult, String>> + Send + 'a>>
    {
        let ms = self.ms;
        Box::pin(async move {
            sleep(Duration::from_millis(ms)).await;
            Ok(ToolResult {
                call_id: String::new(),
                content: format!("slept {}ms", ms),
                title: None,
            })
        })
    }
}

fn make_sleep_tool(name: &str, ms: u64) -> Arc<dyn Tool> {
    Arc::new(DelayTool::new(name, ms))
}

/// Parallel execution: 3 × 100ms via join_all → ≈100ms (not 300ms).
#[tokio::test]
async fn parallel_tool_execution() {
    let env = Environment::new("/tmp");
    let mut resources = Resources::new();
    resources = resources
        .with_tool(make_sleep_tool("a", 100))
        .with_tool(make_sleep_tool("b", 100))
        .with_tool(make_sleep_tool("c", 100));
    resources.enable("a").unwrap();
    resources.enable("b").unwrap();
    resources.enable("c").unwrap();

    let fragments = vec![
        Fragment::tool_call("c1", "a", serde_json::json!({})),
        Fragment::tool_call("c2", "b", serde_json::json!({})),
        Fragment::tool_call("c3", "c", serde_json::json!({})),
    ];

    // Extract tool references and build futures
    use futures_util::future::join_all;
    let mut futures = Vec::new();
    for frag in &fragments {
        if let Content::ToolCall(tc) = &frag.content {
            if let Some(tool) = resources.lookup(&tc.name) {
                futures.push(tool.execute(tc.arguments.clone(), &env));
            }
        }
    }

    let t0 = Instant::now();
    let results = join_all(futures).await;
    let elapsed = t0.elapsed();

    assert!(
        elapsed < Duration::from_millis(200),
        "parallel 3 × 100ms should take < 200ms (was {:?})",
        elapsed
    );
    assert_eq!(results.len(), 3);
    for result in results {
        assert!(result.is_ok());
    }
}

/// Baseline: serial execution (same as old reactor). 3 × 100ms → ≥ 250ms.
#[tokio::test]
async fn serial_tool_execution_baseline() {
    let env = Environment::new("/tmp");
    let mut resources = Resources::new();
    resources = resources
        .with_tool(make_sleep_tool("a", 100))
        .with_tool(make_sleep_tool("b", 100))
        .with_tool(make_sleep_tool("c", 100));
    resources.enable("a").unwrap();
    resources.enable("b").unwrap();
    resources.enable("c").unwrap();

    let fragments = vec![
        Fragment::tool_call("c1", "a", serde_json::json!({})),
        Fragment::tool_call("c2", "b", serde_json::json!({})),
        Fragment::tool_call("c3", "c", serde_json::json!({})),
    ];

    let t0 = Instant::now();
    for frag in &fragments {
        if let Content::ToolCall(tc) = &frag.content {
            if let Some(tool) = resources.lookup(&tc.name) {
                let _ = tool.execute(tc.arguments.clone(), &env).await;
            }
        }
    }
    let elapsed = t0.elapsed();

    assert!(
        elapsed >= Duration::from_millis(250),
        "serial 3 × 100ms should take >= 250ms (was {:?})",
        elapsed
    );
}

/// Verify that mixing text and tool calls works: text pushed immediately,
/// tools execute in parallel behind it.
#[tokio::test]
async fn text_and_tools_mixed() {
    let env = Environment::new("/tmp");
    let mut resources = Resources::new();
    resources = resources.with_tool(make_sleep_tool("slow", 50));
    resources.enable("slow").unwrap();

    let fragments = vec![
        Fragment::user("hello before"),
        Fragment::tool_call("c1", "slow", serde_json::json!({})),
        Fragment::assistant("text after"),
    ];

    // Simulate the new reactor: text pushed immediately, tools joined.
    let mut inbox = Vec::new();
    let mut futures = Vec::new();
    for frag in &fragments {
        if let Content::ToolCall(tc) = &frag.content {
            if let Some(tool) = resources.lookup(&tc.name) {
                futures.push(tool.execute(tc.arguments.clone(), &env));
            }
        } else {
            inbox.push(frag.as_text().unwrap_or("").to_string());
        }
    }

    assert_eq!(
        inbox,
        vec!["hello before", "text after"],
        "text fragments pushed before tool execution"
    );

    let _results = futures_util::future::join_all(futures).await;
}

/// One failing tool does not block other tools from executing.
#[tokio::test]
async fn one_failure_does_not_block_others() {
    use machine::ToolResult;

    struct FailingTool;

    impl Tool for FailingTool {
        fn name(&self) -> &str {
            "fail"
        }

        fn description(&self) -> &str {
            "always fails"
        }

        fn parameters(&self) -> serde_json::Value {
            serde_json::json!({})
        }

        fn execute<'a>(
            &'a self,
            _args: serde_json::Value,
            _env: &'a Environment,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<ToolResult, String>> + Send + 'a>,
        > {
            Box::pin(async { Err("intentional failure".into()) })
        }
    }

    let env = Environment::new("/tmp");
    let mut resources = Resources::new();
    resources = resources
        .with_tool(make_sleep_tool("slow", 50))
        .with_tool(Arc::new(FailingTool))
        .with_tool(make_sleep_tool("also-slow", 50));
    resources.enable("slow").unwrap();
    resources.enable("fail").unwrap();
    resources.enable("also-slow").unwrap();

    let fragments = vec![
        Fragment::tool_call("c1", "slow", serde_json::json!({})),
        Fragment::tool_call("c2", "fail", serde_json::json!({})),
        Fragment::tool_call("c3", "also-slow", serde_json::json!({})),
    ];

    let mut futures = Vec::new();
    for frag in &fragments {
        if let Content::ToolCall(tc) = &frag.content {
            if let Some(tool) = resources.lookup(&tc.name) {
                futures.push(tool.execute(tc.arguments.clone(), &env));
            }
        }
    }

    let t0 = Instant::now();
    let results = futures_util::future::join_all(futures).await;
    let elapsed = t0.elapsed();

    // 3 tools in parallel, even with one failure, should complete fast.
    assert!(
        elapsed < Duration::from_millis(150),
        "parallel with failure should take < 150ms (was {:?})",
        elapsed
    );

    assert_eq!(results.len(), 3);
    assert!(results[0].is_ok(), "first tool should succeed");
    assert!(results[1].is_err(), "failing tool should error");
    assert!(results[2].is_ok(), "third tool should succeed");
}
