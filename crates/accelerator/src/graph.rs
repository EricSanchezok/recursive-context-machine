use std::collections::{HashMap, VecDeque};

use utils::{AssemblyId, FluxId, GraphId, Name};

use crate::accelerator::{Accelerator, AcceleratorRef, Channel, Port};
use crate::assembly::{Assembly, Slot};
use crate::flux::{Flux, FluxMode, FluxRef};
use crate::state::State;

/// Build a multi-agent execution graph.
pub struct Graph {
    id: GraphId,
    pub name: Name,
    accelerators: Vec<Accelerator>,
    fluxes: Vec<Flux>,
    wires: Vec<(Port, Port)>,
}

impl Graph {
    pub fn new() -> Self {
        Self::named("graph")
    }

    pub fn named(name: impl Into<String>) -> Self {
        Self {
            id: GraphId::new(),
            name: Name::new(name).expect("graph name must be valid"),
            accelerators: Vec::new(),
            fluxes: Vec::new(),
            wires: Vec::new(),
        }
    }

    pub fn id(&self) -> &GraphId {
        &self.id
    }

    pub fn rename(&mut self, name: impl Into<String>) {
        self.name = Name::new(name).expect("graph name must be valid");
    }

    pub fn spawn(&mut self, state: State) -> AcceleratorRef {
        self.spawn_named("accelerator", state)
    }

    pub fn spawn_named(&mut self, name: impl Into<String>, state: State) -> AcceleratorRef {
        let index = self.accelerators.len();
        self.accelerators.push(Accelerator::named(name, state));
        AcceleratorRef {
            index,
            id: self.accelerators[index].id().clone(),
        }
    }

    pub fn weave(&mut self, arity: usize, mode: FluxMode) -> FluxRef {
        self.weave_named(mode.name(), arity, mode)
    }

    pub fn weave_named(
        &mut self,
        name: impl Into<String>,
        arity: usize,
        mode: FluxMode,
    ) -> FluxRef {
        let index = self.fluxes.len();
        let channel = mode.channel();
        self.fluxes.push(Flux::new(name, mode, arity));
        FluxRef {
            index,
            id: self.fluxes[index].id().clone(),
            channel,
        }
    }

    pub fn rename_accelerator(&mut self, reference: AcceleratorRef, name: impl Into<String>) {
        self.assert_accelerator_ref(&reference);
        self.accelerators[reference.index].name =
            Name::new(name).expect("accelerator name must be valid");
    }

    pub fn rename_flux(&mut self, reference: FluxRef, name: impl Into<String>) {
        self.assert_flux_ref(&reference);
        self.fluxes[reference.index].name = Name::new(name).expect("flux name must be valid");
    }

    pub fn wire(&mut self, from: Port, to: Port) {
        self.assert_port_ref(&from);
        self.assert_port_ref(&to);
        assert!(from.is_output(), "source must be an output pin");
        assert!(to.is_input(), "target must be an input pin");
        assert_eq!(from.channel(), to.channel(), "channel mismatch");
        assert!(
            !self.wires.iter().any(|(_, target)| target == &to),
            "input pin already wired"
        );
        self.wires.push((from, to));
    }

    fn assert_accelerator_ref(&self, reference: &AcceleratorRef) {
        let Some(accelerator) = self.accelerators.get(reference.index) else {
            panic!("accelerator reference does not belong to this graph");
        };
        assert_eq!(
            accelerator.id(),
            &reference.id,
            "accelerator reference does not belong to this graph"
        );
    }

    fn assert_flux_ref(&self, reference: &FluxRef) {
        let Some(flux) = self.fluxes.get(reference.index) else {
            panic!("flux reference does not belong to this graph");
        };
        assert_eq!(
            flux.id(),
            &reference.id,
            "flux reference does not belong to this graph"
        );
    }

    fn assert_port_ref(&self, port: &Port) {
        match port {
            Port::Accel {
                index,
                accelerator_id,
                ..
            } => {
                let Some(accelerator) = self.accelerators.get(*index) else {
                    panic!("accelerator port does not belong to this graph");
                };
                assert_eq!(
                    accelerator.id(),
                    accelerator_id,
                    "accelerator port does not belong to this graph"
                );
            }
            Port::FluxOut { index, flux_id, .. } => self.assert_flux_port(*index, flux_id),
            Port::FluxSlot {
                index,
                flux_id,
                slot,
                ..
            } => {
                self.assert_flux_port(*index, flux_id);
                assert!(
                    *slot < self.fluxes[*index].arity,
                    "flux slot index is out of range"
                );
            }
        }
    }

    fn assert_flux_port(&self, index: usize, id: &FluxId) {
        let Some(flux) = self.fluxes.get(index) else {
            panic!("flux port does not belong to this graph");
        };
        assert_eq!(flux.id(), id, "flux port does not belong to this graph");
    }

    pub fn build(self) -> Result<Assembly, BuildError> {
        self.validate_acyclic()?;

        let num = self.accelerators.len();
        let mut downstream = vec![Vec::new(); num];
        let mut pending = vec![0usize; num];

        for (from, to) in &self.wires {
            if let (
                Port::Accel {
                    index: src,
                    channel: Channel::Pulse,
                    ..
                },
                Port::Accel {
                    index: dst,
                    channel: Channel::Pulse,
                    ..
                },
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
                Port::Accel { channel, .. } if *channel != Channel::Pulse => {
                    state_wires.insert(to.clone(), from.clone());
                }
                Port::FluxSlot { index, slot, .. } => {
                    flux_slot_wires.insert((*index, *slot), from.clone());
                }
                _ => {}
            }
        }

        let mut is_sink = vec![true; num];
        for (from, _) in &self.wires {
            if let Port::Accel {
                index,
                channel: Channel::Pulse,
                ..
            } = from
            {
                is_sink[*index] = false;
            }
        }

        let slots = self
            .accelerators
            .into_iter()
            .map(|accelerator| {
                Slot::new(
                    accelerator.id().clone(),
                    accelerator.name,
                    accelerator.state,
                )
            })
            .collect();

        Ok(Assembly {
            id: AssemblyId::new(),
            name: self.name,
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
