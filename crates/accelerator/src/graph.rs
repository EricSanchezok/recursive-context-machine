use std::collections::VecDeque;

use machine::{Context, Environment, Policy, Resources};

use crate::accelerator::{Accelerator, AcceleratorRef, InPin, NodeId, OutPin};
use crate::assembly::{Assembly, Slot};
use crate::flux::{Flux, FluxMode, FluxRef, IntoFluxMode};

/// Build a multi-agent execution graph.
pub struct Graph {
    accelerators: Vec<Accelerator>,
    fluxes: Vec<Flux>,
    wires: Vec<(OutPin, InPin)>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            accelerators: Vec::new(),
            fluxes: Vec::new(),
            wires: Vec::new(),
        }
    }

    pub fn spawn(
        &mut self,
        purpose: impl Into<String>,
        ctx: Context,
        env: Environment,
        policy: Box<dyn Policy>,
        res: Resources,
    ) -> AcceleratorRef {
        let id = self.accelerators.len();
        self.accelerators
            .push(Accelerator::new(purpose, ctx, env, policy, res));
        AcceleratorRef { id }
    }

    pub fn weave(&mut self, arity: usize, mode: impl IntoFluxMode) -> FluxRef {
        let id = self.fluxes.len();
        self.fluxes.push(Flux {
            mode: mode.into_mode(),
            arity,
        });
        FluxRef { id }
    }

    pub fn wire(&mut self, from: OutPin, to: InPin) {
        assert_eq!(
            self.pin_type(&from),
            self.pin_type(&to),
            "pin type mismatch"
        );
        assert!(
            !self.wires.iter().any(|(_, t)| *t == to),
            "input pin already wired"
        );
        self.wires.push((from, to));
    }

    pub fn build(self) -> Result<Assembly, BuildError> {
        self.validate_acyclic()?;

        let num = self.accelerators.len();
        let mut downstream = vec![Vec::new(); num];
        let mut pending = vec![0usize; num];

        for (from, to) in &self.wires {
            if let (
                OutPin::Pulse(NodeId::Accelerator(src)),
                InPin::Pulse(NodeId::Accelerator(dst)),
            ) = (from, to)
            {
                downstream[*src].push(*dst);
                pending[*dst] += 1;
            }
        }

        let slots = self
            .accelerators
            .into_iter()
            .map(|a| Slot {
                purpose: a.purpose,
                ctx: a.ctx,
                env: a.env,
                policy: Some(a.policy),
                res: a.res,
                out_purpose: None,
                out_ctx: None,
                out_env: None,
                out_res: None,
            })
            .collect();

        Ok(Assembly {
            slots,
            fluxes: self.fluxes,
            wires: self.wires,
            downstream,
            pending,
        })
    }

    fn validate_acyclic(&self) -> Result<(), BuildError> {
        let total = self.accelerators.len() + self.fluxes.len();
        let mut adj = vec![Vec::new(); total];

        for (from, to) in &self.wires {
            let src = self.node_index_from_out(from);
            let dst = self.node_index_from_in(to);
            adj[src].push(dst);
        }

        let mut in_degree = vec![0; total];
        for neighbors in &adj {
            for node in neighbors {
                in_degree[*node] += 1;
            }
        }

        let mut queue = VecDeque::new();
        for (idx, deg) in in_degree.iter().enumerate() {
            if *deg == 0 {
                queue.push_back(idx);
            }
        }

        let mut visited = 0;
        while let Some(node) = queue.pop_front() {
            visited += 1;
            for next in &adj[node] {
                in_degree[*next] -= 1;
                if in_degree[*next] == 0 {
                    queue.push_back(*next);
                }
            }
        }

        if visited == total {
            Ok(())
        } else {
            Err(BuildError::Cycle)
        }
    }

    fn node_index_from_out(&self, pin: &OutPin) -> usize {
        match pin {
            OutPin::Purpose(id)
            | OutPin::Context(id)
            | OutPin::Environment(id)
            | OutPin::Policy(id)
            | OutPin::Resources(id)
            | OutPin::Pulse(id) => match id {
                NodeId::Accelerator(i) => *i,
                NodeId::Flux(i) => self.accelerators.len() + i,
            },
            OutPin::FluxOut(i) => self.accelerators.len() + i,
        }
    }

    fn node_index_from_in(&self, pin: &InPin) -> usize {
        match pin {
            InPin::Purpose(id)
            | InPin::Context(id)
            | InPin::Environment(id)
            | InPin::Policy(id)
            | InPin::Resources(id)
            | InPin::Pulse(id) => match id {
                NodeId::Accelerator(i) => *i,
                NodeId::Flux(i) => self.accelerators.len() + i,
            },
            InPin::FluxSlot(i, _) => self.accelerators.len() + i,
        }
    }

    fn pin_type(&self, pin: &dyn PinLike) -> PinType {
        pin.ty(self)
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildError {
    Cycle,
}

// ── Pin type system ──

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PinType {
    Purpose,
    Context,
    Environment,
    Policy,
    Resources,
    Pulse,
}

trait PinLike {
    fn ty(&self, graph: &Graph) -> PinType;
}

impl PinLike for OutPin {
    fn ty(&self, graph: &Graph) -> PinType {
        match self {
            OutPin::Purpose(_) => PinType::Purpose,
            OutPin::Context(_) => PinType::Context,
            OutPin::Environment(_) => PinType::Environment,
            OutPin::Policy(_) => PinType::Policy,
            OutPin::Resources(_) => PinType::Resources,
            OutPin::Pulse(_) => PinType::Pulse,
            OutPin::FluxOut(id) => match &graph.fluxes[*id].mode {
                FluxMode::Purpose(_) => PinType::Purpose,
                FluxMode::Context(_) => PinType::Context,
                FluxMode::Environment(_) => PinType::Environment,
                FluxMode::Resources(_) => PinType::Resources,
            },
        }
    }
}

impl PinLike for InPin {
    fn ty(&self, graph: &Graph) -> PinType {
        match self {
            InPin::Purpose(_) => PinType::Purpose,
            InPin::Context(_) => PinType::Context,
            InPin::Environment(_) => PinType::Environment,
            InPin::Policy(_) => PinType::Policy,
            InPin::Resources(_) => PinType::Resources,
            InPin::Pulse(_) => PinType::Pulse,
            InPin::FluxSlot(id, _) => match &graph.fluxes[*id].mode {
                FluxMode::Purpose(_) => PinType::Purpose,
                FluxMode::Context(_) => PinType::Context,
                FluxMode::Environment(_) => PinType::Environment,
                FluxMode::Resources(_) => PinType::Resources,
            },
        }
    }
}
