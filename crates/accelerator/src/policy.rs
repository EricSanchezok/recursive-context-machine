use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU8, Ordering};

use machine::{Action, Context, Environment, Inbox, Policy, Resources};

/// Default policy — a minimal three-step loop.
///
/// ```text
///   1. Halt — trigger LLM completion
///   2. Take — pop the assistant response into context
///   3. Done
/// ```
///
/// This is intentionally minimal. Users can override with
/// [`Accelerator::with_policy`](crate::Accelerator::with_policy).
pub struct DefaultPolicy {
    step: AtomicU8,
}

impl DefaultPolicy {
    pub fn new() -> Self {
        Self {
            step: AtomicU8::new(1),
        }
    }
}

impl Policy for DefaultPolicy {
    fn decide<'a>(
        &'a self,
        _ctx: &'a Context,
        _env: &'a Environment,
        _resources: &'a Resources,
        _inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        let current = self.step.fetch_add(1, Ordering::Relaxed) + 1;
        Box::pin(async move {
            match current {
                1 => Action::Halt,
                2 => Action::Take,
                _ => Action::Done,
            }
        })
    }
}
