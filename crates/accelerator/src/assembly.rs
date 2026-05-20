use std::collections::HashMap;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;

use machine::hook;
use machine::{Context, Environment, Policy, Resources};
use tracing::trace;
use utils::{AcceleratorId, AssemblyId, Name, SlotId};

use crate::accelerator::{Channel, Port, fire};
use crate::flux::Flux;
use crate::state::State;

/// A frozen multi-agent graph ready to run.
pub struct Assembly {
    pub(crate) id: AssemblyId,
    pub name: Name,
    pub(crate) slots: Vec<Slot>,
    pub(crate) fluxes: Vec<Flux>,
    pub(crate) downstream: Vec<Vec<usize>>,
    pub(crate) pending: Vec<usize>,
    pub(crate) state_wires: HashMap<Port, Port>,
    pub(crate) flux_slot_wires: HashMap<(usize, usize), Port>,
    pub(crate) is_sink: Vec<bool>,
}

pub(crate) struct Slot {
    id: SlotId,
    accelerator_id: AcceleratorId,
    pub name: Name,
    pub input: State,
    pub output: Option<State>,
}

impl Slot {
    pub fn new(accelerator_id: AcceleratorId, name: Name, state: State) -> Self {
        Self {
            id: SlotId::new(),
            accelerator_id,
            name,
            input: state,
            output: None,
        }
    }

    fn port(&self, index: usize, channel: Channel) -> Port {
        Port::Accel {
            index,
            accelerator_id: self.accelerator_id.clone(),
            channel,
        }
    }
}

impl Assembly {
    pub fn id(&self) -> &AssemblyId {
        &self.id
    }

    pub fn run(mut self) -> Pin<Box<dyn Future<Output = Vec<State>> + Send>> {
        Box::pin(async move {
            let mut queue = VecDeque::new();
            for (id, count) in self.pending.iter().enumerate() {
                if *count == 0 {
                    queue.push_back(id);
                }
            }

            while let Some(id) = queue.pop_front() {
                let slot_id = self.slots[id].id.clone();
                let slot_name = self.slots[id].name.clone();
                trace!(assembly = %self.id, graph = %self.name, slot = id, slot_id = %slot_id, slot_name = %slot_name, "running");

                hook!(
                    event = "slot_started",
                    assembly = self.id.as_str(),
                    graph = self.name.as_str(),
                    slot = id,
                    slot_id = slot_id.as_str(),
                    name = slot_name.as_str()
                );

                let mut state = self.slots[id].input.clone();
                state.purpose = self.resolve_purpose(id);
                state.ctx = self.resolve_ctx(id);
                state.env = self.resolve_env(id);
                state.policy = self.resolve_policy(id);
                state.res = self.resolve_res(id);

                let output = fire(state).await;
                self.slots[id].output = Some(output);

                hook!(
                    event = "slot_finished",
                    assembly = self.id.as_str(),
                    graph = self.name.as_str(),
                    slot = id,
                    slot_id = slot_id.as_str(),
                    name = slot_name.as_str()
                );

                for next in &self.downstream[id] {
                    self.pending[*next] -= 1;
                    if self.pending[*next] == 0 {
                        queue.push_back(*next);
                    }
                }
            }

            self.sink_outputs()
        })
    }

    fn sink_outputs(&mut self) -> Vec<State> {
        let mut sinks = Vec::new();
        for (id, slot) in self.slots.iter_mut().enumerate() {
            if self.is_sink[id] {
                sinks.push(slot.output.take().expect("sink slot did not run"));
            }
        }
        sinks
    }

    fn resolve_purpose(&self, slot_id: usize) -> String {
        let pin = self.slots[slot_id].port(slot_id, Channel::Purpose);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_purpose(from.clone()),
            None => self.slots[slot_id].input.purpose.clone(),
        }
    }

    fn read_purpose(&self, pin: Port) -> String {
        match pin {
            Port::Accel {
                index,
                channel: Channel::Purpose,
                ..
            } => self.slots[index]
                .output
                .as_ref()
                .expect("upstream not ready")
                .purpose
                .clone(),
            Port::FluxOut {
                index,
                channel: Channel::Purpose,
                ..
            } => crate::flux::apply_purpose(&self.fluxes[index], |slot| {
                let from = self.flux_slot_wires[&(index, slot)].clone();
                self.read_purpose(from)
            }),
            _ => panic!("type mismatch in purpose wire"),
        }
    }

    fn resolve_ctx(&self, slot_id: usize) -> Context {
        let pin = self.slots[slot_id].port(slot_id, Channel::Context);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_ctx(from.clone()),
            None => self.slots[slot_id].input.ctx.clone(),
        }
    }

    fn read_ctx(&self, pin: Port) -> Context {
        match pin {
            Port::Accel {
                index,
                channel: Channel::Context,
                ..
            } => self.slots[index]
                .output
                .as_ref()
                .expect("upstream not ready")
                .ctx
                .clone(),
            Port::FluxOut {
                index,
                channel: Channel::Context,
                ..
            } => crate::flux::apply_ctx(&self.fluxes[index], |slot| {
                let from = self.flux_slot_wires[&(index, slot)].clone();
                self.read_ctx(from)
            }),
            _ => panic!("type mismatch in ctx wire"),
        }
    }

    fn resolve_env(&self, slot_id: usize) -> Environment {
        let pin = self.slots[slot_id].port(slot_id, Channel::Environment);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_env(from.clone()),
            None => self.slots[slot_id].input.env.clone(),
        }
    }

    fn read_env(&self, pin: Port) -> Environment {
        match pin {
            Port::Accel {
                index,
                channel: Channel::Environment,
                ..
            } => self.slots[index]
                .output
                .as_ref()
                .expect("upstream not ready")
                .env
                .clone(),
            Port::FluxOut {
                index,
                channel: Channel::Environment,
                ..
            } => crate::flux::apply_env(&self.fluxes[index], |slot| {
                let from = self.flux_slot_wires[&(index, slot)].clone();
                self.read_env(from)
            }),
            _ => panic!("type mismatch in env wire"),
        }
    }

    fn resolve_policy(&self, slot_id: usize) -> Box<dyn Policy> {
        let pin = self.slots[slot_id].port(slot_id, Channel::Policy);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_policy(from.clone()),
            None => self.slots[slot_id].input.policy.clone(),
        }
    }

    fn read_policy(&self, pin: Port) -> Box<dyn Policy> {
        match pin {
            Port::Accel {
                index,
                channel: Channel::Policy,
                ..
            } => self.slots[index]
                .output
                .as_ref()
                .expect("upstream not ready")
                .policy
                .clone(),
            Port::FluxOut {
                index,
                channel: Channel::Policy,
                ..
            } => crate::flux::apply_policy(&self.fluxes[index], |slot| {
                let from = self.flux_slot_wires[&(index, slot)].clone();
                self.read_policy(from)
            }),
            _ => panic!("type mismatch in policy wire"),
        }
    }

    fn resolve_res(&self, slot_id: usize) -> Resources {
        let pin = self.slots[slot_id].port(slot_id, Channel::Resources);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_res(from.clone()),
            None => self.slots[slot_id].input.res.clone(),
        }
    }

    fn read_res(&self, pin: Port) -> Resources {
        match pin {
            Port::Accel {
                index,
                channel: Channel::Resources,
                ..
            } => self.slots[index]
                .output
                .as_ref()
                .expect("upstream not ready")
                .res
                .clone(),
            Port::FluxOut {
                index,
                channel: Channel::Resources,
                ..
            } => crate::flux::apply_res(&self.fluxes[index], |slot| {
                let from = self.flux_slot_wires[&(index, slot)].clone();
                self.read_res(from)
            }),
            _ => panic!("type mismatch in res wire"),
        }
    }
}
