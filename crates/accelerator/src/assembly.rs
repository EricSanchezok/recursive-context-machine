use std::collections::HashMap;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;

use machine::{Context, Environment, Resources};
use tracing::trace;

use crate::accelerator::{Channel, NodeId, Output, Port, fire};
use crate::flux::{ContextFlux, EnvFlux, Flux, FluxMode, PurposeFlux, ResFlux};

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
    pub purpose: String,
    pub ctx: Context,
    pub env: Environment,
    pub policy: Option<Box<dyn machine::Policy>>,
    pub res: Resources,
    pub out_ctx: Option<Context>,
    pub out_env: Option<Environment>,
    pub out_res: Option<Resources>,
}

impl Assembly {
    pub fn run(mut self) -> Pin<Box<dyn Future<Output = Vec<Output>> + Send>> {
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

                let output = fire(purpose, ctx, env, policy, res).await;

                let slot = &mut self.slots[id];
                slot.out_ctx = Some(output.context);
                slot.out_env = Some(output.environment);
                slot.out_res = Some(output.resources);

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

    fn sink_outputs(&self) -> Vec<Output> {
        let mut sinks = Vec::new();
        for (id, slot) in self.slots.iter().enumerate() {
            if self.is_sink[id] {
                let ctx = slot.out_ctx.clone().unwrap_or_default();
                sinks.push(Output {
                    purpose: ctx.purpose.clone(),
                    context: ctx,
                    environment: slot
                        .out_env
                        .clone()
                        .unwrap_or_else(|| Environment::new(".")),
                    resources: slot.out_res.clone().unwrap_or_default(),
                });
            }
        }
        sinks
    }

    fn resolve_purpose(&self, slot_id: usize) -> String {
        let pin = Port::Node(NodeId::Accelerator(slot_id), Channel::Purpose);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_purpose(*from),
            None => self.slots[slot_id].purpose.clone(),
        }
    }

    fn read_purpose(&self, pin: Port) -> String {
        match pin {
            Port::Node(NodeId::Accelerator(id), Channel::Purpose) => self.slots[id]
                .out_ctx
                .as_ref()
                .expect("upstream not ready")
                .purpose
                .clone(),
            Port::FluxOut(id, Channel::Purpose) => self.eval_flux_purpose(id),
            _ => panic!("type mismatch in purpose wire"),
        }
    }

    fn eval_flux_purpose(&self, flux_id: usize) -> String {
        let flux = &self.fluxes[flux_id];
        match &flux.mode {
            FluxMode::Purpose(PurposeFlux::Concat) => {
                let mut parts = Vec::with_capacity(flux.arity);
                for slot in 0..flux.arity {
                    let from = self.flux_slot_wires[&(flux_id, slot)];
                    parts.push(self.read_purpose(from));
                }
                parts.concat()
            }
            _ => panic!("unexpected flux mode for purpose"),
        }
    }

    fn resolve_ctx(&self, slot_id: usize) -> Context {
        let pin = Port::Node(NodeId::Accelerator(slot_id), Channel::Context);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_ctx(*from),
            None => self.slots[slot_id].ctx.clone(),
        }
    }

    fn read_ctx(&self, pin: Port) -> Context {
        match pin {
            Port::Node(NodeId::Accelerator(id), Channel::Context) => self.slots[id]
                .out_ctx
                .as_ref()
                .expect("upstream not ready")
                .clone(),
            Port::FluxOut(id, Channel::Context) => self.eval_flux_ctx(id),
            _ => panic!("type mismatch in ctx wire"),
        }
    }

    fn eval_flux_ctx(&self, flux_id: usize) -> Context {
        let flux = &self.fluxes[flux_id];
        match &flux.mode {
            FluxMode::Context(ContextFlux::Append) => {
                let mut result = Context::new();
                for slot in 0..flux.arity {
                    let from = self.flux_slot_wires[&(flux_id, slot)];
                    let ctx = self.read_ctx(from);
                    for frag in ctx.fragments().iter() {
                        result.append(frag.clone());
                    }
                }
                result
            }
            FluxMode::Context(ContextFlux::Replace) => {
                let mut result = Context::new();
                for slot in 0..flux.arity {
                    let from = self.flux_slot_wires[&(flux_id, slot)];
                    let ctx = self.read_ctx(from);
                    if !ctx.is_empty() {
                        result = ctx;
                    }
                }
                result
            }
            _ => panic!("unexpected flux mode for context"),
        }
    }

    fn resolve_env(&self, slot_id: usize) -> Environment {
        let pin = Port::Node(NodeId::Accelerator(slot_id), Channel::Environment);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_env(*from),
            None => self.slots[slot_id].env.clone(),
        }
    }

    fn read_env(&self, pin: Port) -> Environment {
        match pin {
            Port::Node(NodeId::Accelerator(id), Channel::Environment) => self.slots[id]
                .out_env
                .as_ref()
                .expect("upstream not ready")
                .clone(),
            Port::FluxOut(id, Channel::Environment) => self.eval_flux_env(id),
            _ => panic!("type mismatch in env wire"),
        }
    }

    fn eval_flux_env(&self, flux_id: usize) -> Environment {
        let flux = &self.fluxes[flux_id];
        match &flux.mode {
            FluxMode::Environment(EnvFlux::Overlay) => {
                let mut result = Environment::new(".");
                for slot in 0..flux.arity {
                    let from = self.flux_slot_wires[&(flux_id, slot)];
                    let env = self.read_env(from);
                    result.cwd.clone_from(&env.cwd);
                    for (key, value) in &env.vars {
                        result.vars.insert(key.clone(), value.clone());
                    }
                    result.root.clone_from(&env.root);
                }
                result
            }
            _ => panic!("unexpected flux mode for environment"),
        }
    }

    fn resolve_res(&self, slot_id: usize) -> Resources {
        let pin = Port::Node(NodeId::Accelerator(slot_id), Channel::Resources);
        match self.state_wires.get(&pin) {
            Some(from) => self.read_res(*from),
            None => self.slots[slot_id].res.clone(),
        }
    }

    fn read_res(&self, pin: Port) -> Resources {
        match pin {
            Port::Node(NodeId::Accelerator(id), Channel::Resources) => self.slots[id]
                .out_res
                .as_ref()
                .expect("upstream not ready")
                .clone(),
            Port::FluxOut(id, Channel::Resources) => self.eval_flux_res(id),
            _ => panic!("type mismatch in res wire"),
        }
    }

    fn eval_flux_res(&self, flux_id: usize) -> Resources {
        let flux = &self.fluxes[flux_id];
        match &flux.mode {
            FluxMode::Resources(ResFlux::Merge) => {
                let mut result = Resources::new();
                for slot in 0..flux.arity {
                    let from = self.flux_slot_wires[&(flux_id, slot)];
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
            _ => panic!("unexpected flux mode for resources"),
        }
    }
}
