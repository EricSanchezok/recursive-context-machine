use machine::{Context, Environment, Fragment, Resources, Role, RunState};
use utils::{FluxId, Name};

use crate::wire::Channel;

#[derive(Clone)]
pub enum PurposeFlux {
    Concat,
}

#[derive(Clone)]
pub enum ContextFlux {
    Append,
    Last,
    Digest,
    Thread,
}

#[derive(Clone)]
pub enum EnvFlux {
    Overlay,
}

#[derive(Clone)]
pub enum ResFlux {
    Merge,
}

#[derive(Clone, Copy)]
pub enum BridgeKind {
    ContextToPurpose,
}

#[derive(Clone)]
pub enum FluxMode {
    Purpose(PurposeFlux),
    Context(ContextFlux),
    Environment(EnvFlux),
    Resources(ResFlux),
    Bridge {
        from: Channel,
        to: Channel,
        kind: BridgeKind,
    },
}

impl FluxMode {
    pub fn input_channel(&self) -> Channel {
        match self {
            FluxMode::Purpose(_) => Channel::Purpose,
            FluxMode::Context(_) => Channel::Context,
            FluxMode::Environment(_) => Channel::Environment,
            FluxMode::Resources(_) => Channel::Resources,
            FluxMode::Bridge { from, .. } => *from,
        }
    }

    pub fn output_channel(&self) -> Channel {
        match self {
            FluxMode::Purpose(_) => Channel::Purpose,
            FluxMode::Context(_) => Channel::Context,
            FluxMode::Environment(_) => Channel::Environment,
            FluxMode::Resources(_) => Channel::Resources,
            FluxMode::Bridge { to, .. } => *to,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            FluxMode::Purpose(PurposeFlux::Concat) => "purpose_concat",
            FluxMode::Context(ContextFlux::Append) => "context_append",
            FluxMode::Context(ContextFlux::Last) => "context_last",
            FluxMode::Context(ContextFlux::Digest) => "context_digest",
            FluxMode::Context(ContextFlux::Thread) => "context_thread",
            FluxMode::Environment(EnvFlux::Overlay) => "environment_overlay",
            FluxMode::Resources(ResFlux::Merge) => "resources_merge",
            FluxMode::Bridge {
                kind: BridgeKind::ContextToPurpose,
                ..
            } => "bridge_context_to_purpose",
        }
    }
}

#[derive(Clone)]
pub struct Flux {
    id: FluxId,
    pub name: Name,
    pub mode: FluxMode,
    pub arity: usize,
}

impl Flux {
    pub fn new(name: impl Into<String>, mode: FluxMode, arity: usize) -> Self {
        Self {
            id: FluxId::new(),
            name: Name::new(name).expect("flux name must be valid"),
            mode,
            arity,
        }
    }

    pub fn id(&self) -> &FluxId {
        &self.id
    }

    pub fn apply(&self, slots: &[RunState]) -> RunState {
        let mut state = RunState::default();
        match &self.mode {
            FluxMode::Purpose(_) => {
                state.purpose.text = apply_purpose(self, |slot| slots[slot].purpose.text.clone())
            }
            FluxMode::Context(_) => {
                state.context = apply_context(self, |slot| slots[slot].context.clone())
            }
            FluxMode::Environment(_) => {
                state.environment = apply_environment(self, |slot| slots[slot].environment.clone())
            }
            FluxMode::Resources(_) => {
                state.resources = apply_resources(self, |slot| slots[slot].resources.clone())
            }
            FluxMode::Bridge { kind, .. } => match kind {
                BridgeKind::ContextToPurpose => state.purpose.text = flatten_context_to_text(slots),
            },
        }
        state
    }
}

fn flatten_context_to_text(slots: &[RunState]) -> String {
    let mut parts = Vec::new();
    for slot in slots {
        for fragment in slot.context.fragments() {
            if let Some(text) = fragment.as_text() {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    parts.push(trimmed.to_string());
                }
            }
        }
    }
    parts.join("\n\n")
}

fn apply_purpose(flux: &Flux, mut read: impl FnMut(usize) -> String) -> String {
    match &flux.mode {
        FluxMode::Purpose(PurposeFlux::Concat) => {
            let mut parts = Vec::with_capacity(flux.arity);
            for slot in 0..flux.arity {
                parts.push(read(slot));
            }
            parts.concat()
        }
        _ => unreachable!("flux mode channel already matched"),
    }
}

fn apply_context(flux: &Flux, mut read: impl FnMut(usize) -> Context) -> Context {
    match &flux.mode {
        FluxMode::Context(ContextFlux::Append) => {
            let mut result = Context::new();
            for slot in 0..flux.arity {
                let context = read(slot);
                for fragment in context.fragments() {
                    result.append(fragment.clone());
                }
            }
            result
        }
        FluxMode::Context(ContextFlux::Last) => {
            let mut result = Context::new();
            for slot in 0..flux.arity {
                let context = read(slot);
                if let Some(last) = context.fragments().last() {
                    result.append(last.clone());
                }
            }
            result
        }
        FluxMode::Context(ContextFlux::Digest) => {
            let mut result = Context::new();
            for slot in 0..flux.arity {
                let context = read(slot);
                digest_context_into(&context, &mut result);
            }
            result
        }
        FluxMode::Context(ContextFlux::Thread) => {
            let mut result = Context::new();
            for slot in 0..flux.arity {
                let context = read(slot);
                thread_context_into(slot, &context, &mut result);
            }
            result
        }
        _ => unreachable!("flux mode channel already matched"),
    }
}

fn apply_environment(flux: &Flux, mut read: impl FnMut(usize) -> Environment) -> Environment {
    match &flux.mode {
        FluxMode::Environment(EnvFlux::Overlay) => {
            if flux.arity == 0 {
                return Environment::named(flux.name.as_str(), ".");
            }
            let first = read(0);
            let mut result = Environment::named(flux.name.as_str(), first.cwd.clone());
            result.vars = first.vars.clone();
            result.root = first.root.clone();
            result.platform = first.platform.clone();
            for slot in 1..flux.arity {
                let environment = read(slot);
                result.cwd.clone_from(&environment.cwd);
                for (key, value) in &environment.vars {
                    result.vars.insert(key.clone(), value.clone());
                }
                result.root.clone_from(&environment.root);
                result.platform.clone_from(&environment.platform);
            }
            result
        }
        _ => unreachable!("flux mode channel already matched"),
    }
}

fn apply_resources(flux: &Flux, mut read: impl FnMut(usize) -> Resources) -> Resources {
    match &flux.mode {
        FluxMode::Resources(ResFlux::Merge) => {
            let mut result = Resources::named(flux.name.as_str());
            for slot in 0..flux.arity {
                let resources = read(slot);
                for model_name in &resources.model_order {
                    if let Some(model) = resources.models.get(model_name) {
                        result = result.with_model(model.clone());
                    }
                }
                for (name, definition) in &resources.tool_definitions {
                    result
                        .tool_definitions
                        .entry(name.clone())
                        .or_insert(definition.clone());
                }
                if result.active_model.is_empty() && !resources.active_model.is_empty() {
                    result.active_model.clone_from(&resources.active_model);
                }
                for name in &resources.active_tools {
                    result.active_tools.insert(name.clone());
                }
                for (name, prompt) in &resources.prompts {
                    result.prompts.entry(name.clone()).or_insert(prompt.clone());
                }
            }
            result
        }
        _ => unreachable!("flux mode channel already matched"),
    }
}

fn digest_context_into(source: &Context, target: &mut Context) {
    let fragments = source.fragments();
    if fragments.is_empty() {
        return;
    }
    let mut lines = Vec::new();
    for fragment in fragments {
        match &fragment.content {
            machine::Content::Text(text) => {
                if fragment.role == Role::System {
                    continue;
                }
                let trimmed = text.text.trim();
                if !trimmed.is_empty() {
                    lines.push(trimmed.to_string());
                }
            }
            machine::Content::ToolResult(result) => {
                lines.push(format!("[Tool result] {}", result.content.trim()));
            }
            machine::Content::Hitch { message, .. } => {
                lines.push(format!("[Error] {}", message.trim()));
            }
            machine::Content::ToolCall(_) => {}
            _ => {}
        }
    }
    if lines.is_empty() {
        return;
    }
    target.append(Fragment::assistant(lines.join("\n")));
}

fn thread_context_into(slot: usize, source: &Context, target: &mut Context) {
    let fragments = source.fragments();
    if fragments.is_empty() {
        return;
    }
    let mut digest = Context::new();
    digest_context_into(source, &mut digest);
    let answer_text = digest
        .fragments()
        .last()
        .and_then(|fragment| fragment.as_text())
        .unwrap_or("(no output)");
    target.append(Fragment::user(format!(
        "[Task {}] Please complete the following task.",
        slot + 1
    )));
    target.append(Fragment::assistant(answer_text));
}
