use std::future::Future;
use std::pin::Pin;

use crate::context::Context;
use crate::env::Environment;
use crate::fragment::Fragment;
use crate::inbox::Inbox;
use crate::resources::Resources;

/// The next action the Policy should take.
///
/// These are atomic, discrete operations. The Policy composes them
/// across multiple decision steps to build the full context state.
#[derive(Debug, Clone)]
pub enum Action {
    // ── Context ──
    /// Append a fragment to the end of the context.
    Append(Fragment),

    /// Insert a fragment after the cell with the given id.
    Insert { after: u64, fragment: Fragment },

    /// Replace the fragment at the given id.
    Replace { id: u64, fragment: Fragment },

    /// Remove the fragment with the given id.
    Remove(u64),

    /// Swap the positions of two fragments by id.
    Swap(u64, u64),

    // ── Resources ──
    /// Set the active model. Only one model can be active.
    Model(String),

    /// Catch a tool — add it to the active set.
    Catch(String),

    /// Drop a tool — remove it from the active set.
    Drop(String),

    // ── Control ──
    /// Pop the inbox head and append it to the context.
    Take,

    /// Stop the π phase and trigger ω.
    Halt,

    /// Stop the machine.
    Done,
}

/// Policy — the context engineering function π.
///
/// Observes the current context, environment, resources, and inbox,
/// and decides the next [`Action`]. This is the primary extension point.
pub trait Policy: Send + Sync {
    fn decide<'a>(
        &'a self,
        ctx: &'a Context,
        env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>>;
}
