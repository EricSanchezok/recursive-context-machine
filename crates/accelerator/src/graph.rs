use std::collections::{HashSet, VecDeque};

use machine::hook;
use tracing::{Instrument, warn};
use utils::{GraphId, Name};

use crate::accelerator::Accelerator;
use crate::condition::{Condition, ConditionBranch};
use crate::flux::{Flux, FluxMode};
use crate::wire::{Channel, ComponentId, ComponentRef, Endpoint, Port, PortOwner, Wire};
use machine::RunState;

/// Maximum number of components executed concurrently within a single frontier.
/// Bounds a wide `map` fan-out (one worker per item over a runtime-sized list) so
/// it does not launch hundreds of model / arXiv calls at once, while still
/// allowing healthy parallelism. Ordinary frontiers — sequential pipelines, the
/// handful of parallel scouts/judges — are smaller than this and run unaffected.
const FRONTIER_CONCURRENCY: usize = 16;

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
        self.validate_flux_inputs()?;
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

    pub async fn run(self, input: RunState) -> RunState {
        self.validate().expect("invalid graph");
        GraphRun::new(self, input).run().await
    }

    /// Run with specific per-component initial inputs (by component id), on top of
    /// any boundary-input wiring. Used by a `Map` to seed each dynamically-added
    /// worker with its own item before execution.
    pub async fn run_seeded(
        self,
        input: RunState,
        seeds: Vec<(ComponentId, RunState)>,
    ) -> RunState {
        self.validate().expect("invalid graph");
        let mut run = GraphRun::new(self, input);
        for (id, state) in seeds {
            run.inputs[id.index()] = state;
        }
        run.run().await
    }

    fn validate_flux_inputs(&self) -> Result<(), String> {
        let mut filled_slots = self
            .components
            .iter()
            .map(|component| match &component.kind {
                ComponentKind::Flux(flux) => vec![false; flux.arity],
                _ => Vec::new(),
            })
            .collect::<Vec<_>>();

        for wire in &self.wires {
            let Some(component) = component_id(&wire.to) else {
                continue;
            };
            let Endpoint::FluxSlot { slot, .. } = wire.to.endpoint else {
                continue;
            };
            if let Some(slot_filled) = filled_slots[component.index()].get_mut(slot) {
                *slot_filled = true;
            }
        }

        for (index, slots) in filled_slots.iter().enumerate() {
            if let Some(slot) = slots.iter().position(|filled| !filled) {
                return Err(format!(
                    "flux {} missing input for slot {}",
                    self.components[index].name.as_str(),
                    slot
                ));
            }
        }
        Ok(())
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

impl ComponentKind {
    fn name(&self) -> &'static str {
        match self {
            ComponentKind::Accelerator(_) => "accelerator",
            ComponentKind::Flux(_) => "flux",
            ComponentKind::Condition(_) => "condition",
        }
    }
}

struct GraphRun {
    graph: Graph,
    inputs: Vec<RunState>,
    outputs: Vec<Option<RunState>>,
    flux_slots: Vec<Vec<Option<RunState>>>,
    condition_inputs: Vec<Option<RunState>>,
    branches: Vec<Option<ConditionBranch>>,
    remaining_deps: Vec<usize>,
    active_incoming: Vec<usize>,
    skipped: Vec<bool>,
    result: RunState,
    next_frontier: u64,
}

impl GraphRun {
    fn new(graph: Graph, input: RunState) -> Self {
        let mut inputs = Vec::with_capacity(graph.components.len());
        let mut flux_slots = Vec::with_capacity(graph.components.len());

        for component in &graph.components {
            match &component.kind {
                ComponentKind::Accelerator(_) => {
                    inputs.push(RunState::default());
                    flux_slots.push(Vec::new());
                }
                ComponentKind::Flux(flux) => {
                    inputs.push(RunState::default());
                    flux_slots.push(vec![None; flux.arity]);
                }
                ComponentKind::Condition(_) => {
                    inputs.push(RunState::default());
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
            remaining_deps: vec![0; component_count],
            active_incoming: vec![0; component_count],
            skipped: vec![false; component_count],
            result: RunState::default(),
            next_frontier: 1,
        };

        run.count_dependencies();
        run.apply_boundary_input(&input);
        run
    }

    async fn run(mut self) -> RunState {
        hook!(event = "graph_start", graph = self.graph.name.as_str());
        let mut queue = VecDeque::new();
        for (index, count) in self.remaining_deps.iter().enumerate() {
            if *count == 0 {
                queue.push_back(index);
            }
        }

        while !queue.is_empty() {
            let frontier = self.drain_ready_frontier(&mut queue);
            let completed = self.run_frontier(frontier).await;
            for index in completed {
                self.propagate_component(index, &mut queue);
            }
        }

        hook!(event = "graph_done", graph = self.graph.name.as_str());
        self.result
    }

    fn drain_ready_frontier(&self, queue: &mut VecDeque<usize>) -> Vec<usize> {
        let mut frontier = Vec::new();
        while let Some(index) = queue.pop_front() {
            if self.skipped[index] || self.outputs[index].is_some() {
                continue;
            }
            frontier.push(index);
        }
        frontier
    }

    async fn run_frontier(&mut self, frontier: Vec<usize>) -> Vec<usize> {
        let frontier_id = self.next_frontier;
        self.next_frontier += 1;
        hook!(
            event = "frontier_start",
            graph = self.graph.name.as_str(),
            frontier = frontier_id,
            count = frontier.len()
        );
        let mut completed = Vec::new();
        // Run the frontier in bounded waves so a wide `map` fan-out cannot launch
        // hundreds of model/arXiv calls at once. Each wave is fully awaited before
        // the next starts.
        for chunk in frontier.chunks(FRONTIER_CONCURRENCY) {
            let mut tasks = tokio::task::JoinSet::new();
            for &index in chunk {
                let component = self.graph.components[index].clone();
                let input = self.inputs[index].clone();
                let flux_slots = self.flux_slots[index].clone();
                let condition_input = self.condition_inputs[index].clone();
                let graph_name = self.graph.name.to_string();
                let component_name = component.name.to_string();
                let component_kind = component.kind.name();
                let span = tracing::trace_span!(
                    target: "hook",
                    "component",
                    graph = graph_name.as_str(),
                    frontier = frontier_id,
                    component = component_name.as_str(),
                    component_index = index,
                    component_kind
                );
                tasks.spawn(
                    async move {
                        hook!(
                            event = "component_start",
                            graph = graph_name.as_str(),
                            frontier = frontier_id,
                            component = component_name.as_str(),
                            component_index = index,
                            component_kind
                        );
                        let (state, branch) = match component.kind {
                            ComponentKind::Accelerator(accelerator) => {
                                (accelerator.run_with(input).await, None)
                            }
                            ComponentKind::Flux(flux) => {
                                let slots = flux_slots
                                    .into_iter()
                                    .map(|slot| slot.unwrap_or_default())
                                    .collect::<Vec<_>>();
                                (flux.apply(&slots), None)
                            }
                            ComponentKind::Condition(condition) => {
                                let state = condition_input.unwrap_or_default();
                                let branch = condition.route(&state);
                                (state, Some(branch))
                            }
                        };
                        hook!(
                            event = "component_done",
                            graph = graph_name.as_str(),
                            frontier = frontier_id,
                            component = component_name.as_str(),
                            component_index = index,
                            component_kind
                        );
                        (index, state, branch)
                    }
                    .instrument(span),
                );
            }

            while let Some(result) = tasks.join_next().await {
                let Ok((index, state, branch)) = result else {
                    if let Err(error) = result {
                        warn!(?error, "graph component task panicked");
                    }
                    continue;
                };
                if let Some(branch) = branch {
                    self.branches[index] = Some(branch);
                }
                self.outputs[index] = Some(state);
                completed.push(index);
            }
        }
        hook!(
            event = "frontier_done",
            graph = self.graph.name.as_str(),
            frontier = frontier_id,
            count = completed.len()
        );
        completed
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
                self.remaining_deps[target.index()] += 1;
            }
        }
    }

    fn apply_boundary_input(&mut self, input: &RunState) {
        let wire_indices = self
            .graph
            .wires
            .iter()
            .enumerate()
            .filter_map(|(index, wire)| {
                (wire.from.owner == PortOwner::BoundaryInput).then_some(index)
            })
            .collect::<Vec<_>>();
        for index in wire_indices {
            let wire = self.graph.wires[index].clone();
            self.apply_wire(&wire, input);
        }
    }

    fn propagate_component(&mut self, source: usize, queue: &mut VecDeque<usize>) {
        let source_id = ComponentId::new(source);
        let wire_indices = self
            .graph
            .wires
            .iter()
            .enumerate()
            .filter_map(|(index, wire)| {
                (component_id(&wire.from) == Some(source_id)).then_some(index)
            })
            .collect::<Vec<_>>();
        let mut released = HashSet::new();
        for index in wire_indices {
            let wire = self.graph.wires[index].clone();
            let active = self.branch_is_active(source, &wire.from.endpoint);
            if active {
                let state = self.source_state(source, &wire.from.endpoint);
                self.apply_wire(&wire, &state);
            }
            if let Some(target) = component_id(&wire.to)
                && released.insert(target.index())
            {
                self.resolve_dependency(target.index(), active, queue);
            }
        }
    }

    fn propagate_skip(&mut self, source: usize, queue: &mut VecDeque<usize>) {
        let source_id = ComponentId::new(source);
        let wire_indices = self
            .graph
            .wires
            .iter()
            .enumerate()
            .filter_map(|(index, wire)| {
                (component_id(&wire.from) == Some(source_id)).then_some(index)
            })
            .collect::<Vec<_>>();
        let mut released = HashSet::new();
        for index in wire_indices {
            let wire = self.graph.wires[index].clone();
            if let Some(target) = component_id(&wire.to)
                && released.insert(target.index())
            {
                self.resolve_dependency(target.index(), false, queue);
            }
        }
    }

    fn branch_is_active(&self, source: usize, endpoint: &Endpoint) -> bool {
        match endpoint {
            Endpoint::ConditionOut(branch) => self.branches[source] == Some(*branch),
            _ => true,
        }
    }

    fn source_state(&self, source: usize, endpoint: &Endpoint) -> RunState {
        match endpoint {
            Endpoint::ConditionOut(_) => self.outputs[source].clone().unwrap_or_default(),
            _ => self.outputs[source]
                .clone()
                .expect("component output missing during propagation"),
        }
    }

    fn apply_wire(&mut self, wire: &Wire, state: &RunState) {
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

    fn resolve_dependency(
        &mut self,
        target: usize,
        active_branch: bool,
        queue: &mut VecDeque<usize>,
    ) {
        if self.skipped[target] || self.outputs[target].is_some() {
            return;
        }
        if active_branch {
            self.active_incoming[target] += 1;
        }
        self.remaining_deps[target] -= 1;
        if self.remaining_deps[target] != 0 {
            return;
        }
        if self.active_incoming[target] > 0 {
            queue.push_back(target);
        } else {
            self.skipped[target] = true;
            hook!(
                event = "component_skipped",
                graph = self.graph.name.as_str(),
                component = self.graph.components[target].name.as_str(),
                component_index = target,
                component_kind = self.graph.components[target].kind.name()
            );
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

fn state_with_channel(channel: Channel, source: &RunState) -> RunState {
    let mut state = RunState::default();
    match channel {
        Channel::Purpose => state.purpose.clone_from(&source.purpose),
        Channel::Context => state.context = source.context.clone(),
        Channel::Environment => state.environment = source.environment.clone(),
        Channel::Resources => state.resources = source.resources.clone(),
        Channel::Pulse => {}
    }
    state
}

fn set_channel(target: &mut RunState, channel: Channel, source: RunState) {
    match channel {
        Channel::Purpose => target.purpose = source.purpose,
        Channel::Context => target.context = source.context,
        Channel::Environment => target.environment = source.environment,
        Channel::Resources => target.resources = source.resources,
        Channel::Pulse => {}
    }
}
