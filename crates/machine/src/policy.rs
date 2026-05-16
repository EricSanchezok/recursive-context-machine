use std::future::Future;
use std::pin::Pin;

use crate::context::Context;
use crate::env::Environment;
use crate::fragment::Fragment;
use crate::inbox::Inbox;
use crate::purpose::Purpose;
use crate::resources::Resources;

/// Atomic, discrete operations composed by Policy across decision steps.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
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

    /// Set the active model. Only one model can be active.
    Model(String),

    /// Add a tool to the active set.
    Activate(String),

    /// Remove a tool from the active set.
    Deactivate(String),

    /// Pop the head of the inbox and append it to context.
    Take,

    /// Trigger LLM completion.
    Halt,

    /// Stop the machine.
    Done,
}

/// The outcome of a single Phase decision step.
#[derive(Debug, Clone, PartialEq)]
pub enum PhaseOutcome {
    /// Execute this Action and call the Phase again.
    Action(Action),
    /// This Phase is complete — proceed to the next Phase or the core Policy.
    Done,
}

/// Phase — a reusable context-preparation step.
///
/// Executed by Machine before or after the core Policy. Each call produces
/// a [`PhaseOutcome`]: either an Action to apply (followed by another call)
/// or Done to signal completion.
pub trait Phase: Send + Sync {
    fn clone_box(&self) -> Box<dyn Phase>;
    fn name(&self) -> &str;

    fn decide(
        &self,
        purpose: &Purpose,
        ctx: &Context,
        env: &Environment,
        resources: &Resources,
    ) -> PhaseOutcome;
}

impl Clone for Box<dyn Phase> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Policy — the context engineering function π.
///
/// Observes the current context, environment, resources, and inbox,
/// and decides the next [`Action`]. This is the primary extension point.
pub trait Policy: Send + Sync {
    fn clone_box(&self) -> Box<dyn Policy>;

    /// Preparation phases executed before the core loop.
    fn pre(&self) -> Vec<Box<dyn Phase>> {
        Vec::new()
    }

    /// Post-processing phases executed after the core loop returns Done.
    fn post(&self) -> Vec<Box<dyn Phase>> {
        Vec::new()
    }

    fn decide<'a>(
        &'a self,
        purpose: &'a Purpose,
        ctx: &'a Context,
        env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>>;
}

impl Clone for Box<dyn Policy> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
