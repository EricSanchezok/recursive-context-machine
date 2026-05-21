use std::future::Future;
use std::pin::Pin;

use machine::{
    Action, Content, Context, Environment, Inbox, Phase, PhaseOutcome, Policy, Purpose, Resources,
    Role,
};
use tracing::{trace, warn};

use super::phases::{self, Bootstrap, Env, Instructions};
use super::retry::{HTTP_FORBIDDEN, HTTP_UNAUTHORIZED, Retry};

/// Captain — a simple single-agent Policy.
///
///   Inbox not empty             → Take
///   Inbox empty:
///     first call ever           → Halt
///     last is Hitch:
///       401/403                 → Done
///       transient, budget > 0   → backoff, Halt (retry)
///       budget exhausted        → Done
///     last is Tool              → Halt
///     last is not Tool          → Done
pub struct Captain {
    first_call: std::sync::atomic::AtomicBool,
    retry: Retry,
}

impl Clone for Captain {
    fn clone(&self) -> Self {
        Self {
            first_call: std::sync::atomic::AtomicBool::new(false),
            retry: self.retry.clone(),
        }
    }
}

impl Default for Captain {
    fn default() -> Self {
        Self {
            first_call: std::sync::atomic::AtomicBool::new(false),
            retry: Retry::default(),
        }
    }
}

impl Captain {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone)]
struct CaptainResourceSetup;

impl Phase for CaptainResourceSetup {
    fn clone_box(&self) -> Box<dyn Phase> {
        Box::new(Self)
    }

    fn name(&self) -> &str {
        "captain_resources"
    }

    fn decide(
        &self,
        _purpose: &Purpose,
        _ctx: &Context,
        _env: &Environment,
        resources: &Resources,
    ) -> PhaseOutcome {
        if resources.active_model.is_empty() {
            if let Some(model_name) = resources.model_order.first() {
                return PhaseOutcome::Action(Action::Model(model_name.clone()));
            }
        }

        if let Some(tool_name) = resources
            .tools
            .keys()
            .find(|tool_name| !resources.active_tools.contains(*tool_name))
        {
            return PhaseOutcome::Action(Action::Activate(tool_name.clone()));
        }

        PhaseOutcome::Done
    }
}

impl Policy for Captain {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn name(&self) -> &str {
        "captain"
    }

    fn pre(&self) -> Vec<Box<dyn Phase>> {
        vec![
            Box::new(Bootstrap::new("captain")),
            Box::new(Instructions),
            Box::new(phases::Purpose),
            Box::new(CaptainResourceSetup),
        ]
    }

    fn pre_halt(&self) -> Vec<Box<dyn Phase>> {
        vec![Box::new(Env)]
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

            if !self
                .first_call
                .swap(true, std::sync::atomic::Ordering::Relaxed)
            {
                trace!("decide: first call, halting");
                return Action::Halt;
            }

            let last = ctx.fragments().last();

            if let Some(frag) = last {
                if let Content::Hitch { code, .. } = &frag.content {
                    if let Some(c) = code {
                        if *c == HTTP_UNAUTHORIZED || *c == HTTP_FORBIDDEN {
                            warn!(code = *c, "decide: permanent hitch, done");
                            return Action::Done;
                        }
                    }

                    if self.retry.backoff().await {
                        let attempts = self.retry.count();
                        trace!(attempts, "decide: hitched, retrying");
                        return Action::Halt;
                    }
                    warn!("decide: retry budget exhausted, done");
                    return Action::Done;
                }
            }

            self.retry.reset();

            match last.map(|f| f.role) {
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
