mod common;

use machine::{Action, Context, Environment, Fragment, Machine};

#[tokio::test]
async fn halt_immediately_empty_inbox() {
    let policy = common::SeqPolicy::new(vec![Action::Halt]);
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let resources = common::test_resources();

    let ctx = machine
        .run(Context::new(), Environment::new("/tmp"), &resources)
        .await;
    assert!(ctx.is_empty());
}

#[tokio::test]
async fn add_then_halt() {
    let policy = common::SeqPolicy::new(vec![
        Action::Add(Fragment::system("you are helpful")),
        Action::Halt,
    ]);
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let resources = common::test_resources();

    let ctx = machine
        .run(Context::new(), Environment::new("/tmp"), &resources)
        .await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("you are helpful"));
}

#[tokio::test]
async fn build_context_then_react() {
    // Policy adds system + user, halts. Reactor returns assistant.
    // Policy halts again. Reactor returns empty → stop.
    let policy = common::SeqPolicy::new(vec![
        Action::Add(Fragment::system("sys")),
        Action::Add(Fragment::user("hello")),
        Action::Halt,
        Action::Halt,
    ]);
    let reactor = common::SeqReactor::new(vec![
        vec![Fragment::assistant("hi there")].into_iter().collect(),
    ]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let resources = common::test_resources();

    let ctx = machine
        .run(Context::new(), Environment::new("/tmp"), &resources)
        .await;
    assert_eq!(ctx.len(), 3);
    assert_eq!(ctx.fragments()[0].as_text(), Some("sys"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("hello"));
    assert_eq!(ctx.fragments()[2].as_text(), Some("hi there"));
}

#[tokio::test]
async fn multi_turn_conversation() {
    // Policy adds all messages, halts. Reactor returns all assistant responses at once.
    let policy = common::SeqPolicy::new(vec![
        Action::Add(Fragment::system("sys")),
        Action::Add(Fragment::user("q1")),
        Action::Add(Fragment::user("q2")),
        Action::Halt,
        Action::Halt,
    ]);
    let reactor = common::SeqReactor::new(vec![
        vec![Fragment::assistant("a1"), Fragment::assistant("a2")]
            .into_iter()
            .collect(),
    ]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let resources = common::test_resources();

    let ctx = machine
        .run(Context::new(), Environment::new("/tmp"), &resources)
        .await;
    assert_eq!(ctx.len(), 5);
    let texts: Vec<&str> = ctx.fragments().iter().filter_map(|f| f.as_text()).collect();
    assert_eq!(texts, vec!["sys", "q1", "q2", "a1", "a2"]);
}

#[tokio::test]
async fn reactor_returns_multiple_fragments() {
    let policy = common::SeqPolicy::new(vec![
        Action::Add(Fragment::system("sys")),
        Action::Add(Fragment::user("run tool")),
        Action::Halt,
        Action::Halt,
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
    let resources = common::test_resources();

    let ctx = machine
        .run(Context::new(), Environment::new("/tmp"), &resources)
        .await;
    assert_eq!(ctx.len(), 4);
    assert_eq!(ctx.fragments()[2].as_text(), Some("calling tool"));
    assert_eq!(ctx.fragments()[3].tag, "tool_result");
}

#[tokio::test]
async fn policy_can_remove() {
    let policy = common::SeqPolicy::new(vec![
        Action::Add(Fragment::system("sys")),
        Action::Add(Fragment::user("env").with_tag("env")),
        Action::Remove(2),
        Action::Halt,
    ]);
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let resources = common::test_resources();

    let ctx = machine
        .run(Context::new(), Environment::new("/tmp"), &resources)
        .await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("sys"));
}

#[tokio::test]
async fn policy_can_swap() {
    let policy = common::SeqPolicy::new(vec![
        Action::Add(Fragment::system("first")),
        Action::Add(Fragment::system("second")),
        Action::Swap(1, 2),
        Action::Halt,
    ]);
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let resources = common::test_resources();

    let ctx = machine
        .run(Context::new(), Environment::new("/tmp"), &resources)
        .await;
    assert_eq!(ctx.len(), 2);
    assert_eq!(ctx.fragments()[0].as_text(), Some("second"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("first"));
}

#[tokio::test]
async fn policy_can_set_model() {
    let policy = common::SeqPolicy::new(vec![Action::SetModel("test-model".into()), Action::Halt]);
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let resources = common::test_resources();

    let ctx = machine
        .run(Context::new(), Environment::new("/tmp"), &resources)
        .await;
    assert_eq!(ctx.model(), Some("test-model"));
}

#[tokio::test]
async fn policy_can_manage_tools() {
    let policy = common::SeqPolicy::new(vec![
        Action::AddTool("tool-a".into()),
        Action::AddTool("tool-b".into()),
        Action::RemoveTool("tool-a".into()),
        Action::Halt,
    ]);
    let reactor = common::SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let resources = common::test_resources();

    let ctx = machine
        .run(Context::new(), Environment::new("/tmp"), &resources)
        .await;
    assert_eq!(ctx.tools(), &["tool-b"]);
}

#[tokio::test]
async fn many_cycles() {
    // Round 1: Policy adds 5 user fragments, halts.
    // Reactor returns 5 assistant fragments. Machine appends them.
    // Round 2: Policy halts again. Reactor returns empty -> stop.
    let mut actions = Vec::new();
    for i in 0..5 {
        actions.push(Action::Add(Fragment::user(format!("q{}", i))));
    }
    actions.push(Action::Halt);
    actions.push(Action::Halt);

    let assistant_frags: Vec<_> = (0..5)
        .map(|i| Fragment::assistant(format!("a{}", i)))
        .collect();

    let policy = common::SeqPolicy::new(actions);
    let reactor = common::SeqReactor::new(vec![assistant_frags.into_iter().collect()]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));
    let resources = common::test_resources();

    let ctx = machine
        .run(Context::new(), Environment::new("/tmp"), &resources)
        .await;
    assert_eq!(ctx.len(), 10);
}
