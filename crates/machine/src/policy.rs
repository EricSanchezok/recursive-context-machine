use std::future::Future;
use std::pin::Pin;

use crate::context::Context;
use crate::env::Environment;
use crate::fragment::Fragment;
use crate::resources::Resources;

/// The next action the Policy should take.
///
/// These are atomic, discrete operations. The Policy composes them
/// across multiple decision steps to build the full context state.
/// When ready, the Policy returns [`Action::Halt`] to trigger the
/// Reactor phase.
#[derive(Debug, Clone)]
pub enum Action {
    /// Append a fragment to the context.
    Add(Fragment),

    /// Remove the fragment with the given id.
    Remove(u64),

    /// Swap the positions of two fragments by id.
    Swap(u64, u64),

    /// Set the model name for the next Reactor invocation.
    SetModel(String),

    /// Add a tool name for the next Reactor invocation.
    AddTool(String),

    /// Remove a tool name.
    RemoveTool(String),

    /// Stop the π phase and trigger ω.
    Halt,
}

/// Policy — the context engineering function π.
///
/// Observes the current context, environment, and resources, and decides
/// the next [`Action`]. This is the primary extension point.
pub trait Policy: Send + Sync {
    fn decide<'a>(
        &'a self,
        ctx: &'a Context,
        env: &'a Environment,
        resources: &'a Resources,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>>;
}
