mod common;

use machine::edit::{ContentSpec, EditOp, Position, Selector};
use machine::{
    Action, CellPredicate, ExecutionMode, Fragment, Inbox, Machine, MachineState, Overlay,
    ToolRuntime,
};

use serde_json::json;

async fn run_actions(actions: &[Action]) -> MachineState {
    let mut state = MachineState::default();
    state.run.environment = machine::Environment::new("/tmp");
    state.run.resources = common::test_resources();
    let mut machine = Machine::new("test", "test-machine");
    let tool_runtime = ToolRuntime::new();
    for action in actions {
        let result = machine
            .apply(
                action.clone(),
                &mut state,
                ExecutionMode::Live {
                    tool_runtime: &tool_runtime,
                    overlay: &Overlay::default(),
                },
            )
            .await;
        if result.done {
            break;
        }
    }
    state
}

fn insert_end(text: &str, role: machine::Role) -> Action {
    Action::Edit {
        ops: vec![EditOp::Insert {
            position: Position::End,
            content: ContentSpec::Literal {
                text: text.to_string(),
                role,
                tag: None,
            },
            anchor: None,
        }],
        because: None,
    }
}

fn edit(ops: Vec<EditOp>) -> Action {
    Action::Edit { ops, because: None }
}

fn first_hitch_text(state: &MachineState) -> String {
    state
        .frame
        .inbox
        .peek()
        .map(|item| item.fragment.content_as_text())
        .unwrap_or_default()
}

#[tokio::test]
async fn done_stops_immediately() {
    let state = run_actions(&[Action::Done, insert_end("ignored", machine::Role::User)]).await;
    assert!(state.run.context.is_empty());
    assert!(state.frame.status.is_done());
}

#[tokio::test]
async fn one_edit_batch_appends_multiple_cells() {
    // The v2 idiom: one action commits a batch of ops.
    let state = run_actions(&[edit(vec![
        EditOp::Insert {
            position: Position::End,
            content: ContentSpec::Literal {
                text: "sys".into(),
                role: machine::Role::System,
                tag: None,
            },
            anchor: None,
        },
        EditOp::Insert {
            position: Position::End,
            content: ContentSpec::Literal {
                text: "hello".into(),
                role: machine::Role::User,
                tag: None,
            },
            anchor: None,
        },
    ])])
    .await;
    assert_eq!(state.run.context.len(), 2);
    assert_eq!(state.run.context.fragments()[0].as_text(), Some("sys"));
    assert_eq!(state.run.context.fragments()[1].as_text(), Some("hello"));
}

#[tokio::test]
async fn consume_from_empty_inbox_surfaces_hitch() {
    // v2 semantic: consuming a missing inbox item is a policy decision
    // error, surfaced as a hitch effect — not a silent no-op.
    let state = run_actions(&[edit(vec![EditOp::Insert {
        position: Position::End,
        content: ContentSpec::Inbox { call_id: None },
        anchor: None,
    }])])
    .await;
    assert!(state.run.context.is_empty());
    assert!(!state.frame.inbox.is_empty());
    assert!(first_hitch_text(&state).contains("inbox item unavailable"));
}

#[tokio::test]
async fn set_is_idempotent_and_ordered() {
    // One batch writes two slots; a second action rewrites @agent. The
    // dedicated anchor semantics live in context_cells — here we prove the
    // machine dispatches Set ops onto the document.
    let state = run_actions(&[
        edit(vec![
            EditOp::Set {
                anchor: "@agent".into(),
                content: ContentSpec::Literal {
                    text: "agent v1".into(),
                    role: machine::Role::System,
                    tag: None,
                },
            },
            EditOp::Set {
                anchor: "@env".into(),
                content: ContentSpec::Literal {
                    text: "env".into(),
                    role: machine::Role::System,
                    tag: None,
                },
            },
        ]),
        edit(vec![EditOp::Set {
            anchor: "@agent".into(),
            content: ContentSpec::Literal {
                text: "agent v2".into(),
                role: machine::Role::System,
                tag: None,
            },
        }]),
        edit(vec![EditOp::Set {
            anchor: "@summary".into(),
            content: ContentSpec::Literal {
                text: "compact".into(),
                role: machine::Role::System,
                tag: None,
            },
        }]),
        Action::Done,
    ])
    .await;
    assert!(state.run.context.find_anchor("@summary").is_some());
    let agent_text = state
        .run
        .context
        .find_anchor("@agent")
        .and_then(|id| state.run.context.get(id))
        .and_then(|cell| cell.as_text().map(String::from));
    assert_eq!(agent_text.as_deref(), Some("agent v2"));
}

#[tokio::test]
async fn move_reorders_cells() {
    let state = run_actions(&[
        insert_end("first", machine::Role::System),
        edit(vec![EditOp::Set {
            anchor: "@notes".into(),
            content: ContentSpec::Literal {
                text: "notes".into(),
                role: machine::Role::System,
                tag: None,
            },
        }]),
        edit(vec![EditOp::Move {
            anchor: "@notes".into(),
            after: Position::End,
        }]),
        Action::Done,
    ])
    .await;
    // Move to End is refused (not a valid move target) — the hitch proves
    // the guard; document order stays intact.
    assert_eq!(state.run.context.len(), 2);
}

#[tokio::test]
async fn delete_by_predicate_removes_matching_cells() {
    let state = run_actions(&[
        insert_end("keep me", machine::Role::User),
        insert_end("stale", machine::Role::System),
        edit(vec![EditOp::Delete {
            selector: Selector::Where(CellPredicate {
                role: Some("system".into()),
                ..CellPredicate::default()
            }),
        }]),
        Action::Done,
    ])
    .await;
    assert_eq!(state.run.context.len(), 1);
    assert_eq!(state.run.context.fragments()[0].as_text(), Some("keep me"));
}

#[tokio::test]
async fn delete_protected_anchor_is_refused() {
    let state = run_actions(&[
        edit(vec![EditOp::Set {
            anchor: "@purpose".into(),
            content: ContentSpec::Literal {
                text: "goal".into(),
                role: machine::Role::User,
                tag: None,
            },
        }]),
        edit(vec![EditOp::Delete {
            selector: Selector::Anchor("@purpose".into()),
        }]),
        Action::Done,
    ])
    .await;
    // The scaffolding survives; the refusal lands as a hitch.
    assert!(state.run.context.find_anchor("@purpose").is_some());
    assert!(first_hitch_text(&state).contains("protected anchors"));
}

#[tokio::test]
async fn delete_unknown_id_returns_hitch() {
    let state = run_actions(&[
        insert_end("existing", machine::Role::System),
        edit(vec![EditOp::Delete {
            selector: Selector::Range {
                from: Position::Id(999),
                to: Position::Id(999),
            },
        }]),
    ])
    .await;
    assert_eq!(state.run.context.len(), 1);
    assert!(!state.frame.inbox.is_empty());
    assert!(first_hitch_text(&state).contains("cell id 999 not found"));
}

#[tokio::test]
async fn edit_consumes_inbox_into_document() {
    let mut state = MachineState::default();
    state.run.environment = machine::Environment::new("/tmp");
    state.run.resources = common::test_resources();
    state.frame.inbox.push(Fragment::assistant("reply"));
    state
        .frame
        .inbox
        .push(Fragment::tool_result("call-1", "5", None));

    let mut machine = Machine::new("test", "test-machine");
    let tool_runtime = ToolRuntime::new();

    let consume = |call_id: Option<&str>| {
        edit(vec![EditOp::Insert {
            position: Position::End,
            content: ContentSpec::Inbox {
                call_id: call_id.map(String::from),
            },
            anchor: None,
        }])
    };

    machine
        .apply(
            consume(None),
            &mut state,
            ExecutionMode::Live {
                tool_runtime: &tool_runtime,
                overlay: &Overlay::default(),
            },
        )
        .await;
    machine
        .apply(
            consume(Some("call-1")),
            &mut state,
            ExecutionMode::Live {
                tool_runtime: &tool_runtime,
                overlay: &Overlay::default(),
            },
        )
        .await;

    assert_eq!(state.run.context.len(), 2);
    assert_eq!(state.run.context.fragments()[0].as_text(), Some("reply"));
    assert!(matches!(
        state.run.context.fragments()[1].content,
        machine::Content::ToolResult(_)
    ));
    assert!(state.frame.inbox.is_empty());
}

#[tokio::test]
async fn failed_op_does_not_abort_remaining_batch() {
    let state = run_actions(&[edit(vec![
        EditOp::Set {
            anchor: "@agent".into(),
            content: ContentSpec::Literal {
                text: "agent".into(),
                role: machine::Role::System,
                tag: None,
            },
        },
        EditOp::Delete {
            selector: Selector::Anchor("@ghost".into()),
        },
        EditOp::Set {
            anchor: "@env".into(),
            content: ContentSpec::Literal {
                text: "env".into(),
                role: machine::Role::System,
                tag: None,
            },
        },
    ])])
    .await;
    // Both sets landed; the middle op's hitch is recorded.
    assert!(state.run.context.find_anchor("@agent").is_some());
    assert!(state.run.context.find_anchor("@env").is_some());
    assert!(first_hitch_text(&state).contains("anchor '@ghost' not found"));
}

#[test]
fn context_holds_mixed_assistant_fragments() {
    let mut context = machine::Context::new();
    let text_id = context.append(Fragment::assistant("我来计算"));
    let call_id = context.append(Fragment::tool_call(
        "call_1",
        "add",
        json!({"a": 3, "b": 2}),
    ));

    assert_eq!(context.len(), 2);
    assert_eq!(context.get(text_id).unwrap().as_text(), Some("我来计算"));
    assert!(matches!(
        context.get(call_id).unwrap().content,
        machine::Content::ToolCall(_)
    ));
}

#[test]
fn inbox_drain_order_with_mixed_response() {
    let mut inbox = Inbox::new();
    inbox.push(Fragment::assistant("我来计算"));
    inbox.push(Fragment::tool_call("1", "add", json!({"a": 3, "b": 2})));
    inbox.push(Fragment::tool_result("1", "5", None));

    let mut context = machine::Context::new();
    while inbox.peek().is_some() {
        if let Some(item) = inbox.pop() {
            context.append(item.fragment);
        }
    }

    assert_eq!(context.len(), 3);
    assert_eq!(context.fragments()[0].as_text(), Some("我来计算"));
    assert!(matches!(
        context.fragments()[1].content,
        machine::Content::ToolCall(_)
    ));
    assert!(matches!(
        context.fragments()[2].content,
        machine::Content::ToolResult(_)
    ));
}

#[test]
fn use_model_nonexistent_returns_error() {
    let mut resources = common::test_resources();
    let error = resources.use_model("nonexistent").unwrap_err();
    assert_eq!(error.to_string(), "model 'nonexistent' not registered");
}

#[test]
fn enable_nonexistent_tool_returns_error() {
    let mut resources = common::test_resources();
    let error = resources.enable("nonexistent").unwrap_err();
    assert_eq!(error.to_string(), "tool 'nonexistent' not registered");
}

#[tokio::test]
async fn dispatch_model_nonexistent_pushes_hitch() {
    let state = run_actions(&[Action::Model("nonexistent".to_string())]).await;
    assert_eq!(state.frame.inbox.len(), 1);
    let fragment = &state.frame.inbox.peek().unwrap().fragment;
    assert!(matches!(fragment.content, machine::Content::Hitch { .. }));
    assert_eq!(fragment.role, machine::Role::System);
    assert!(fragment.content_as_text().contains("nonexistent"));
}

#[tokio::test]
async fn dispatch_activate_nonexistent_pushes_hitch() {
    let state = run_actions(&[Action::Activate("unknown".to_string())]).await;
    assert_eq!(state.frame.inbox.len(), 1);
    let fragment = &state.frame.inbox.peek().unwrap().fragment;
    assert!(matches!(fragment.content, machine::Content::Hitch { .. }));
    assert_eq!(fragment.role, machine::Role::System);
    assert!(fragment.content_as_text().contains("unknown"));
}

#[tokio::test]
async fn complete_no_active_model_returns_hitch() {
    let mut context = machine::Context::new();
    context.append(Fragment::user("hello"));
    let mut resources = common::test_resources();
    resources.deactivate_model();

    let (fragments, tokens) =
        machine::completion::complete(&context, &resources, &Overlay::default()).await;
    assert_eq!(fragments.len(), 1);
    assert_eq!(fragments[0].role, machine::Role::System);
    assert_eq!(tokens.total_tokens, 0);
    assert!(matches!(
        fragments[0].content,
        machine::Content::Hitch { .. }
    ));
    assert!(fragments[0].content_as_text().contains("no active model"));
}
