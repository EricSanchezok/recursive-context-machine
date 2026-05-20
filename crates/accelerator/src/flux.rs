use machine::{Context, Environment, Policy, Resources};
use utils::{FluxId, Name};

use crate::state::State;
use crate::wire::Channel;

#[derive(Clone)]
pub enum PurposeFlux {
    Concat,
}

#[derive(Clone)]
pub enum ContextFlux {
    Append,
    Replace,
}

#[derive(Clone)]
pub enum EnvFlux {
    Overlay,
}

#[derive(Clone)]
pub enum ResFlux {
    Merge,
}

#[derive(Clone)]
pub enum PolicyFlux {
    Replace,
}

#[derive(Clone)]
pub enum FluxMode {
    Purpose(PurposeFlux),
    Context(ContextFlux),
    Environment(EnvFlux),
    Resources(ResFlux),
    Policy(PolicyFlux),
}

impl FluxMode {
    pub fn channel(&self) -> Channel {
        match self {
            FluxMode::Purpose(_) => Channel::Purpose,
            FluxMode::Context(_) => Channel::Context,
            FluxMode::Environment(_) => Channel::Environment,
            FluxMode::Resources(_) => Channel::Resources,
            FluxMode::Policy(_) => Channel::Policy,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            FluxMode::Purpose(PurposeFlux::Concat) => "purpose_concat",
            FluxMode::Context(ContextFlux::Append) => "context_append",
            FluxMode::Context(ContextFlux::Replace) => "context_replace",
            FluxMode::Environment(EnvFlux::Overlay) => "environment_overlay",
            FluxMode::Resources(ResFlux::Merge) => "resources_merge",
            FluxMode::Policy(PolicyFlux::Replace) => "policy_replace",
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
        match self.mode.channel() {
            Channel::Purpose => {
                state.purpose = apply_purpose(self, |slot| slots[slot].purpose.clone())
            }
            Channel::Context => state.ctx = apply_ctx(self, |slot| slots[slot].ctx.clone()),
            Channel::Environment => state.env = apply_env(self, |slot| slots[slot].env.clone()),
            Channel::Policy => state.policy = apply_policy(self, |slot| slots[slot].policy.clone()),
            Channel::Resources => state.res = apply_res(self, |slot| slots[slot].res.clone()),
            Channel::Pulse => {}
        }
        state
    }
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
        FluxMode::Context(ContextFlux::Replace) => {
            let mut result = Context::new();
            for slot in 0..flux.arity {
                let context = read(slot);
                if !context.is_empty() {
                    result = context;
                }
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
                for (name, model) in &res.models {
                    result.models.entry(name.clone()).or_insert(model.clone());
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

fn apply_policy(flux: &Flux, mut read: impl FnMut(usize) -> Box<dyn Policy>) -> Box<dyn Policy> {
    match &flux.mode {
        FluxMode::Policy(PolicyFlux::Replace) => {
            let mut result = None;
            for slot in 0..flux.arity {
                result = Some(read(slot));
            }
            result.expect("policy flux requires at least one input")
        }
        _ => unreachable!("flux mode channel already matched"),
    }
}
