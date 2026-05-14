use crate::context::Context;
use crate::env::Environment;
use crate::inbox::Inbox;
use crate::policy::{Action, Policy};
use crate::reactor;
use crate::resources::Resources;

/// Machine — the composition of a Policy (π) and a Reactor (ω).
///
/// A machine is a triple ℳ = (ℂ, ℰ, Φ) where Φ is implemented by [`Machine::run`].
pub struct Machine {
    policy: Box<dyn Policy>,
}

impl Machine {
    /// Create a machine with the given policy.
    pub fn new(policy: Box<dyn Policy>) -> Self {
        Self { policy }
    }

    /// Run the machine until [`Action::Done`].
    ///
    /// Borrows `ctx`, `env`, and `resources` from the caller. The inbox is
    /// internal to the machine loop.
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
                Action::Catch(name) => {
                    resources.catch_tool(name);
                }
                Action::Drop(name) => {
                    resources.drop_tool(&name);
                }
                Action::Take => {
                    if let Some(frag) = inbox.pop() {
                        ctx.append(frag);
                    }
                }
                Action::Done => return,
                Action::Halt => {
                    reactor::react(ctx, env, resources, &mut inbox).await;
                }
            }
        }
    }
}
