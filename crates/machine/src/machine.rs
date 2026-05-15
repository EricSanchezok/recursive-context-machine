use crate::context::Context;
use crate::env::Environment;
use crate::inbox::Inbox;
use crate::policy::{Action, Policy};
use crate::reactor;
use crate::resources::Resources;

/// Machine — the composition of a Policy and a Reactor.
pub struct Machine {
    policy: Box<dyn Policy>,
}

impl Machine {
    pub fn new(policy: Box<dyn Policy>) -> Self {
        Self { policy }
    }

    /// Run the machine until [`Action::Done`].
    ///
    /// When the reactor executes tools, the machine automatically drains
    /// the results into context and re-invokes the reactor so the LLM
    /// can see the tool output — no policy intervention needed.
    pub async fn run(&self, ctx: &mut Context, env: &mut Environment, resources: &mut Resources) {
        let mut inbox = Inbox::new();

        loop {
            let action = self.policy.decide(ctx, env, resources, &inbox).await;

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
                    resources.set_active_model(name);
                }
                Action::Activate(name) => {
                    resources.activate_tool(name);
                }
                Action::Deactivate(name) => {
                    resources.deactivate_tool(&name);
                }
                Action::Take => {
                    if let Some(frag) = inbox.pop() {
                        ctx.append(frag);
                    }
                }
                Action::Done => return,
                Action::Halt => {
                    // Reactor loop: complete + execute tools until stable.
                    loop {
                        let executed = reactor::react(ctx, env, resources, &mut inbox).await;

                        // Drain all results into context.
                        while let Some(frag) = inbox.pop() {
                            ctx.append(frag);
                        }

                        if !executed {
                            break;
                        }
                    }
                }
            }
        }
    }
}
