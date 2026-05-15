use crate::completion;
use crate::context::Context;
use crate::env::Environment;
use crate::inbox::Inbox;
use crate::resources::Resources;

/// Calls [`completion::complete`] and pushes response fragments into the inbox.
pub async fn react(ctx: &Context, _env: &Environment, resources: &Resources, inbox: &mut Inbox) {
    let fragments = completion::complete(ctx, resources).await;
    for frag in fragments {
        inbox.push(frag);
    }
}
