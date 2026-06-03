use std::collections::HashMap;

use crate::context::Context;
use crate::env::Environment;
use crate::event::{content_kind, preview, role_name};
use crate::fragment::{Fragment, Role};
use crate::hook;
use crate::inbox::Inbox;
use crate::policy::Action;
use crate::reactor;
use crate::resources::Resources;
use crate::usage::Usage;
use utils::{MachineId, Name};

pub struct Machine {
    pub id: MachineId,
    pub name: Name,
    pub usages: Vec<Usage>,
    pub counts: HashMap<&'static str, u64>,
}

impl Machine {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: MachineId::from_raw(id.into()).unwrap_or_else(|_| MachineId::new()),
            name: Name::new(name).unwrap_or_else(|_| Name::from_static("rcm")),
            usages: Vec::new(),
            counts: HashMap::new(),
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
        *self.counts.entry(action.name()).or_default() += 1;
        let mid = self.id.to_string();

        if let Action::Halt = &action {
            let model_name = resources
                .active_model()
                .map(|m| m.name.as_str())
                .unwrap_or("none");
            hook!(
                event = "halt",
                machine_id = mid,
                step,
                model = %model_name,
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
                    tag = frag.tag.as_str(),
                    preview = %preview(frag),
                );
            }
            Action::Insert { after, fragment } => match ctx.insert(after, fragment) {
                Ok(id) => {
                    let frag = ctx.get(id).expect("just inserted");
                    hook!(
                        event = "inserted",
                        machine_id = mid,
                        id,
                        after,
                        step,
                        role = role_name(frag.role),
                        kind = content_kind(frag),
                        tag = frag.tag.as_str(),
                        preview = %preview(frag),
                    );
                }
                Err(err) => {
                    inbox.push(Fragment::hitch(
                        err.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                }
            },
            Action::Replace { id, fragment } => {
                if let Err(err) = ctx.replace(id, fragment) {
                    inbox.push(Fragment::hitch(
                        err.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                } else {
                    let frag = ctx.get(id).expect("just replaced");
                    hook!(
                        event = "replaced",
                        machine_id = mid,
                        id,
                        step,
                        role = role_name(frag.role),
                        kind = content_kind(frag),
                        tag = frag.tag.as_str(),
                        preview = %preview(frag),
                    );
                }
            }
            Action::Remove(id) => {
                if let Err(err) = ctx.remove(id) {
                    inbox.push(Fragment::hitch(
                        err.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                } else {
                    hook!(event = "removed", machine_id = mid, id, step);
                }
            }
            Action::Swap(id1, id2) => {
                if let Err(err) = ctx.swap(id1, id2) {
                    inbox.push(Fragment::hitch(
                        err.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                } else {
                    hook!(event = "swapped", machine_id = mid, id1, id2, step);
                }
            }
            Action::Model(name) => {
                if let Err(err) = resources.use_model(&name) {
                    inbox.push(Fragment::hitch(
                        err.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                } else {
                    hook!(
                        event = "model",
                        machine_id = mid,
                        name = name.as_str(),
                        step
                    );
                }
            }
            Action::Activate(name) => {
                if let Err(err) = resources.enable(&name) {
                    inbox.push(Fragment::hitch(
                        err.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                } else {
                    hook!(
                        event = "activate",
                        machine_id = mid,
                        name = name.as_str(),
                        step
                    );
                }
            }
            Action::Deactivate(name) => {
                resources.disable(&name);
                hook!(
                    event = "deactivate",
                    machine_id = mid,
                    name = name.as_str(),
                    step
                );
            }
            Action::Take => {
                if let Some(frag) = inbox.pop() {
                    let id = ctx.append(frag);
                    if let Some(u) = self.usages.last_mut() {
                        u.fragment_ids.push(id)
                    }
                    let frag = ctx.get(id).expect("just taken");
                    hook!(
                        event = "taken",
                        machine_id = mid,
                        id,
                        step,
                        role = role_name(frag.role),
                        kind = content_kind(frag),
                        tag = frag.tag.as_str(),
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
