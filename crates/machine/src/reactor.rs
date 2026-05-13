use std::future::Future;
use std::pin::Pin;

use crate::context::Context;
use crate::env::Environment;
use crate::inbox::Inbox;
use crate::model::Model;
use crate::tool::Tool;

/// Reactor — the environment transition function ω.
///
/// Receives the current context, environment, and the Policy's selected
/// tools and model. Invokes the language model, executes any tool calls,
/// and returns new fragments in an [`Inbox`].
///
/// No concrete implementations live in this crate. They belong in
/// downstream crates that wire up specific LLM providers.
pub trait Reactor: Send + Sync {
    fn react<'a>(
        &'a self,
        ctx: &'a Context,
        env: &'a Environment,
        tools: &'a [&'a dyn Tool],
        model: Option<&'a Model>,
    ) -> Pin<Box<dyn Future<Output = Inbox> + Send + 'a>>;
}
