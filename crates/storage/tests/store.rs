use machine::edit::{ContentSpec, EditOp, Position, Selector};
use machine::{
    Action, Effect, Fragment, InboxItem, MachineState, Model, Obs, Resources, StoredEvent,
    ToolDefinition,
};
use serde_json::json;
use storage::{Store, TrajectoryEvent};
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
    let mut state = MachineState::default();
    state.run.environment = machine::Environment::empty("/tmp");
    state.run.resources = resources;
    state
}

fn trajectory(step: u64, action: Action, effects: Vec<Effect>) -> TrajectoryEvent {
    TrajectoryEvent {
        step,
        obs: Obs::default(),
        ledger_transitions: Vec::new(),
        registry_events: Vec::new(),
        drain_effects: Vec::new(),
        event: StoredEvent::new(step, action, effects),
    }
}

fn edit_action(ops: Vec<EditOp>) -> Action {
    Action::Edit { ops, because: None }
}

fn insert_end(text: &str) -> EditOp {
    EditOp::Insert {
        position: Position::End,
        content: ContentSpec::Literal {
            text: text.to_string(),
            role: machine::Role::User,
            tag: None,
        },
        anchor: None,
    }
}

#[tokio::test]
async fn restore_returns_none_for_empty_store() {
    let dir = TempDir::new().unwrap();
    let store = Store::open(dir.path()).unwrap();
    assert!(store.restore().await.unwrap().is_none());
}

#[tokio::test]
async fn record_and_restore_edit_insertions() {
    let dir = TempDir::new().unwrap();
    let mut store = Store::open(dir.path()).unwrap();

    store
        .record_trajectory(&trajectory(
            1,
            edit_action(vec![insert_end("hello")]),
            vec![
                Effect::ActionCounted {
                    action: "edit".into(),
                },
                Effect::ContextInserted {
                    id: 1,
                    after: None,
                    fragment: Fragment::user("hello"),
                    source_completion: None,
                },
            ],
        ))
        .unwrap();
    store
        .record_trajectory(&trajectory(
            2,
            edit_action(vec![EditOp::Insert {
                position: Position::End,
                content: ContentSpec::Literal {
                    text: "world".into(),
                    role: machine::Role::Assistant,
                    tag: None,
                },
                anchor: None,
            }]),
            vec![
                Effect::ActionCounted {
                    action: "edit".into(),
                },
                Effect::ContextInserted {
                    id: 2,
                    after: None,
                    fragment: Fragment::assistant("world"),
                    source_completion: None,
                },
            ],
        ))
        .unwrap();

    let restored = store.restore().await.unwrap().unwrap();
    assert_eq!(restored.run.context.fragments().len(), 2);
    assert_eq!(restored.run.context.fragments()[0].as_text(), Some("hello"));
    assert_eq!(restored.run.context.fragments()[1].as_text(), Some("world"));
    assert_eq!(restored.frame.step, 2);
}

#[tokio::test]
async fn restore_replays_runtime_resource_actions() {
    let dir = TempDir::new().unwrap();
    let mut store = Store::open(dir.path()).unwrap();
    let mut state = state_with_resources();
    store.checkpoint(&state).unwrap();

    store
        .record_trajectory(&trajectory(
            1,
            Action::Model("fast".into()),
            vec![
                Effect::ActionCounted {
                    action: "model".into(),
                },
                Effect::ModelSelected {
                    name: "fast".into(),
                },
            ],
        ))
        .unwrap();
    store
        .record_trajectory(&trajectory(
            2,
            Action::Activate("search".into()),
            vec![
                Effect::ActionCounted {
                    action: "activate".into(),
                },
                Effect::ToolActivated {
                    name: "search".into(),
                },
            ],
        ))
        .unwrap();
    store
        .record_trajectory(&trajectory(
            3,
            Action::Deactivate("search".into()),
            vec![
                Effect::ActionCounted {
                    action: "deactivate".into(),
                },
                Effect::ToolDeactivated {
                    name: "search".into(),
                },
            ],
        ))
        .unwrap();

    let restored = store.restore().await.unwrap().unwrap();
    assert_eq!(restored.run.resources.active_model, "fast");
    assert!(!restored.run.resources.active_tools.contains("search"));
    assert_eq!(restored.frame.step, 3);

    state.run.resources.use_model("fast").unwrap();
    state.run.resources.enable("search").unwrap();
    state.run.resources.disable("search");
    assert_eq!(
        restored.run.resources.active_model,
        state.run.resources.active_model
    );
    assert_eq!(
        restored.run.resources.active_tools,
        state.run.resources.active_tools
    );
}

#[tokio::test]
async fn failed_op_replays_recorded_inbox_hitch() {
    let dir = TempDir::new().unwrap();
    let mut store = Store::open(dir.path()).unwrap();
    let hitch = Fragment::hitch(
        "anchor '@ghost' not found in context",
        None,
        machine::Role::System,
        None::<&str>,
    );

    store
        .record_trajectory(&trajectory(
            1,
            edit_action(vec![EditOp::Delete {
                selector: Selector::Anchor("@ghost".into()),
            }]),
            vec![
                Effect::ActionCounted {
                    action: "edit".into(),
                },
                Effect::InboxPushed {
                    item: InboxItem::new(hitch, None),
                },
            ],
        ))
        .unwrap();

    let restored = store.restore().await.unwrap().unwrap();
    assert_eq!(restored.run.context.fragments().len(), 0);
    assert_eq!(restored.frame.inbox.len(), 1);
    assert!(
        restored
            .frame
            .inbox
            .peek()
            .unwrap()
            .fragment
            .content_as_text()
            .contains("anchor '@ghost' not found")
    );
}

#[tokio::test]
async fn completion_output_replays_through_inbox_and_consume() {
    let dir = TempDir::new().unwrap();
    let mut store = Store::open(dir.path()).unwrap();
    let completion_id = machine::CompletionId(1);

    store
        .record_trajectory(&trajectory(
            1,
            Action::Halt,
            vec![
                Effect::ActionCounted {
                    action: "halt".into(),
                },
                Effect::CompletionRecorded {
                    record: machine::CompletionRecord {
                        id: completion_id,
                        step: 1,
                        model: Some("fast".into()),
                        tokens: machine::TokenUsage {
                            input_tokens: 1,
                            output_tokens: 1,
                            total_tokens: 2,
                            cached_input_tokens: 0,
                            cache_creation_input_tokens: 0,
                        },
                        output_fragment_ids: Vec::new(),
                    },
                    inbox_items: vec![InboxItem::new(
                        Fragment::assistant("answer"),
                        Some(completion_id),
                    )],
                },
            ],
        ))
        .unwrap();
    store
        .record_trajectory(&trajectory(
            2,
            edit_action(vec![EditOp::Insert {
                position: Position::End,
                content: ContentSpec::Inbox { call_id: None },
                anchor: None,
            }]),
            vec![
                Effect::ActionCounted {
                    action: "edit".into(),
                },
                Effect::InboxConsumed {
                    call_id: None,
                    item: InboxItem::new(Fragment::assistant("answer"), Some(completion_id)),
                },
                Effect::ContextInserted {
                    id: 1,
                    after: None,
                    fragment: Fragment::assistant("answer"),
                    source_completion: Some(completion_id),
                },
            ],
        ))
        .unwrap();

    let restored = store.restore().await.unwrap().unwrap();
    assert_eq!(restored.run.context.fragments().len(), 1);
    assert_eq!(
        restored.run.context.fragments()[0].as_text(),
        Some("answer")
    );
    assert!(restored.frame.inbox.is_empty());
    assert_eq!(restored.run.telemetry.completions.len(), 1);
    assert_eq!(restored.run.telemetry.completions[0].tokens.total_tokens, 2);
    assert_eq!(
        restored.run.telemetry.completions[0].output_fragment_ids,
        [1]
    );
}

#[tokio::test]
async fn checkpoint_skips_replaying_old_events() {
    let dir = TempDir::new().unwrap();
    let mut store = Store::open(dir.path()).unwrap();

    store
        .record_trajectory(&trajectory(
            1,
            edit_action(vec![insert_end("before")]),
            vec![
                Effect::ActionCounted {
                    action: "edit".into(),
                },
                Effect::ContextInserted {
                    id: 1,
                    after: None,
                    fragment: Fragment::user("before"),
                    source_completion: None,
                },
            ],
        ))
        .unwrap();
    let mut checkpoint = MachineState::default();
    checkpoint.run.context.append(Fragment::user("checkpoint"));
    checkpoint.frame.step = 1;
    store.checkpoint(&checkpoint).unwrap();
    store
        .record_trajectory(&trajectory(
            2,
            edit_action(vec![EditOp::Insert {
                position: Position::End,
                content: ContentSpec::Literal {
                    text: "after".into(),
                    role: machine::Role::Assistant,
                    tag: None,
                },
                anchor: None,
            }]),
            vec![
                Effect::ActionCounted {
                    action: "edit".into(),
                },
                Effect::ContextInserted {
                    id: 2,
                    after: None,
                    fragment: Fragment::assistant("after"),
                    source_completion: None,
                },
            ],
        ))
        .unwrap();

    let restored = store.restore().await.unwrap().unwrap();
    assert_eq!(restored.run.context.fragments().len(), 2);
    assert_eq!(
        restored.run.context.fragments()[0].as_text(),
        Some("checkpoint")
    );
    assert_eq!(restored.run.context.fragments()[1].as_text(), Some("after"));
}

#[tokio::test]
async fn trajectories_round_trip_observation_snapshots() {
    let dir = TempDir::new().unwrap();
    let mut store = Store::open(dir.path()).unwrap();

    let mut observed = Obs::default();
    observed.budget.context_limit = 128_000;
    observed.budget.estimated_input = 4_096;
    observed.budget.last_actual_input = Some(3_850);
    let recorded = TrajectoryEvent {
        step: 7,
        obs: observed,
        ledger_transitions: Vec::new(),
        registry_events: Vec::new(),
        drain_effects: Vec::new(),
        event: StoredEvent::new(
            7,
            Action::Halt,
            vec![Effect::ActionCounted {
                action: "halt".into(),
            }],
        ),
    };
    store.record_trajectory(&recorded).unwrap();

    let replayed = store.trajectories().await.unwrap();
    assert_eq!(replayed.len(), 1);
    assert_eq!(replayed[0].step, 7);
    assert_eq!(replayed[0].obs.budget.context_limit, 128_000);
    assert_eq!(replayed[0].obs.budget.estimated_input, 4_096);
    assert_eq!(replayed[0].obs.budget.last_actual_input, Some(3_850));
    assert_eq!(replayed[0].event.action, Action::Halt);
}
