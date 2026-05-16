use machine::{Machine, Purpose};
use std::future::Future;
use std::pin::Pin;

use crate::state::State;

/// A single agent — runs the Context Machine.
pub struct Accelerator {
    pub(crate) state: State,
}

impl Accelerator {
    pub fn new(state: State) -> Self {
        Self { state }
    }

    pub fn run(self) -> Pin<Box<dyn Future<Output = State> + Send>> {
        Box::pin(async move { fire(self.state).await })
    }
}

pub(crate) async fn fire(mut state: State) -> State {
    let purpose = Purpose::new(&state.purpose);
    let machine = Machine::new(state.policy.clone());
    machine
        .run(&purpose, &mut state.ctx, &mut state.env, &mut state.res)
        .await;
    state
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
