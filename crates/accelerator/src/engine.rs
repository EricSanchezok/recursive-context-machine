use std::future::Future;
use std::pin::Pin;

use serde_json::Value;

use crate::context::Context;
use crate::fragment::Fragment;
use crate::register::Register;

/// The next action the machine should take.
///
/// These are the primitive vocabulary of context manipulation.
/// Concrete strategies (when to call LLM, when to inject memory,
/// when to halt) live in downstream Engine implementations.
pub enum Action {
    /// Append a fragment to the end of the context.
    Append { fragment: Fragment },

    /// Insert a fragment after the cell with the given id.
    Insert { id: u64, fragment: Fragment },

    /// Replace the fragment at the given id.
    Replace { id: u64, fragment: Fragment },

    /// Remove the fragment with the given id.
    Remove { id: u64 },

    /// Set a value in the register.
    Set { key: String, value: Value },

    /// Stop the machine.
    Halt { reason: String },
}

/// Engine — the transition decision function.
///
/// Observes the current Context + Register and decides the next Action.
/// This is the primary extension point. Swap the Engine to change
/// how the machine decides what to do next.
///
/// No concrete implementations live in this crate. They belong in
/// downstream crates that compose specific transition strategies.
pub trait Engine: Send + Sync {
    fn decide<'a>(
        &'a self,
        ctx: &'a Context,
        register: &'a Register,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>>;
}
