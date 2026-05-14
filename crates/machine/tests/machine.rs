mod common;

use machine::{Action, Context, Environment, Fragment, Inbox, Machine, Resources};

#[tokio::test]
async fn done_immediately() {
    let policy = common::SeqPolicy::new(vec![Action::Done]);
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
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
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("you are helpful"));
}

#[tokio::test]
async fn build_context_then_react() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("sys")),
        Action::Append(Fragment::user("hello")),
        Action::Halt,
        Action::Take,
        Action::Done,
    ]);
    let reactor = common::SeqReactor::new(vec![
        vec![Fragment::assistant("hi there")].into_iter().collect(),
    ]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert_eq!(ctx.len(), 3);
    assert_eq!(ctx.fragments()[0].as_text(), Some("sys"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("hello"));
    assert_eq!(ctx.fragments()[2].as_text(), Some("hi there"));
}

#[tokio::test]
async fn multi_turn_conversation() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("sys")),
        Action::Append(Fragment::user("q1")),
        Action::Append(Fragment::user("q2")),
        Action::Halt,
        Action::Take,
        Action::Take,
        Action::Halt,
        Action::Done,
    ]);
    let reactor = common::SeqReactor::new(vec![
        vec![Fragment::assistant("a1"), Fragment::assistant("a2")]
            .into_iter()
            .collect(),
        Inbox::new(),
    ]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert_eq!(ctx.len(), 5);
    let texts: Vec<&str> = ctx.fragments().iter().filter_map(|f| f.as_text()).collect();
    assert_eq!(texts, vec!["sys", "q1", "q2", "a1", "a2"]);
}

#[tokio::test]
async fn reactor_returns_multiple_fragments() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("sys")),
        Action::Append(Fragment::user("run tool")),
        Action::Halt,
        Action::Take,
        Action::Take,
        Action::Done,
    ]);
    let reactor = common::SeqReactor::new(vec![
        vec![
            Fragment::assistant("calling tool"),
            Fragment::tool_result("call_1", "result data"),
        ]
        .into_iter()
        .collect(),
    ]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert_eq!(ctx.len(), 4);
    assert_eq!(ctx.fragments()[2].as_text(), Some("calling tool"));
    assert_eq!(ctx.fragments()[3].tag, "tool_result");
}

#[tokio::test]
async fn policy_can_remove() {
    let policy = common::SeqPolicy::new(vec![
        Action::Append(Fragment::system("sys")),
        Action::Append(Fragment::user("env").with_tag("env")),
        Action::Remove(2),
        Action::Done,
    ]);
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
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
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
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
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
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
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("new"));
    assert_eq!(ctx.fragments()[0].id, 1);
}

#[tokio::test]
async fn model_passes_to_reactor() {
    use machine::{Model, Reactor, Tool};
    use std::sync::{Arc, Mutex};

    struct CheckReactor {
        received_model: Arc<Mutex<Option<String>>>,
    }

    impl Reactor for CheckReactor {
        fn react<'a>(
            &'a self,
            _ctx: &'a Context,
            _env: &'a Environment,
            resources: &'a Resources,
            _inbox: &'a mut Inbox,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
            *self.received_model.lock().unwrap() = resources.active_model().map(|m| m.name.clone());
            Box::pin(async move {})
        }
    }

    let received = Arc::new(Mutex::new(None));
    let reactor = CheckReactor {
        received_model: received.clone(),
    };
    let policy = common::SeqPolicy::new(vec![
        Action::Model("test".into()),
        Action::Halt,
        Action::Done,
    ]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert_eq!(received.lock().unwrap().as_deref(), Some("test"));
}

#[tokio::test]
async fn catch_and_drop_tools() {
    let policy = common::SeqPolicy::new(vec![
        Action::Catch("tool-a".into()),
        Action::Catch("tool-b".into()),
        Action::Drop("tool-a".into()),
        Action::Done,
    ]);
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources_with_tools();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    let active: Vec<&str> = resources.active_tools().iter().map(|t| t.name()).collect();
    assert_eq!(active, vec!["tool-b"]);
}

#[tokio::test]
async fn many_cycles() {
    let mut actions = Vec::new();
    for i in 0..5 {
        actions.push(Action::Append(Fragment::user(format!("q{}", i))));
    }
    actions.push(Action::Halt);
    for _ in 0..5 {
        actions.push(Action::Take);
    }
    actions.push(Action::Halt);
    actions.push(Action::Done);

    let assistant_frags: Vec<_> = (0..5)
        .map(|i| Fragment::assistant(format!("a{}", i)))
        .collect();

    let policy = common::SeqPolicy::new(actions);
    let reactor =
        common::SeqReactor::new(vec![assistant_frags.into_iter().collect(), Inbox::new()]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let mut ctx = Context::new();
    let mut env = Environment::new("/tmp");
    let mut resources = common::test_resources();

    machine.run(&mut ctx, &mut env, &mut resources).await;
    assert_eq!(ctx.len(), 10);
}
