use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

use machine::{Action, Context, Environment, Inbox, Policy};

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

#[tokio::test]
async fn policy_can_take() {
    let policy = SeqPolicy::new(vec![Action::Take, Action::Halt]);
    let ctx = Context::new();
    let env = Environment::new("/tmp");
    let inbox = Inbox::new();

    let action = policy.decide(&ctx, &env, &inbox).await;
    assert!(matches!(action, Action::Take));
}

#[tokio::test]
async fn policy_can_halt() {
    let policy = SeqPolicy::new(vec![Action::Halt]);
    let ctx = Context::new();
    let env = Environment::new("/tmp");
    let inbox = Inbox::new();

    let action = policy.decide(&ctx, &env, &inbox).await;
    assert!(matches!(action, Action::Halt));
}

#[tokio::test]
async fn policy_sees_inbox_state() {
    let policy = SeqPolicy::new(vec![Action::Halt]);
    let ctx = Context::new();
    let env = Environment::new("/tmp");
    let mut inbox = Inbox::new();
    inbox.push(machine::Fragment::system("sys"));

    // Policy receives the inbox — it can observe it
    let action = policy.decide(&ctx, &env, &inbox).await;
    assert!(matches!(action, Action::Halt));
}

#[tokio::test]
async fn policy_sees_context_state() {
    let policy = SeqPolicy::new(vec![Action::Halt]);
    let mut ctx = Context::new();
    ctx.append(machine::Fragment::system("sys"));
    ctx.append(machine::Fragment::user("hello"));
    let env = Environment::new("/tmp");
    let inbox = Inbox::new();

    let action = policy.decide(&ctx, &env, &inbox).await;
    assert!(matches!(action, Action::Halt));
}

#[tokio::test]
async fn policy_sequence() {
    let policy = SeqPolicy::new(vec![Action::Take, Action::Take, Action::Halt]);
    let ctx = Context::new();
    let env = Environment::new("/tmp");
    let inbox = Inbox::new();

    let a1 = policy.decide(&ctx, &env, &inbox).await;
    assert!(matches!(a1, Action::Take));

    let a2 = policy.decide(&ctx, &env, &inbox).await;
    assert!(matches!(a2, Action::Take));

    let a3 = policy.decide(&ctx, &env, &inbox).await;
    assert!(matches!(a3, Action::Halt));
}
