use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

use machine::{Action, Context, Environment, Fragment, Inbox, Machine, Policy, Reactor};

/// A policy that replays a fixed sequence of actions.
struct SeqPolicy {
    actions: Vec<Action>,
    pos: Mutex<usize>,
}

impl SeqPolicy {
    fn new(actions: Vec<Action>) -> Self {
        Self {
            actions,
            pos: Mutex::new(0),
        }
    }
}

impl Policy for SeqPolicy {
    fn decide<'a>(
        &'a self,
        _ctx: &'a Context,
        _env: &'a Environment,
        _inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        let mut pos = self.pos.lock().unwrap();
        let action = self.actions[*pos].clone();
        *pos += 1;
        Box::pin(async move { action })
    }
}

/// A reactor that returns a fixed sequence of inboxes.
struct SeqReactor {
    responses: Vec<Inbox>,
    pos: Mutex<usize>,
}

impl SeqReactor {
    fn new(responses: Vec<Inbox>) -> Self {
        Self {
            responses,
            pos: Mutex::new(0),
        }
    }
}

impl Reactor for SeqReactor {
    fn react<'a>(
        &'a self,
        _ctx: &'a Context,
        _env: &'a mut Environment,
    ) -> Pin<Box<dyn Future<Output = Inbox> + Send + 'a>> {
        let mut pos = self.pos.lock().unwrap();
        if *pos >= self.responses.len() {
            return Box::pin(async move { Inbox::new() });
        }
        let response = self.responses[*pos].clone();
        *pos += 1;
        Box::pin(async move { response })
    }
}

fn inbox_of(fragments: Vec<Fragment>) -> Inbox {
    let mut inbox = Inbox::new();
    for f in fragments {
        inbox.push(f);
    }
    inbox
}

#[tokio::test]
async fn halt_immediately_empty_inbox() {
    let policy = SeqPolicy::new(vec![Action::Halt]);
    let reactor = SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));

    let (ctx, _env) = machine
        .run(Context::new(), Environment::new("/tmp"), Inbox::new())
        .await;
    assert!(ctx.is_empty());
}

#[tokio::test]
async fn take_from_inbox_then_halt() {
    let policy = SeqPolicy::new(vec![Action::Take, Action::Halt]);
    let reactor = SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));

    let mut inbox = Inbox::new();
    inbox.push(Fragment::system("you are helpful"));

    let (ctx, _env) = machine
        .run(Context::new(), Environment::new("/tmp"), inbox)
        .await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("you are helpful"));
}

#[tokio::test]
async fn build_context_then_react() {
    // Policy consumes system + user, halts. Reactor returns assistant.
    // Policy consumes assistant, halts. Reactor returns empty → stop.
    let policy = SeqPolicy::new(vec![
        Action::Take, // system
        Action::Take, // user
        Action::Halt, // → reactor
        Action::Take, // assistant
        Action::Halt, // → reactor empty → stop
    ]);
    let reactor = SeqReactor::new(vec![inbox_of(vec![Fragment::assistant("hi there")])]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));

    let mut inbox = Inbox::new();
    inbox.push(Fragment::system("sys"));
    inbox.push(Fragment::user("hello"));

    let (ctx, _env) = machine
        .run(Context::new(), Environment::new("/tmp"), inbox)
        .await;
    assert_eq!(ctx.len(), 3);
    assert_eq!(ctx.fragments()[0].as_text(), Some("sys"));
    assert_eq!(ctx.fragments()[1].as_text(), Some("hello"));
    assert_eq!(ctx.fragments()[2].as_text(), Some("hi there"));
}

#[tokio::test]
async fn multi_turn_conversation() {
    // All user messages are in the initial inbox. Policy consumes them all,
    // then halts. Reactor returns all assistant responses at once.
    let policy = SeqPolicy::new(vec![
        Action::Take, // system
        Action::Take, // q1
        Action::Take, // q2
        Action::Halt, // → reactor
        Action::Take, // a1
        Action::Take, // a2
        Action::Halt, // → reactor empty → stop
    ]);
    let reactor = SeqReactor::new(vec![inbox_of(vec![
        Fragment::assistant("a1"),
        Fragment::assistant("a2"),
    ])]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));

    let mut inbox = Inbox::new();
    inbox.push(Fragment::system("sys"));
    inbox.push(Fragment::user("q1"));
    inbox.push(Fragment::user("q2"));

    let (ctx, _env) = machine
        .run(Context::new(), Environment::new("/tmp"), inbox)
        .await;
    assert_eq!(ctx.len(), 5);
    let texts: Vec<&str> = ctx
        .fragments()
        .iter()
        .filter_map(|f: &Fragment| f.as_text())
        .collect();
    assert_eq!(texts, vec!["sys", "q1", "q2", "a1", "a2"]);
}

#[tokio::test]
async fn reactor_returns_multiple_fragments() {
    let policy = SeqPolicy::new(vec![
        Action::Take, // system
        Action::Take, // user
        Action::Halt, // → reactor
        Action::Take, // assistant
        Action::Take, // tool_result
        Action::Halt, // → reactor empty → stop
    ]);
    let reactor = SeqReactor::new(vec![inbox_of(vec![
        Fragment::assistant("calling tool"),
        Fragment::tool_result("call_1", "result data"),
    ])]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));

    let mut inbox = Inbox::new();
    inbox.push(Fragment::system("sys"));
    inbox.push(Fragment::user("run tool"));

    let (ctx, _env) = machine
        .run(Context::new(), Environment::new("/tmp"), inbox)
        .await;
    assert_eq!(ctx.len(), 4);
    assert_eq!(ctx.fragments()[2].as_text(), Some("calling tool"));
    assert_eq!(ctx.fragments()[3].tag, "tool_result");
}

#[tokio::test]
async fn policy_can_drop() {
    let policy = SeqPolicy::new(vec![
        Action::Take,           // append system → id=1
        Action::Take,           // append env → id=2
        Action::Drop { id: 2 }, // remove env
        Action::Halt,
    ]);
    let reactor = SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));

    let mut inbox = Inbox::new();
    inbox.push(Fragment::system("sys"));
    inbox.push(Fragment::user("env").with_tag("env"));

    let (ctx, _env) = machine
        .run(Context::new(), Environment::new("/tmp"), inbox)
        .await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("sys"));
}

#[tokio::test]
async fn policy_can_swap() {
    let policy = SeqPolicy::new(vec![
        Action::Take,           // append system → id=1
        Action::Swap { id: 1 }, // replace system with new content
        Action::Halt,
    ]);
    let reactor = SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));

    let mut inbox = Inbox::new();
    inbox.push(Fragment::system("old"));
    inbox.push(Fragment::system("new"));

    let (ctx, _env) = machine
        .run(Context::new(), Environment::new("/tmp"), inbox)
        .await;
    assert_eq!(ctx.len(), 1);
    assert_eq!(ctx.fragments()[0].as_text(), Some("new"));
    assert_eq!(ctx.fragments()[0].id, 1);
}

#[tokio::test]
async fn policy_can_set_environment() {
    let policy = SeqPolicy::new(vec![
        Action::Set {
            key: "config.model".into(),
            value: serde_json::json!("gpt-4o"),
        },
        Action::Halt,
    ]);
    let reactor = SeqReactor::new(vec![]);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));

    let (_ctx, env) = machine
        .run(Context::new(), Environment::new("/tmp"), Inbox::new())
        .await;
    assert_eq!(env.config.model, "gpt-4o");
}

#[tokio::test]
async fn many_cycles() {
    // 5 user messages → reactor returns 5 assistant messages
    let mut actions = Vec::new();
    for _ in 0..5 {
        actions.push(Action::Take); // user
    }
    actions.push(Action::Halt); // → reactor
    for _ in 0..5 {
        actions.push(Action::Take); // assistant
    }
    actions.push(Action::Halt); // → reactor empty → stop

    let mut responses = Vec::new();
    let mut assistant_frags = Vec::new();
    for i in 0..5 {
        assistant_frags.push(Fragment::assistant(format!("a{}", i)));
    }
    responses.push(inbox_of(assistant_frags));

    let policy = SeqPolicy::new(actions);
    let reactor = SeqReactor::new(responses);
    let machine = Machine::new(Box::new(policy), Box::new(reactor));

    let mut inbox = Inbox::new();
    for i in 0..5 {
        inbox.push(Fragment::user(format!("q{}", i)));
    }

    let (ctx, _env) = machine
        .run(Context::new(), Environment::new("/tmp"), inbox)
        .await;
    assert_eq!(ctx.len(), 10);
}
