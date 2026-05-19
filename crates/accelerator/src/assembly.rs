use std::collections::HashMap;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;

use machine::hook;
use machine::{Context, Environment, Policy, Resources};
use tracing::trace;
use utils::Name;

use crate::accelerator::{Channel, Port, fire};
use crate::flux::Flux;
use crate::state::State;

/// A frozen multi-agent graph ready to run.
pub struct Assembly {
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
    pub name: Name,
    pub input: State,
    pub output: Option<State>,
}

impl Slot {
    pub fn new(name: Name, state: State) -> Self {
        Self {
            name,
            input: state,
            output: None,
        }
    }
}

impl Assembly {
    pub fn run(mut self) -> Pin<Box<dyn Future<Output = Vec<State>> + Send>> {
        Box::pin(async move {
            let mut queue = VecDeque::new();
            for (id, count) in self.pending.iter().enumerate() {
                if *count == 0 {
                    queue.push_back(id);
                }
            }

            while let Some(id) = queue.pop_front() {
                let slot_name = self.slots[id].name.clone();
                trace!(graph = %self.name, slot = id, slot_name = %slot_name, "running");

                hook!(
                    event = "slot_started",
                    graph = self.name.as_str(),
                    slot = id,
                    name = slot_name.as_str()
                );

                let mut s = self.slots[id].input.clone();
                s.purpose = self.resolve_purpose(id);
                s.ctx = self.resolve_ctx(id);
                s.env = self.resolve_env(id);
                s.policy = self.resolve_policy(id);
                s.res = self.resolve_res(id);

                let output = fire(s).await;
                self.slots[id].output = Some(output);

                hook!(
                    event = "slot_finished",
                    graph = self.name.as_str(),
                    slot = id,
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
                sinks.push(slot.output.take().unwrap_or_default());
            }
        }
        sinks
    }

    fn resolve_purpose(&self, slot_id: usize) -> String {
        let pin = Port::Accel(slot_id, Channel::Purpose);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_purpose(*from),
            None => self.slots[slot_id].input.purpose.clone(),
        }
    }

    fn read_purpose(&self, pin: Port) -> String {
        match pin {
            Port::Accel(id, Channel::Purpose) => self.slots[id]
                .output
                .as_ref()
                .expect("upstream not ready")
                .purpose
                .clone(),
            Port::FluxOut(id, Channel::Purpose) => {
                let flux = &self.fluxes[id];
                crate::flux::apply_purpose(flux, |slot| {
                    let from = self.flux_slot_wires[&(id, slot)];
                    self.read_purpose(from)
                })
            }
            _ => panic!("type mismatch in purpose wire"),
        }
    }

    fn resolve_ctx(&self, slot_id: usize) -> Context {
        let pin = Port::Accel(slot_id, Channel::Context);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_ctx(*from),
            None => self.slots[slot_id].input.ctx.clone(),
        }
    }

    fn read_ctx(&self, pin: Port) -> Context {
        match pin {
            Port::Accel(id, Channel::Context) => self.slots[id]
                .output
                .as_ref()
                .expect("upstream not ready")
                .ctx
                .clone(),
            Port::FluxOut(id, Channel::Context) => {
                let flux = &self.fluxes[id];
                crate::flux::apply_ctx(flux, |slot| {
                    let from = self.flux_slot_wires[&(id, slot)];
                    self.read_ctx(from)
                })
            }
            _ => panic!("type mismatch in ctx wire"),
        }
    }

    fn resolve_env(&self, slot_id: usize) -> Environment {
        let pin = Port::Accel(slot_id, Channel::Environment);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_env(*from),
            None => self.slots[slot_id].input.env.clone(),
        }
    }

    fn read_env(&self, pin: Port) -> Environment {
        match pin {
            Port::Accel(id, Channel::Environment) => self.slots[id]
                .output
                .as_ref()
                .expect("upstream not ready")
                .env
                .clone(),
            Port::FluxOut(id, Channel::Environment) => {
                let flux = &self.fluxes[id];
                crate::flux::apply_env(flux, |slot| {
                    let from = self.flux_slot_wires[&(id, slot)];
                    self.read_env(from)
                })
            }
            _ => panic!("type mismatch in env wire"),
        }
    }

    fn resolve_policy(&self, slot_id: usize) -> Box<dyn Policy> {
        let pin = Port::Accel(slot_id, Channel::Policy);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_policy(*from),
            None => self.slots[slot_id].input.policy.clone(),
        }
    }

    fn read_policy(&self, pin: Port) -> Box<dyn Policy> {
        match pin {
            Port::Accel(id, Channel::Policy) => self.slots[id]
                .output
                .as_ref()
                .expect("upstream not ready")
                .policy
                .clone(),
            Port::FluxOut(id, Channel::Policy) => {
                let flux = &self.fluxes[id];
                crate::flux::apply_policy(flux, |slot| {
                    let from = self.flux_slot_wires[&(id, slot)];
                    self.read_policy(from)
                })
            }
            _ => panic!("type mismatch in policy wire"),
        }
    }

    fn resolve_res(&self, slot_id: usize) -> Resources {
        let pin = Port::Accel(slot_id, Channel::Resources);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_res(*from),
            None => self.slots[slot_id].input.res.clone(),
        }
    }

    fn read_res(&self, pin: Port) -> Resources {
        match pin {
            Port::Accel(id, Channel::Resources) => self.slots[id]
                .output
                .as_ref()
                .expect("upstream not ready")
                .res
                .clone(),
            Port::FluxOut(id, Channel::Resources) => {
                let flux = &self.fluxes[id];
                crate::flux::apply_res(flux, |slot| {
                    let from = self.flux_slot_wires[&(id, slot)];
                    self.read_res(from)
                })
            }
            _ => panic!("type mismatch in res wire"),
        }
    }
}
