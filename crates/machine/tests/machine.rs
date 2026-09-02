mod common;

use machine::{
    Action, ExecutionMode, Fragment, Inbox, Machine, MachineState, Overlay, ToolRuntime,
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

#[tokio::test]
async fn done_stops_immediately() {
    let state = run_actions(&[Action::Done, Action::Append(Fragment::user("ignored"))]).await;
    assert!(state.run.context.is_empty());
    assert!(state.frame.status.is_done());
}

#[tokio::test]
async fn append_and_take_flow() {
    let state = run_actions(&[
        Action::Append(Fragment::system("sys")),
        Action::Append(Fragment::user("hello")),
        Action::Done,
    ])
    .await;
    assert_eq!(state.run.context.len(), 2);
    assert_eq!(state.run.context.fragments()[0].as_text(), Some("sys"));
    assert_eq!(state.run.context.fragments()[1].as_text(), Some("hello"));
}

#[tokio::test]
async fn take_empty_inbox_is_noop() {
    let state = run_actions(&[Action::Take, Action::Done]).await;
    assert!(state.run.context.is_empty());
}

#[tokio::test]
async fn swap_preserves_count() {
    let state = run_actions(&[
        Action::Append(Fragment::system("first")),
        Action::Append(Fragment::system("second")),
        Action::Swap(1, 2),
        Action::Done,
    ])
    .await;
    assert_eq!(state.run.context.len(), 2);
    assert_eq!(state.run.context.fragments()[0].as_text(), Some("second"));
    assert_eq!(state.run.context.fragments()[1].as_text(), Some("first"));
}

#[tokio::test]
async fn replace_preserves_id() {
    let state = run_actions(&[
        Action::Append(Fragment::system("old")),
        Action::Replace {
            id: 1,
            fragment: Fragment::system("new"),
        },
        Action::Done,
    ])
    .await;
    assert_eq!(state.run.context.len(), 1);
    assert_eq!(state.run.context.fragments()[0].as_text(), Some("new"));
    assert_eq!(state.run.context.fragments()[0].id(), 1);
}

#[tokio::test]
async fn insert_after_id() {
    let state = run_actions(&[
        Action::Append(Fragment::system("first")),
        Action::Append(Fragment::system("third")),
        Action::Insert {
            after: 1,
            fragment: Fragment::system("second"),
        },
        Action::Done,
    ])
    .await;
    assert_eq!(state.run.context.len(), 3);
    assert_eq!(state.run.context.fragments()[0].as_text(), Some("first"));
    assert_eq!(state.run.context.fragments()[1].as_text(), Some("second"));
    assert_eq!(state.run.context.fragments()[2].as_text(), Some("third"));
}

#[tokio::test]
async fn remove_and_check_context() {
    let state = run_actions(&[
        Action::Append(Fragment::system("a")),
        Action::Append(Fragment::user("b")),
        Action::Remove(1),
        Action::Done,
    ])
    .await;
    assert_eq!(state.run.context.len(), 1);
    assert_eq!(state.run.context.fragments()[0].as_text(), Some("b"));
}

#[tokio::test]
async fn remove_unknown_returns_hitch() {
    let state = run_actions(&[
        Action::Append(Fragment::system("existing")),
        Action::Remove(999),
    ])
    .await;
    assert_eq!(state.frame.inbox.len(), 1);
    let fragment = &state.frame.inbox.peek().unwrap().fragment;
    assert!(matches!(fragment.content, machine::Content::Hitch { .. }));
    assert!(
        fragment
            .content_as_text()
            .contains("fragment id 999 not found")
    );
}

#[tokio::test]
async fn take_drains_inbox_into_context() {
    let mut state = MachineState::default();
    state.run.environment = machine::Environment::new("/tmp");
    state.run.resources = common::test_resources();
    state.frame.inbox.push(Fragment::assistant("reply"));
    state
        .frame
        .inbox
        .push(Fragment::tool_result("1", "5", None));

    let mut machine = Machine::new("test", "test-machine");
    let tool_runtime = ToolRuntime::new();
    machine
        .apply(
            Action::Take,
            &mut state,
            ExecutionMode::Live {
                tool_runtime: &tool_runtime,
                overlay: &Overlay::default(),
            },
        )
        .await;
    machine
        .apply(
            Action::Take,
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
