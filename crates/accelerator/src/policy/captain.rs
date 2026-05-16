use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use machine::{Action, Context, Environment, Fragment, Inbox, Policy, Purpose, Resources, Role};
use tracing::{debug, trace, warn};

/// Captain — the default steering policy.
///
/// `Bootstrap → Inject → Halt → Drain → [loop] → Done`
///
/// - **Bootstrap**: replace or inject the `tag == "agent"` system prompt.
/// - **Inject**: append the user's purpose as a user fragment.
/// - **Halt**: call the LLM.
/// - **Drain**: take inbox into context. If any [`Role::Tool`] fragments were
///   taken, halt again so the LLM can see results and call more tools.
/// - **Done**: final answer is in context.
pub struct Captain {
    phase: AtomicU8,
    tool_seen: AtomicBool,
}

impl Clone for Captain {
    fn clone(&self) -> Self {
        Self {
            phase: AtomicU8::new(self.phase.load(Ordering::Relaxed)),
            tool_seen: AtomicBool::new(self.tool_seen.load(Ordering::Relaxed)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Phase {
    Bootstrap = 1,
    Inject = 2,
    Halt = 3,
    Drain = 4,
    Done = 5,
}

impl Phase {
    fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Bootstrap,
            2 => Self::Inject,
            3 => Self::Halt,
            4 => Self::Drain,
            5 => Self::Done,
            other => {
                warn!(phase = other, "captain: unknown phase, forcing done");
                Self::Done
            }
        }
    }
}

impl Default for Captain {
    fn default() -> Self {
        Self {
            phase: AtomicU8::new(Phase::Bootstrap as u8),
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

    fn decide<'a>(
        &'a self,
        purpose: &'a Purpose,
        ctx: &'a Context,
        _env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async move {
            let phase = Phase::from_u8(self.phase.load(Ordering::Relaxed));
            let (next, action) = match phase {
                Phase::Bootstrap => bootstrap(purpose, ctx, resources),
                Phase::Inject => inject(purpose),
                Phase::Halt => {
                    self.tool_seen.store(false, Ordering::Relaxed);
                    (Phase::Drain, Action::Halt)
                }
                Phase::Drain => self.drain(inbox),
                Phase::Done => (Phase::Done, Action::Done),
            };
            self.phase.store(next as u8, Ordering::Relaxed);
            trace!(?phase, ?next, "captain decide");
            action
        })
    }
}

/// Ensure the `tag == "agent"` system prompt is in scope.
///
/// - Exists → **Replace** with the default prompt (preserves position).
/// - Missing → **Append** the default prompt.
fn bootstrap(_purpose: &Purpose, ctx: &Context, resources: &Resources) -> (Phase, Action) {
    let prompt = resources
        .prompts
        .get("default")
        .cloned()
        .unwrap_or_default();

    if let Some(frag) = ctx
        .fragments()
        .iter()
        .find(|f| f.role == Role::System && f.tag == "agent")
    {
        debug!("bootstrap: replacing agent prompt");
        (
            Phase::Inject,
            Action::Replace {
                id: frag.id(),
                fragment: Fragment::system(prompt).with_tag("agent"),
            },
        )
    } else {
        debug!("bootstrap: injecting new agent prompt");
        (
            Phase::Inject,
            Action::Append(Fragment::system(prompt).with_tag("agent")),
        )
    }
}

/// Append the user's purpose as a [`Role::User`] fragment.
fn inject(purpose: &Purpose) -> (Phase, Action) {
    if purpose.is_empty() {
        debug!("inject: no purpose, skipping");
        (Phase::Halt, Action::Halt)
    } else {
        debug!(purpose = purpose.text, "inject: appending purpose");
        (Phase::Halt, Action::Append(Fragment::user(&purpose.text)))
    }
}

impl Captain {
    fn drain(&self, inbox: &Inbox) -> (Phase, Action) {
        if let Some(frag) = inbox.peek() {
            if frag.role == Role::Tool {
                self.tool_seen.store(true, Ordering::Relaxed);
            }
            (Phase::Drain, Action::Take)
        } else if self.tool_seen.load(Ordering::Relaxed) {
            trace!("drain: tool results seen, halting again");
            (Phase::Halt, Action::Halt)
        } else {
            trace!("drain: final answer ready");
            (Phase::Done, Action::Done)
        }
    }
}
