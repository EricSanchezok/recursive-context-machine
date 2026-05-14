use crate::completion;
use crate::context::Context;
use crate::env::Environment;
use crate::inbox::Inbox;
use crate::resources::Resources;

/// Reactor — the environment transition function ω.
///
/// Calls the LLM via [`completion::complete`] and pushes the response
/// fragments into the inbox. Tool execution is not yet implemented.
pub async fn react(ctx: &Context, _env: &Environment, resources: &Resources, inbox: &mut Inbox) {
    let fragments = completion::complete(ctx, resources).await;
    for frag in fragments {
        inbox.push(frag);
    }
}
