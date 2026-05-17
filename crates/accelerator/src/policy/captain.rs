use std::future::Future;
use std::pin::Pin;

use machine::{Action, Context, Environment, Inbox, Phase, Policy, Purpose, Resources, Role};
use tracing::trace;

use super::phases::{BootstrapAgent, InjectEnv, InjectPurpose};

/// Captain — a simple single-agent Policy.
///
/// Decides purely based on inbox state and the last fragment in context:
///
///   Inbox not empty  → Take (drain one fragment into context)
///   Inbox empty:
///     last in context is Tool  → Halt (tool was just run, show LLM the result)
///     last in context is not Tool, first call already happened → Done
///     first call ever → Halt (kick off the LLM)
pub struct Captain {
    started: std::sync::atomic::AtomicBool,
}

impl Clone for Captain {
    fn clone(&self) -> Self {
        Self {
            started: std::sync::atomic::AtomicBool::new(
                self.started.load(std::sync::atomic::Ordering::Relaxed),
            ),
        }
    }
}

impl Default for Captain {
    fn default() -> Self {
        Self {
            started: std::sync::atomic::AtomicBool::new(false),
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
            Box::new(InjectPurpose),
        ]
    }

    fn pre_halt(&self) -> Vec<Box<dyn Phase>> {
        vec![Box::new(InjectEnv)]
    }

    fn decide<'a>(
        &'a self,
        _purpose: &'a Purpose,
        ctx: &'a Context,
        _env: &'a Environment,
        _resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async move {
            if inbox.peek().is_some() {
                return Action::Take;
            }

            // Inbox is empty.
            let not_started = !self
                .started
                .swap(true, std::sync::atomic::Ordering::Relaxed);
            if not_started {
                trace!("decide: first call, halting");
                return Action::Halt;
            }

            match ctx.fragments().last().map(|f| f.role) {
                Some(Role::Tool) => {
                    trace!("decide: last is Tool, halting");
                    Action::Halt
                }
                _ => {
                    trace!("decide: done");
                    Action::Done
                }
            }
        })
    }
}
