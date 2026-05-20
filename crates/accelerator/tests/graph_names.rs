use std::future::Future;
use std::pin::Pin;

use accelerator::{
    Accelerator, Channel, ComponentKind, ContextFlux, Endpoint, FluxMode, Graph, State,
};
use machine::{Action, Context, Environment, Fragment, Inbox, Policy, Purpose, Resources};

#[derive(Clone)]
struct DonePolicy;

impl Policy for DonePolicy {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn decide<'a>(
        &'a self,
        _purpose: &'a Purpose,
        _ctx: &'a Context,
        _env: &'a Environment,
        _resources: &'a Resources,
        _inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async { Action::Done })
    }
}

fn state_with_purpose(purpose: &str) -> State {
    State {
        purpose: purpose.to_string(),
        policy: Box::new(DonePolicy),
        ..State::default()
    }
}

fn run(accelerator: Accelerator) -> State {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async { accelerator.run().await })
}

#[test]
fn primitive_accelerator_keeps_its_id_when_cloned() {
    let accelerator = Accelerator::primitive_named("review", state_with_purpose("review"));
    let clone = accelerator.clone();

    assert_eq!(accelerator.id(), clone.id());
}

#[test]
fn graph_components_have_stable_component_ids() {
    let mut graph = Graph::new();
    let first = graph.add_accelerator("first", Accelerator::primitive(state_with_purpose("first")));
    let second = graph.add_accelerator(
        "second",
        Accelerator::primitive(state_with_purpose("second")),
    );

    assert_ne!(first.id(), second.id());
}

#[test]
fn graph_and_component_names_are_independent() {
    let mut graph = Graph::named("pipeline");
    graph.add_accelerator(
        "pipeline",
        Accelerator::primitive(state_with_purpose("leaf")),
    );

    assert_eq!(graph.name.as_str(), "pipeline");
}

#[test]
fn flux_and_accelerator_are_distinct_component_kinds() {
    let mut graph = Graph::new();
    graph.add_accelerator("agent", Accelerator::primitive(state_with_purpose("agent")));
    graph.add_flux("join", FluxMode::Context(ContextFlux::Append), 1);

    let accelerator = &graph.components()[0];
    let flux = &graph.components()[1];

    assert!(matches!(accelerator.kind, ComponentKind::Accelerator(_)));
    assert!(matches!(flux.kind, ComponentKind::Flux(_)));
}

#[test]
fn composite_accelerator_routes_context_to_output() {
    let mut source_state = state_with_purpose("source");
    source_state.ctx.append(Fragment::assistant("done"));

    let mut graph = Graph::new();
    let source = graph.add_accelerator("source", Accelerator::primitive(source_state));
    graph.wire(
        source.context(),
        Graph::output(Endpoint::State(Channel::Context)),
    );
    graph.wire(source.done(), Graph::output(Endpoint::Done));

    let output = run(Accelerator::composite_named("pipeline", graph));

    assert_eq!(output.ctx.fragments().len(), 1);
}
