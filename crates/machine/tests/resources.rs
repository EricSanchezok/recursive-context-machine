use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use machine::{Environment, Resources, Tool, ToolResult, ToolStatus};

struct StubTool(&'static str);

impl Tool for StubTool {
    fn name(&self) -> &str {
        self.0
    }
    fn description(&self) -> &str {
        "stub"
    }
    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({})
    }
    fn execute<'a>(
        &'a self,
        _args: serde_json::Value,
        _env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move { Err("stub".to_string()) })
    }
}

#[test]
fn tool_status_active_after_enable() {
    let mut res = Resources::new().with_tool(Arc::new(StubTool("grep")));
    res.enable("grep");
    assert_eq!(res.tool_status("grep"), ToolStatus::Active);
}

#[test]
fn tool_status_disabled_after_register() {
    let res = Resources::new().with_tool(Arc::new(StubTool("grep")));
    assert_eq!(res.tool_status("grep"), ToolStatus::Disabled);
}

#[test]
fn tool_status_disabled_after_disable() {
    let mut res = Resources::new().with_tool(Arc::new(StubTool("grep")));
    res.enable("grep");
    res.disable("grep");
    assert_eq!(res.tool_status("grep"), ToolStatus::Disabled);
}

#[test]
fn tool_status_not_found_for_unknown() {
    let res = Resources::new();
    assert_eq!(res.tool_status("no_such_tool"), ToolStatus::NotFound);
}

#[test]
fn lookup_returns_some_only_when_active() {
    let mut res = Resources::new().with_tool(Arc::new(StubTool("grep")));
    assert!(res.lookup("grep").is_none());
    res.enable("grep");
    assert!(res.lookup("grep").is_some());
}
