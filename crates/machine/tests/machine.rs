mod common;

use machine::{
    Action, Context, Environment, Fragment, Inbox, Machine, MachineRuntime, ToolRuntime,
};
use serde_json::json;

async fn run_actions(
    actions: &[Action],
    ctx: &mut Context,
    env: &mut Environment,
    resources: &mut machine::Resources,
) {
    let mut machine = Machine::new("test", "test-machine");
    let mut inbox = Inbox::new();
    let tool_runtime = ToolRuntime::new();
    let mut step = 0u64;
    for action in actions {
        step += 1;
        let done = machine
            .apply(
                action.clone(),
                step,
                MachineRuntime {
                    ctx,
                    env,
                    resources,
                    tool_runtime: &tool_runtime,
                    inbox: &mut inbox,
                },
            )
            .await;
        if done.done {
            break;
        }
    }
}

#[tokio::test]
async fn done_stops_immediately() {
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();
    run_actions(&[Action::Done], &mut ctx, &mut env, &mut resources).await;
    assert!(ctx.is_empty());
}

#[tokio::test]
async fn append_and_take_flow() {
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();
    run_actions(
        &[
            Action::Append(Fragment::system("sys")),
            Action::Append(Fragment::user("hello")),
            Action::Done,
        ],
        &mut ctx,
        &mut env,
        &mut resources,
    )
    .await;
    assert_eq!(ctx.len(), 2);
    assert_eq!(ctx.fragments()[0].as_text(), Some("sys"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("hello"));
}

#[tokio::test]
async fn take_empty_inbox_is_noop() {
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();
    run_actions(
        &[Action::Take, Action::Done],
        &mut ctx,
        &mut env,
        &mut resources,
    )
    .await;
    assert!(ctx.is_empty());
}

#[tokio::test]
async fn swap_preserves_count() {
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();
    run_actions(
        &[
            Action::Append(Fragment::system("first")),
            Action::Append(Fragment::system("second")),
            Action::Swap(1, 2),
            Action::Done,
        ],
        &mut ctx,
        &mut env,
        &mut resources,
    )
    .await;
    assert_eq!(ctx.len(), 2);
    assert_eq!(ctx.fragments()[0].as_text(), Some("second"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("first"));
}

#[tokio::test]
async fn replace_preserves_id() {
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();
    run_actions(
        &[
            Action::Append(Fragment::system("old")),
            Action::Replace {
                id: 1,
                fragment: Fragment::system("new"),
            },
            Action::Done,
        ],
        &mut ctx,
        &mut env,
        &mut resources,
    )
    .await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("new"));
    assert_eq!(ctx.fragments()[0].id(), 1);
}

#[tokio::test]
async fn insert_after_id() {
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();
    run_actions(
        &[
            Action::Append(Fragment::system("first")),
            Action::Append(Fragment::system("third")),
            Action::Insert {
                after: 1,
                fragment: Fragment::system("second"),
            },
            Action::Done,
        ],
        &mut ctx,
        &mut env,
        &mut resources,
    )
    .await;
    assert_eq!(ctx.len(), 3);
    assert_eq!(ctx.fragments()[0].as_text(), Some("first"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("second"));
    assert_eq!(ctx.fragments()[2].as_text(), Some("third"));
}

#[tokio::test]
async fn remove_and_check_context() {
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();
    run_actions(
        &[
            Action::Append(Fragment::system("a")),
            Action::Append(Fragment::user("b")),
            Action::Remove(1),
            Action::Done,
        ],
        &mut ctx,
        &mut env,
        &mut resources,
    )
    .await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("b"));
}

#[tokio::test]
async fn remove_unknown_returns_hitch() {
    let mut ctx = Context::new();
    ctx.append(Fragment::system("existing"));
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    // Remove a non-existent id — should NOT panic.
    // The Apply action will push a hitch.
    let mut machine = Machine::new("test", "test-machine");
    let mut inbox = Inbox::new();
    let tool_runtime = ToolRuntime::new();
    let done = machine
        .apply(
            Action::Remove(999),
            1,
            MachineRuntime {
                ctx: &mut ctx,
                env: &mut env,
                resources: &mut resources,
                tool_runtime: &tool_runtime,
                inbox: &mut inbox,
            },
        )
        .await;

    assert!(!done.done, "remove with stale id should not terminate");
    assert_eq!(inbox.len(), 1);
    let frag = inbox.pop().unwrap();
    assert!(matches!(frag.content, machine::Content::Hitch { .. }));
    assert!(frag.content_as_text().contains("fragment id 999 not found"));
}

#[tokio::test]
async fn take_drains_inbox_into_context() {
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();
    let mut inbox = Inbox::new();
    let mut machine = Machine::new("test", "test-machine");
    let tool_runtime = ToolRuntime::new();

    inbox.push(Fragment::assistant("reply"));
    inbox.push(Fragment::tool_result("1", "5", None));

    machine
        .apply(
            Action::Take,
            1,
            MachineRuntime {
                ctx: &mut ctx,
                env: &mut env,
                resources: &mut resources,
                tool_runtime: &tool_runtime,
                inbox: &mut inbox,
            },
        )
        .await;
    machine
        .apply(
            Action::Take,
            2,
            MachineRuntime {
                ctx: &mut ctx,
                env: &mut env,
                resources: &mut resources,
                tool_runtime: &tool_runtime,
                inbox: &mut inbox,
            },
        )
        .await;

    assert_eq!(ctx.len(), 2);
    assert_eq!(ctx.fragments()[0].as_text(), Some("reply"));
    assert!(matches!(
        ctx.fragments()[1].content,
        machine::Content::ToolResult(_)
    ));
    assert!(inbox.is_empty());
}

#[test]
fn context_holds_mixed_assistant_fragments() {
    let mut ctx = Context::new();
    let id1 = ctx.append(Fragment::assistant("我来计算"));
    let id2 = ctx.append(Fragment::tool_call(
        "call_1",
        "add",
        json!({"a": 3, "b": 2}),
    ));

    assert_eq!(ctx.len(), 2);
    assert_eq!(ctx.get(id1).unwrap().as_text(), Some("我来计算"));
    assert!(matches!(
        ctx.get(id2).unwrap().content,
        machine::Content::ToolCall(_)
    ));
}

#[test]
fn inbox_drain_order_with_mixed_response() {
    let mut inbox = Inbox::new();

    inbox.push(Fragment::assistant("我来计算"));
    inbox.push(Fragment::tool_call("1", "add", json!({"a": 3, "b": 2})));
    inbox.push(Fragment::tool_result("1", "5", None));

    let mut ctx = Context::new();
    while inbox.peek().is_some() {
        if let Some(frag) = inbox.pop() {
            ctx.append(frag);
        }
    }

    assert_eq!(ctx.len(), 3);
    assert_eq!(ctx.fragments()[0].as_text(), Some("我来计算"));
    assert!(matches!(
        ctx.fragments()[1].content,
        machine::Content::ToolCall(_)
    ));
    assert!(matches!(
        ctx.fragments()[2].content,
        machine::Content::ToolResult(_)
    ));
}

// ── Issue #43 panic → Result / Option ──

#[test]
fn use_model_nonexistent_returns_error() {
    let mut res = common::test_resources();
    let err = res.use_model("nonexistent").unwrap_err();
    assert_eq!(err.to_string(), "model 'nonexistent' not registered");
}

#[test]
fn enable_nonexistent_tool_returns_error() {
    let mut res = common::test_resources();
    let err = res.enable("nonexistent").unwrap_err();
    assert_eq!(err.to_string(), "tool 'nonexistent' not registered");
}

#[tokio::test]
async fn dispatch_model_nonexistent_pushes_hitch() {
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut res = common::test_resources();
    let mut inbox = Inbox::new();
    let mut machine = Machine::new("test", "test");
    let tool_runtime = ToolRuntime::new();

    machine
        .apply(
            Action::Model("nonexistent".to_string()),
            1,
            MachineRuntime {
                ctx: &mut ctx,
                env: &mut env,
                resources: &mut res,
                tool_runtime: &tool_runtime,
                inbox: &mut inbox,
            },
        )
        .await;

    assert_eq!(inbox.len(), 1);
    let frag = inbox.pop().unwrap();
    assert!(matches!(frag.content, machine::Content::Hitch { .. }));
    assert_eq!(frag.role, machine::Role::System);
    if let machine::Content::Hitch { message, .. } = &frag.content {
        assert!(message.contains("nonexistent"));
    }
}

#[tokio::test]
async fn dispatch_activate_nonexistent_pushes_hitch() {
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut res = common::test_resources();
    let mut inbox = Inbox::new();
    let mut machine = Machine::new("test", "test");
    let tool_runtime = ToolRuntime::new();

    machine
        .apply(
            Action::Activate("unknown".to_string()),
            2,
            MachineRuntime {
                ctx: &mut ctx,
                env: &mut env,
                resources: &mut res,
                tool_runtime: &tool_runtime,
                inbox: &mut inbox,
            },
        )
        .await;

    assert_eq!(inbox.len(), 1);
    let frag = inbox.pop().unwrap();
    assert!(matches!(frag.content, machine::Content::Hitch { .. }));
    assert_eq!(frag.role, machine::Role::System);
    if let machine::Content::Hitch { message, .. } = &frag.content {
        assert!(message.contains("unknown"));
    }
}

#[tokio::test]
async fn complete_no_active_model_returns_hitch() {
    let mut ctx = Context::new();
    ctx.append(Fragment::user("hello"));
    let mut res = common::test_resources();
    res.deactivate_model(); // remove active model

    let (fragments, _usage) = machine::completion::complete(&ctx, &res).await;
    assert_eq!(fragments.len(), 1);
    assert_eq!(fragments[0].role, machine::Role::System);
    assert!(matches!(
        fragments[0].content,
        machine::Content::Hitch { .. }
    ));
    if let machine::Content::Hitch { message, .. } = &fragments[0].content {
        assert!(message.contains("no active model"));
    }
}
