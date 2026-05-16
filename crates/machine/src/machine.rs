use crate::context::Context;
use crate::env::Environment;
use crate::event::preview;
use crate::hook;
use crate::inbox::Inbox;
use crate::policy::{Action, Policy};
use crate::purpose::Purpose;
use crate::reactor;
use crate::resources::Resources;
use tracing::trace;

/// Machine — the composition of a Policy and a Reactor.
pub struct Machine {
    policy: Box<dyn Policy>,
}

impl Machine {
    pub fn new(policy: Box<dyn Policy>) -> Self {
        Self { policy }
    }

    /// Run the machine until [`Action::Done`].
    pub async fn run(
        &self,
        purpose: &Purpose,
        ctx: &mut Context,
        env: &mut Environment,
        resources: &mut Resources,
    ) {
        let mut inbox = Inbox::new();

        loop {
            let action = self
                .policy
                .decide(purpose, ctx, env, resources, &inbox)
                .await;
            trace!(?action, "machine step");

            match action {
                Action::Append(frag) => {
                    let id = ctx.append(frag);
                    let frag = ctx.get(id).expect("just appended");
                    hook!(
                        event = "appended",
                        id,
                        role = ?frag.role,
                        preview = %preview(frag),
                    );
                }
                Action::Insert { after, fragment } => {
                    let id = ctx.insert(after, fragment);
                    let frag = ctx.get(id).expect("just inserted");
                    hook!(
                        event = "inserted",
                        id,
                        role = ?frag.role,
                        preview = %preview(frag),
                    );
                }
                Action::Replace { id, fragment } => {
                    ctx.replace(id, fragment);
                    let frag = ctx.get(id).expect("just replaced");
                    hook!(
                        event = "replaced",
                        id,
                        role = ?frag.role,
                        preview = %preview(frag),
                    );
                }
                Action::Remove(id) => {
                    ctx.remove(id);
                    hook!(event = "removed", id);
                }
                Action::Swap(id1, id2) => {
                    ctx.swap(id1, id2);
                    hook!(event = "swapped", id1, id2);
                }
                Action::Model(name) => {
                    hook!(event = "model", name);
                    resources.use_model(name);
                }
                Action::Activate(name) => {
                    hook!(event = "activate", name);
                    resources.enable(name);
                }
                Action::Deactivate(name) => {
                    hook!(event = "deactivate", name);
                    resources.disable(name);
                }
                Action::Take => {
                    if let Some(frag) = inbox.pop() {
                        let id = ctx.append(frag);
                        let frag = ctx.get(id).expect("just appended");
                        hook!(
                            event = "taken",
                            id,
                            role = ?frag.role,
                            preview = %preview(frag),
                        );
                    }
                }
                Action::Done => {
                    hook!(event = "done");
                    return;
                }
                Action::Halt => {
                    hook!(
                        event = "halt",
                        model = %resources.active_model().name,
                        messages = ctx.fragments().len(),
                        tools = resources.active_tools.len(),
                    );
                    reactor::react(ctx, env, resources, &mut inbox).await;
                }
            }
        }
    }
}
