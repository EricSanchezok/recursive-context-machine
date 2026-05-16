use std::collections::{HashMap, VecDeque};

use crate::accelerator::{Accelerator, AcceleratorRef, Channel, NodeId, Port};
use crate::assembly::{Assembly, Slot};
use crate::flux::{Flux, FluxMode, FluxRef};
use crate::state::State;

/// Build a multi-agent execution graph.
pub struct Graph {
    accelerators: Vec<Accelerator>,
    fluxes: Vec<Flux>,
    wires: Vec<(Port, Port)>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            accelerators: Vec::new(),
            fluxes: Vec::new(),
            wires: Vec::new(),
        }
    }

    pub fn spawn(&mut self, purpose: impl Into<String>, state: State) -> AcceleratorRef {
        let id = self.accelerators.len();
        self.accelerators.push(Accelerator::new(purpose, state));
        AcceleratorRef { id }
    }

    pub fn weave(&mut self, arity: usize, mode: FluxMode) -> FluxRef {
        let id = self.fluxes.len();
        let channel = mode.channel();
        self.fluxes.push(Flux { mode, arity });
        FluxRef { id, channel }
    }

    pub fn wire(&mut self, from: Port, to: Port) {
        assert!(from.is_output(), "source must be an output pin");
        assert!(to.is_input(), "target must be an input pin");
        assert_eq!(from.channel(), to.channel(), "channel mismatch");
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
                Port::Node(NodeId::Accelerator(src), Channel::Pulse),
                Port::Node(NodeId::Accelerator(dst), Channel::Pulse),
            ) = (from, to)
            {
                downstream[*src].push(*dst);
                pending[*dst] += 1;
            }
        }

        let mut state_wires = HashMap::new();
        let mut flux_slot_wires = HashMap::new();
        for (from, to) in &self.wires {
            match to {
                Port::Node(NodeId::Accelerator(id), ch) if *ch != Channel::Pulse => {
                    state_wires.insert(*to, *from);
                }
                Port::FluxSlot(flux_id, slot_idx, _) => {
                    flux_slot_wires.insert((*flux_id, *slot_idx), *from);
                }
                _ => {}
            }
        }

        let mut is_sink = vec![true; num];
        for (from, _) in &self.wires {
            if let Port::Node(NodeId::Accelerator(id), Channel::Pulse) = from {
                is_sink[*id] = false;
            }
        }

        let slots = self
            .accelerators
            .into_iter()
            .map(|a| {
                Slot::new(
                    a.purpose,
                    a.state.ctx,
                    a.state.env,
                    a.state.policy,
                    a.state.res,
                )
            })
            .collect();

        Ok(Assembly {
            slots,
            fluxes: self.fluxes,
            downstream,
            pending,
            state_wires,
            flux_slot_wires,
            is_sink,
        })
    }

    fn validate_acyclic(&self) -> Result<(), BuildError> {
        let total = self.accelerators.len() + self.fluxes.len();
        let mut adj = vec![Vec::new(); total];

        for (from, to) in &self.wires {
            let src = from.node_index(self.accelerators.len());
            let dst = to.node_index(self.accelerators.len());
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
