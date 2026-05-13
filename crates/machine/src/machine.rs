use crate::context::Context;
use crate::env::Environment;
use crate::inbox::Inbox;
use crate::policy::{Action, Policy};
use crate::reactor::Reactor;

/// Machine — the composition of a Policy ($\pi$) and a Reactor ($\omega$).
///
/// A machine is a triple $\mathcal{M} = (\mathcal{C}, \mathcal{E}, \Phi)$
/// where $\Phi$ is implemented by [`Machine::run`].
pub struct Machine {
    policy: Box<dyn Policy>,
    reactor: Box<dyn Reactor>,
}

impl Machine {
    pub fn new(policy: Box<dyn Policy>, reactor: Box<dyn Reactor>) -> Self {
        Self { policy, reactor }
    }

    /// Run the machine to completion.
    ///
    /// The loop alternates between two phases:
    ///
    /// **$\pi$ phase** — The Policy consumes the inbox and builds context.
    /// Each [`Action`] is executed immediately. When the Policy returns
    /// [`Action::Halt`], the phase ends.
    ///
    /// **$\omega$ phase** — The Reactor invokes the language model against
    /// the current context, executes any tool calls, and returns a new inbox.
    /// An empty inbox signals termination.
    pub async fn run(
        &self,
        mut ctx: Context,
        mut env: Environment,
        mut inbox: Inbox,
    ) -> (Context, Environment) {
        loop {
            // ── π phase ──
            loop {
                let action = self.policy.decide(&ctx, &env, &inbox).await;

                match action {
                    Action::Take => {
                        if let Some(f) = inbox.pop() {
                            ctx.append(f);
                        }
                    }
                    Action::TakeAfter { id } => {
                        if let Some(f) = inbox.pop() {
                            ctx.insert(id, f);
                        }
                    }
                    Action::Swap { id } => {
                        if let Some(f) = inbox.pop() {
                            ctx.replace(id, f);
                        }
                    }
                    Action::Drop { id } => {
                        ctx.remove(id);
                    }
                    Action::Set { key, value } => {
                        env.set(&key, value);
                    }
                    Action::Halt => break,
                }
            }

            // ── ω phase ──
            inbox = self.reactor.react(&ctx, &mut env).await;

            if inbox.is_empty() {
                return (ctx, env);
            }
        }
    }
}
