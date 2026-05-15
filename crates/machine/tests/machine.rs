mod common;

use machine::{Action, Context, Environment, Fragment, Machine, Resources};

#[tokio::test]
async fn done_immediately() {
    let policy = common::SeqPolicy::new(vec![Action::Done]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert!(ctx.is_empty());
}

#[tokio::test]
async fn append_then_done() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("you are helpful")),
        Action::Done,
    ]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("you are helpful"));
}

#[tokio::test]
async fn halt_and_take() {
    // Halt calls the real reactor, which calls completion.
    // With no real API key, it produces an error fragment.
    // We test that the Take → inbox → context flow works.
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("sys")),
        Action::Append(Fragment::user("hello")),
        Action::Model("test".into()),
        Action::Halt,
        Action::Take,
        Action::Done,
    ]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    // sys + user + one fragment from reactor (likely an error)
    assert_eq!(ctx.len(), 3);
    assert_eq!(ctx.fragments()[0].as_text(), Some("sys"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("hello"));
    // The third fragment is whatever the reactor produced
    assert_eq!(ctx.fragments()[2].tag, "hitch");
}

#[tokio::test]
async fn take_empty_inbox_is_noop() {
    // Take when inbox is empty should not panic
    let policy = common::SeqPolicy::new(vec![Action::Take, Action::Done]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert!(ctx.is_empty());
}

#[tokio::test]
async fn policy_can_remove() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("sys")),
        Action::Append(Fragment::user("env").with_tag("env")),
        Action::Remove(2),
        Action::Done,
    ]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("sys"));
}

#[tokio::test]
async fn policy_can_swap() {
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

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert_eq!(ctx.len(), 2);
    assert_eq!(ctx.fragments()[0].as_text(), Some("second"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("first"));
}

#[tokio::test]
async fn policy_can_insert() {
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

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert_eq!(ctx.len(), 3);
    assert_eq!(ctx.fragments()[0].as_text(), Some("first"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("second"));
    assert_eq!(ctx.fragments()[2].as_text(), Some("third"));
}

#[tokio::test]
async fn policy_can_replace() {
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

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("new"));
    assert_eq!(ctx.fragments()[0].id, 1);
}

#[tokio::test]
async fn activate_and_deactivate_tools() {
    let policy = common::SeqPolicy::new(vec![
        Action::Activate("tool-a".into()),
        Action::Activate("tool-b".into()),
        Action::Deactivate("tool-a".into()),
        Action::Done,
    ]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources_with_tools();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    let active: Vec<&str> = resources.active_tools().iter().map(|t| t.name()).collect();
    assert_eq!(active, vec!["tool-b"]);
}

#[tokio::test]
async fn model_sets_active() {
    let policy = common::SeqPolicy::new(vec![
        Action::Model("test".into()),
        Action::Halt,
        Action::Done,
    ]);
    let machine = Machine::new(Box::new(policy));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    // The model was set; reactor pushed a hitch to inbox, but Take was never called.
    assert_eq!(ctx.len(), 0);
}
