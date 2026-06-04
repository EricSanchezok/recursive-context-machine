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
    /// Structured digest — extract key information from each slot's context,
    /// producing a condensed summary per slot. Inspired by Synergy's
    /// TurnDigest: tool outputs are truncated, reasoning is dropped, and only
    /// the essential segments (final text, tool summaries, errors) are kept.
    Digest,
    /// Thread-style assembly — for each slot, prepend the slot's purpose as a
    /// user question, followed by the slot's last meaningful fragment as the
    /// answer. This creates a Q&A thread where downstream sees "question →
    /// answer, question → answer, now solve this".
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

/// Cross-channel data transfer: reads from one channel, transforms, writes to
/// another. Bridge does not produce output on its input channel — the
/// transformed data is written exclusively to the output channel.
#[derive(Clone, Copy)]
pub enum BridgeKind {
    /// For each slot: extract the last assistant text from the context,
    /// join with "\n\n", and write as a purpose string.
    ContextLastTextToPurpose,
}

#[derive(Clone)]
pub enum FluxMode {
    Purpose(PurposeFlux),
    Context(ContextFlux),
    Environment(EnvFlux),
    Resources(ResFlux),
    /// Cross-channel bridge: reads data from `from` channel via slots,
    /// transforms it according to `kind`, and writes the result to `to`
    /// channel.
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
                kind: BridgeKind::ContextLastTextToPurpose,
                ..
            } => "bridge_context_last_to_purpose",
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
                BridgeKind::ContextLastTextToPurpose => {
                    state.purpose = extract_last_assistant_text(slots);
                }
            },
        }
        state
    }
}

// ── helper: extract last assistant text from each slot's context ──

fn extract_last_assistant_text(slots: &[State]) -> String {
    let mut parts: Vec<String> = Vec::with_capacity(slots.len());
    for slot in slots {
        if let Some(text) = slot
            .ctx
            .fragments()
            .iter()
            .rev()
            .find(|f| f.role == Role::Assistant)
            .and_then(|f| f.as_text())
        {
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                parts.push(trimmed.to_string());
            }
        }
    }
    parts.join("\n\n")
}

// ── per-channel apply helpers ──

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

/// Extract a structured digest from a context, appending condensed fragments
/// to `target`. Inspired by Synergy's TurnDigest:
/// - Final assistant text → kept
/// - Tool results → summarized as `[Tool: {name}] {title}`
/// - Tool errors → kept as `[Tool: {name}] Error: {msg}`
/// - Hitches → kept (they are error signals)
/// - System prompts → dropped (they are scaffolding)
/// - Tool calls → dropped (the result or error carries the information)
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

/// Assemble a Q&A thread from a slot's context.
///
/// For each slot, we first digest the context (extract key information,
/// dropping scaffolding), then emit a user question followed by the digest
/// as the answer. This creates a thread where downstream sees:
///
///   User: [Task 1] Please complete the following task.
///   Assistant: {digest of slot 0}
///   User: [Task 2] Please complete the following task.
///   Assistant: {digest of slot 1}
///   ...
///
/// The downstream accelerator then receives this as its context, making it
/// clear that multiple upstream tasks have been completed and it should
/// focus on the next step.
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
