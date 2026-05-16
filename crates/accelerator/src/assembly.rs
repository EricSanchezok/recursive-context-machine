use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;

use machine::{Context, Environment, Resources};
use tracing::trace;

use crate::accelerator::{InPin, NodeId, OutPin, fire};
use crate::flux::Flux;

/// A frozen multi-agent graph ready to run.
pub struct Assembly {
    pub(crate) slots: Vec<Slot>,
    pub(crate) fluxes: Vec<Flux>,
    pub(crate) wires: Vec<(OutPin, InPin)>,
    pub(crate) downstream: Vec<Vec<usize>>,
    pub(crate) pending: Vec<usize>,
}

pub(crate) struct Slot {
    pub purpose: String,
    pub ctx: Context,
    pub env: Environment,
    pub policy: Option<Box<dyn machine::Policy>>,
    pub res: Resources,
    pub out_purpose: Option<String>,
    pub out_ctx: Option<Context>,
    pub out_env: Option<Environment>,
    pub out_res: Option<Resources>,
}

impl Assembly {
    pub fn run(
        mut self,
    ) -> Pin<Box<dyn Future<Output = Vec<(String, Context, Environment, Resources)>> + Send>> {
        Box::pin(async move {
            let mut queue = VecDeque::new();
            for (id, count) in self.pending.iter().enumerate() {
                if *count == 0 {
                    queue.push_back(id);
                }
            }

            while let Some(id) = queue.pop_front() {
                trace!(slot = id, "running");

                let purpose = self.resolve_purpose(id);
                let ctx = self.resolve_ctx(id);
                let env = self.resolve_env(id);
                let policy = self.slots[id].policy.take().expect("policy missing");
                let res = self.resolve_res(id);

                let (out_purpose, out_ctx, out_env, out_res) =
                    fire(purpose, ctx, env, policy, res).await;

                let slot = &mut self.slots[id];
                slot.out_purpose = Some(out_purpose);
                slot.out_ctx = Some(out_ctx);
                slot.out_env = Some(out_env);
                slot.out_res = Some(out_res);

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

    fn sink_outputs(&self) -> Vec<(String, Context, Environment, Resources)> {
        let mut sinks = Vec::new();
        for (id, slot) in self.slots.iter().enumerate() {
            let has_downstream = self
                .wires
                .iter()
                .any(|(from, _)| matches!(from, OutPin::Pulse(NodeId::Accelerator(i)) if *i == id));
            if !has_downstream {
                sinks.push((
                    slot.out_purpose.clone().unwrap_or_default(),
                    slot.out_ctx.clone().unwrap_or_default(),
                    slot.out_env
                        .clone()
                        .unwrap_or_else(|| Environment::new(".")),
                    slot.out_res.clone().unwrap_or_default(),
                ));
            }
        }
        sinks
    }

    fn resolve_purpose(&self, slot_id: usize) -> String {
        for (from, to) in &self.wires {
            if let InPin::Purpose(NodeId::Accelerator(id)) = to {
                if *id == slot_id {
                    return self.read_purpose(*from);
                }
            }
        }
        self.slots[slot_id].purpose.clone()
    }

    fn read_purpose(&self, pin: OutPin) -> String {
        match pin {
            OutPin::Purpose(NodeId::Accelerator(id)) => self.slots[id]
                .out_purpose
                .as_ref()
                .expect("upstream purpose not ready")
                .clone(),
            OutPin::FluxOut(id) => self.eval_flux_purpose(id),
            _ => panic!("type mismatch in purpose wire"),
        }
    }

    fn eval_flux_purpose(&self, flux_id: usize) -> String {
        let flux = &self.fluxes[flux_id];
        let mut parts = Vec::with_capacity(flux.arity);
        for slot in 0..flux.arity {
            let from = self.find_wire_to_flux(flux_id, slot);
            parts.push(self.read_purpose(from));
        }
        parts.concat()
    }

    fn resolve_ctx(&self, slot_id: usize) -> Context {
        for (from, to) in &self.wires {
            if let InPin::Context(NodeId::Accelerator(id)) = to {
                if *id == slot_id {
                    return self.read_ctx(*from);
                }
            }
        }
        self.slots[slot_id].ctx.clone()
    }

    fn read_ctx(&self, pin: OutPin) -> Context {
        match pin {
            OutPin::Context(NodeId::Accelerator(id)) => self.slots[id]
                .out_ctx
                .as_ref()
                .expect("upstream ctx not ready")
                .clone(),
            OutPin::FluxOut(id) => self.eval_flux_ctx(id),
            _ => panic!("type mismatch in ctx wire"),
        }
    }

    fn eval_flux_ctx(&self, flux_id: usize) -> Context {
        let flux = &self.fluxes[flux_id];
        let mut result = Context::new();
        for slot in 0..flux.arity {
            let from = self.find_wire_to_flux(flux_id, slot);
            let ctx = self.read_ctx(from);
            for frag in ctx.fragments().iter() {
                result.append(frag.clone());
            }
        }
        result
    }

    fn resolve_env(&self, slot_id: usize) -> Environment {
        for (from, to) in &self.wires {
            if let InPin::Environment(NodeId::Accelerator(id)) = to {
                if *id == slot_id {
                    return self.read_env(*from);
                }
            }
        }
        self.slots[slot_id].env.clone()
    }

    fn read_env(&self, pin: OutPin) -> Environment {
        match pin {
            OutPin::Environment(NodeId::Accelerator(id)) => self.slots[id]
                .out_env
                .as_ref()
                .expect("upstream env not ready")
                .clone(),
            OutPin::FluxOut(id) => self.eval_flux_env(id),
            _ => panic!("type mismatch in env wire"),
        }
    }

    fn eval_flux_env(&self, flux_id: usize) -> Environment {
        let flux = &self.fluxes[flux_id];
        let mut result = Environment::new(".");
        for slot in 0..flux.arity {
            let from = self.find_wire_to_flux(flux_id, slot);
            let env = self.read_env(from);
            result.cwd.clone_from(&env.cwd);
            for (key, value) in &env.vars {
                result.vars.insert(key.clone(), value.clone());
            }
            result.root.clone_from(&env.root);
        }
        result
    }

    fn resolve_res(&self, slot_id: usize) -> Resources {
        for (from, to) in &self.wires {
            if let InPin::Resources(NodeId::Accelerator(id)) = to {
                if *id == slot_id {
                    return self.read_res(*from);
                }
            }
        }
        self.slots[slot_id].res.clone()
    }

    fn read_res(&self, pin: OutPin) -> Resources {
        match pin {
            OutPin::Resources(NodeId::Accelerator(id)) => self.slots[id]
                .out_res
                .as_ref()
                .expect("upstream res not ready")
                .clone(),
            OutPin::FluxOut(id) => self.eval_flux_res(id),
            _ => panic!("type mismatch in res wire"),
        }
    }

    fn eval_flux_res(&self, flux_id: usize) -> Resources {
        let flux = &self.fluxes[flux_id];
        let mut result = Resources::new();
        for slot in 0..flux.arity {
            let from = self.find_wire_to_flux(flux_id, slot);
            let res = self.read_res(from);
            for (name, model) in &res.models {
                result
                    .models
                    .entry(name.clone())
                    .or_insert_with(|| model.clone());
            }
            if result.active_model.is_empty() && !res.active_model.is_empty() {
                result.active_model.clone_from(&res.active_model);
            }
            for name in &res.active_tools {
                result.active_tools.insert(name.clone());
            }
            for (name, prompt) in &res.prompts {
                result
                    .prompts
                    .entry(name.clone())
                    .or_insert_with(|| prompt.clone());
            }
        }
        result
    }

    fn find_wire_to_flux(&self, flux_id: usize, slot: usize) -> OutPin {
        for (from, to) in &self.wires {
            if let InPin::FluxSlot(id, idx) = to {
                if *id == flux_id && *idx == slot {
                    return *from;
                }
            }
        }
        panic!("flux slot {slot} of flux {flux_id} is unwired");
    }
}
