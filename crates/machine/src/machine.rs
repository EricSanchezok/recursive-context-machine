use crate::context::Context;
use crate::env::Environment;
use crate::event::preview;
use crate::hook;
use crate::inbox::Inbox;
use crate::policy::{Action, Phase, PhaseOutcome, Policy};
use crate::purpose::Purpose;
use crate::reactor;
use crate::resources::Resources;
use tracing::{trace, warn};

pub struct Machine {
    policy: Box<dyn Policy>,
}

impl Machine {
    pub fn new(policy: Box<dyn Policy>) -> Self {
        Self { policy }
    }

    pub async fn run(
        &self,
        purpose: &Purpose,
        ctx: &mut Context,
        env: &mut Environment,
        resources: &mut Resources,
    ) {
        let mut inbox = Inbox::new();
        let mut round = 0u32;

        hook!(event = "machine_start", purpose = %purpose.text);

        if self
            .run_phases(purpose, ctx, env, resources, &mut inbox, self.policy.pre())
            .await
        {
            return;
        }

        loop {
            let action = self
                .policy
                .decide(purpose, ctx, env, resources, &inbox)
                .await;
            trace!(?action, "machine step");

            if self
                .apply(action, purpose, ctx, env, resources, &mut inbox, &mut round)
                .await
            {
                break;
            }
        }

        self.run_phases(purpose, ctx, env, resources, &mut inbox, self.policy.post())
            .await;
    }

    async fn run_phases(
        &self,
        purpose: &Purpose,
        ctx: &mut Context,
        env: &mut Environment,
        resources: &mut Resources,
        inbox: &mut Inbox,
        phases: Vec<Box<dyn Phase>>,
    ) -> bool {
        for phase in phases {
            loop {
                match phase.decide(purpose, ctx, env, resources) {
                    PhaseOutcome::Action(Action::Halt) => {
                        warn!(phase = phase.name(), "phase produced Halt, ignoring");
                    }
                    PhaseOutcome::Action(action) => {
                        if self.apply_action(action, ctx, resources, inbox).await {
                            return true;
                        }
                    }
                    PhaseOutcome::Done => break,
                }
            }
        }
        false
    }

    async fn apply(
        &self,
        action: Action,
        purpose: &Purpose,
        ctx: &mut Context,
        env: &mut Environment,
        resources: &mut Resources,
        inbox: &mut Inbox,
        round: &mut u32,
    ) -> bool {
        match action {
            Action::Halt => {
                if self
                    .run_phases(purpose, ctx, env, resources, inbox, self.policy.pre_halt())
                    .await
                {
                    return true;
                }

                *round += 1;
                hook!(
                    event = "halt",
                    round = *round,
                    model = %resources.active_model().name,
                    messages = ctx.fragments().len(),
                    tools = resources.active_tools.len(),
                );
                reactor::react(ctx, env, resources, inbox).await;

                self.run_phases(purpose, ctx, env, resources, inbox, self.policy.post_halt())
                    .await
            }
            other => self.apply_action(other, ctx, resources, inbox).await,
        }
    }

    async fn apply_action(
        &self,
        action: Action,
        ctx: &mut Context,
        resources: &mut Resources,
        inbox: &mut Inbox,
    ) -> bool {
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
                return true;
            }
            Action::Halt => {
                warn!("apply_action received Halt, this should be handled by apply()");
            }
        }
        false
    }
}
