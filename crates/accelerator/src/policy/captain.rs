use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU8, Ordering};

use machine::{Action, Context, Environment, Fragment, Inbox, Policy, Resources, Role};
use tracing::{debug, trace, warn};

/// Captain — the default steering policy.
///
/// Boot → Halt → Drain → Done
pub struct Captain {
    state: AtomicU8,
}

impl Clone for Captain {
    fn clone(&self) -> Self {
        Self {
            state: AtomicU8::new(self.state.load(Ordering::Relaxed)),
        }
    }
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
            4 => Self::Done,
            other => {
                warn!(state = other, "captain: unknown state, forcing done");
                Self::Done
            }
        }
    }
}

impl Default for Captain {
    fn default() -> Self {
        Self {
            state: AtomicU8::new(State::Boot as u8),
        }
    }
}

impl Captain {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Policy for Captain {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

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
            trace!(?state, ?next, "captain decide");
            action
        })
    }
}

fn boot(ctx: &Context, resources: &Resources) -> (State, Action) {
    if ctx.fragments().iter().any(|f| f.role == Role::System) {
        if ctx.purpose.is_empty() {
            debug!("boot: system present, skipping");
            return (State::Halt, Action::Halt);
        }
        // System already injected but purpose is new → just inject purpose
        debug!(purpose = ctx.purpose, "boot: injecting purpose");
        return (State::Halt, Action::Append(Fragment::user(&ctx.purpose)));
    }

    let prompt = resources
        .prompts
        .get("default")
        .map(|s| s.to_owned())
        .unwrap_or_default();
    let content = if ctx.purpose.is_empty() {
        debug!("boot: injecting system prompt");
        prompt
    } else {
        debug!(purpose = ctx.purpose, "boot: injecting system + purpose");
        format!("{}\n\nUser intent: {}", prompt, ctx.purpose)
    };
    (State::Halt, Action::Append(Fragment::system(content)))
}

fn halt() -> (State, Action) {
    (State::Drain, Action::Halt)
}

fn drain(inbox: &Inbox) -> (State, Action) {
    if inbox.is_empty() {
        trace!("drain: inbox empty, advancing to done");
        (State::Done, Action::Done)
    } else {
        trace!("drain: taking one fragment");
        (State::Drain, Action::Take)
    }
}

fn done() -> (State, Action) {
    trace!("done: machine stopping");
    (State::Done, Action::Done)
}
