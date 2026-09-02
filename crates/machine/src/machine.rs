use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::env::Environment;
use crate::event::{content_kind, preview, role_name};
use crate::fragment::Fragment;
use crate::hook;
use crate::inbox::{Inbox, InboxItem};
use crate::policy::Action;
use crate::purpose::Purpose;
use crate::reactor;
use crate::record::{Effect, StepResult, StoredEvent};
use crate::resources::Resources;
use crate::tool::ToolRuntime;
use crate::usage::{CompletionRecord, Telemetry};
use utils::{MachineId, Name};

pub struct Machine {
    pub id: MachineId,
    pub name: Name,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunState {
    pub purpose: Purpose,
    pub run_dir: Option<PathBuf>,
    pub context: Context,
    pub environment: Environment,
    pub resources: Resources,
    pub telemetry: Telemetry,
}

impl Default for RunState {
    fn default() -> Self {
        Self {
            purpose: Purpose::default(),
            run_dir: None,
            context: Context::new(),
            environment: Environment::new("."),
            resources: Resources::new(),
            telemetry: Telemetry::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MachineState {
    pub run: RunState,
    pub frame: MachineFrame,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineFrame {
    pub inbox: Inbox,
    pub step: u64,
    pub status: MachineStatus,
}

impl Default for MachineFrame {
    fn default() -> Self {
        Self {
            inbox: Inbox::new(),
            step: 0,
            status: MachineStatus::Running,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MachineStatus {
    Running,
    Done,
}

impl MachineStatus {
    pub fn is_done(self) -> bool {
        self == Self::Done
    }
}

pub enum ExecutionMode<'a> {
    Live {
        tool_runtime: &'a ToolRuntime,
        overlay: &'a crate::overlay::Overlay,
    },
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
        state: &mut MachineState,
        mode: ExecutionMode<'_>,
    ) -> StepResult {
        state.frame.step += 1;
        let step = state.frame.step;
        let mut effects = vec![Effect::ActionCounted {
            action: action.name().to_string(),
        }];

        match &action {
            Action::Halt => {
                let ExecutionMode::Live {
                    tool_runtime,
                    overlay,
                } = mode;
                let machine_id = self.id.to_string();
                let model_name = state
                    .run
                    .resources
                    .active_model()
                    .map(|model| model.name.as_str())
                    .unwrap_or("none");
                hook!(
                    event = "halt",
                    machine_id,
                    step,
                    model = %model_name,
                    messages = state.run.context.fragments().len(),
                    tools = state.run.resources.active_tools.len(),
                );

                let (fragments, tokens) = reactor::react(
                    &machine_id,
                    &state.run.context,
                    &state.run.environment,
                    &state.run.resources,
                    tool_runtime,
                    overlay,
                )
                .await;
                let completion_id = state.run.telemetry.next_completion_id();
                let record = CompletionRecord {
                    id: completion_id,
                    step,
                    model: state
                        .run
                        .resources
                        .active_model()
                        .map(|model| model.name.clone()),
                    tokens,
                    output_fragment_ids: Vec::new(),
                };
                let inbox_items = fragments
                    .into_iter()
                    .map(|fragment| InboxItem::new(fragment, Some(completion_id)))
                    .collect();
                effects.push(Effect::CompletionRecorded {
                    record,
                    inbox_items,
                });
            }
            Action::Append(fragment) => {
                effects.push(Effect::ContextAppended {
                    id: state.run.context.next_id(),
                    fragment: fragment.clone(),
                });
            }
            Action::Insert { after, fragment } => {
                if state.run.context.get(*after).is_some() {
                    effects.push(Effect::ContextInserted {
                        id: state.run.context.next_id(),
                        after: *after,
                        fragment: fragment.clone(),
                    });
                } else {
                    push_system_hitch_effect(
                        &mut effects,
                        format!("fragment id {} not found in context", after),
                    );
                }
            }
            Action::Replace { id, fragment } => {
                if state.run.context.get(*id).is_some() {
                    effects.push(Effect::ContextReplaced {
                        id: *id,
                        fragment: fragment.clone(),
                    });
                } else {
                    push_system_hitch_effect(
                        &mut effects,
                        format!("fragment id {} not found in context", id),
                    );
                }
            }
            Action::Remove(id) => {
                if state.run.context.get(*id).is_some() {
                    effects.push(Effect::ContextRemoved { id: *id });
                } else {
                    push_system_hitch_effect(
                        &mut effects,
                        format!("fragment id {} not found in context", id),
                    );
                }
            }
            Action::Swap(first_id, second_id) => {
                if state.run.context.get(*first_id).is_none() {
                    push_system_hitch_effect(
                        &mut effects,
                        format!("fragment id {} not found in context", first_id),
                    );
                } else if state.run.context.get(*second_id).is_none() {
                    push_system_hitch_effect(
                        &mut effects,
                        format!("fragment id {} not found in context", second_id),
                    );
                } else {
                    effects.push(Effect::ContextSwapped {
                        first: *first_id,
                        second: *second_id,
                    });
                }
            }
            Action::Model(name) => {
                if state.run.resources.models.contains_key(name) {
                    effects.push(Effect::ModelSelected { name: name.clone() });
                } else {
                    push_system_hitch_effect(
                        &mut effects,
                        format!("model '{}' not registered", name),
                    );
                }
            }
            Action::Activate(name) => {
                if state.run.resources.tool_definitions.contains_key(name) {
                    effects.push(Effect::ToolActivated { name: name.clone() });
                } else {
                    push_system_hitch_effect(
                        &mut effects,
                        format!("tool '{}' not registered", name),
                    );
                }
            }
            Action::Deactivate(name) => {
                effects.push(Effect::ToolDeactivated { name: name.clone() });
            }
            Action::Take => {
                if let Some(item) = state.frame.inbox.peek() {
                    effects.push(Effect::InboxTaken {
                        source_completion: item.source_completion,
                        fragment_id: state.run.context.next_id(),
                    });
                }
            }
            Action::Done => {
                effects.push(Effect::StatusChanged {
                    status: MachineStatus::Done,
                });
            }
        }

        self.apply_live_effects(state, &effects);
        let done = state.frame.status.is_done();
        StepResult {
            done,
            event: StoredEvent::new(step, action, effects),
        }
    }

    pub fn replay_effects(&self, state: &mut MachineState, effects: &[Effect]) {
        self.apply_effects(state, effects, HookMode::Suppress);
    }

    fn apply_live_effects(&self, state: &mut MachineState, effects: &[Effect]) {
        self.apply_effects(state, effects, HookMode::Emit);
    }

    fn apply_effects(&self, state: &mut MachineState, effects: &[Effect], hook_mode: HookMode) {
        for effect in effects {
            self.apply_effect(state, effect, hook_mode);
        }
    }

    fn apply_effect(&self, state: &mut MachineState, effect: &Effect, hook_mode: HookMode) {
        match effect {
            Effect::ActionCounted { action } => state.run.telemetry.count_action(action.clone()),
            Effect::ContextAppended { id, fragment } => {
                state.run.context.append_with_id(*id, fragment.clone());
                if hook_mode.emits() {
                    self.hook_fragment("appended", state.frame.step, *id, &state.run.context);
                }
            }
            Effect::ContextInserted {
                id,
                after,
                fragment,
            } => {
                let _ = state
                    .run
                    .context
                    .insert_with_id(*after, *id, fragment.clone());
                if hook_mode.emits() {
                    self.hook_fragment("inserted", state.frame.step, *id, &state.run.context);
                }
            }
            Effect::ContextReplaced { id, fragment } => {
                let _ = state.run.context.replace(*id, fragment.clone());
                if hook_mode.emits() {
                    self.hook_fragment("replaced", state.frame.step, *id, &state.run.context);
                }
            }
            Effect::ContextRemoved { id } => {
                let _ = state.run.context.remove(*id);
                if hook_mode.emits() {
                    let machine_id = self.id.to_string();
                    hook!(
                        event = "removed",
                        machine_id = machine_id,
                        id,
                        step = state.frame.step
                    );
                }
            }
            Effect::ContextSwapped { first, second } => {
                let _ = state.run.context.swap(*first, *second);
                if hook_mode.emits() {
                    let machine_id = self.id.to_string();
                    hook!(
                        event = "swapped",
                        machine_id = machine_id,
                        id1 = first,
                        id2 = second,
                        step = state.frame.step
                    );
                }
            }
            Effect::ModelSelected { name } => {
                let _ = state.run.resources.use_model(name);
                if hook_mode.emits() {
                    let machine_id = self.id.to_string();
                    hook!(
                        event = "model",
                        machine_id = machine_id,
                        name = name.as_str(),
                        step = state.frame.step
                    );
                }
            }
            Effect::ToolActivated { name } => {
                let _ = state.run.resources.enable(name);
                if hook_mode.emits() {
                    let machine_id = self.id.to_string();
                    hook!(
                        event = "activate",
                        machine_id = machine_id,
                        name = name.as_str(),
                        step = state.frame.step
                    );
                }
            }
            Effect::ToolDeactivated { name } => {
                state.run.resources.disable(name);
                if hook_mode.emits() {
                    let machine_id = self.id.to_string();
                    hook!(
                        event = "deactivate",
                        machine_id = machine_id,
                        name = name.as_str(),
                        step = state.frame.step
                    );
                }
            }
            Effect::InboxPushed { item } => state.frame.inbox.push_item(item.clone()),
            Effect::CompletionRecorded {
                record,
                inbox_items,
            } => {
                state.run.telemetry.completions.push(record.clone());
                state.frame.inbox.extend_items(inbox_items.clone());
            }
            Effect::InboxTaken {
                source_completion,
                fragment_id,
            } => {
                if let Some(item) = state.frame.inbox.pop() {
                    state
                        .run
                        .context
                        .append_with_id(*fragment_id, item.fragment);
                    if let Some(completion_id) = source_completion {
                        state
                            .run
                            .telemetry
                            .record_output_fragment(*completion_id, *fragment_id);
                    }
                    if hook_mode.emits() {
                        self.hook_fragment(
                            "taken",
                            state.frame.step,
                            *fragment_id,
                            &state.run.context,
                        );
                    }
                }
            }
            Effect::StatusChanged { status } => {
                state.frame.status = *status;
                if status.is_done() && hook_mode.emits() {
                    let machine_id = self.id.to_string();
                    hook!(
                        event = "done",
                        machine_id = machine_id,
                        step = state.frame.step
                    );
                }
            }
        }
    }

    fn hook_fragment(&self, event: &'static str, step: u64, id: u64, context: &Context) {
        let Some(fragment) = context.get(id) else {
            return;
        };
        let machine_id = self.id.to_string();
        hook!(
            event = event,
            machine_id,
            id,
            step,
            role = role_name(fragment.role),
            kind = content_kind(fragment),
            tag = fragment.tag.as_str(),
            preview = %preview(fragment),
        );
    }
}

#[derive(Clone, Copy)]
enum HookMode {
    Emit,
    Suppress,
}

impl HookMode {
    fn emits(self) -> bool {
        matches!(self, Self::Emit)
    }
}

fn push_system_hitch_effect(effects: &mut Vec<Effect>, message: String) {
    let fragment = Fragment::hitch(message, None, crate::fragment::Role::System, None::<&str>);
    effects.push(Effect::InboxPushed {
        item: InboxItem::new(fragment, None),
    });
}
