use crate::accelerator::{Channel, Port};

#[derive(Clone, Copy, Debug)]
pub struct FluxRef {
    pub(crate) id: usize,
    pub(crate) channel: Channel,
}

impl FluxRef {
    pub fn slot(&self, idx: usize) -> Port {
        Port::FluxSlot(self.id, idx, self.channel)
    }
    pub fn out(&self) -> Port {
        Port::FluxOut(self.id, self.channel)
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

pub(crate) struct Flux {
    pub mode: FluxMode,
    pub arity: usize,
}
