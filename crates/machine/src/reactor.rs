use std::future::Future;
use std::pin::Pin;

use crate::context::Context;
use crate::env::Environment;
use crate::inbox::Inbox;
use crate::resources::Resources;

/// Reactor — the environment transition function ω.
///
/// Receives the current context, environment, and resources (with
/// activation state set by the Policy). Invokes the language model,
/// executes any tool calls, and pushes new fragments into the inbox.
///
/// No concrete implementations live in this crate. They belong in
/// downstream crates that wire up specific LLM providers.
pub trait Reactor: Send + Sync {
    fn react<'a>(
        &'a self,
        ctx: &'a Context,
        env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a mut Inbox,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}
