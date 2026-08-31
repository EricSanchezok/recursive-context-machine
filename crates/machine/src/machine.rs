use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::edit::{ContentSpec, EditOp, Position, Selector};
use crate::env::Environment;
use crate::event::{content_kind, preview, role_name};
use crate::fragment::{Fragment, Role};
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

/// Unique call ids for policy-initiated tool invocations.
static TOOL_CALL_SEQUENCE: AtomicU64 = AtomicU64::new(1);

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
        // Edit applies its ops inline (sequential semantics); every other
        // arm defers to the single tail application below.
        let mut effects_applied = false;

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
            Action::Edit { ops, .. } => {
                // Sequential per-op resolution and application: later ops
                // see the document produced by earlier ops of the same
                // batch. Effects accumulate in WAL order; replay re-applies
                // them in that order for identical results.
                self.apply_live_effects(
                    state,
                    &[Effect::ActionCounted {
                        action: action.name().to_string(),
                    }],
                );
                for op in ops.clone() {
                    let mut op_effects = Vec::new();
                    self.resolve_op(state, op, &mut op_effects);
                    self.apply_live_effects(state, &op_effects);
                    effects.extend(op_effects);
                }
                effects_applied = true;
            }
            Action::Tool { name, args, .. } => {
                let ExecutionMode::Live { tool_runtime, .. } = mode;
                let name = name.clone();
                let args = args.clone();
                let machine_id = self.id.to_string();
                let call_id = format!(
                    "policy-{}-{}",
                    step,
                    TOOL_CALL_SEQUENCE.fetch_add(1, Ordering::Relaxed)
                );
                hook!(
                    event = "tool_call",
                    machine_id,
                    call_id,
                    tool = name.as_str(),
                    arguments = %args,
                );
                let fragment = reactor::execute_single_tool(
                    &machine_id,
                    &name,
                    &call_id,
                    args,
                    &state.run.environment,
                    &state.run.resources,
                    tool_runtime,
                )
                .await;
                effects.push(Effect::ToolCompleted {
                    name: name.clone(),
                    call_id: call_id.clone(),
                    tokens: None,
                });
                effects.push(Effect::InboxPushed {
                    item: InboxItem::new(fragment, None),
                });
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
            Action::Done => {
                effects.push(Effect::StatusChanged {
                    status: MachineStatus::Done,
                });
            }
        }

        if !effects_applied {
            self.apply_live_effects(state, &effects);
        }
        // Request-assembly bookkeeping: every cell currently in the document
        // counts as seen this step (no WAL effect — derived observation).
        if effects
            .iter()
            .any(|effect| matches!(effect, Effect::CompletionRecorded { .. }))
        {
            let seen: Vec<u64> = state
                .run
                .context
                .fragments()
                .iter()
                .map(Fragment::id)
                .collect();
            state.run.context.note_seen(&seen, step, None);
        }
        let done = state.frame.status.is_done();
        StepResult {
            done,
            event: StoredEvent::new(step, action, effects),
        }
    }

    /// Resolve ONE edit op against the current state into effects. Reads
    /// only (the caller applies the effects before resolving the next op,
    /// giving batches sequential semantics). Per-op failures push hitches
    /// and never abort the remaining batch.
    fn resolve_op(&self, state: &MachineState, op: EditOp, effects: &mut Vec<Effect>) {
        match op {
            EditOp::Set { anchor, content } => {
                let item = consume_preview(state, &content);
                let Some(fragment) = content.resolve(item.clone().map(|entry| entry.fragment))
                else {
                    push_system_hitch_effect(
                        effects,
                        format!("edit op 'set' on '{anchor}' failed: inbox item unavailable"),
                    );
                    return;
                };
                let id = state
                    .run
                    .context
                    .find_anchor(&anchor)
                    .unwrap_or_else(|| state.run.context.next_id());
                let source_completion = item.as_ref().and_then(|entry| entry.source_completion);
                if let Some(item) = item {
                    effects.push(Effect::InboxConsumed {
                        call_id: call_id_of(&content),
                        item,
                    });
                }
                effects.push(Effect::ContextSet {
                    id,
                    anchor,
                    fragment,
                    source_completion,
                });
            }
            EditOp::Insert {
                position,
                content,
                anchor,
            } => {
                let item = consume_preview(state, &content);
                let Some(mut fragment) = content.resolve(item.clone().map(|entry| entry.fragment))
                else {
                    push_system_hitch_effect(
                        effects,
                        "edit op 'insert' failed: inbox item unavailable".to_string(),
                    );
                    return;
                };
                if let Some(anchor) = &anchor {
                    fragment.anchor = Some(anchor.clone());
                }
                let after = match resolve_position(&state.run.context, &position) {
                    Ok(after) => after,
                    Err(message) => {
                        push_system_hitch_effect(effects, message);
                        return;
                    }
                };
                let id = state.run.context.next_id();
                let source_completion = item.as_ref().and_then(|entry| entry.source_completion);
                if let Some(item) = item {
                    effects.push(Effect::InboxConsumed {
                        call_id: call_id_of(&content),
                        item,
                    });
                }
                effects.push(Effect::ContextInserted {
                    id,
                    after,
                    fragment,
                    source_completion,
                });
            }
            EditOp::Delete { selector } => match resolve_selector(&state.run.context, &selector) {
                Ok(ids) => {
                    let mut protected: Vec<String> = Vec::new();
                    for id in ids {
                        let is_protected = state
                            .run
                            .context
                            .get(id)
                            .and_then(|cell| cell.anchor.clone())
                            .is_some_and(|anchor| {
                                crate::context::PROTECTED_ANCHORS.contains(&anchor.as_str())
                            });
                        if is_protected {
                            if let Some(cell) = state.run.context.get(id) {
                                protected.push(cell.anchor.clone().unwrap_or_default());
                            }
                            continue;
                        }
                        effects.push(Effect::ContextRemoved { id });
                    }
                    if !protected.is_empty() {
                        push_system_hitch_effect(
                            effects,
                            format!("delete skipped protected anchors: {}", protected.join(", ")),
                        );
                    }
                }
                Err(message) => push_system_hitch_effect(effects, message),
            },
            EditOp::Move { anchor, after } => {
                let Some(id) = state.run.context.find_anchor(&anchor) else {
                    push_system_hitch_effect(
                        effects,
                        format!("move failed: anchor '{anchor}' not found"),
                    );
                    return;
                };
                match resolve_position(&state.run.context, &after) {
                    Ok(Some(after_id)) => {
                        effects.push(Effect::ContextMoved {
                            id,
                            after: after_id,
                        });
                    }
                    Ok(None) => {
                        push_system_hitch_effect(
                            effects,
                            "move failed: 'end' is not a valid move target".to_string(),
                        );
                    }
                    Err(message) => push_system_hitch_effect(effects, message),
                }
            }
        }
    }

    /// Apply tool-returned edit payloads outside the step loop (drain
    /// channel). Ops pass the same validation as Edit actions, applied with
    /// the same sequential semantics; effects nest under
    /// [`Effect::DrainEdits`] so one replay entry reproduces them.
    pub fn apply_drain_edits(&self, state: &mut MachineState, ops: Vec<EditOp>) -> Vec<Effect> {
        let mut outer = Vec::new();
        for op in ops {
            let mut op_effects = Vec::new();
            self.resolve_op(state, op, &mut op_effects);
            self.apply_live_effects(state, &op_effects);
            outer.extend(op_effects);
        }
        if outer.is_empty() {
            return Vec::new();
        }
        let effects = outer.clone();
        let drain = Effect::DrainEdits {
            ops: Vec::new(),
            effects,
        };
        vec![drain]
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
            Effect::ContextSet {
                id,
                anchor,
                fragment,
                source_completion,
            } => {
                let existed = state.run.context.find_anchor(anchor).is_some();
                state
                    .run
                    .context
                    .set_named_with_id(anchor, *id, fragment.clone());
                state.run.context.note_created(
                    *id,
                    state.frame.step,
                    source_completion.map(|c| c.0),
                );
                if let Some(completion_id) = source_completion {
                    state
                        .run
                        .telemetry
                        .record_output_fragment(*completion_id, *id);
                }
                if hook_mode.emits() {
                    let event = if existed { "replaced" } else { "inserted" };
                    self.hook_fragment(event, state.frame.step, *id, &state.run.context);
                }
            }
            Effect::ContextInserted {
                id,
                after,
                fragment,
                source_completion,
            } => {
                match after {
                    Some(after_id) => {
                        let _ = state
                            .run
                            .context
                            .insert_with_id(*after_id, *id, fragment.clone());
                    }
                    None => state.run.context.append_with_id(*id, fragment.clone()),
                }
                state.run.context.note_created(
                    *id,
                    state.frame.step,
                    source_completion.map(|c| c.0),
                );
                if let Some(completion_id) = source_completion {
                    state
                        .run
                        .telemetry
                        .record_output_fragment(*completion_id, *id);
                }
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
            Effect::ContextMoved { id, after } => {
                let _ = state.run.context.move_after(*id, *after);
                if hook_mode.emits() {
                    let machine_id = self.id.to_string();
                    hook!(
                        event = "moved",
                        machine_id = machine_id,
                        id,
                        after,
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
            Effect::InboxConsumed { call_id, .. } => {
                let popped = match call_id {
                    Some(call_id) => state.frame.inbox.pop_by_call_id(call_id),
                    None => state.frame.inbox.pop(),
                };
                if hook_mode.emits() && popped.is_some() {
                    let machine_id = self.id.to_string();
                    hook!(
                        event = "consumed",
                        machine_id = machine_id,
                        call_id = call_id.as_deref().unwrap_or(""),
                        step = state.frame.step
                    );
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
            Effect::ToolCompleted { name, call_id, .. } => {
                state.run.telemetry.count_action(format!("tool:{name}"));
                if hook_mode.emits() {
                    let machine_id = self.id.to_string();
                    hook!(
                        event = "tool_completed",
                        machine_id = machine_id,
                        call_id = call_id.as_str(),
                        tool = name.as_str(),
                        step = state.frame.step
                    );
                }
            }
            Effect::DrainEdits { effects, .. } => {
                self.apply_effects(state, effects, hook_mode);
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

/// Preview the inbox item a consume-op would take (no mutation).
fn consume_preview(state: &MachineState, content: &ContentSpec) -> Option<InboxItem> {
    match content {
        ContentSpec::Inbox { call_id } => state.frame.inbox.find_item(call_id.as_deref()),
        ContentSpec::Literal { .. } => None,
    }
}

fn call_id_of(content: &ContentSpec) -> Option<String> {
    match content {
        ContentSpec::Inbox { call_id } => call_id.clone(),
        ContentSpec::Literal { .. } => None,
    }
}

/// Resolve an insert/move position to an "after" cell id. `End` resolves to
/// None (append). Anchors and ids must exist.
fn resolve_position(context: &Context, position: &Position) -> Result<Option<u64>, String> {
    match position {
        Position::End => Ok(None),
        Position::Id(id) => context
            .get(*id)
            .map(|_| Some(*id))
            .ok_or_else(|| format!("cell id {id} not found in context")),
        Position::Anchor(anchor) => context
            .find_anchor(anchor)
            .map(Some)
            .ok_or_else(|| format!("anchor '{anchor}' not found in context")),
    }
}

/// Resolve a delete selector to an ordered, deduplicated id set.
fn resolve_selector(context: &Context, selector: &Selector) -> Result<Vec<u64>, String> {
    match selector {
        Selector::Anchor(anchor) => {
            let id = context
                .find_anchor(anchor)
                .ok_or_else(|| format!("anchor '{anchor}' not found in context"))?;
            Ok(vec![id])
        }
        Selector::Id(id) => context
            .get(*id)
            .map(|_| vec![*id])
            .ok_or_else(|| format!("cell id {id} not found in context")),
        Selector::Range { from, to } => {
            let from_position = resolve_position(context, from)?
                .and_then(|id| context.position_of(id))
                .ok_or_else(|| "range 'from' cannot resolve to a cell".to_string())?;
            let to_position = resolve_position(context, to)?
                .and_then(|id| context.position_of(id))
                .ok_or_else(|| "range 'to' cannot resolve to a cell".to_string())?;
            let (start, end) = if from_position <= to_position {
                (from_position, to_position)
            } else {
                (to_position, from_position)
            };
            Ok(context.fragments()[start..=end]
                .iter()
                .map(Fragment::id)
                .collect())
        }
        Selector::Where(predicate) => {
            let mut matches: Vec<u64> = Vec::new();
            for cell in context.fragments() {
                if predicate_matches(predicate, cell) {
                    matches.push(cell.id());
                }
            }
            if let Some(skip) = predicate.skip_newest {
                let skip = skip as usize;
                if matches.len() > skip {
                    matches.truncate(matches.len() - skip);
                } else {
                    matches.clear();
                }
            }
            Ok(matches)
        }
    }
}

fn predicate_matches(predicate: &crate::edit::CellPredicate, cell: &Fragment) -> bool {
    if let Some(role) = &predicate.role
        && !role.eq_ignore_ascii_case(&format!("{:?}", cell.role))
    {
        return false;
    }
    if let Some(tag) = &predicate.tag
        && cell.tag != *tag
    {
        return false;
    }
    if let Some(kind) = &predicate.kind
        && !kind.eq_ignore_ascii_case(content_kind_of(cell))
    {
        return false;
    }
    if let Some(bytes) = predicate.bytes_gt
        && Context::cell_bytes(cell) <= bytes
    {
        return false;
    }
    true
}

fn content_kind_of(cell: &Fragment) -> &str {
    content_kind(cell)
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
    let fragment = Fragment::hitch(message, None, Role::System, None::<&str>);
    effects.push(Effect::InboxPushed {
        item: InboxItem::new(fragment, None),
    });
}
