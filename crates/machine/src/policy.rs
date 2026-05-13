use std::future::Future;
use std::pin::Pin;

use serde_json::Value;

use crate::context::Context;
use crate::env::Environment;
use crate::inbox::Inbox;

/// The next action the Policy should take.
///
/// These are the primitive vocabulary of context manipulation.
/// The Policy returns one action per decision step. Multiple actions
/// compose to form the full $\pi$ transition.
#[derive(Debug, Clone)]
pub enum Action {
    /// Pop the inbox head and append it to the context.
    Take,

    /// Pop the inbox head and insert it after the fragment with the given id.
    TakeAfter { id: u64 },

    /// Pop the inbox head and replace the fragment at the given id.
    Swap { id: u64 },

    /// Remove the fragment at the given id (no inbox consumption).
    Drop { id: u64 },

    /// Modify the environment (e.g. switch model, set parameter).
    Set { key: String, value: Value },

    /// Stop the $\pi$ phase and trigger $\omega$.
    Halt,
}

/// Policy — the context engineering function $\pi$.
///
/// Observes the current context, environment, and inbox, and decides
/// the next [`Action`]. This is the primary extension point. Swap the
/// Policy to change how the machine assembles context.
///
/// No concrete implementations live in this crate. They belong in
/// downstream crates that compose specific strategies.
pub trait Policy: Send + Sync {
    fn decide<'a>(
        &'a self,
        ctx: &'a Context,
        env: &'a Environment,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>>;
}
