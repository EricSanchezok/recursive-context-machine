use std::future::Future;
use std::pin::Pin;

use machine::{
    Action, Content, Context, Environment, Inbox, Phase, Policy, Purpose, Resources, Role,
};
use tracing::{trace, warn};

use super::phases::{self, Bootstrap, Env, Instructions};
use super::retry::{HTTP_FORBIDDEN, HTTP_UNAUTHORIZED, Retry};

/// Captain — a simple single-agent Policy.
///
/// Decides purely based on inbox state and the last fragment in context:
///
///   Inbox not empty                 → Take (drain one fragment into context)
///   Inbox empty:
///     first call ever               → Halt (kick off the LLM)
///     last is Hitch:
///       permanent (401/403)         → Done
///       transient, budget > 0       → backoff, Halt (retry)
///       transient, budget exhausted → Done (give up)
///     last is Tool                  → Halt (show LLM the result)
///     last is not Tool              → Done
pub struct Captain {
    started: std::sync::atomic::AtomicBool,
    retry: Retry,
}

impl Clone for Captain {
    fn clone(&self) -> Self {
        Self {
            started: std::sync::atomic::AtomicBool::new(
                self.started.load(std::sync::atomic::Ordering::Relaxed),
            ),
            retry: self.retry.clone(),
        }
    }
}

impl Default for Captain {
    fn default() -> Self {
        Self {
            started: std::sync::atomic::AtomicBool::new(false),
            retry: Retry::default(),
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

    fn name(&self) -> &str {
        "captain"
    }

    fn pre(&self) -> Vec<Box<dyn Phase>> {
        vec![
            Box::new(Bootstrap::new("captain")),
            Box::new(Instructions),
            Box::new(phases::Purpose),
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

            let not_started = !self
                .started
                .swap(true, std::sync::atomic::Ordering::Relaxed);
            if not_started {
                trace!("decide: first call, halting");
                return Action::Halt;
            }

            let last = ctx.fragments().last();

            // Hitch path: any fragment whose content is Hitch.
            if let Some(frag) = last {
                if let Content::Hitch { code, .. } = &frag.content {
                    // Permanent auth failures — no point retrying.
                    if let Some(c) = code {
                        if *c == HTTP_UNAUTHORIZED || *c == HTTP_FORBIDDEN {
                            warn!(code = *c, "decide: permanent hitch, done");
                            return Action::Done;
                        }
                    }

                    let attempts = self.retry.bump();

                    // backoff() sleeps and returns false when budget is exhausted.
                    if self.retry.backoff().await {
                        trace!(attempts, "decide: retryable hitch, halting");
                        return Action::Halt;
                    }
                    warn!(attempts, "decide: hitch retries exhausted, giving up");
                    return Action::Done;
                }
            }

            // Any non-hitch path resets the retry budget.
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
