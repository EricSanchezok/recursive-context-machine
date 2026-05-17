use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use machine::{Action, Context, Environment, Inbox, Phase, Policy, Purpose, Resources, Role};
use tracing::{trace, warn};

use super::phases::{BootstrapAgent, InjectEnv, InjectPurpose};

pub struct Captain {
    state: AtomicU8,
    tool_seen: AtomicBool,
}

impl Clone for Captain {
    fn clone(&self) -> Self {
        Self {
            state: AtomicU8::new(self.state.load(Ordering::Relaxed)),
            tool_seen: AtomicBool::new(self.tool_seen.load(Ordering::Relaxed)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum State {
    Halt = 1,
    Drain = 2,
}

impl State {
    fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Halt,
            2 => Self::Drain,
            other => {
                warn!(state = other, "captain: unknown state, forcing drain");
                Self::Drain
            }
        }
    }
}

impl Default for Captain {
    fn default() -> Self {
        Self {
            state: AtomicU8::new(State::Halt as u8),
            tool_seen: AtomicBool::new(false),
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

    fn pre(&self) -> Vec<Box<dyn Phase>> {
        vec![
            Box::new(BootstrapAgent::new("captain")),
        ]
    }

    fn pre_halt(&self) -> Vec<Box<dyn Phase>> {
        vec![
            Box::new(InjectEnv),
            Box::new(InjectPurpose),
        ]
    }

    fn decide<'a>(
        &'a self,
        _purpose: &'a Purpose,
        _ctx: &'a Context,
        _env: &'a Environment,
        _resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async move {
            let state = State::from_u8(self.state.load(Ordering::Relaxed));

            let (next, action) = match state {
                State::Halt => {
                    self.tool_seen.store(false, Ordering::Relaxed);
                    (State::Drain, Action::Halt)
                }
                State::Drain => {
                    if let Some(frag) = inbox.peek() {
                        if frag.role == Role::Tool {
                            self.tool_seen.store(true, Ordering::Relaxed);
                        }
                        (State::Drain, Action::Take)
                    } else if self.tool_seen.load(Ordering::Relaxed) {
                        trace!("drain: tool results seen, halting again");
                        (State::Halt, Action::Halt)
                    } else {
                        trace!("drain: final answer ready");
                        (State::Drain, Action::Done)
                    }
                }
            };

            self.state.store(next as u8, Ordering::Relaxed);
            action
        })
    }
}
