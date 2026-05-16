use std::future::Future;
use std::pin::Pin;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;

pub struct AddTool;

impl Tool for AddTool {
    fn name(&self) -> &str {
        "add"
    }

    fn description(&self) -> &str {
        "Add two integers together."
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "first": { "type": "integer", "description": "First integer" },
                "second": { "type": "integer", "description": "Second integer" }
            },
            "required": ["first", "second"]
        })
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        _env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let first = args["first"]
                .as_i64()
                .ok_or("invalid 'first': expected integer")?;
            let second = args["second"]
                .as_i64()
                .ok_or("invalid 'second': expected integer")?;
            Ok(ToolResult {
                call_id: String::new(),
                content: format!("{}", first + second),
                title: Some(format!("add({first}, {second})")),
            })
        })
    }
}
