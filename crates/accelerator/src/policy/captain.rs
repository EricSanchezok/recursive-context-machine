use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU8, Ordering};

use machine::{Action, Content, Context, Environment, Inbox, Policy, Purpose, Resources, Role};
use tracing::{trace, warn};

use super::retry::{HTTP_FORBIDDEN, HTTP_UNAUTHORIZED, Retry};
use super::{Step, moves};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Agent = 0,
    Instruction = 1,
    Environment = 2,
    Purpose = 3,
    Resources = 4,
    Respond = 5,
    Running = 6,
}

impl Phase {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Agent,
            1 => Self::Instruction,
            2 => Self::Environment,
            3 => Self::Purpose,
            4 => Self::Resources,
            5 => Self::Respond,
            _ => Self::Running,
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Agent => Self::Instruction,
            Self::Instruction => Self::Environment,
            Self::Environment => Self::Purpose,
            Self::Purpose => Self::Resources,
            Self::Resources => Self::Respond,
            Self::Respond => Self::Running,
            Self::Running => Self::Running,
        }
    }
}

pub struct Captain {
    phase: AtomicU8,
    retry: Retry,
}

impl Clone for Captain {
    fn clone(&self) -> Self {
        Self {
            phase: AtomicU8::new(Phase::Agent as u8),
            retry: self.retry.clone(),
        }
    }
}

impl Default for Captain {
    fn default() -> Self {
        Self {
            phase: AtomicU8::new(Phase::Agent as u8),
            retry: Retry::default(),
        }
    }
}

impl Captain {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Captain {
    fn phase(&self) -> Phase {
        Phase::from_u8(self.phase.load(Ordering::Relaxed))
    }

    fn enter(&self, phase: Phase) {
        self.phase.store(phase as u8, Ordering::Relaxed);
    }

    fn advance(&self) {
        self.enter(self.phase().next());
    }

    fn drive(&self, step: Step) -> Option<Action> {
        match step {
            Step::Emit(action) => Some(action),
            Step::Ready => {
                self.advance();
                None
            }
        }
    }

    fn setup(
        &self,
        ctx: &Context,
        env: &Environment,
        resources: &Resources,
        purpose: &Purpose,
    ) -> Option<Action> {
        loop {
            let step = match self.phase() {
                Phase::Agent => moves::agent::prepare(ctx, resources, "captain"),
                Phase::Instruction => moves::instruction::load(ctx),
                Phase::Environment => moves::env::refresh(ctx, env),
                Phase::Purpose => moves::purpose::append(ctx, purpose),
                Phase::Resources => moves::resources::activate(resources),
                _ => return None,
            };

            if let Some(action) = self.drive(step) {
                return Some(action);
            }
        }
    }

    fn respond(&self, ctx: &Context, env: &Environment) -> Action {
        match moves::env::refresh(ctx, env) {
            Step::Emit(action) => action,
            Step::Ready => {
                self.enter(Phase::Running);
                Action::Halt
            }
        }
    }
}

impl Policy for Captain {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn name(&self) -> &str {
        "captain"
    }

    fn decide<'a>(
        &'a self,
        purpose: &'a Purpose,
        ctx: &'a Context,
        env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async move {
            if let Some(action) = self.setup(ctx, env, resources, purpose) {
                return action;
            }

            if self.phase() == Phase::Respond {
                return self.respond(ctx, env);
            }

            if inbox.peek().is_some() {
                return Action::Take;
            }

            let last = ctx.fragments().last();

            if let Some(fragment) = last
                && let Content::Hitch { code, .. } = &fragment.content
            {
                if let Some(status_code) = code
                    && (*status_code == HTTP_UNAUTHORIZED || *status_code == HTTP_FORBIDDEN)
                {
                    warn!(code = *status_code, "decide: permanent hitch, done");
                    return Action::Done;
                }

                if self.retry.backoff().await {
                    let attempts = self.retry.count();
                    trace!(attempts, "decide: hitched, retrying");
                    self.enter(Phase::Respond);
                    return self.respond(ctx, env);
                }

                warn!("decide: retry budget exhausted, done");
                return Action::Done;
            }

            self.retry.reset();

            if last.is_some_and(|fragment| fragment.role == Role::Tool) {
                trace!("decide: last is Tool, halting");
                self.enter(Phase::Respond);
                return self.respond(ctx, env);
            }

            trace!("decide: done");
            Action::Done
        })
    }
}
