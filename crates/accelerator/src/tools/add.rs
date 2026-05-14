use std::future::Future;
use std::pin::Pin;

use machine::{Tool, ToolResult};
use serde_json::Value;

/// Built-in tool: adds two integers.
///
/// Usage: `add { "a": 3, "b": 5 }` → `"8"`
pub struct AddTool;

impl Tool for AddTool {
    fn name(&self) -> &str {
        "add"
    }

    fn description(&self) -> &str {
        "Add two integers together. Input: { a: int, b: int }."
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "a": { "type": "integer", "description": "First integer" },
                "b": { "type": "integer", "description": "Second integer" }
            },
            "required": ["a", "b"]
        })
    }

    fn execute<'a>(
        &'a self,
        args: Value,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let a = args["a"].as_i64().ok_or("missing or invalid 'a'")?;
            let b = args["b"].as_i64().ok_or("missing or invalid 'b'")?;
            Ok(ToolResult {
                call_id: String::new(),
                content: format!("{}", a + b),
                title: Some(format!("add({a}, {b})")),
            })
        })
    }
}
