use std::collections::HashMap;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;

use machine::{Context, Environment, Resources};
use tracing::trace;

use crate::accelerator::{Channel, NodeId, Port, fire};
use crate::flux::Flux;
use crate::state::State;

/// A frozen multi-agent graph ready to run.
pub struct Assembly {
    pub(crate) slots: Vec<Slot>,
    pub(crate) fluxes: Vec<Flux>,
    pub(crate) downstream: Vec<Vec<usize>>,
    pub(crate) pending: Vec<usize>,
    pub(crate) state_wires: HashMap<Port, Port>,
    pub(crate) flux_slot_wires: HashMap<(usize, usize), Port>,
    pub(crate) is_sink: Vec<bool>,
}

pub(crate) struct Slot {
    pub input: State,
    pub output: Option<State>,
}

impl Slot {
    pub fn new(state: State) -> Self {
        Self {
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
                trace!(slot = id, "running");

                let mut s = self.slots[id].input.clone();
                s.purpose = self.resolve_purpose(id);
                s.ctx = self.resolve_ctx(id);
                s.env = self.resolve_env(id);
                s.res = self.resolve_res(id);

                let output = fire(s).await;
                self.slots[id].output = Some(output);

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
        let pin = Port::Node(NodeId::Accelerator(slot_id), Channel::Purpose);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_purpose(*from),
            None => self.slots[slot_id].input.purpose.clone(),
        }
    }

    fn read_purpose(&self, pin: Port) -> String {
        match pin {
            Port::Node(NodeId::Accelerator(id), Channel::Purpose) => self.slots[id]
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
        let pin = Port::Node(NodeId::Accelerator(slot_id), Channel::Context);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_ctx(*from),
            None => self.slots[slot_id].input.ctx.clone(),
        }
    }

    fn read_ctx(&self, pin: Port) -> Context {
        match pin {
            Port::Node(NodeId::Accelerator(id), Channel::Context) => self.slots[id]
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
        let pin = Port::Node(NodeId::Accelerator(slot_id), Channel::Environment);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_env(*from),
            None => self.slots[slot_id].input.env.clone(),
        }
    }

    fn read_env(&self, pin: Port) -> Environment {
        match pin {
            Port::Node(NodeId::Accelerator(id), Channel::Environment) => self.slots[id]
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

    fn resolve_res(&self, slot_id: usize) -> Resources {
        let pin = Port::Node(NodeId::Accelerator(slot_id), Channel::Resources);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_res(*from),
            None => self.slots[slot_id].input.res.clone(),
        }
    }

    fn read_res(&self, pin: Port) -> Resources {
        match pin {
            Port::Node(NodeId::Accelerator(id), Channel::Resources) => self.slots[id]
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
