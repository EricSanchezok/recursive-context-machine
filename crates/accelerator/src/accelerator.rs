use std::future::Future;
use std::pin::Pin;

use machine::{Context, Machine};

use crate::state::State;

/// A single agent — runs the Context Machine.
pub struct Accelerator {
    pub(crate) purpose: String,
    pub(crate) state: State,
}

impl Accelerator {
    pub fn new(purpose: impl Into<String>, state: State) -> Self {
        Self {
            purpose: purpose.into(),
            state,
        }
    }

    pub fn with_defaults(purpose: impl Into<String>) -> Self {
        Self::new(purpose, State::default())
    }

    pub fn run(self) -> Pin<Box<dyn Future<Output = Output> + Send>> {
        Box::pin(async move { fire(self.purpose, self.state).await })
    }
}

/// The result of running an agent.
pub struct Output {
    pub purpose: String,
    pub context: Context,
    pub environment: machine::Environment,
    pub resources: machine::Resources,
}

pub(crate) async fn fire(purpose: String, state: State) -> Output {
    let mut ctx = state.ctx;
    let mut env = state.env;
    let mut res = state.res;

    ctx.purpose = purpose;
    let machine = Machine::new(state.policy);
    machine.run(&mut ctx, &mut env, &mut res).await;
    let purpose = std::mem::take(&mut ctx.purpose);
    Output {
        purpose,
        context: ctx,
        environment: env,
        resources: res,
    }
}

// ── Graph wiring ──

#[derive(Clone, Copy, Debug)]
pub struct AcceleratorRef {
    pub(crate) id: usize,
}

impl AcceleratorRef {
    pub fn purpose_out(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Purpose)
    }
    pub fn ctx_out(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Context)
    }
    pub fn env_out(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Environment)
    }
    pub fn policy_out(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Policy)
    }
    pub fn res_out(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Resources)
    }
    pub fn done(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Pulse)
    }

    pub fn purpose_in(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Purpose)
    }
    pub fn ctx_in(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Context)
    }
    pub fn env_in(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Environment)
    }
    pub fn policy_in(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Policy)
    }
    pub fn res_in(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Resources)
    }
    pub fn run(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Pulse)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NodeId {
    Accelerator(usize),
    Flux(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Channel {
    Purpose,
    Context,
    Environment,
    Policy,
    Resources,
    Pulse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Port {
    Node(NodeId, Channel),
    FluxOut(usize, Channel),
    FluxSlot(usize, usize, Channel),
}

impl Port {
    pub fn is_output(&self) -> bool {
        matches!(
            self,
            Port::Node(NodeId::Accelerator(_), _) | Port::FluxOut(_, _)
        )
    }

    pub fn is_input(&self) -> bool {
        matches!(
            self,
            Port::Node(NodeId::Accelerator(_), _) | Port::FluxSlot(_, _, _)
        )
    }

    pub fn channel(&self) -> Channel {
        match self {
            Port::Node(_, ch) => *ch,
            Port::FluxOut(_, ch) => *ch,
            Port::FluxSlot(_, _, ch) => *ch,
        }
    }

    pub(crate) fn node_index(&self, num_accelerators: usize) -> usize {
        let offset = |id: usize| num_accelerators + id;
        match self {
            Port::Node(NodeId::Accelerator(id), _) => *id,
            Port::Node(NodeId::Flux(id), _) => offset(*id),
            Port::FluxOut(id, _) => offset(*id),
            Port::FluxSlot(id, _, _) => offset(*id),
        }
    }
}
