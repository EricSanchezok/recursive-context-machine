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

#[derive(Clone, Copy, Debug)]
pub(crate) enum FluxKind {
    Purpose,
    Context,
    Environment,
    Resources,
}

pub(crate) struct Flux {
    pub kind: FluxKind,
    pub arity: usize,
}
