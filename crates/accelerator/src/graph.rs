use std::collections::{HashSet, VecDeque};

use utils::{GraphId, Name};

use crate::accelerator::Accelerator;
use crate::condition::{Condition, ConditionBranch};
use crate::flux::{Flux, FluxMode};
use crate::state::State;
use crate::wire::{Channel, ComponentId, ComponentRef, Endpoint, Port, PortOwner, Wire};

#[derive(Clone)]
pub struct Graph {
    id: GraphId,
    pub name: Name,
    components: Vec<Component>,
    wires: Vec<Wire>,
}

impl Graph {
    pub fn new() -> Self {
        Self::named("graph")
    }

    pub fn named(name: impl Into<String>) -> Self {
        Self {
            id: GraphId::new(),
            name: Name::new(name).expect("graph name must be valid"),
            components: Vec::new(),
            wires: Vec::new(),
        }
    }

    pub fn id(&self) -> &GraphId {
        &self.id
    }

    pub fn rename(&mut self, name: impl Into<String>) {
        self.name = Name::new(name).expect("graph name must be valid");
    }

    pub fn add_accelerator(
        &mut self,
        name: impl Into<String>,
        accelerator: Accelerator,
    ) -> ComponentRef {
        let id = ComponentId::new(self.components.len());
        let accelerator_id = accelerator.id().clone();
        self.components.push(Component {
            name: Name::new(name).expect("component name must be valid"),
            kind: ComponentKind::Accelerator(Box::new(accelerator)),
        });
        ComponentRef::new(id, Some(accelerator_id))
    }

    pub fn add_flux(
        &mut self,
        name: impl Into<String>,
        mode: FluxMode,
        arity: usize,
    ) -> ComponentRef {
        let id = ComponentId::new(self.components.len());
        let flux = Flux::new(name, mode, arity);
        let component_name = flux.name.clone();
        self.components.push(Component {
            name: component_name,
            kind: ComponentKind::Flux(flux),
        });
        ComponentRef::new(id, None)
    }

    pub fn add_condition(
        &mut self,
        name: impl Into<String>,
        predicate: crate::condition::Predicate,
    ) -> ComponentRef {
        let id = ComponentId::new(self.components.len());
        let condition = Condition::new(name, predicate);
        let component_name = condition.name.clone();
        self.components.push(Component {
            name: component_name,
            kind: ComponentKind::Condition(condition),
        });
        ComponentRef::new(id, None)
    }

    pub fn components(&self) -> &[Component] {
        &self.components
    }

    pub fn validate(&self) -> Result<(), String> {
        self.validate_conditions()?;
        self.validate_acyclic()
    }

    pub fn wire(&mut self, from: Port, to: Port) {
        self.wires.push(Wire::new(from, to));
    }

    pub fn input(endpoint: Endpoint) -> Port {
        Port::input(endpoint)
    }

    pub fn output(endpoint: Endpoint) -> Port {
        Port::output(endpoint)
    }

    pub async fn run(self, input: State) -> State {
        GraphRun::new(self, input).run().await
    }

    fn validate_conditions(&self) -> Result<(), String> {
        let mut condition_inputs = vec![0usize; self.components.len()];
        let mut true_outputs = vec![0usize; self.components.len()];
        let mut false_outputs = vec![0usize; self.components.len()];

        for wire in &self.wires {
            if let Some(component) = component_id(&wire.to)
                && matches!(wire.to.endpoint, Endpoint::ConditionIn)
            {
                condition_inputs[component.index()] += 1;
            }
            if let Some(component) = component_id(&wire.from) {
                match wire.from.endpoint {
                    Endpoint::ConditionOut(ConditionBranch::True) => {
                        true_outputs[component.index()] += 1
                    }
                    Endpoint::ConditionOut(ConditionBranch::False) => {
                        false_outputs[component.index()] += 1
                    }
                    _ => {}
                }
            }
        }

        for (index, component) in self.components.iter().enumerate() {
            if !matches!(component.kind, ComponentKind::Condition(_)) {
                continue;
            }
            if condition_inputs[index] != 1 {
                return Err(format!(
                    "condition {} requires exactly one trigger input",
                    component.name.as_str()
                ));
            }
            if true_outputs[index] == 0 || false_outputs[index] == 0 {
                return Err(format!(
                    "condition {} requires true and false outputs",
                    component.name.as_str()
                ));
            }
        }
        Ok(())
    }

    fn validate_acyclic(&self) -> Result<(), String> {
        let mut incoming = vec![0usize; self.components.len()];
        let mut downstream = vec![Vec::new(); self.components.len()];
        let mut edges = HashSet::new();

        for wire in &self.wires {
            let Some(source) = component_id(&wire.from) else {
                continue;
            };
            let Some(target) = component_id(&wire.to) else {
                continue;
            };
            if source == target {
                return Err("graph contains a self cycle".to_string());
            }
            if edges.insert((source.index(), target.index())) {
                incoming[target.index()] += 1;
                downstream[source.index()].push(target.index());
            }
        }

        let mut queue = VecDeque::new();
        for (index, count) in incoming.iter().enumerate() {
            if *count == 0 {
                queue.push_back(index);
            }
        }

        let mut visited = 0usize;
        while let Some(index) = queue.pop_front() {
            visited += 1;
            for target in &downstream[index] {
                incoming[*target] -= 1;
                if incoming[*target] == 0 {
                    queue.push_back(*target);
                }
            }
        }

        if visited == self.components.len() {
            Ok(())
        } else {
            Err("graph contains a cycle".to_string())
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct Component {
    pub name: Name,
    pub kind: ComponentKind,
}

#[derive(Clone)]
pub enum ComponentKind {
    Accelerator(Box<Accelerator>),
    Flux(Flux),
    Condition(Condition),
}

struct GraphRun {
    graph: Graph,
    inputs: Vec<State>,
    outputs: Vec<Option<State>>,
    flux_slots: Vec<Vec<Option<State>>>,
    condition_inputs: Vec<Option<State>>,
    branches: Vec<Option<ConditionBranch>>,
    pending: Vec<usize>,
    active_inputs: Vec<usize>,
    skipped: Vec<bool>,
    result: State,
}

impl GraphRun {
    fn new(graph: Graph, input: State) -> Self {
        let mut inputs = Vec::with_capacity(graph.components.len());
        let mut flux_slots = Vec::with_capacity(graph.components.len());

        for component in &graph.components {
            match &component.kind {
                ComponentKind::Accelerator(accelerator) => {
                    inputs.push(accelerator.default_input());
                    flux_slots.push(Vec::new());
                }
                ComponentKind::Flux(flux) => {
                    inputs.push(State::default());
                    flux_slots.push(vec![None; flux.arity]);
                }
                ComponentKind::Condition(_) => {
                    inputs.push(State::default());
                    flux_slots.push(Vec::new());
                }
            }
        }

        let component_count = graph.components.len();
        let mut run = Self {
            graph,
            inputs,
            outputs: vec![None; component_count],
            flux_slots,
            condition_inputs: vec![None; component_count],
            branches: vec![None; component_count],
            pending: vec![0; component_count],
            active_inputs: vec![0; component_count],
            skipped: vec![false; component_count],
            result: State::default(),
        };

        run.count_dependencies();
        run.apply_boundary_input(&input);
        run
    }

    async fn run(mut self) -> State {
        let mut queue = VecDeque::new();
        for (index, count) in self.pending.iter().enumerate() {
            if *count == 0 {
                queue.push_back(index);
            }
        }

        while let Some(index) = queue.pop_front() {
            if self.skipped[index] || self.outputs[index].is_some() {
                continue;
            }
            self.run_component(index).await;
            self.propagate_component(index, &mut queue);
        }

        self.result
    }

    fn count_dependencies(&mut self) {
        let mut dependencies = HashSet::new();
        for wire in &self.graph.wires {
            let Some(source) = component_id(&wire.from) else {
                continue;
            };
            let Some(target) = component_id(&wire.to) else {
                continue;
            };
            if source != target && dependencies.insert((source.index(), target.index())) {
                self.pending[target.index()] += 1;
            }
        }
    }

    fn apply_boundary_input(&mut self, input: &State) {
        let wires = self.graph.wires.clone();
        for wire in wires {
            if wire.from.owner != PortOwner::BoundaryInput {
                continue;
            }
            self.apply_wire(&wire, input);
        }
    }

    async fn run_component(&mut self, index: usize) {
        match self.graph.components[index].kind.clone() {
            ComponentKind::Accelerator(accelerator) => {
                let output = accelerator.run_with(self.inputs[index].clone()).await;
                self.outputs[index] = Some(output);
            }
            ComponentKind::Flux(flux) => {
                let slots = self.flux_slots[index]
                    .iter()
                    .map(|slot| slot.clone().expect("flux slot missing input"))
                    .collect::<Vec<_>>();
                self.outputs[index] = Some(flux.apply(&slots));
            }
            ComponentKind::Condition(condition) => {
                let state = self.condition_inputs[index].clone().unwrap_or_default();
                self.branches[index] = Some(condition.route(&state));
                self.outputs[index] = Some(state);
            }
        }
    }

    fn propagate_component(&mut self, source: usize, queue: &mut VecDeque<usize>) {
        let wires = self.graph.wires.clone();
        let mut released = HashSet::new();
        for wire in wires {
            if component_id(&wire.from) != Some(ComponentId::new(source)) {
                continue;
            }
            let active = self.branch_is_active(source, &wire.from.endpoint);
            if active {
                let state = self.source_state(source, &wire.from.endpoint);
                self.apply_wire(&wire, &state);
            }
            if let Some(target) = component_id(&wire.to)
                && released.insert(target.index())
            {
                self.release(target.index(), active, queue);
            }
        }
    }

    fn propagate_skip(&mut self, source: usize, queue: &mut VecDeque<usize>) {
        let wires = self.graph.wires.clone();
        let mut released = HashSet::new();
        for wire in wires {
            if component_id(&wire.from) != Some(ComponentId::new(source)) {
                continue;
            }
            if let Some(target) = component_id(&wire.to)
                && released.insert(target.index())
            {
                self.release(target.index(), false, queue);
            }
        }
    }

    fn branch_is_active(&self, source: usize, endpoint: &Endpoint) -> bool {
        match endpoint {
            Endpoint::ConditionOut(branch) => self.branches[source] == Some(*branch),
            _ => true,
        }
    }

    fn source_state(&self, source: usize, endpoint: &Endpoint) -> State {
        match endpoint {
            Endpoint::ConditionOut(_) => self.outputs[source].clone().unwrap_or_default(),
            _ => self.outputs[source]
                .clone()
                .expect("component output missing during propagation"),
        }
    }

    fn apply_wire(&mut self, wire: &Wire, state: &State) {
        match (&wire.to.owner, wire.to.endpoint) {
            (PortOwner::Component(component), Endpoint::State(channel)) => {
                set_channel(
                    &mut self.inputs[component.index()],
                    channel,
                    state_with_channel(channel, state),
                );
            }
            (PortOwner::Component(component), Endpoint::FluxSlot { slot, channel }) => {
                self.flux_slots[component.index()][slot] = Some(state_with_channel(channel, state));
            }
            (PortOwner::Component(component), Endpoint::ConditionIn) => {
                self.condition_inputs[component.index()] = Some(state.clone());
            }
            (PortOwner::Component(_), Endpoint::Trigger) => {}
            (PortOwner::BoundaryOutput, Endpoint::State(channel)) => {
                set_channel(
                    &mut self.result,
                    channel,
                    state_with_channel(channel, state),
                );
            }
            (PortOwner::BoundaryOutput, Endpoint::Done) => {}
            _ => unreachable!("wire validation accepted an unsupported endpoint pair"),
        }
    }

    fn release(&mut self, target: usize, active: bool, queue: &mut VecDeque<usize>) {
        if self.skipped[target] || self.outputs[target].is_some() {
            return;
        }
        if active {
            self.active_inputs[target] += 1;
        }
        self.pending[target] -= 1;
        if self.pending[target] != 0 {
            return;
        }
        if self.active_inputs[target] > 0 {
            queue.push_back(target);
        } else {
            self.skipped[target] = true;
            self.propagate_skip(target, queue);
        }
    }
}

fn component_id(port: &Port) -> Option<ComponentId> {
    match port.owner {
        PortOwner::Component(component) => Some(component),
        _ => None,
    }
}

fn state_with_channel(channel: Channel, source: &State) -> State {
    let mut state = State::default();
    match channel {
        Channel::Purpose => state.purpose.clone_from(&source.purpose),
        Channel::Context => state.ctx = source.ctx.clone(),
        Channel::Environment => state.env = source.env.clone(),
        Channel::Policy => state.policy = source.policy.clone(),
        Channel::Resources => state.res = source.res.clone(),
        Channel::Pulse => {}
    }
    state
}

fn set_channel(target: &mut State, channel: Channel, source: State) {
    match channel {
        Channel::Purpose => target.purpose = source.purpose,
        Channel::Context => target.ctx = source.ctx,
        Channel::Environment => target.env = source.env,
        Channel::Policy => target.policy = source.policy,
        Channel::Resources => target.res = source.res,
        Channel::Pulse => {}
    }
}
