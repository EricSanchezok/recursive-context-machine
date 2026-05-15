use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU8, Ordering};

use machine::{Action, Context, Environment, Fragment, Inbox, Policy, Resources, Role};
use tracing::debug;

/// Captain — the default steering policy.
///
/// Boot → Halt → Drain → Done
pub struct Captain {
    state: AtomicU8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum State {
    Boot = 1,
    Halt = 2,
    Drain = 3,
    Done = 4,
}

impl State {
    fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Boot,
            2 => Self::Halt,
            3 => Self::Drain,
            _ => Self::Done,
        }
    }
}

impl Default for Captain {
    fn default() -> Self {
        Self::new()
    }
}

impl Captain {
    pub fn new() -> Self {
        Self {
            state: AtomicU8::new(State::Boot as u8),
        }
    }
}

impl Policy for Captain {
    fn decide<'a>(
        &'a self,
        ctx: &'a Context,
        _env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async move {
            let state = State::from_u8(self.state.load(Ordering::Relaxed));
            let (next, action) = match state {
                State::Boot => boot(ctx, resources),
                State::Halt => halt(),
                State::Drain => drain(inbox),
                State::Done => done(),
            };
            self.state.store(next as u8, Ordering::Relaxed);
            action
        })
    }
}

fn boot(ctx: &Context, resources: &Resources) -> (State, Action) {
    if ctx.fragments().iter().any(|f| f.role == Role::System) {
        debug!("boot: system prompt present, skipping");
        return (State::Halt, Action::Halt);
    }
    let prompt = resources
        .prompts
        .get("default")
        .cloned()
        .unwrap_or_default();
    debug!("boot: injecting system prompt");
    (State::Halt, Action::Append(Fragment::system(prompt)))
}

fn halt() -> (State, Action) {
    (State::Drain, Action::Halt)
}

fn drain(inbox: &Inbox) -> (State, Action) {
    if inbox.is_empty() {
        (State::Done, Action::Done)
    } else {
        (State::Drain, Action::Take)
    }
}

fn done() -> (State, Action) {
    (State::Done, Action::Done)
}
