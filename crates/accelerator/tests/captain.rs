use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use accelerator::Captain;
use machine::{
    Action, Context, Environment, Inbox, Machine, Model, Phase, Policy, Purpose, Resources, Tool,
    ToolResult,
};
use serde_json::json;

#[derive(Clone)]
struct CaptainSetupOnly;

struct NamedTool {
    name: &'static str,
}

impl Policy for CaptainSetupOnly {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn pre(&self) -> Vec<Box<dyn Phase>> {
        Captain::new().pre()
    }

    fn decide<'a>(
        &'a self,
        _purpose: &'a Purpose,
        _ctx: &'a Context,
        _env: &'a Environment,
        _resources: &'a Resources,
        _inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async { Action::Done })
    }
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
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async {
            Ok(ToolResult {
                call_id: String::new(),
                content: String::new(),
                title: None,
            })
        })
    }
}

fn model(name: &str) -> Model {
    Model {
        name: name.to_string(),
        ..Default::default()
    }
}

fn named_tool(name: &'static str) -> Arc<dyn Tool> {
    Arc::new(NamedTool { name })
}

#[tokio::test]
async fn captain_prepares_prompt_first_model_and_all_tools() {
    let mut resources = Resources::named("test")
        .with_model(model("fast"))
        .with_model(model("careful"))
        .with_tool(named_tool("read"))
        .with_tool(named_tool("search"));
    resources
        .prompts
        .insert("captain".into(), "Captain prompt".into());
    resources
        .prompts
        .insert("other".into(), "Other prompt".into());
    let machine = Machine::new(Box::new(CaptainSetupOnly));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");

    machine
        .run(&Purpose::new("inspect"), &mut ctx, &mut env, &mut resources)
        .await;

    assert_eq!(resources.active_model, "fast");
    assert_eq!(resources.active_tools.len(), 2);
    assert!(resources.active_tools.contains("read"));
    assert!(resources.active_tools.contains("search"));
    assert_eq!(ctx.fragments()[0].as_text(), Some("Captain prompt"));
}
