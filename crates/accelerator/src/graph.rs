use std::collections::VecDeque;

use machine::{Context, Environment, Policy, Resources};

use crate::accelerator::{Accelerator, CoreRuntime};
use crate::core::{Core, CoreRef, InPin, NodeId, OutPin};
use crate::flux::{Flux, FluxKind, FluxRef};

/// Build a multi-agent execution graph.
pub struct Graph {
    cores: Vec<Core>,
    fluxes: Vec<Flux>,
    wires: Vec<(OutPin, InPin)>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            cores: Vec::new(),
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
    ) -> CoreRef {
        let id = self.cores.len();
        self.cores.push(Core {
            purpose: purpose.into(),
            ctx,
            env,
            policy: Some(policy),
            res,
        });
        CoreRef { id }
    }

    pub fn weave_purpose(&mut self, arity: usize) -> FluxRef {
        let id = self.fluxes.len();
        self.fluxes.push(Flux {
            kind: FluxKind::Purpose,
            arity,
        });
        FluxRef { id }
    }

    pub fn weave_ctx(&mut self, arity: usize) -> FluxRef {
        let id = self.fluxes.len();
        self.fluxes.push(Flux {
            kind: FluxKind::Context,
            arity,
        });
        FluxRef { id }
    }

    pub fn weave_env(&mut self, arity: usize) -> FluxRef {
        let id = self.fluxes.len();
        self.fluxes.push(Flux {
            kind: FluxKind::Environment,
            arity,
        });
        FluxRef { id }
    }

    pub fn weave_res(&mut self, arity: usize) -> FluxRef {
        let id = self.fluxes.len();
        self.fluxes.push(Flux {
            kind: FluxKind::Resources,
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

    pub fn build(self) -> Result<Accelerator, BuildError> {
        self.validate_acyclic()?;

        let num_cores = self.cores.len();
        let mut downstream = vec![Vec::new(); num_cores];
        let mut pending = vec![0usize; num_cores];

        for (from, to) in &self.wires {
            if let (OutPin::Pulse(NodeId::Core(src)), InPin::Pulse(NodeId::Core(dst))) = (from, to)
            {
                downstream[*src].push(*dst);
                pending[*dst] += 1;
            }
        }

        let cores = self
            .cores
            .into_iter()
            .map(|core| CoreRuntime {
                purpose: core.purpose,
                ctx: core.ctx,
                env: core.env,
                policy: core.policy,
                res: core.res,
                out_purpose: None,
                out_ctx: None,
                out_env: None,
                out_res: None,
                done: false,
            })
            .collect();

        Ok(Accelerator {
            cores,
            fluxes: self.fluxes,
            wires: self.wires,
            downstream,
            pending,
        })
    }

    fn validate_acyclic(&self) -> Result<(), BuildError> {
        let total = self.cores.len() + self.fluxes.len();
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
                NodeId::Core(i) => *i,
                NodeId::Flux(i) => self.cores.len() + i,
            },
            OutPin::FluxOut(i) => self.cores.len() + i,
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
                NodeId::Core(i) => *i,
                NodeId::Flux(i) => self.cores.len() + i,
            },
            InPin::FluxSlot(i, _) => self.cores.len() + i,
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
            OutPin::FluxOut(id) => match graph.fluxes[*id].kind {
                FluxKind::Purpose => PinType::Purpose,
                FluxKind::Context => PinType::Context,
                FluxKind::Environment => PinType::Environment,
                FluxKind::Resources => PinType::Resources,
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
            InPin::FluxSlot(id, _) => match graph.fluxes[*id].kind {
                FluxKind::Purpose => PinType::Purpose,
                FluxKind::Context => PinType::Context,
                FluxKind::Environment => PinType::Environment,
                FluxKind::Resources => PinType::Resources,
            },
        }
    }
}
