mod common;

use machine::{Action, Content, Context, Environment, Fragment, Inbox, Machine, Purpose};
use serde_json::json;

#[tokio::test]
async fn done_stops_immediately() {
    let policy = common::SeqPolicy::new(vec![Action::Done]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine
        .run(&Purpose::default(), &mut ctx, &mut env, &mut resources)
        .await;
    assert!(ctx.is_empty());
}

#[tokio::test]
async fn append_and_take_flow() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("sys")),
        Action::Append(Fragment::user("hello")),
        Action::Done,
    ]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine
        .run(&Purpose::default(), &mut ctx, &mut env, &mut resources)
        .await;
    assert_eq!(ctx.len(), 2);
    assert_eq!(ctx.fragments()[0].as_text(), Some("sys"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("hello"));
}

#[tokio::test]
async fn take_empty_inbox_is_noop() {
    let policy = common::SeqPolicy::new(vec![Action::Take, Action::Done]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine
        .run(&Purpose::default(), &mut ctx, &mut env, &mut resources)
        .await;
    assert!(ctx.is_empty());
}

#[tokio::test]
async fn swap_preserves_count() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("first")),
        Action::Append(Fragment::system("second")),
        Action::Swap(1, 2),
        Action::Done,
    ]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine
        .run(&Purpose::default(), &mut ctx, &mut env, &mut resources)
        .await;
    assert_eq!(ctx.len(), 2);
    assert_eq!(ctx.fragments()[0].as_text(), Some("second"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("first"));
}

#[tokio::test]
async fn replace_preserves_id() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("old")),
        Action::Replace {
            id: 1,
            fragment: Fragment::system("new"),
        },
        Action::Done,
    ]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine
        .run(&Purpose::default(), &mut ctx, &mut env, &mut resources)
        .await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("new"));
    assert_eq!(ctx.fragments()[0].id(), 1);
}

#[tokio::test]
async fn insert_after_id() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("first")),
        Action::Append(Fragment::system("third")),
        Action::Insert {
            after: 1,
            fragment: Fragment::system("second"),
        },
        Action::Done,
    ]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine
        .run(&Purpose::default(), &mut ctx, &mut env, &mut resources)
        .await;
    assert_eq!(ctx.len(), 3);
    assert_eq!(ctx.fragments()[0].as_text(), Some("first"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("second"));
    assert_eq!(ctx.fragments()[2].as_text(), Some("third"));
}

#[tokio::test]
async fn remove_and_check_context() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("a")),
        Action::Append(Fragment::user("b")),
        Action::Remove(1),
        Action::Done,
    ]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine
        .run(&Purpose::default(), &mut ctx, &mut env, &mut resources)
        .await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("b"));
}

#[tokio::test]
#[should_panic(expected = "not found")]
async fn remove_unknown_panics() {
    let policy = common::SeqPolicy::new(vec![Action::Remove(999), Action::Done]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine
        .run(&Purpose::default(), &mut ctx, &mut env, &mut resources)
        .await;
}

// ── Mixed-content response flow ──

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
        Content::ToolCall(_)
    ));
}

#[test]
fn inbox_drain_order_with_mixed_response() {
    let mut inbox = Inbox::new();

    // Simulate reactor output for a mixed LLM response:
    // assistant text → tool call → tool result
    inbox.push(Fragment::assistant("我来计算"));
    inbox.push(Fragment::tool_call("1", "add", json!({"a": 3, "b": 2})));
    inbox.push(Fragment::tool_result("1", "5"));

    // Simulate Drain (Take) behavior
    let mut ctx = Context::new();
    while inbox.peek().is_some() {
        if let Some(frag) = inbox.pop() {
            ctx.append(frag);
        }
    }

    assert_eq!(ctx.len(), 3);
    assert_eq!(ctx.fragments()[0].as_text(), Some("我来计算"));
    assert!(matches!(ctx.fragments()[1].content, Content::ToolCall(_)));
    assert!(matches!(ctx.fragments()[2].content, Content::ToolResult(_)));
}
