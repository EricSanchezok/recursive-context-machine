use std::sync::Arc;

use accelerator::{Accelerator, Captain, State};
use machine::{Environment, Model, Resources, Tool, ToolResult};
use serde_json::json;

struct NamedTool {
    name: &'static str,
}

impl Tool for NamedTool {
    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        "test tool"
    }

    fn parameters(&self) -> serde_json::Value {
        json!({"type": "object"})
    }

    fn execute<'a>(
        &'a self,
        _args: serde_json::Value,
        _env: &'a Environment,
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

fn named_tool(name: &'static str) -> Arc<dyn Tool> {
    Arc::new(NamedTool { name })
}

#[tokio::test]
async fn captain_prepares_prompt_first_model_and_all_tools() {
    let mut resources = Resources::named("test")
        .with_model(Model {
            name: "fast".into(),
            ..Default::default()
        })
        .with_model(Model {
            name: "careful".into(),
            ..Default::default()
        })
        .with_tool(named_tool("read"))
        .with_tool(named_tool("search"));
    resources
        .prompts
        .insert("captain".into(), "Captain prompt".into());
    resources
        .prompts
        .insert("other".into(), "Other prompt".into());

    let state = State {
        res: resources,
        ..State::default()
    };

    // Run through the full Captain policy — setup steps run inline inside
    // decide(). The policy emits Done when the context has no pending items.
    let accelerator = Accelerator::primitive(state, Box::new(Captain::new()), "captain-test");

    let output = accelerator.run_with(State::default()).await;

    // After setup completes and decide emits Done:
    // - The first available model should be activated
    // - All tools should be activated
    // - The captain prompt should be in context
    assert_eq!(output.res.active_model, "fast");
    assert_eq!(output.res.active_tools.len(), 2);
    assert!(output.res.active_tools.contains("read"));
    assert!(output.res.active_tools.contains("search"));
    assert_eq!(output.ctx.fragments()[0].as_text(), Some("Captain prompt"));
}
