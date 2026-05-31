use machine::{LookupResult, Resources, Tool, ToolResult};
use serde_json::json;
use std::sync::Arc;

struct TestTool;
impl Tool for TestTool {
    fn name(&self) -> &str {
        "test"
    }
    fn description(&self) -> &str {
        "test tool"
    }
    fn parameters(&self) -> serde_json::Value {
        json!({})
    }
    fn execute<'a>(
        &'a self,
        _args: serde_json::Value,
        _env: &machine::Environment,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ToolResult, String>> + Send + 'a>>
    {
        Box::pin(async {
            Ok(ToolResult {
                call_id: String::new(),
                content: String::new(),
                title: None,
            })
        })
    }
}

struct AnotherTool;
impl Tool for AnotherTool {
    fn name(&self) -> &str {
        "other"
    }
    fn description(&self) -> &str {
        "another tool"
    }
    fn parameters(&self) -> serde_json::Value {
        json!({})
    }
    fn execute<'a>(
        &'a self,
        _args: serde_json::Value,
        _env: &machine::Environment,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ToolResult, String>> + Send + 'a>>
    {
        Box::pin(async {
            Ok(ToolResult {
                call_id: String::new(),
                content: String::new(),
                title: None,
            })
        })
    }
}

fn tool() -> Arc<dyn Tool> {
    Arc::new(TestTool)
}
fn other_tool() -> Arc<dyn Tool> {
    Arc::new(AnotherTool)
}

#[test]
fn enable_makes_lookup_active() {
    let mut res = Resources::new().with_tool(tool());
    assert_eq!(res.lookup("test"), LookupResult::Inactive);
    res.enable("test").unwrap();
    assert_eq!(res.lookup("test"), LookupResult::Active);
}

#[test]
fn disable_makes_lookup_inactive() {
    let mut res = Resources::new().with_tool(tool());
    res.enable("test").unwrap();
    assert_eq!(res.lookup("test"), LookupResult::Active);
    res.disable("test");
    assert_eq!(res.lookup("test"), LookupResult::Inactive);
}

#[test]
fn enable_nonexistent_returns_err() {
    let mut res = Resources::new();
    assert!(res.enable("nonexistent").is_err());
}

#[test]
fn lookup_active_implies_get_some() {
    let mut res = Resources::new().with_tool(tool()).with_tool(other_tool());
    res.enable("test").unwrap();
    res.enable("other").unwrap();
    for name in &["test", "other"] {
        assert_eq!(res.lookup(name), LookupResult::Active);
        assert!(
            res.get(name).is_some(),
            "get() must return Some for tool '{}' when lookup() == Active",
            name
        );
    }
}

#[test]
fn lookup_inactive_implies_get_none() {
    let res = Resources::new().with_tool(tool());
    assert_eq!(res.lookup("test"), LookupResult::Inactive);
    assert!(
        res.get("test").is_none(),
        "get() must return None when lookup() == Inactive"
    );
}

#[test]
fn lookup_not_found_implies_get_none() {
    let res = Resources::new();
    assert_eq!(res.lookup("nonexistent"), LookupResult::NotFound);
    assert!(
        res.get("nonexistent").is_none(),
        "get() must return None when lookup() == NotFound"
    );
}

#[test]
fn inactive_hitch_mentions_disabled() {
    use machine::Fragment;
    use machine::fragment::Content;
    let res = Resources::new().with_tool(tool());
    assert_eq!(res.lookup("test"), LookupResult::Inactive);
    let hitch = Fragment::hitch(
        format!("tool '{}' is disabled — activate it before use", "test"),
        None,
        machine::Role::Tool,
        Some(String::from("call_1")),
    );
    assert!(matches!(hitch.content, Content::Hitch { .. }));
    assert!(hitch.content_as_text().contains("is disabled"));
}

#[test]
fn not_found_hitch_mentions_not_found() {
    use machine::Fragment;
    use machine::fragment::Content;
    let res = Resources::new();
    assert_eq!(res.lookup("nonexistent"), LookupResult::NotFound);
    let hitch = Fragment::hitch(
        format!("tool '{}' not found", "nonexistent"),
        None,
        machine::Role::Tool,
        None::<String>,
    );
    assert!(matches!(hitch.content, Content::Hitch { .. }));
    assert!(hitch.content_as_text().contains("not found"));
}
