use machine::{Context, Environment, Resources};

use crate::accelerator::Channel;

// ── Public API ──

#[derive(Clone, Copy, Debug)]
pub struct FluxRef {
    pub(crate) id: usize,
    pub(crate) channel: Channel,
}

impl FluxRef {
    pub fn slot(&self, idx: usize) -> crate::accelerator::Port {
        crate::accelerator::Port::FluxSlot(self.id, idx, self.channel)
    }
    pub fn out(&self) -> crate::accelerator::Port {
        crate::accelerator::Port::FluxOut(self.id, self.channel)
    }
}

// ── Behavior enums ──

pub enum PurposeFlux {
    Concat,
}

pub enum ContextFlux {
    Append,
    Replace,
}

pub enum EnvFlux {
    Overlay,
}

pub enum ResFlux {
    Merge,
}

// ── Mode ──

pub enum FluxMode {
    Purpose(PurposeFlux),
    Context(ContextFlux),
    Environment(EnvFlux),
    Resources(ResFlux),
}

impl FluxMode {
    pub fn channel(&self) -> Channel {
        match self {
            FluxMode::Purpose(_) => Channel::Purpose,
            FluxMode::Context(_) => Channel::Context,
            FluxMode::Environment(_) => Channel::Environment,
            FluxMode::Resources(_) => Channel::Resources,
        }
    }
}

// ── Internal representation ──

pub(crate) struct Flux {
    pub mode: FluxMode,
    pub arity: usize,
}

// ── Evaluation — pure functions called by Assembly ──

pub(crate) fn eval_flux_purpose(flux: &Flux, mut read: impl FnMut(usize) -> String) -> String {
    match &flux.mode {
        FluxMode::Purpose(PurposeFlux::Concat) => {
            let mut parts = Vec::with_capacity(flux.arity);
            for slot in 0..flux.arity {
                parts.push(read(slot));
            }
            parts.concat()
        }
        _ => panic!("unexpected flux mode for purpose"),
    }
}

pub(crate) fn eval_flux_ctx(flux: &Flux, mut read: impl FnMut(usize) -> Context) -> Context {
    match &flux.mode {
        FluxMode::Context(ContextFlux::Append) => {
            let mut result = Context::new();
            for slot in 0..flux.arity {
                let ctx = read(slot);
                for frag in ctx.fragments().iter() {
                    result.append(frag.clone());
                }
            }
            result
        }
        FluxMode::Context(ContextFlux::Replace) => {
            let mut result = Context::new();
            for slot in 0..flux.arity {
                let ctx = read(slot);
                if !ctx.is_empty() {
                    result = ctx;
                }
            }
            result
        }
        _ => panic!("unexpected flux mode for context"),
    }
}

pub(crate) fn eval_flux_env(
    flux: &Flux,
    mut read: impl FnMut(usize) -> Environment,
) -> Environment {
    match &flux.mode {
        FluxMode::Environment(EnvFlux::Overlay) => {
            let mut result = Environment::new(".");
            for slot in 0..flux.arity {
                let env = read(slot);
                result.cwd.clone_from(&env.cwd);
                for (key, value) in &env.vars {
                    result.vars.insert(key.clone(), value.clone());
                }
                result.root.clone_from(&env.root);
            }
            result
        }
        _ => panic!("unexpected flux mode for environment"),
    }
}

pub(crate) fn eval_flux_res(flux: &Flux, mut read: impl FnMut(usize) -> Resources) -> Resources {
    match &flux.mode {
        FluxMode::Resources(ResFlux::Merge) => {
            let mut result = Resources::new();
            for slot in 0..flux.arity {
                let res = read(slot);
                for (name, model) in &res.models {
                    result
                        .models
                        .entry(name.clone())
                        .or_insert_with(|| model.clone());
                }
                if result.active_model.is_empty() && !res.active_model.is_empty() {
                    result.active_model.clone_from(&res.active_model);
                }
                for name in &res.active_tools {
                    result.active_tools.insert(name.clone());
                }
                for (name, prompt) in &res.prompts {
                    result
                        .prompts
                        .entry(name.clone())
                        .or_insert_with(|| prompt.clone());
                }
            }
            result
        }
        _ => panic!("unexpected flux mode for resources"),
    }
}
