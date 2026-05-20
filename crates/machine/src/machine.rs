use std::collections::HashMap;

use crate::context::Context;
use crate::env::Environment;
use crate::event::{content_kind, preview, role_name};
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
        let mut step = 0u64;
        let mut round_counts: HashMap<&'static str, u64> = HashMap::new();

        hook!(event = "machine_start", purpose = %purpose.text);

        if self
            .run_phases(
                purpose,
                ctx,
                env,
                resources,
                &mut inbox,
                &mut step,
                &mut round_counts,
                self.policy.pre(),
            )
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
                .apply(
                    action,
                    purpose,
                    ctx,
                    env,
                    resources,
                    &mut inbox,
                    &mut step,
                    &mut round_counts,
                )
                .await
            {
                break;
            }
        }

        self.run_phases(
            purpose,
            ctx,
            env,
            resources,
            &mut inbox,
            &mut step,
            &mut round_counts,
            self.policy.post(),
        )
        .await;
    }

    async fn run_phases(
        &self,
        purpose: &Purpose,
        ctx: &mut Context,
        env: &mut Environment,
        resources: &mut Resources,
        inbox: &mut Inbox,
        step: &mut u64,
        round_counts: &mut HashMap<&'static str, u64>,
        phases: Vec<Box<dyn Phase>>,
    ) -> bool {
        for phase in phases {
            loop {
                match phase.decide(purpose, ctx, env, resources) {
                    PhaseOutcome::Action(Action::Halt) => {
                        warn!(phase = phase.name(), "phase produced Halt, ignoring");
                    }
                    PhaseOutcome::Action(action) => {
                        *step += 1;
                        *round_counts.entry(action.name()).or_insert(0) += 1;
                        if self
                            .dispatch(action, ctx, resources, inbox, *step, round_counts)
                            .await
                        {
                            return true;
                        }
                    }
                    PhaseOutcome::Done => break,
                }
            }
        }
        false
    }

    /// Full action dispatch. Halt runs pre-halt phases + reactor + post-halt phases;
    /// all other actions apply directly.
    async fn apply(
        &self,
        action: Action,
        purpose: &Purpose,
        ctx: &mut Context,
        env: &mut Environment,
        resources: &mut Resources,
        inbox: &mut Inbox,
        step: &mut u64,
        round_counts: &mut HashMap<&'static str, u64>,
    ) -> bool {
        match action {
            Action::Halt => {
                if self
                    .run_phases(
                        purpose,
                        ctx,
                        env,
                        resources,
                        inbox,
                        step,
                        round_counts,
                        self.policy.pre_halt(),
                    )
                    .await
                {
                    return true;
                }

                hook!(
                    event = "halt",
                    step = *step,
                    model = %resources.active_model().name,
                    messages = ctx.fragments().len(),
                    tools = resources.active_tools.len(),
                );
                reactor::react(ctx, env, resources, inbox).await;

                self.run_phases(
                    purpose,
                    ctx,
                    env,
                    resources,
                    inbox,
                    step,
                    round_counts,
                    self.policy.post_halt(),
                )
                .await
            }
            other => {
                let action_name = other.name();
                *step += 1;
                *round_counts.entry(action_name).or_insert(0) += 1;
                self.dispatch(other, ctx, resources, inbox, *step, round_counts)
                    .await
            }
        }
    }

    /// Apply a single action to context / resources / inbox. Does NOT handle Halt
    /// and does NOT run phases or reactor — pure dispatch only.
    async fn dispatch(
        &self,
        action: Action,
        ctx: &mut Context,
        resources: &mut Resources,
        inbox: &mut Inbox,
        step: u64,
        round_counts: &mut HashMap<&'static str, u64>,
    ) -> bool {
        match action {
            Action::Append(frag) => {
                let id = ctx.append(frag);
                let frag = ctx.get(id).expect("just appended");
                hook!(event = "appended", id, step, role = role_name(frag.role), kind = content_kind(frag), preview = %preview(frag));
            }
            Action::Insert { after, fragment } => {
                let id = ctx.insert(after, fragment);
                let frag = ctx.get(id).expect("just inserted");
                hook!(event = "inserted", id, step, role = role_name(frag.role), kind = content_kind(frag), preview = %preview(frag));
            }
            Action::Replace { id, fragment } => {
                ctx.replace(id, fragment);
                let frag = ctx.get(id).expect("just replaced");
                hook!(event = "replaced", id, step, role = role_name(frag.role), kind = content_kind(frag), preview = %preview(frag));
            }
            Action::Remove(id) => ctx.remove(id),
            Action::Swap(id1, id2) => ctx.swap(id1, id2),
            Action::Model(name) => resources.use_model(name),
            Action::Activate(name) => resources.enable(name),
            Action::Deactivate(name) => resources.disable(name),
            Action::Take => {
                if let Some(frag) = inbox.pop() {
                    ctx.append(frag);
                }
            }
            Action::Done => {
                hook!(event = "done", step, round_counts = ?round_counts);
                return true;
            }
            Action::Halt => {
                // Halt is handled by apply(); this arm is never reached from phases.
            }
        }
        false
    }
}
