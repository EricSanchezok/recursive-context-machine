use std::future::Future;
use std::pin::Pin;

use machine::{Environment, Tool, ToolResult};
use serde_json::json;

struct EchoTool;
struct FailingTool;

impl Tool for EchoTool {
    fn name(&self) -> &str {
        "echo"
    }
    fn description(&self) -> &str {
        "Echo the input"
    }
    fn parameters(&self) -> serde_json::Value {
        json!({"type": "object", "properties": {"msg": {"type": "string"}}})
    }
    fn execute<'a>(
        &'a self,
        args: serde_json::Value,
        _env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        let content = args["msg"].as_str().unwrap_or("").to_string();
        Box::pin(async move {
            Ok(ToolResult {
                call_id: String::new(),
                content,
                title: Some("echo".to_string()),
            })
        })
    }
}

impl Tool for FailingTool {
    fn name(&self) -> &str {
        "fail"
    }
    fn description(&self) -> &str {
        "Always fails"
    }
    fn parameters(&self) -> serde_json::Value {
        json!({})
    }
    fn execute<'a>(
        &'a self,
        _args: serde_json::Value,
        _env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move { Err("intentional failure".to_string()) })
    }
}

#[tokio::test]
async fn execute_ok() {
    let t = EchoTool;
    let result = t
        .execute(json!({"msg": "hello"}), &Environment::new("/tmp"))
        .await
        .unwrap();
    assert_eq!(result.content, "hello");
}

#[tokio::test]
async fn execute_err() {
    let t = FailingTool;
    let result = t.execute(json!({}), &Environment::new("/tmp")).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "intentional failure");
}
