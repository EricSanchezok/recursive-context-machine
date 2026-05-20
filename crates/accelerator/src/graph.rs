use std::collections::{HashMap, VecDeque};

use utils::{AssemblyId, ConditionId, FluxId, GraphId, Name};

use crate::accelerator::{Accelerator, AcceleratorRef, Channel, Port};
use crate::assembly::{Assembly, ConditionRouting, Slot};
use crate::condition::{Condition, ConditionBranch, ConditionRef, Predicate};
use crate::flux::{Flux, FluxMode, FluxRef};
use crate::state::State;

/// Build a multi-agent execution graph.
pub struct Graph {
    id: GraphId,
    pub name: Name,
    accelerators: Vec<Accelerator>,
    fluxes: Vec<Flux>,
    conditions: Vec<Condition>,
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
            conditions: Vec::new(),
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

    pub fn condition(&mut self, predicate: Predicate) -> ConditionRef {
        self.condition_named("condition", predicate)
    }

    pub fn condition_named(
        &mut self,
        name: impl Into<String>,
        predicate: Predicate,
    ) -> ConditionRef {
        let index = self.conditions.len();
        self.conditions.push(Condition::new(name, predicate));
        ConditionRef {
            index,
            id: self.conditions[index].id().clone(),
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

    pub fn rename_condition(&mut self, reference: ConditionRef, name: impl Into<String>) {
        self.assert_condition_ref(&reference);
        self.conditions[reference.index].name =
            Name::new(name).expect("condition name must be valid");
    }

    pub fn wire(&mut self, from: Port, to: Port) {
        self.assert_port_ref(&from);
        self.assert_port_ref(&to);
        assert!(from.is_output(), "source must be an output pin");
        assert!(to.is_input(), "target must be an input pin");
        assert_eq!(from.channel(), to.channel(), "channel mismatch");
        self.assert_supported_pulse_wire(&from, &to);
        if to.channel() != Channel::Pulse {
            assert!(
                !self.wires.iter().any(|(_, target)| target == &to),
                "input pin already wired"
            );
        }
        self.wires.push((from, to));
    }

    fn assert_supported_pulse_wire(&self, from: &Port, to: &Port) {
        if from.channel() != Channel::Pulse {
            return;
        }

        match (from, to) {
            (Port::Accel { .. }, Port::Accel { .. })
            | (Port::Accel { .. }, Port::ConditionIn { .. })
            | (Port::ConditionOut { .. }, Port::Accel { .. }) => {}
            _ => panic!("unsupported pulse wire"),
        }
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

    fn assert_condition_ref(&self, reference: &ConditionRef) {
        let Some(condition) = self.conditions.get(reference.index) else {
            panic!("condition reference does not belong to this graph");
        };
        assert_eq!(
            condition.id(),
            &reference.id,
            "condition reference does not belong to this graph"
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
            Port::ConditionIn {
                index,
                condition_id,
            }
            | Port::ConditionOut {
                index,
                condition_id,
                ..
            } => self.assert_condition_port(*index, condition_id),
        }
    }

    fn assert_flux_port(&self, index: usize, id: &FluxId) {
        let Some(flux) = self.fluxes.get(index) else {
            panic!("flux port does not belong to this graph");
        };
        assert_eq!(flux.id(), id, "flux port does not belong to this graph");
    }

    fn assert_condition_port(&self, index: usize, id: &ConditionId) {
        let Some(condition) = self.conditions.get(index) else {
            panic!("condition port does not belong to this graph");
        };
        assert_eq!(
            condition.id(),
            id,
            "condition port does not belong to this graph"
        );
    }

    pub fn build(self) -> Result<Assembly, BuildError> {
        self.validate_acyclic()?;

        let num = self.accelerators.len();
        let mut downstream = vec![Vec::new(); num];
        let mut pending = vec![0usize; num];
        let active_inputs = vec![0usize; num];
        let skipped = vec![false; num];
        let mut condition_sources = vec![None; self.conditions.len()];
        let mut condition_branches = vec![ConditionBranches::default(); self.conditions.len()];

        for (from, to) in &self.wires {
            match (from, to) {
                (
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
                ) => {
                    downstream[*src].push(*dst);
                    pending[*dst] += 1;
                }
                (
                    Port::Accel {
                        index: src,
                        channel: Channel::Pulse,
                        ..
                    },
                    Port::ConditionIn { index, .. },
                ) => {
                    if condition_sources[*index].replace(*src).is_some() {
                        return Err(BuildError::DuplicateConditionSource);
                    }
                }
                (
                    Port::ConditionOut { index, branch, .. },
                    Port::Accel {
                        index: dst,
                        channel: Channel::Pulse,
                        ..
                    },
                ) => {
                    condition_branches[*index].set(*branch, *dst)?;
                    pending[*dst] += 1;
                }
                _ => {}
            }
        }

        let mut condition_map: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut condition_routing = Vec::with_capacity(self.conditions.len());
        for (index, condition) in self.conditions.into_iter().enumerate() {
            let source = condition_sources[index].ok_or(BuildError::MissingConditionSource)?;
            let branches = condition_branches[index];
            let true_target = branches
                .true_target
                .ok_or(BuildError::MissingConditionTrueBranch)?;
            let false_target = branches
                .false_target
                .ok_or(BuildError::MissingConditionFalseBranch)?;
            let routing_index = condition_routing.len();
            condition_map.entry(source).or_default().push(routing_index);
            condition_routing.push(ConditionRouting {
                predicate: condition.predicate,
                true_target,
                false_target,
            });
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

        for (flux_index, flux) in self.fluxes.iter().enumerate() {
            for slot in 0..flux.arity {
                if !flux_slot_wires.contains_key(&(flux_index, slot)) {
                    return Err(BuildError::UnwiredFluxSlot {
                        flux: flux.name.to_string(),
                        slot,
                    });
                }
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
            active_inputs,
            skipped,
            state_wires,
            flux_slot_wires,
            is_sink,
            condition_routing,
            condition_map,
        })
    }

    fn validate_acyclic(&self) -> Result<(), BuildError> {
        let total = self.accelerators.len() + self.fluxes.len() + self.conditions.len();
        let mut adj = vec![Vec::new(); total];

        for (from, to) in &self.wires {
            let src = from.node_index(self.accelerators.len(), self.fluxes.len());
            let dst = to.node_index(self.accelerators.len(), self.fluxes.len());
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildError {
    Cycle,
    DuplicateConditionSource,
    DuplicateConditionBranch,
    MissingConditionSource,
    MissingConditionTrueBranch,
    MissingConditionFalseBranch,
    UnwiredFluxSlot { flux: String, slot: usize },
}

#[derive(Clone, Copy, Debug, Default)]
struct ConditionBranches {
    true_target: Option<usize>,
    false_target: Option<usize>,
}

impl ConditionBranches {
    fn set(&mut self, branch: ConditionBranch, target: usize) -> Result<(), BuildError> {
        let slot = match branch {
            ConditionBranch::True => &mut self.true_target,
            ConditionBranch::False => &mut self.false_target,
        };
        if slot.replace(target).is_some() {
            return Err(BuildError::DuplicateConditionBranch);
        }
        Ok(())
    }
}
