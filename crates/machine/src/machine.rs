use std::collections::HashMap;

use crate::context::Context;
use crate::env::Environment;
use crate::event::{content_kind, preview, role_name};
use crate::fragment::{Fragment, Role};
use crate::hook;
use crate::inbox::Inbox;
use crate::policy::Action;
use crate::reactor;
use crate::record::{ActionOutcome, ApplyResult, MachineEvent};
use crate::resources::Resources;
use crate::tool::ToolRuntime;
use crate::usage::Usage;
use utils::{MachineId, Name};

pub struct Machine {
    pub id: MachineId,
    pub name: Name,
    pub usages: Vec<Usage>,
    pub counts: HashMap<String, u64>,
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
        tool_runtime: &ToolRuntime,
        inbox: &mut Inbox,
    ) -> ApplyResult {
        *self.counts.entry(action.name().to_string()).or_default() += 1;
        let machine_id = self.id.to_string();

        if let Action::Halt = &action {
            let model_name = resources
                .active_model()
                .map(|model| model.name.as_str())
                .unwrap_or("none");
            hook!(
                event = "halt",
                machine_id = machine_id,
                step,
                model = %model_name,
                messages = ctx.fragments().len(),
                tools = resources.active_tools.len(),
            );
            let outcome = reactor::react(&machine_id, ctx, env, resources, tool_runtime).await;
            if let ActionOutcome::Reactor { fragments, usage } = &outcome {
                inbox.extend(fragments.iter().cloned());
                self.usages.push(usage.clone());
            }
            return ApplyResult {
                done: false,
                event: MachineEvent {
                    step,
                    action,
                    outcome,
                },
            };
        }

        let done = match &action {
            Action::Append(fragment) => {
                let id = ctx.append(fragment.clone());
                let fragment = ctx.get(id).expect("just appended");
                hook!(
                    event = "appended",
                    machine_id = machine_id,
                    id,
                    step,
                    role = role_name(fragment.role),
                    kind = content_kind(fragment),
                    tag = fragment.tag.as_str(),
                    preview = %preview(fragment),
                );
                false
            }
            Action::Insert { after, fragment } => match ctx.insert(*after, fragment.clone()) {
                Ok(id) => {
                    let fragment = ctx.get(id).expect("just inserted");
                    hook!(
                        event = "inserted",
                        machine_id = machine_id,
                        id,
                        after,
                        step,
                        role = role_name(fragment.role),
                        kind = content_kind(fragment),
                        tag = fragment.tag.as_str(),
                        preview = %preview(fragment),
                    );
                    false
                }
                Err(error) => {
                    inbox.push(Fragment::hitch(
                        error.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                    false
                }
            },
            Action::Replace { id, fragment } => {
                if let Err(error) = ctx.replace(*id, fragment.clone()) {
                    inbox.push(Fragment::hitch(
                        error.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                } else {
                    let fragment = ctx.get(*id).expect("just replaced");
                    hook!(
                        event = "replaced",
                        machine_id = machine_id,
                        id,
                        step,
                        role = role_name(fragment.role),
                        kind = content_kind(fragment),
                        tag = fragment.tag.as_str(),
                        preview = %preview(fragment),
                    );
                }
                false
            }
            Action::Remove(id) => {
                if let Err(error) = ctx.remove(*id) {
                    inbox.push(Fragment::hitch(
                        error.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                } else {
                    hook!(event = "removed", machine_id = machine_id, id, step);
                }
                false
            }
            Action::Swap(first_id, second_id) => {
                if let Err(error) = ctx.swap(*first_id, *second_id) {
                    inbox.push(Fragment::hitch(
                        error.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                } else {
                    hook!(
                        event = "swapped",
                        machine_id = machine_id,
                        id1 = first_id,
                        id2 = second_id,
                        step
                    );
                }
                false
            }
            Action::Model(name) => {
                if let Err(error) = resources.use_model(name) {
                    inbox.push(Fragment::hitch(
                        error.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                } else {
                    hook!(
                        event = "model",
                        machine_id = machine_id,
                        name = name.as_str(),
                        step
                    );
                }
                false
            }
            Action::Activate(name) => {
                if let Err(error) = resources.enable(name) {
                    inbox.push(Fragment::hitch(
                        error.to_string(),
                        None,
                        Role::System,
                        None::<&str>,
                    ));
                } else {
                    hook!(
                        event = "activate",
                        machine_id = machine_id,
                        name = name.as_str(),
                        step
                    );
                }
                false
            }
            Action::Deactivate(name) => {
                resources.disable(name);
                hook!(
                    event = "deactivate",
                    machine_id = machine_id,
                    name = name.as_str(),
                    step
                );
                false
            }
            Action::Take => {
                if let Some(fragment) = inbox.pop() {
                    let id = ctx.append(fragment);
                    if let Some(usage) = self.usages.last_mut() {
                        usage.fragment_ids.push(id)
                    }
                    let fragment = ctx.get(id).expect("just taken");
                    hook!(
                        event = "taken",
                        machine_id = machine_id,
                        id,
                        step,
                        role = role_name(fragment.role),
                        kind = content_kind(fragment),
                        tag = fragment.tag.as_str(),
                        preview = %preview(fragment),
                    );
                }
                false
            }
            Action::Done => {
                hook!(event = "done", machine_id = machine_id, step);
                true
            }
            Action::Halt => unreachable!("halt handled before state-only actions"),
        };

        ApplyResult {
            done,
            event: MachineEvent::state_only(step, action),
        }
    }
}
