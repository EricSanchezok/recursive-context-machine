use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use accelerator::{
    Accelerator, Channel, ComponentKind, ContextFlux, Endpoint, FluxMode, Graph, ResFlux, State,
};
use machine::{Action, Context, Environment, Fragment, Inbox, Model, Policy, Purpose, Resources};

#[derive(Clone)]
struct DonePolicy;

#[derive(Clone)]
struct BarrierPolicy {
    barrier: Arc<tokio::sync::Barrier>,
}

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

impl Policy for BarrierPolicy {
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
        Box::pin(async move {
            self.barrier.wait().await;
            Action::Done
        })
    }
}

fn state_with_purpose(purpose: &str) -> State {
    State {
        purpose: purpose.to_string(),
        policy: Box::new(DonePolicy),
        ..State::default()
    }
}

fn state_with_policy(purpose: &str, policy: Box<dyn Policy>) -> State {
    State {
        purpose: purpose.to_string(),
        policy,
        ..State::default()
    }
}

fn model(name: &str) -> Model {
    Model {
        name: name.to_string(),
        ..Default::default()
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

#[test]
fn resource_flux_preserves_model_order_and_tool_pool() {
    let mut first_state = state_with_purpose("first");
    first_state.res = Resources::named("first")
        .with_model(model("fast"))
        .with_tool(Arc::new(accelerator::tools::FindTool));
    let mut second_state = state_with_purpose("second");
    second_state.res = Resources::named("second")
        .with_model(model("careful"))
        .with_tool(Arc::new(accelerator::tools::ShellTool));

    let mut graph = Graph::new();
    let first = graph.add_accelerator("first", Accelerator::primitive(first_state));
    let second = graph.add_accelerator("second", Accelerator::primitive(second_state));
    let join = graph.add_flux("join", FluxMode::Resources(ResFlux::Merge), 2);

    graph.wire(first.resources(), join.slot(0, Channel::Resources));
    graph.wire(second.resources(), join.slot(1, Channel::Resources));
    graph.wire(
        join.flux_out(Channel::Resources),
        Graph::output(Endpoint::State(Channel::Resources)),
    );

    let output = run(Accelerator::composite_named("resources", graph));

    assert_eq!(output.res.model_order, vec!["fast", "careful"]);
    assert!(output.res.tools.contains_key("find"));
    assert!(output.res.tools.contains_key("shell"));
}

#[test]
fn independent_accelerators_run_in_parallel() {
    let barrier = Arc::new(tokio::sync::Barrier::new(2));
    let mut graph = Graph::new();
    graph.add_accelerator(
        "first",
        Accelerator::primitive(state_with_policy(
            "first",
            Box::new(BarrierPolicy {
                barrier: barrier.clone(),
            }),
        )),
    );
    graph.add_accelerator(
        "second",
        Accelerator::primitive(state_with_policy(
            "second",
            Box::new(BarrierPolicy { barrier }),
        )),
    );

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let completed = runtime.block_on(async {
        tokio::time::timeout(
            Duration::from_millis(200),
            Accelerator::composite_named("parallel", graph).run(),
        )
        .await
        .is_ok()
    });

    assert!(completed);
}

#[test]
fn downstream_waits_for_parallel_sources() {
    let mut first_state = state_with_purpose("first");
    first_state.ctx.append(Fragment::assistant("first"));
    let mut second_state = state_with_purpose("second");
    second_state.ctx.append(Fragment::assistant("second"));

    let mut graph = Graph::new();
    let first = graph.add_accelerator("first", Accelerator::primitive(first_state));
    let second = graph.add_accelerator("second", Accelerator::primitive(second_state));
    let join = graph.add_flux("join", FluxMode::Context(ContextFlux::Append), 2);

    graph.wire(first.context(), join.slot(0, Channel::Context));
    graph.wire(second.context(), join.slot(1, Channel::Context));
    graph.wire(
        join.flux_out(Channel::Context),
        Graph::output(Endpoint::State(Channel::Context)),
    );

    let output = run(Accelerator::composite_named("join", graph));

    assert_eq!(output.ctx.fragments().len(), 2);
}
