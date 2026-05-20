use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use machine::{Action, Content, Context, Environment, Inbox, Phase, Policy, Purpose, Resources, Role};
use tracing::{trace, warn};

use super::phases::{self, Bootstrap, Env, Instructions};

/// Maximum number of consecutive retryable hitches Captain will tolerate
/// before giving up. Reset to zero on any successful turn (last fragment is
/// `Tool` or `Assistant` non-hitch).
const MAX_HITCH_RETRIES: u32 = 3;

/// Captain — a simple single-agent Policy.
///
/// Decides purely based on inbox state and the last fragment in context:
///
///   Inbox not empty                 → Take (drain one fragment into context)
///   Inbox empty:
///     last in context is Hitch:
///       hitch.retryable && attempts < MAX_HITCH_RETRIES → Halt (retry)
///       otherwise                                       → Done (give up)
///     last in context is Tool                           → Halt (show LLM the result)
///     last in context is not Tool, first call already happened → Done
///     first call ever                                   → Halt (kick off the LLM)
pub struct Captain {
    started: AtomicBool,
    hitch_attempts: AtomicU32,
}

impl Clone for Captain {
    fn clone(&self) -> Self {
        // Clone-as-fresh: a cloned Captain represents a separate logical run,
        // so internal counters reset rather than carry over.
        Self {
            started: AtomicBool::new(false),
            hitch_attempts: AtomicU32::new(0),
        }
    }
}

impl Default for Captain {
    fn default() -> Self {
        Self {
            started: AtomicBool::new(false),
            hitch_attempts: AtomicU32::new(0),
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

            let not_started = !self.started.swap(true, Ordering::Relaxed);
            if not_started {
                trace!("decide: first call, halting");
                return Action::Halt;
            }

            let last = ctx.fragments().last();
            let last_role = last.map(|f| f.role);

            // Hitch path: retry transient failures up to MAX_HITCH_RETRIES.
            if last_role == Some(Role::Hitch) {
                let retryable = matches!(
                    &last.unwrap().content,
                    Content::Hitch { retryable: true, .. }
                );
                if retryable {
                    let attempts = self.hitch_attempts.fetch_add(1, Ordering::Relaxed);
                    if attempts + 1 <= MAX_HITCH_RETRIES {
                        trace!(attempts = attempts + 1, "decide: retryable hitch, halting");
                        return Action::Halt;
                    }
                    warn!(
                        attempts = attempts + 1,
                        max = MAX_HITCH_RETRIES,
                        "decide: hitch retries exhausted, giving up"
                    );
                } else {
                    trace!("decide: non-retryable hitch, done");
                }
                return Action::Done;
            }

            // Any non-hitch path resets the retry counter.
            self.hitch_attempts.store(0, Ordering::Relaxed);

            match last_role {
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
