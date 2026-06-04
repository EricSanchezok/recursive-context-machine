use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::env::Environment;
use crate::event::{content_kind, preview, role_name};
use crate::fragment::Fragment;
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineState {
    pub context: Context,
    pub environment: Environment,
    pub resources: Resources,
    pub inbox: Inbox,
    pub usages: Vec<Usage>,
    pub counts: HashMap<String, u64>,
    pub step: u64,
    pub done: bool,
}

impl Default for MachineState {
    fn default() -> Self {
        Self {
            context: Context::new(),
            environment: Environment::new("."),
            resources: Resources::new(),
            inbox: Inbox::new(),
            usages: Vec::new(),
            counts: HashMap::new(),
            step: 0,
            done: false,
        }
    }
}

pub struct ApplyContext<'a> {
    pub ctx: &'a mut Context,
    pub env: &'a mut Environment,
    pub resources: &'a mut Resources,
    pub inbox: &'a mut Inbox,
    pub usages: &'a mut Vec<Usage>,
    pub counts: &'a mut HashMap<String, u64>,
}

pub enum ApplyMode<'a> {
    Live { tool_runtime: &'a ToolRuntime },
    Replay { cached_outcome: ActionOutcome },
}

impl Machine {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: MachineId::from_raw(id.into()).unwrap_or_else(|_| MachineId::new()),
            name: Name::new(name).unwrap_or_else(|_| Name::from_static("rcm")),
        }
    }

    pub async fn apply(
        &mut self,
        action: Action,
        step: u64,
        context: ApplyContext<'_>,
        mode: ApplyMode<'_>,
    ) -> ApplyResult {
        let ApplyContext {
            ctx,
            env,
            resources,
            inbox,
            usages,
            counts,
        } = context;

        let is_replay = matches!(mode, ApplyMode::Replay { .. });
        *counts.entry(action.name().to_string()).or_default() += 1;
        let machine_id = self.id.to_string();
        let mut inbox_fragments = Vec::new();

        if let Action::Halt = &action {
            let outcome = match mode {
                ApplyMode::Replay { cached_outcome } => cached_outcome,
                ApplyMode::Live { tool_runtime } => {
                    let model_name = resources
                        .active_model()
                        .map(|model| model.name.as_str())
                        .unwrap_or("none");
                    hook!(
                        event = "halt",
                        machine_id,
                        step,
                        model = %model_name,
                        messages = ctx.fragments().len(),
                        tools = resources.active_tools.len(),
                    );
                    reactor::react(&machine_id, ctx, env, resources, tool_runtime).await
                }
            };
            if let ActionOutcome::Reactor { fragments, usage } = &outcome {
                inbox.extend(fragments.iter().cloned());
                usages.push(usage.clone());
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
                if !is_replay {
                    let fragment = ctx.get(id).expect("just appended");
                    hook!(
                        event = "appended",
                        machine_id,
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
            Action::Insert { after, fragment } => match ctx.insert(*after, fragment.clone()) {
                Ok(id) => {
                    if !is_replay {
                        let fragment = ctx.get(id).expect("just inserted");
                        hook!(
                            event = "inserted",
                            machine_id,
                            id,
                            after,
                            step,
                            role = role_name(fragment.role),
                            kind = content_kind(fragment),
                            tag = fragment.tag.as_str(),
                            preview = %preview(fragment),
                        );
                    }
                    false
                }
                Err(error) => {
                    push_system_hitch(inbox, &mut inbox_fragments, error.to_string());
                    false
                }
            },
            Action::Replace { id, fragment } => {
                if let Err(error) = ctx.replace(*id, fragment.clone()) {
                    push_system_hitch(inbox, &mut inbox_fragments, error.to_string());
                } else if !is_replay {
                    let fragment = ctx.get(*id).expect("just replaced");
                    hook!(
                        event = "replaced",
                        machine_id,
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
                    push_system_hitch(inbox, &mut inbox_fragments, error.to_string());
                } else if !is_replay {
                    hook!(event = "removed", machine_id = machine_id, id, step);
                }
                false
            }
            Action::Swap(first_id, second_id) => {
                if let Err(error) = ctx.swap(*first_id, *second_id) {
                    push_system_hitch(inbox, &mut inbox_fragments, error.to_string());
                } else if !is_replay {
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
                    push_system_hitch(inbox, &mut inbox_fragments, error.to_string());
                } else if !is_replay {
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
                    push_system_hitch(inbox, &mut inbox_fragments, error.to_string());
                } else if !is_replay {
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
                if !is_replay {
                    hook!(
                        event = "deactivate",
                        machine_id = machine_id,
                        name = name.as_str(),
                        step
                    );
                }
                false
            }
            Action::Take => {
                if let Some(fragment) = inbox.pop() {
                    let id = ctx.append(fragment);
                    if let Some(usage) = usages.last_mut() {
                        usage.fragment_ids.push(id)
                    }
                    if !is_replay {
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
                }
                false
            }
            Action::Done => {
                if !is_replay {
                    hook!(event = "done", machine_id = machine_id, step);
                }
                true
            }
            Action::Halt => unreachable!("halt handled before state-only actions"),
        };

        ApplyResult {
            done,
            event: MachineEvent::state(step, action, inbox_fragments),
        }
    }

    pub async fn apply_state(
        &mut self,
        action: Action,
        state: &mut MachineState,
        mode: ApplyMode<'_>,
    ) -> ApplyResult {
        state.step += 1;
        let step = state.step;
        let result = self
            .apply(
                action,
                step,
                ApplyContext {
                    ctx: &mut state.context,
                    env: &mut state.environment,
                    resources: &mut state.resources,
                    inbox: &mut state.inbox,
                    usages: &mut state.usages,
                    counts: &mut state.counts,
                },
                mode,
            )
            .await;
        state.done = result.done;
        result
    }
}

fn push_system_hitch(inbox: &mut Inbox, recorded: &mut Vec<Fragment>, message: String) {
    let fragment = Fragment::hitch(message, None, crate::fragment::Role::System, None::<&str>);
    inbox.push(fragment.clone());
    recorded.push(fragment);
}
