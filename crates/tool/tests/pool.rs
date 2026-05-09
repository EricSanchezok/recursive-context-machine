use tool::pool::ToolPool;
use tool::tool_def::ToolDef;

#[test]
fn test_register_and_get() {
    let mut pool = ToolPool::new();
    pool.register(ToolDef::new("read", "Read a file", serde_json::json!({})));

    assert!(pool.get("read").is_some());
    assert!(pool.get("write").is_none());
}

#[test]
fn test_link_subset() {
    let mut pool = ToolPool::new();
    pool.register(ToolDef::new("read", "...", serde_json::json!({})));
    pool.register(ToolDef::new("write", "...", serde_json::json!({})));
    pool.register(ToolDef::new("search", "...", serde_json::json!({})));

    let linked = pool.link(&["read", "search"]);
    assert_eq!(linked.len(), 2);
    assert!(linked.iter().any(|t| t.name == "read"));
    assert!(linked.iter().any(|t| t.name == "search"));
}

#[test]
fn test_link_all() {
    let mut pool = ToolPool::new();
    pool.register(ToolDef::new("read", "...", serde_json::json!({})));
    pool.register(ToolDef::new("write", "...", serde_json::json!({})));

    let linked = pool.link(&[]);
    assert_eq!(linked.len(), 2);
}
