use machine::{Context, Environment, Fragment, Resources, Role};
use utils::{FluxId, Name};

use crate::state::State;
use crate::wire::Channel;

#[derive(Clone)]
pub enum PurposeFlux {
    Concat,
}

#[derive(Clone)]
pub enum ContextFlux {
    /// Concatenate all fragments from all slots in order.
    Append,
    /// Keep only the last fragment from each slot.
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
    /// The channel this flux reads slot data from.
    pub fn input_channel(&self) -> Channel {
        match self {
            FluxMode::Purpose(_) => Channel::Purpose,
            FluxMode::Context(_) => Channel::Context,
            FluxMode::Environment(_) => Channel::Environment,
            FluxMode::Resources(_) => Channel::Resources,
            FluxMode::Bridge { from, .. } => *from,
        }
    }

    /// The channel this flux writes its output to.
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

    pub fn apply(&self, slots: &[State]) -> State {
        let mut state = State::default();
        match &self.mode {
            FluxMode::Purpose(_) => {
                state.purpose = apply_purpose(self, |slot| slots[slot].purpose.clone())
            }
            FluxMode::Context(_) => state.ctx = apply_ctx(self, |slot| slots[slot].ctx.clone()),
            FluxMode::Environment(_) => state.env = apply_env(self, |slot| slots[slot].env.clone()),
            FluxMode::Resources(_) => state.res = apply_res(self, |slot| slots[slot].res.clone()),
            FluxMode::Bridge { kind, .. } => match kind {
                BridgeKind::ContextToPurpose => {
                    state.purpose = flatten_context_to_text(slots);
                }
            },
        }
        state
    }
}

fn flatten_context_to_text(slots: &[State]) -> String {
    let mut parts: Vec<String> = Vec::new();
    for slot in slots {
        for frag in slot.ctx.fragments().iter() {
            if let Some(text) = frag.as_text() {
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

fn apply_ctx(flux: &Flux, mut read: impl FnMut(usize) -> Context) -> Context {
    match &flux.mode {
        FluxMode::Context(ContextFlux::Append) => {
            let mut result = Context::new();
            for slot in 0..flux.arity {
                let context = read(slot);
                for frag in context.fragments().iter() {
                    result.append(frag.clone());
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

fn apply_env(flux: &Flux, mut read: impl FnMut(usize) -> Environment) -> Environment {
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
                let env = read(slot);
                result.cwd.clone_from(&env.cwd);
                for (key, value) in &env.vars {
                    result.vars.insert(key.clone(), value.clone());
                }
                result.root.clone_from(&env.root);
                result.platform.clone_from(&env.platform);
            }
            result
        }
        _ => unreachable!("flux mode channel already matched"),
    }
}

fn apply_res(flux: &Flux, mut read: impl FnMut(usize) -> Resources) -> Resources {
    match &flux.mode {
        FluxMode::Resources(ResFlux::Merge) => {
            let mut result = Resources::named(flux.name.as_str());
            for slot in 0..flux.arity {
                let res = read(slot);
                for model_name in &res.model_order {
                    if let Some(model) = res.models.get(model_name) {
                        result = result.with_model(model.clone());
                    }
                }
                for (name, definition) in &res.tool_definitions {
                    result
                        .tool_definitions
                        .entry(name.clone())
                        .or_insert(definition.clone());
                }
                if result.active_model.is_empty() && !res.active_model.is_empty() {
                    result.active_model.clone_from(&res.active_model);
                }
                for name in &res.active_tools {
                    result.active_tools.insert(name.clone());
                }
                for (name, prompt) in &res.prompts {
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

    let mut lines: Vec<String> = Vec::new();

    for frag in fragments {
        match &frag.content {
            machine::Content::Text(text) => {
                if frag.role == Role::System {
                    continue;
                }
                let trimmed = text.text.trim();
                if !trimmed.is_empty() {
                    lines.push(trimmed.to_string());
                }
            }
            machine::Content::ToolResult(tr) => {
                lines.push(format!("[Tool result] {}", tr.content.trim()));
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

    let digest = lines.join("\n");
    target.append(Fragment::assistant(digest));
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
        .and_then(|frag| frag.as_text())
        .unwrap_or("(no output)");

    target.append(Fragment::user(format!(
        "[Task {}] Please complete the following task.",
        slot + 1
    )));

    target.append(Fragment::assistant(answer_text));
}
