use crate::context::Context;
use crate::env::Environment;
use crate::policy::{Action, Policy};
use crate::reactor::Reactor;
use crate::resources::Resources;
use crate::tool::Tool;

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

    /// Run the machine to completion.
    ///
    /// The loop alternates between two phases:
    ///
    /// **π phase** — The Policy executes atomic actions on the context.
    /// When the Policy returns [`Action::Halt`], the phase ends.
    ///
    /// **ω phase** — The Machine reads the selected model and tools from
    /// the context, invokes the Reactor, and appends the returned fragments
    /// back into the context. An empty inbox signals termination.
    pub async fn run(&self, mut ctx: Context, env: Environment, resources: &Resources) -> Context {
        loop {
            // ── π phase ──
            loop {
                let action = self.policy.decide(&ctx, &env, resources).await;

                match action {
                    Action::Add(frag) => {
                        ctx.append(frag);
                    }
                    Action::Remove(id) => {
                        ctx.remove(id);
                    }
                    Action::Swap(id1, id2) => {
                        ctx.swap(id1, id2);
                    }
                    Action::SetModel(name) => {
                        ctx.set_model(name);
                    }
                    Action::AddTool(name) => {
                        ctx.add_tool(name);
                    }
                    Action::RemoveTool(name) => {
                        ctx.remove_tool(&name);
                    }
                    Action::Halt => break,
                }
            }

            // ── ω phase ──
            let model = ctx
                .model()
                .and_then(|name| resources.models.iter().find(|m| m.name == name));
            let tools: Vec<&dyn Tool> = ctx
                .tools()
                .iter()
                .filter_map(|k| {
                    resources
                        .tools
                        .iter()
                        .find(|t| t.name() == k)
                        .map(|t| t.as_ref())
                })
                .collect();

            let inbox = self.reactor.react(&ctx, &env, &tools, model).await;

            if inbox.is_empty() {
                return ctx;
            }

            for frag in inbox {
                ctx.append(frag);
            }
        }
    }
}
