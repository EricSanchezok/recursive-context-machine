use std::sync::Arc;

use accelerator::Captain;
use machine::{
    Action, Context, Environment, ExecutionMode, Fragment, Machine, MachineState, Model, Policy,
    PolicyView, Purpose, Resources, Role, RunState, Tool, ToolDefinition, ToolResult, ToolRuntime,
};
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

fn resources() -> Resources {
    let mut resources = Resources::named("test")
        .with_model(Model {
            name: "fast".into(),
            ..Default::default()
        })
        .with_model(Model {
            name: "careful".into(),
            ..Default::default()
        })
        .with_tool_definition(ToolDefinition::from_tool(named_tool("read").as_ref()))
        .with_tool_definition(ToolDefinition::from_tool(named_tool("search").as_ref()));
    resources
        .prompts
        .insert("captain".into(), "Captain prompt".into());
    resources
}

async fn drive_until_halt(
    captain: &Captain,
    ctx: &mut Context,
    resources: &mut Resources,
    purpose: &str,
) {
    let mut state = MachineState {
        run: RunState {
            purpose: Purpose::new(purpose),
            run_dir: None,
            context: ctx.clone(),
            environment: Environment::new("."),
            resources: resources.clone(),
            telemetry: machine::Telemetry::default(),
        },
        frame: machine::MachineFrame::default(),
    };
    let mut machine = Machine::new("test", "test");
    let tool_runtime = ToolRuntime::new();

    for _ in 0..100 {
        let obs = machine::obs::measure(&state.run);
        let action = captain
            .decide(PolicyView {
                run: &state.run,
                inbox: &state.frame.inbox,
                step: state.frame.step,
                status: state.frame.status,
                obs: &obs,
            })
            .await;
        match action {
            Action::Halt => {
                *ctx = state.run.context;
                *resources = state.run.resources;
                return;
            }
            Action::Done => panic!("captain ended before first halt"),
            action => {
                machine
                    .apply(
                        action,
                        &mut state,
                        ExecutionMode::Live {
                            tool_runtime: &tool_runtime,
                            overlay: &machine::Overlay::default(),
                        },
                    )
                    .await;
            }
        }
    }

    panic!("captain did not halt within step budget");
}

#[tokio::test]
async fn captain_prepares_prompt_first_model_and_all_tools() {
    let captain = Captain::new();
    let mut ctx = Context::new();
    let mut resources = resources();

    drive_until_halt(&captain, &mut ctx, &mut resources, "").await;

    assert_eq!(resources.active_model, "fast");
    assert_eq!(resources.active_tools.len(), 2);
    assert!(resources.active_tools.contains("read"));
    assert!(resources.active_tools.contains("search"));
    assert_eq!(ctx.fragments()[0].as_text(), Some("Captain prompt"));
}

#[tokio::test]
async fn captain_normalizes_agent_prompt_to_unique_anchored_slot() {
    let captain = Captain::new();
    let mut ctx = Context::new();
    ctx.append(Fragment::user("existing user content"));
    // v2 canonical input: the agent cell is an anchored slot with stale
    // content; captain must replace it in place (idempotent Set).
    ctx.set_named("@agent", Fragment::system("old prompt").with_tag("agent"));
    let mut resources = resources();

    drive_until_halt(&captain, &mut ctx, &mut resources, "").await;

    let agent_cells: Vec<_> = ctx
        .fragments()
        .iter()
        .filter(|fragment| fragment.anchor.as_deref() == Some("@agent"))
        .collect();
    assert_eq!(agent_cells.len(), 1, "exactly one @agent slot");
    assert_eq!(agent_cells[0].tag, "agent");
    assert_eq!(agent_cells[0].as_text(), Some("Captain prompt"));
    // The slot outranks unanchored content (SLOT_ORDER header region).
    assert!(
        ctx.fragments()
            .iter()
            .position(|fragment| fragment.anchor.as_deref() == Some("@agent"))
            .unwrap()
            < ctx
                .fragments()
                .iter()
                .position(|fragment| fragment.tag == "user")
                .unwrap()
    );
}

#[tokio::test]
async fn captain_appends_runtime_purpose_as_user_message() {
    let captain = Captain::new();
    let mut ctx = Context::new();
    ctx.append(Fragment::system("Captain prompt").with_tag("agent"));
    ctx.append(Fragment::system("Instructions").with_tag("instruction"));
    ctx.append(Fragment::user("initial").with_tag("purpose"));
    ctx.append(Fragment::system("env").with_tag("env"));
    ctx.append(Fragment::assistant("first answer"));
    let mut resources = resources();

    drive_until_halt(&captain, &mut ctx, &mut resources, "second").await;

    let purposes: Vec<_> = ctx
        .fragments()
        .iter()
        .filter(|fragment| fragment.tag == "purpose")
        .collect();
    assert_eq!(purposes.len(), 2);
    assert!(purposes.iter().all(|fragment| fragment.role == Role::User));
    assert_eq!(
        purposes.last().and_then(|fragment| fragment.as_text()),
        Some("second")
    );
}
