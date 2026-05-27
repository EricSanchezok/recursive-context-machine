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

    use futures_util::future::join_all;
    let mut futures = Vec::new();
    for frag in &fragments {
        if let Content::ToolCall(tc) = &frag.content {
            if let Some(tool) = resources.get(&tc.name) {
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
            if let Some(tool) = resources.get(&tc.name) {
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

/// Text pushed before tools execute.
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

    let mut inbox = Vec::new();
    let mut futures = Vec::new();
    for frag in &fragments {
        if let Content::ToolCall(tc) = &frag.content {
            if let Some(tool) = resources.get(&tc.name) {
                futures.push(tool.execute(tc.arguments.clone(), &env));
            }
        } else {
            inbox.push(frag.as_text().unwrap_or("").to_string());
        }
    }
    assert_eq!(inbox, vec!["hello before", "text after"]);
    let _results = futures_util::future::join_all(futures).await;
}

/// One failing tool does not block other tools.
#[tokio::test]
async fn one_failure_does_not_block_others() {
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
            if let Some(tool) = resources.get(&tc.name) {
                futures.push(tool.execute(tc.arguments.clone(), &env));
            }
        }
    }

    let t0 = Instant::now();
    let results = futures_util::future::join_all(futures).await;
    let elapsed = t0.elapsed();
    assert!(elapsed < Duration::from_millis(150));
    assert_eq!(results.len(), 3);
    assert!(results[0].is_ok());
    assert!(results[1].is_err());
    assert!(results[2].is_ok());
}

// ── Panic reproduction ──

#[test]
#[should_panic(expected = "byte index")]
fn one_tool_panic_kills_all_join_all_tools() {
    let padding = "x".repeat(58);
    let s = format!("{padding}并");
    let _sliced = &s[..60];
}

#[tokio::test]
async fn spawn_isolates_tool_panic() {
    struct SpawnPanicTool;
    impl Tool for SpawnPanicTool {
        fn name(&self) -> &str {
            "spawn_panic"
        }
        fn description(&self) -> &str {
            "panics via spawn"
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
            Box::pin(async move {
                let s = format!("{}并", "x".repeat(58));
                let _ = &s[..60];
                unreachable!()
            })
        }
    }

    let env = Arc::new(Environment::new("/tmp"));
    let mut resources = Resources::new();
    resources = resources
        .with_tool(make_sleep_tool("a", 50))
        .with_tool(Arc::new(SpawnPanicTool))
        .with_tool(make_sleep_tool("b", 50));
    resources.enable("a").unwrap();
    resources.enable("spawn_panic").unwrap();
    resources.enable("b").unwrap();

    let fragments = vec![
        Fragment::tool_call("c1", "a", serde_json::json!({})),
        Fragment::tool_call("c2", "spawn_panic", serde_json::json!({})),
        Fragment::tool_call("c3", "b", serde_json::json!({})),
    ];

    let mut tool_clones: Vec<Option<Arc<dyn Tool>>> = Vec::new();
    for frag in &fragments {
        if let Content::ToolCall(tc) = &frag.content {
            tool_clones.push(resources.tools.get(&tc.name).cloned());
        } else {
            tool_clones.push(None);
        }
    }

    let mut futures = Vec::new();
    for tool in tool_clones {
        let Some(tool) = tool else { continue };
        let env_arc = env.clone();
        futures.push(async move {
            let _started_at = Instant::now();
            match tokio::spawn(async move { tool.execute(serde_json::json!({}), &env_arc).await })
                .await
            {
                Ok(Ok(tr)) => Ok(tr.content),
                Ok(Err(msg)) => Err(msg),
                Err(_join_err) => Err("tool panicked".into()),
            }
        });
    }
    let results = futures_util::future::join_all(futures).await;
    assert_eq!(results.len(), 3);
    assert!(results[0].is_ok());
    assert!(results[1].is_err());
    assert!(results[2].is_ok());
    let error_msg = results[1].as_ref().unwrap_err();
    assert!(
        error_msg.contains("panicked"),
        "error should mention panic: {error_msg}"
    );
}
