use machine::{Context, Environment, Policy, Resources};

/// An execution node — runs the Context Machine.
pub struct Core {
    pub(crate) purpose: String,
    pub(crate) ctx: Context,
    pub(crate) env: Environment,
    pub(crate) policy: Option<Box<dyn Policy>>,
    pub(crate) res: Resources,
}

#[derive(Clone, Copy, Debug)]
pub struct CoreRef {
    pub(crate) id: usize,
}

impl CoreRef {
    pub fn purpose_out(&self) -> OutPin {
        OutPin::Purpose(NodeId::Core(self.id))
    }
    pub fn ctx_out(&self) -> OutPin {
        OutPin::Context(NodeId::Core(self.id))
    }
    pub fn env_out(&self) -> OutPin {
        OutPin::Environment(NodeId::Core(self.id))
    }
    pub fn policy_out(&self) -> OutPin {
        OutPin::Policy(NodeId::Core(self.id))
    }
    pub fn res_out(&self) -> OutPin {
        OutPin::Resources(NodeId::Core(self.id))
    }
    pub fn done(&self) -> OutPin {
        OutPin::Pulse(NodeId::Core(self.id))
    }

    pub fn purpose_in(&self) -> InPin {
        InPin::Purpose(NodeId::Core(self.id))
    }
    pub fn ctx_in(&self) -> InPin {
        InPin::Context(NodeId::Core(self.id))
    }
    pub fn env_in(&self) -> InPin {
        InPin::Environment(NodeId::Core(self.id))
    }
    pub fn policy_in(&self) -> InPin {
        InPin::Policy(NodeId::Core(self.id))
    }
    pub fn res_in(&self) -> InPin {
        InPin::Resources(NodeId::Core(self.id))
    }
    pub fn run(&self) -> InPin {
        InPin::Pulse(NodeId::Core(self.id))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeId {
    Core(usize),
    Flux(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutPin {
    Purpose(NodeId),
    Context(NodeId),
    Environment(NodeId),
    Policy(NodeId),
    Resources(NodeId),
    Pulse(NodeId),
    FluxOut(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InPin {
    Purpose(NodeId),
    Context(NodeId),
    Environment(NodeId),
    Policy(NodeId),
    Resources(NodeId),
    Pulse(NodeId),
    FluxSlot(usize, usize),
}
