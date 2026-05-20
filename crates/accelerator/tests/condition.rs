use std::future::Future;
use std::pin::Pin;

use accelerator::{
    ContextPredicate, EnvironmentPredicate, Graph, Predicate, PurposePredicate, ResourcesPredicate,
    State,
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

#[test]
fn predicate_composition_reads_purpose_and_environment() {
    let mut state = state_with_purpose("done: ready");
    state.env.vars.insert("READY".into(), "yes".into());

    let predicate = Predicate::All(vec![
        Predicate::Purpose(PurposePredicate::Contains("done".into())),
        Predicate::Environment(EnvironmentPredicate::VarEquals(
            "READY".into(),
            "yes".into(),
        )),
    ]);

    assert!(predicate.evaluate(&state));
}

#[test]
fn context_and_resources_predicates_cover_common_cases() {
    let mut state = state_with_purpose("irrelevant");
    state
        .ctx
        .append(Fragment::assistant("research complete").with_tag("research"));
    state.res.prompts.insert("judge".into(), "...".into());

    assert!(Predicate::Context(ContextPredicate::HasTag("research".into())).evaluate(&state));
    assert!(Predicate::Context(ContextPredicate::Contains("complete".into())).evaluate(&state));
    assert!(Predicate::Resources(ResourcesPredicate::HasPrompt("judge".into())).evaluate(&state));
}

#[test]
fn condition_routes_true_branch() {
    let mut graph = Graph::new();
    let source = graph.spawn_named("source", state_with_purpose("done"));
    let true_target = graph.spawn_named("true", state_with_purpose("true-target"));
    let false_target = graph.spawn_named("false", state_with_purpose("false-target"));
    let condition = graph.condition_named(
        "route",
        Predicate::Purpose(PurposePredicate::Contains("done".into())),
    );

    graph.wire(source.done(), condition.trigger());
    graph.wire(condition.pulse_true(), true_target.trigger());
    graph.wire(condition.pulse_false(), false_target.trigger());

    let outputs = run(graph);
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].purpose, "true-target");
}

#[test]
fn condition_routes_false_branch() {
    let mut graph = Graph::new();
    let source = graph.spawn_named("source", state_with_purpose("retry"));
    let true_target = graph.spawn_named("true", state_with_purpose("true-target"));
    let false_target = graph.spawn_named("false", state_with_purpose("false-target"));
    let condition = graph.condition_named(
        "route",
        Predicate::Purpose(PurposePredicate::Contains("done".into())),
    );

    graph.wire(source.done(), condition.trigger());
    graph.wire(condition.pulse_true(), true_target.trigger());
    graph.wire(condition.pulse_false(), false_target.trigger());

    let outputs = run(graph);
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].purpose, "false-target");
}

#[test]
fn selected_branch_can_rejoin_after_unselected_branch_is_skipped() {
    let mut graph = Graph::new();
    let source = graph.spawn_named("source", state_with_purpose("done"));
    let true_target = graph.spawn_named("true", state_with_purpose("true-target"));
    let false_target = graph.spawn_named("false", state_with_purpose("false-target"));
    let join = graph.spawn_named("join", state_with_purpose("joined"));
    let condition = graph.condition_named(
        "route",
        Predicate::Purpose(PurposePredicate::Contains("done".into())),
    );

    graph.wire(source.done(), condition.trigger());
    graph.wire(condition.pulse_true(), true_target.trigger());
    graph.wire(condition.pulse_false(), false_target.trigger());
    graph.wire(true_target.done(), join.trigger());
    graph.wire(false_target.done(), join.trigger());

    let outputs = run(graph);
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].purpose, "joined");
}

#[test]
fn missing_condition_branch_is_invalid() {
    let mut graph = Graph::new();
    let source = graph.spawn_named("source", state_with_purpose("done"));
    let target = graph.spawn_named("target", state_with_purpose("target"));
    let condition = graph.condition(Predicate::Purpose(PurposePredicate::Contains(
        "done".into(),
    )));

    graph.wire(source.done(), condition.trigger());
    graph.wire(condition.pulse_true(), target.trigger());

    assert!(graph.build().is_err());
}

#[test]
#[should_panic(expected = "condition port does not belong to this graph")]
fn stale_condition_port_is_rejected() {
    let mut first = Graph::new();
    let mut second = Graph::new();
    let source = second.spawn_named("source", state_with_purpose("done"));
    let condition = first.condition(Predicate::Purpose(PurposePredicate::Contains(
        "done".into(),
    )));

    second.wire(source.done(), condition.trigger());
}

fn run(graph: Graph) -> Vec<State> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async { graph.build().unwrap().run().await })
}
