use crate::context::Context;
use crate::env::Environment;
use crate::event::{content_kind, preview, role_name};
use crate::hook;
use crate::inbox::Inbox;
use crate::policy::Action;
use crate::reactor;
use crate::resources::Resources;
use crate::usage::Usage;
use tracing::trace;
use utils::{MachineId, Name};

pub struct Machine {
    pub id: MachineId,
    pub name: Name,
    pub usages: Vec<Usage>,
}

impl Machine {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: MachineId::from_raw(id.into()).unwrap_or_else(|_| MachineId::new()),
            name: Name::new(name).unwrap_or_else(|_| Name::from_static("rcm")),
            usages: Vec::new(),
        }
    }

    pub async fn apply(
        &mut self,
        action: Action,
        step: u64,
        ctx: &mut Context,
        env: &mut Environment,
        resources: &mut Resources,
        inbox: &mut Inbox,
    ) -> bool {
        let mid = self.id.to_string();
        if let Action::Halt = &action {
            hook!(
                event = "halt",
                machine_id = mid,
                step,
                model = %resources.active_model().name,
                messages = ctx.fragments().len(),
                tools = resources.active_tools.len(),
            );
            let usage = reactor::react(&mid, ctx, env, resources, inbox).await;
            self.usages.push(usage);
            return false;
        }
        match action {
            Action::Append(frag) => {
                let id = ctx.append(frag);
                let frag = ctx.get(id).expect("just appended");
                hook!(
                    event = "appended",
                    machine_id = mid,
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
                    machine_id = mid,
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
                    machine_id = mid,
                    id,
                    step,
                    role = role_name(frag.role),
                    kind = content_kind(frag),
                    preview = %preview(frag),
                );
            }
            Action::Remove(id) => {
                ctx.remove(id);
                hook!(event = "removed", machine_id = mid, id, step);
            }
            Action::Swap(id1, id2) => {
                ctx.swap(id1, id2);
                hook!(event = "swapped", machine_id = mid, id1, id2, step);
            }
            Action::Model(name) => {
                trace!(model = %name, "switch model");
                resources.use_model(name);
            }
            Action::Activate(name) => {
                trace!(tool = %name, "activate");
                resources.enable(name);
            }
            Action::Deactivate(name) => {
                trace!(tool = %name, "deactivate");
                resources.disable(name);
            }
            Action::Take => {
                if let Some(frag) = inbox.pop() {
                    let id = ctx.append(frag);
                    self.usages.last_mut().map(|u| u.fragment_ids.push(id));
                    let frag = ctx.get(id).expect("just taken");
                    hook!(
                        event = "taken",
                        machine_id = mid,
                        id,
                        step,
                        role = role_name(frag.role),
                        kind = content_kind(frag),
                        preview = %preview(frag),
                    );
                }
            }
            Action::Done => {
                hook!(event = "done", machine_id = mid, step);
                return true;
            }
            _ => {}
        }
        false
    }
}
