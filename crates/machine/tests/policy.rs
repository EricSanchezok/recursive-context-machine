mod common;

use machine::{Action, Context, Environment, Inbox, Policy, Resources};

#[tokio::test]
async fn policy_can_append() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(machine::Fragment::system("hello")),
        Action::Halt,
    ]);
    let ctx = Context::new();
    let env = Environment::new("/tmp");
    let resources = Resources::new();
    let inbox = Inbox::new();

    let action = policy.decide(&ctx, &env, &resources, &inbox).await;
    assert!(matches!(action, Action::Append(_)));
}

#[tokio::test]
async fn policy_can_halt() {
    let policy = common::SeqPolicy::new(vec![Action::Halt]);
    let ctx = Context::new();
    let env = Environment::new("/tmp");
    let resources = Resources::new();
    let inbox = Inbox::new();

    let action = policy.decide(&ctx, &env, &resources, &inbox).await;
    assert!(matches!(action, Action::Halt));
}

#[tokio::test]
async fn policy_sees_context_state() {
    let policy = common::SeqPolicy::new(vec![Action::Halt]);
    let mut ctx = Context::new();
    ctx.append(machine::Fragment::system("sys"));
    ctx.append(machine::Fragment::user("hello"));
    let env = Environment::new("/tmp");
    let resources = Resources::new();
    let inbox = Inbox::new();

    let action = policy.decide(&ctx, &env, &resources, &inbox).await;
    assert!(matches!(action, Action::Halt));
}

#[tokio::test]
async fn policy_sequence() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(machine::Fragment::system("a")),
        Action::Append(machine::Fragment::user("b")),
        Action::Halt,
    ]);
    let ctx = Context::new();
    let env = Environment::new("/tmp");
    let resources = Resources::new();
    let inbox = Inbox::new();

    let a1 = policy.decide(&ctx, &env, &resources, &inbox).await;
    assert!(matches!(a1, Action::Append(_)));

    let a2 = policy.decide(&ctx, &env, &resources, &inbox).await;
    assert!(matches!(a2, Action::Append(_)));

    let a3 = policy.decide(&ctx, &env, &resources, &inbox).await;
    assert!(matches!(a3, Action::Halt));
}

#[tokio::test]
async fn policy_can_take_from_inbox() {
    let policy = common::SeqPolicy::new(vec![Action::Take, Action::Done]);
    let ctx = Context::new();
    let env = Environment::new("/tmp");
    let resources = Resources::new();
    let mut inbox = Inbox::new();
    inbox.push(machine::Fragment::assistant("hi"));

    let a1 = policy.decide(&ctx, &env, &resources, &inbox).await;
    assert!(matches!(a1, Action::Take));

    let a2 = policy.decide(&ctx, &env, &resources, &inbox).await;
    assert!(matches!(a2, Action::Done));
}

#[tokio::test]
async fn policy_exhaustion_returns_done() {
    let policy = common::SeqPolicy::new(vec![]);
    let ctx = Context::new();
    let env = Environment::new("/tmp");
    let resources = Resources::new();
    let inbox = Inbox::new();

    let action = policy.decide(&ctx, &env, &resources, &inbox).await;
    assert!(matches!(action, Action::Done));
}
