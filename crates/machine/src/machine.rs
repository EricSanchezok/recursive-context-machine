use crate::Role;
use crate::context::Context;
use crate::env::Environment;
use crate::event::{content_kind, preview, role_name};
use crate::fragment::Fragment;
use crate::hook;
use crate::inbox::Inbox;
use crate::policy::Action;
use crate::reactor;
use crate::resources::Resources;
use tracing::{trace, warn};

pub struct Machine;

impl Machine {
    pub async fn apply(
        action: Action,
        step: u64,
        ctx: &mut Context,
        env: &mut Environment,
        resources: &mut Resources,
        inbox: &mut Inbox,
    ) -> bool {
        match action {
            Action::Halt => {
                let model_name = resources
                    .active_model()
                    .map(|m| m.name.as_str())
                    .unwrap_or("none");
                hook!(
                    event = "halt",
                    step,
                    model = %model_name,
                    messages = ctx.fragments().len(),
                    tools = resources.active_tools.len(),
                );
                reactor::react(ctx, env, resources, inbox).await;
                false
            }
            other => Self::dispatch(other, step, ctx, resources, inbox).await,
        }
    }

    async fn dispatch(
        action: Action,
        step: u64,
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
                    step,
                    role = role_name(frag.role),
                    kind = content_kind(frag),
                    preview = %preview(frag),
                );
            }
            Action::Insert { after, fragment } => {
                let id = ctx.insert(after, fragment);
                let frag = ctx.get(id).expect("just inserted");
                hook!(
                    event = "inserted",
                    id,
                    step,
                    role = role_name(frag.role),
                    kind = content_kind(frag),
                    preview = %preview(frag),
                );
            }
            Action::Replace { id, fragment } => {
                ctx.replace(id, fragment);
                let frag = ctx.get(id).expect("just replaced");
                hook!(
                    event = "replaced",
                    id,
                    step,
                    role = role_name(frag.role),
                    kind = content_kind(frag),
                    preview = %preview(frag),
                );
            }
            Action::Remove(id) => {
                ctx.remove(id);
                hook!(event = "removed", id, step);
            }
            Action::Swap(id1, id2) => {
                ctx.swap(id1, id2);
                hook!(event = "swapped", id1, id2, step);
            }
            Action::Model(name) => match resources.use_model(name.clone()) {
                Ok(()) => {
                    trace!(model = %name, "switch model");
                }
                Err(error) => {
                    warn!(?error, "model switch failed");
                    inbox.push(Fragment::hitch(
                        error.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                }
            },
            Action::Activate(name) => match resources.enable(name.clone()) {
                Ok(()) => {
                    trace!(tool = %name, "activate");
                }
                Err(error) => {
                    warn!(?error, "tool activation failed");
                    inbox.push(Fragment::hitch(
                        error.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                }
            },
            Action::Deactivate(name) => {
                trace!(tool = %name, "deactivate");
                resources.disable(name);
            }
            Action::Take => {
                if let Some(frag) = inbox.pop() {
                    let id = ctx.append(frag);
                    let frag = ctx.get(id).expect("just taken");
                    hook!(
                        event = "taken",
                        id,
                        step,
                        role = role_name(frag.role),
                        kind = content_kind(frag),
                        preview = %preview(frag),
                    );
                }
            }
            Action::Done => {
                hook!(event = "done", step);
                return true;
            }
            Action::Halt => unreachable!("dispatch never receives Halt; apply() intercepts it"),
        }
        false
    }
}
