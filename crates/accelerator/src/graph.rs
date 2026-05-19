use std::collections::{HashMap, HashSet, VecDeque};

use utils::Name;

use crate::accelerator::{Accelerator, AcceleratorRef, Channel, Port};
use crate::assembly::{Assembly, Slot};
use crate::flux::{Flux, FluxMode, FluxRef};
use crate::state::State;

/// Build a multi-agent execution graph.
pub struct Graph {
    name: Name,
    names: HashSet<Name>,
    accelerators: Vec<Accelerator>,
    fluxes: Vec<Flux>,
    wires: Vec<(Port, Port)>,
}

impl Graph {
    pub fn new() -> Self {
        Self::named("graph")
    }

    pub fn named(name: impl Into<String>) -> Self {
        let name = Name::new(name).expect("graph name must be valid");
        let mut names = HashSet::new();
        names.insert(name.clone());
        Self {
            name,
            names,
            accelerators: Vec::new(),
            fluxes: Vec::new(),
            wires: Vec::new(),
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn rename(&mut self, name: impl Into<String>) {
        let old = self.name.clone();
        let new = self.replace_name(old, name);
        self.name = new;
    }

    pub fn spawn(&mut self, state: State) -> AcceleratorRef {
        let name = self.next_name("accelerator");
        self.spawn_named(name, state)
    }

    pub fn spawn_named(&mut self, name: impl Into<String>, state: State) -> AcceleratorRef {
        let id = self.accelerators.len();
        let name = self.reserve_name(name);
        self.accelerators
            .push(Accelerator::named(name.as_str(), state));
        AcceleratorRef { id }
    }

    pub fn weave(&mut self, arity: usize, mode: FluxMode) -> FluxRef {
        let name = self.next_name(mode.name());
        self.weave_named(name, arity, mode)
    }

    pub fn weave_named(
        &mut self,
        name: impl Into<String>,
        arity: usize,
        mode: FluxMode,
    ) -> FluxRef {
        let id = self.fluxes.len();
        let channel = mode.channel();
        let name = self.reserve_name(name);
        self.fluxes.push(Flux { name, mode, arity });
        FluxRef { id, channel }
    }

    fn next_name(&self, base: &str) -> String {
        let mut idx = 0;
        loop {
            let candidate = format!("{base}_{idx}");
            if !self.names.iter().any(|name| name.as_str() == candidate) {
                return candidate;
            }
            idx += 1;
        }
    }

    fn reserve_name(&mut self, name: impl Into<String>) -> Name {
        let name = Name::new(name).expect("node name must be valid");
        assert!(
            self.names.insert(name.clone()),
            "name '{}' already exists",
            name
        );
        name
    }

    fn replace_name(&mut self, old: Name, new: impl Into<String>) -> Name {
        let new = Name::new(new).expect("name must be valid");
        if old == new {
            return old;
        }
        self.names.remove(&old);
        if !self.names.insert(new.clone()) {
            self.names.insert(old);
            panic!("name '{}' already exists", new);
        }
        new
    }

    pub fn rename_accelerator(&mut self, reference: AcceleratorRef, name: impl Into<String>) {
        let old = self.accelerators[reference.id].name.clone();
        let new = self.replace_name(old, name);
        self.accelerators[reference.id].name = new;
    }

    pub fn rename_flux(&mut self, reference: FluxRef, name: impl Into<String>) {
        let old = self.fluxes[reference.id].name.clone();
        let new = self.replace_name(old, name);
        self.fluxes[reference.id].name = new;
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
            if let (Port::Accel(src, Channel::Pulse), Port::Accel(dst, Channel::Pulse)) = (from, to)
            {
                downstream[*src].push(*dst);
                pending[*dst] += 1;
            }
        }

        let mut state_wires = HashMap::new();
        let mut flux_slot_wires = HashMap::new();
        for (from, to) in &self.wires {
            match to {
                Port::Accel(id, ch) if *ch != Channel::Pulse => {
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
            if let Port::Accel(id, Channel::Pulse) = from {
                is_sink[*id] = false;
            }
        }

        let slots = self
            .accelerators
            .into_iter()
            .map(|a| Slot::new(a.name, a.state))
            .collect();

        Ok(Assembly {
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
