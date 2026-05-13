mod common;

use machine::{Action, Context, Environment, Policy, Resources};

#[tokio::test]
async fn policy_can_add() {
    let policy = common::SeqPolicy::new(vec![
        Action::Add(machine::Fragment::system("hello")),
        Action::Halt,
    ]);
    let ctx = Context::new();
    let env = Environment::new("/tmp");
    let resources = Resources::new();

    let action = policy.decide(&ctx, &env, &resources).await;
    assert!(matches!(action, Action::Add(_)));
}

#[tokio::test]
async fn policy_can_halt() {
    let policy = common::SeqPolicy::new(vec![Action::Halt]);
    let ctx = Context::new();
    let env = Environment::new("/tmp");
    let resources = Resources::new();

    let action = policy.decide(&ctx, &env, &resources).await;
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

    let action = policy.decide(&ctx, &env, &resources).await;
    assert!(matches!(action, Action::Halt));
}

#[tokio::test]
async fn policy_sequence() {
    let policy = common::SeqPolicy::new(vec![
        Action::Add(machine::Fragment::system("a")),
        Action::Add(machine::Fragment::user("b")),
        Action::Halt,
    ]);
    let ctx = Context::new();
    let env = Environment::new("/tmp");
    let resources = Resources::new();

    let a1 = policy.decide(&ctx, &env, &resources).await;
    assert!(matches!(a1, Action::Add(_)));

    let a2 = policy.decide(&ctx, &env, &resources).await;
    assert!(matches!(a2, Action::Add(_)));

    let a3 = policy.decide(&ctx, &env, &resources).await;
    assert!(matches!(a3, Action::Halt));
}
