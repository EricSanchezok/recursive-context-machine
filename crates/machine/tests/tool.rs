use std::future::Future;
use std::pin::Pin;

use machine::{Environment, Tool, ToolResult};
use serde_json::json;

struct EchoTool;

impl Tool for EchoTool {
    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "Echo the input back"
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "message": { "type": "string" }
            }
        })
    }

    fn execute<'a>(
        &'a self,
        args: serde_json::Value,
        _env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        let msg = args["message"].as_str().unwrap_or("").to_string();
        Box::pin(async move {
            Ok(ToolResult {
                call_id: String::new(),
                content: msg,
                title: Some("echo".to_string()),
            })
        })
    }
}

struct FailingTool;

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
async fn tool_name_and_description() {
    let t = EchoTool;
    assert_eq!(t.name(), "echo");
    assert_eq!(t.description(), "Echo the input back");
}

#[tokio::test]
async fn tool_execute_returns_result() {
    let t = EchoTool;
    let result = t
        .execute(
            json!({"message": "hello"}),
            &machine::Environment::new("/tmp"),
        )
        .await
        .unwrap();
    assert_eq!(result.content, "hello");
    assert_eq!(result.title, Some("echo".to_string()));
}

#[tokio::test]
async fn tool_execute_can_fail() {
    let t = FailingTool;
    let result = t
        .execute(json!({}), &machine::Environment::new("/tmp"))
        .await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "intentional failure");
}
