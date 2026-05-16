use crate::context::Context;
use crate::env::Environment;
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
                    ctx.append(frag);
                }
                Action::Insert { after, fragment } => {
                    ctx.insert(after, fragment);
                }
                Action::Replace { id, fragment } => {
                    ctx.replace(id, fragment);
                }
                Action::Remove(id) => {
                    ctx.remove(id);
                }
                Action::Swap(id1, id2) => {
                    ctx.swap(id1, id2);
                }
                Action::Model(name) => {
                    resources.use_model(name);
                }
                Action::Activate(name) => {
                    resources.enable(name);
                }
                Action::Deactivate(name) => {
                    resources.disable(name);
                }
                Action::Take => {
                    if let Some(frag) = inbox.pop() {
                        ctx.append(frag);
                    }
                }
                Action::Done => return,
                Action::Halt => reactor::react(ctx, env, resources, &mut inbox).await,
            }
        }
    }
}
