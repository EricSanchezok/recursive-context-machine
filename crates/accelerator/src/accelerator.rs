use machine::{Machine, Purpose};
use std::future::Future;
use std::pin::Pin;
use utils::{AcceleratorId, FluxId, Name};

use crate::state::State;

/// A single agent — runs the Context Machine.
pub struct Accelerator {
    id: AcceleratorId,
    pub name: Name,
    pub(crate) state: State,
}

impl Accelerator {
    pub fn id(&self) -> &AcceleratorId {
        &self.id
    }

    pub fn new(state: State) -> Self {
        Self::named("accelerator", state)
    }

    pub fn named(name: impl Into<String>, state: State) -> Self {
        Self {
            id: AcceleratorId::new(),
            name: Name::new(name).expect("accelerator name must be valid"),
            state,
        }
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

#[derive(Clone, Debug)]
pub struct AcceleratorRef {
    pub(crate) index: usize,
    pub(crate) id: AcceleratorId,
}

impl AcceleratorRef {
    pub fn id(&self) -> &AcceleratorId {
        &self.id
    }

    pub fn purpose_out(&self) -> Port {
        self.port(Channel::Purpose)
    }
    pub fn ctx_out(&self) -> Port {
        self.port(Channel::Context)
    }
    pub fn env_out(&self) -> Port {
        self.port(Channel::Environment)
    }
    pub fn policy_out(&self) -> Port {
        self.port(Channel::Policy)
    }
    pub fn res_out(&self) -> Port {
        self.port(Channel::Resources)
    }
    pub fn done(&self) -> Port {
        self.port(Channel::Pulse)
    }
    pub fn purpose_in(&self) -> Port {
        self.port(Channel::Purpose)
    }
    pub fn ctx_in(&self) -> Port {
        self.port(Channel::Context)
    }
    pub fn env_in(&self) -> Port {
        self.port(Channel::Environment)
    }
    pub fn policy_in(&self) -> Port {
        self.port(Channel::Policy)
    }
    pub fn res_in(&self) -> Port {
        self.port(Channel::Resources)
    }
    pub fn trigger(&self) -> Port {
        self.port(Channel::Pulse)
    }

    fn port(&self, channel: Channel) -> Port {
        Port::Accel {
            index: self.index,
            accelerator_id: self.id.clone(),
            channel,
        }
    }
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Port {
    Accel {
        index: usize,
        accelerator_id: AcceleratorId,
        channel: Channel,
    },
    FluxOut {
        index: usize,
        flux_id: FluxId,
        channel: Channel,
    },
    FluxSlot {
        index: usize,
        flux_id: FluxId,
        slot: usize,
        channel: Channel,
    },
}

impl Port {
    pub fn is_output(&self) -> bool {
        matches!(self, Port::Accel { .. } | Port::FluxOut { .. })
    }
    pub fn is_input(&self) -> bool {
        matches!(self, Port::Accel { .. } | Port::FluxSlot { .. })
    }
    pub fn channel(&self) -> Channel {
        match self {
            Port::Accel { channel, .. }
            | Port::FluxOut { channel, .. }
            | Port::FluxSlot { channel, .. } => *channel,
        }
    }
    pub(crate) fn node_index(&self, num_accelerators: usize) -> usize {
        let offset = |index: usize| num_accelerators + index;
        match self {
            Port::Accel { index, .. } => *index,
            Port::FluxOut { index, .. } | Port::FluxSlot { index, .. } => offset(*index),
        }
    }
}
