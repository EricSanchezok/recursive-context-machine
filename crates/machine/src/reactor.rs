use std::future::Future;
use std::pin::Pin;

use crate::context::Context;
use crate::env::Environment;
use crate::inbox::Inbox;

/// Reactor — the environment transition function $\omega$.
///
/// Takes the current context and a mutable environment, invokes the
/// language model, executes any tool calls, and returns new fragments
/// in an [`Inbox`]. An empty inbox signals termination.
///
/// No concrete implementations live in this crate. They belong in
/// downstream crates that wire up specific LLM providers and tools.
pub trait Reactor: Send + Sync {
    fn react<'a>(
        &'a self,
        ctx: &'a Context,
        env: &'a mut Environment,
    ) -> Pin<Box<dyn Future<Output = Inbox> + Send + 'a>>;
}
