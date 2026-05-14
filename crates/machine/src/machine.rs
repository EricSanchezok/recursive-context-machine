use crate::context::Context;
use crate::env::Environment;
use crate::inbox::Inbox;
use crate::policy::{Action, Policy};
use crate::reactor::Reactor;
use crate::resources::Resources;

/// Machine — the composition of a Policy (π) and a Reactor (ω).
///
/// A machine is a triple ℳ = (ℂ, ℰ, Φ) where Φ is implemented by [`Machine::run`].
pub struct Machine {
    policy: Box<dyn Policy>,
    reactor: Box<dyn Reactor>,
}

impl Machine {
    pub fn new(policy: Box<dyn Policy>, reactor: Box<dyn Reactor>) -> Self {
        Self { policy, reactor }
    }

    /// Run the machine until [`Action::Done`].
    ///
    /// Borrows `ctx`, `env`, and `resources` from the caller. The inbox is
    /// internal to the machine loop.
    ///
    /// - Context operations (`Append`, `Insert`, `Replace`, `Remove`, `Swap`)
    ///   modify the tape.
    /// - Resource operations (`Model`, `Catch`, `Drop`) set activation state
    ///   on Resources.
    /// - [`Action::Take`] pops the inbox head and appends it to the context,
    ///   if any.
    /// - [`Action::Halt`] triggers the Reactor phase, which pushes new
    ///   fragments into the inbox.
    /// - [`Action::Done`] terminates the machine.
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
                    resources.model(name);
                }
                Action::Catch(name) => {
                    resources.catch(name);
                }
                Action::Drop(name) => {
                    resources.drop(&name);
                }
                Action::Take => {
                    if let Some(frag) = inbox.pop() {
                        ctx.append(frag);
                    }
                }
                Action::Done => return,
                Action::Halt => {
                    self.reactor.react(ctx, env, resources, &mut inbox).await;
                }
            }
        }
    }
}
