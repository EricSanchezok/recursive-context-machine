use machine::{
    Action, ActionOutcome, Environment, Fragment, MachineEvent, Model, Resources, ToolDefinition,
};
use serde_json::json;
use storage::{MachineState, Store};
use tempfile::TempDir;

fn model(name: &str) -> Model {
    Model {
        name: name.to_string(),
        ..Default::default()
    }
}

fn tool_definition(name: &str) -> ToolDefinition {
    ToolDefinition {
        name: name.to_string(),
        description: format!("{name} tool"),
        parameters: json!({"type": "object"}),
    }
}

fn state_with_resources() -> MachineState {
    let resources = Resources::new()
        .with_model(model("fast"))
        .with_tool_definition(tool_definition("search"));
    MachineState {
        environment: Environment::empty("/tmp"),
        resources,
        ..MachineState::default()
    }
}

#[test]
fn restore_returns_none_for_empty_store() {
    let dir = TempDir::new().unwrap();
    let store = Store::open(dir.path()).unwrap();
    assert!(store.restore().unwrap().is_none());
}

#[test]
fn record_and_restore_context_actions() {
    let dir = TempDir::new().unwrap();
    let mut store = Store::open(dir.path()).unwrap();

    store
        .record(&MachineEvent::state_only(
            1,
            Action::Append(Fragment::user("hello")),
        ))
        .unwrap();
    store
        .record(&MachineEvent::state_only(
            2,
            Action::Append(Fragment::assistant("world")),
        ))
        .unwrap();

    let restored = store.restore().unwrap().unwrap();
    assert_eq!(restored.context.fragments().len(), 2);
    assert_eq!(restored.context.fragments()[0].as_text(), Some("hello"));
    assert_eq!(restored.context.fragments()[1].as_text(), Some("world"));
    assert_eq!(restored.step, 2);
}

#[test]
fn restore_replays_runtime_resource_actions() {
    let dir = TempDir::new().unwrap();
    let mut store = Store::open(dir.path()).unwrap();
    let mut state = state_with_resources();
    store.checkpoint(&state).unwrap();

    store
        .record(&MachineEvent::state_only(1, Action::Model("fast".into())))
        .unwrap();
    store
        .record(&MachineEvent::state_only(
            2,
            Action::Activate("search".into()),
        ))
        .unwrap();
    store
        .record(&MachineEvent::state_only(
            3,
            Action::Deactivate("search".into()),
        ))
        .unwrap();

    let restored = store.restore().unwrap().unwrap();
    assert_eq!(restored.resources.active_model, "fast");
    assert!(!restored.resources.active_tools.contains("search"));
    assert_eq!(restored.step, 3);

    state.resources.use_model("fast").unwrap();
    state.resources.enable("search").unwrap();
    state.resources.disable("search");
    assert_eq!(
        restored.resources.active_model,
        state.resources.active_model
    );
    assert_eq!(
        restored.resources.active_tools,
        state.resources.active_tools
    );
}

#[test]
fn halt_output_replays_through_inbox_and_take() {
    let dir = TempDir::new().unwrap();
    let mut store = Store::open(dir.path()).unwrap();

    store
        .record(&MachineEvent {
            step: 1,
            action: Action::Halt,
            outcome: ActionOutcome::Reactor {
                fragments: vec![Fragment::assistant("answer")],
                usage: machine::Usage {
                    input_tokens: 1,
                    output_tokens: 1,
                    total_tokens: 2,
                    cached_input_tokens: 0,
                    cache_creation_input_tokens: 0,
                    fragment_ids: Vec::new(),
                },
            },
        })
        .unwrap();
    store
        .record(&MachineEvent::state_only(2, Action::Take))
        .unwrap();

    let restored = store.restore().unwrap().unwrap();
    assert_eq!(restored.context.fragments().len(), 1);
    assert_eq!(restored.context.fragments()[0].as_text(), Some("answer"));
    assert!(restored.inbox.is_empty());
}

#[test]
fn checkpoint_skips_replaying_old_events() {
    let dir = TempDir::new().unwrap();
    let mut store = Store::open(dir.path()).unwrap();

    store
        .record(&MachineEvent::state_only(
            1,
            Action::Append(Fragment::user("before")),
        ))
        .unwrap();
    let mut checkpoint = MachineState::default();
    checkpoint.context.append(Fragment::user("checkpoint"));
    checkpoint.step = 1;
    store.checkpoint(&checkpoint).unwrap();
    store
        .record(&MachineEvent::state_only(
            2,
            Action::Append(Fragment::assistant("after")),
        ))
        .unwrap();

    let restored = store.restore().unwrap().unwrap();
    assert_eq!(restored.context.fragments().len(), 2);
    assert_eq!(
        restored.context.fragments()[0].as_text(),
        Some("checkpoint")
    );
    assert_eq!(restored.context.fragments()[1].as_text(), Some("after"));
}
