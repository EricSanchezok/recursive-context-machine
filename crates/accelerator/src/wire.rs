use utils::AcceleratorId;

use crate::condition::ConditionBranch;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Channel {
    Purpose,
    Context,
    Environment,
    Resources,
    Pulse,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Port {
    pub owner: PortOwner,
    pub endpoint: Endpoint,
}

impl Port {
    pub fn input(endpoint: Endpoint) -> Self {
        Self {
            owner: PortOwner::BoundaryInput,
            endpoint,
        }
    }

    pub fn output(endpoint: Endpoint) -> Self {
        Self {
            owner: PortOwner::BoundaryOutput,
            endpoint,
        }
    }

    pub fn component(component: ComponentId, endpoint: Endpoint) -> Self {
        Self {
            owner: PortOwner::Component(component),
            endpoint,
        }
    }

    pub fn channel(&self) -> Channel {
        match self.endpoint {
            Endpoint::Trigger
            | Endpoint::Done
            | Endpoint::ConditionIn
            | Endpoint::ConditionOut(_) => Channel::Pulse,
            Endpoint::State(channel)
            | Endpoint::FluxSlot { channel, .. }
            | Endpoint::FluxOut(channel) => channel,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PortOwner {
    BoundaryInput,
    BoundaryOutput,
    Component(ComponentId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Endpoint {
    Trigger,
    Done,
    State(Channel),
    FluxSlot { slot: usize, channel: Channel },
    FluxOut(Channel),
    ConditionIn,
    ConditionOut(ConditionBranch),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComponentId(usize);

impl ComponentId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct ComponentRef {
    id: ComponentId,
    accelerator_id: Option<AcceleratorId>,
}

impl ComponentRef {
    pub(crate) fn new(id: ComponentId, accelerator_id: Option<AcceleratorId>) -> Self {
        Self { id, accelerator_id }
    }

    pub fn id(&self) -> ComponentId {
        self.id
    }

    pub fn accelerator_id(&self) -> Option<&AcceleratorId> {
        self.accelerator_id.as_ref()
    }

    pub fn trigger(&self) -> Port {
        self.port(Endpoint::Trigger)
    }

    pub fn done(&self) -> Port {
        self.port(Endpoint::Done)
    }

    pub fn port_state(&self, channel: Channel) -> Port {
        self.port(Endpoint::State(channel))
    }

    pub fn purpose(&self) -> Port {
        self.port_state(Channel::Purpose)
    }

    pub fn context(&self) -> Port {
        self.port(Endpoint::State(Channel::Context))
    }

    pub fn environment(&self) -> Port {
        self.port(Endpoint::State(Channel::Environment))
    }

    pub fn resources(&self) -> Port {
        self.port(Endpoint::State(Channel::Resources))
    }

    pub fn slot(&self, slot: usize, channel: Channel) -> Port {
        self.port(Endpoint::FluxSlot { slot, channel })
    }

    pub fn flux_out(&self, channel: Channel) -> Port {
        self.port(Endpoint::FluxOut(channel))
    }

    pub fn condition_in(&self) -> Port {
        self.port(Endpoint::ConditionIn)
    }

    pub fn condition_out(&self, branch: ConditionBranch) -> Port {
        self.port(Endpoint::ConditionOut(branch))
    }

    fn port(&self, endpoint: Endpoint) -> Port {
        Port::component(self.id, endpoint)
    }
}

#[derive(Clone, Debug)]
pub struct Wire {
    pub from: Port,
    pub to: Port,
}

impl Wire {
    pub fn new(from: Port, to: Port) -> Self {
        Self { from, to }
    }
}
