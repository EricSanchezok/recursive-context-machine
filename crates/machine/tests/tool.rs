use std::future::Future;
use std::pin::Pin;

use machine::{Tool, ToolOutput, tool_def};
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
    ) -> Pin<Box<dyn Future<Output = Result<ToolOutput, String>> + Send + 'a>> {
        let msg = args["message"].as_str().unwrap_or("").to_string();
        Box::pin(async move {
            Ok(ToolOutput {
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
    ) -> Pin<Box<dyn Future<Output = Result<ToolOutput, String>> + Send + 'a>> {
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
    let result = t.execute(json!({"message": "hello"})).await.unwrap();
    assert_eq!(result.content, "hello");
    assert_eq!(result.title, Some("echo".to_string()));
}

#[tokio::test]
async fn tool_execute_can_fail() {
    let t = FailingTool;
    let result = t.execute(json!({})).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "intentional failure");
}

#[test]
fn tool_def_converts_to_fragment_schema() {
    let t = EchoTool;
    let def = tool_def(&t);
    assert_eq!(def.name, "echo");
    assert_eq!(def.description, "Echo the input back");
    assert_eq!(def.parameters["type"], "object");
}

#[tokio::test]
async fn tool_def_matches_tool_properties() {
    let t = EchoTool;
    let def = tool_def(&t);
    assert_eq!(def.name, t.name());
    assert_eq!(def.description, t.description());
    assert_eq!(def.parameters, t.parameters());
}
