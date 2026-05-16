use crate::accelerator::InPin;

#[derive(Clone, Copy, Debug)]
pub struct FluxRef {
    pub(crate) id: usize,
}

impl FluxRef {
    pub fn slot(&self, idx: usize) -> InPin {
        InPin::FluxSlot(self.id, idx)
    }
    pub fn out(&self) -> crate::accelerator::OutPin {
        crate::accelerator::OutPin::FluxOut(self.id)
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

// ── Mode wrapper ──

pub trait IntoFluxMode {
    fn into_mode(self) -> FluxMode;
}

impl IntoFluxMode for PurposeFlux {
    fn into_mode(self) -> FluxMode {
        FluxMode::Purpose(self)
    }
}

impl IntoFluxMode for ContextFlux {
    fn into_mode(self) -> FluxMode {
        FluxMode::Context(self)
    }
}

impl IntoFluxMode for EnvFlux {
    fn into_mode(self) -> FluxMode {
        FluxMode::Environment(self)
    }
}

impl IntoFluxMode for ResFlux {
    fn into_mode(self) -> FluxMode {
        FluxMode::Resources(self)
    }
}

pub enum FluxMode {
    Purpose(PurposeFlux),
    Context(ContextFlux),
    Environment(EnvFlux),
    Resources(ResFlux),
}

pub(crate) struct Flux {
    pub mode: FluxMode,
    pub arity: usize,
}
