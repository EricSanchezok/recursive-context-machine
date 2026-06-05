mod common;

use accelerator::{
    Accelerator, Channel, ConditionBranch, ContextFlux, ContextPredicate, Endpoint,
    EnvironmentPredicate, FluxMode, Graph, Predicate, PurposePredicate, ResourcesPredicate,
};
use machine::{Fragment, Purpose, RunState, ToolRuntime};

fn state_with_purpose(purpose: &str) -> RunState {
    RunState {
        purpose: Purpose::new(purpose),
        ..RunState::default()
    }
}

fn state_with_context(text: &str) -> RunState {
    let mut state = RunState {
        purpose: Purpose::new(text),
        ..RunState::default()
    };
    state.context.append(Fragment::assistant(text));
    state
}

fn primitive_with_context(text: &str) -> Accelerator {
    Accelerator::primitive(
        state_with_context(text),
        Box::new(common::DonePolicy),
        ToolRuntime::new(),
        text,
    )
}

fn run(graph: Graph) -> RunState {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        Accelerator::composite(graph)
            .run_with(RunState::default())
            .await
    })
}

#[test]
fn predicate_composition_reads_purpose_and_environment() {
    let mut state = state_with_purpose("done: ready");
    state.environment.vars.insert("READY".into(), "yes".into());

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
        .context
        .append(Fragment::assistant("research complete").with_tag("research"));
    state.resources.prompts.insert("judge".into(), "...".into());

    assert!(Predicate::Context(ContextPredicate::HasTag("research".into())).evaluate(&state));
    assert!(Predicate::Context(ContextPredicate::Contains("complete".into())).evaluate(&state));
    assert!(Predicate::Resources(ResourcesPredicate::HasPrompt("judge".into())).evaluate(&state));
}

#[test]
fn condition_routes_true_branch() {
    let mut graph = Graph::new();
    let source = graph.add_accelerator("source", common::primitive("done"));
    let true_target = graph.add_accelerator("true", common::primitive("true-target"));
    let false_target = graph.add_accelerator("false", common::primitive("false-target"));
    let condition = graph.add_condition(
        "route",
        Predicate::Purpose(PurposePredicate::Contains("done".into())),
    );

    graph.wire(source.done(), condition.condition_in());
    graph.wire(
        condition.condition_out(ConditionBranch::True),
        true_target.trigger(),
    );
    graph.wire(
        condition.condition_out(ConditionBranch::False),
        false_target.trigger(),
    );
    graph.wire(
        true_target.purpose(),
        Graph::output(Endpoint::State(Channel::Purpose)),
    );
    graph.wire(
        false_target.purpose(),
        Graph::output(Endpoint::State(Channel::Purpose)),
    );

    let output = run(graph);
    assert_eq!(output.purpose.text, "true-target");
}

#[test]
fn condition_routes_false_branch() {
    let mut graph = Graph::new();
    let source = graph.add_accelerator("source", common::primitive("retry"));
    let true_target = graph.add_accelerator("true", common::primitive("true-target"));
    let false_target = graph.add_accelerator("false", common::primitive("false-target"));
    let condition = graph.add_condition(
        "route",
        Predicate::Purpose(PurposePredicate::Contains("done".into())),
    );

    graph.wire(source.done(), condition.condition_in());
    graph.wire(
        condition.condition_out(ConditionBranch::True),
        true_target.trigger(),
    );
    graph.wire(
        condition.condition_out(ConditionBranch::False),
        false_target.trigger(),
    );
    graph.wire(
        true_target.purpose(),
        Graph::output(Endpoint::State(Channel::Purpose)),
    );
    graph.wire(
        false_target.purpose(),
        Graph::output(Endpoint::State(Channel::Purpose)),
    );

    let output = run(graph);
    assert_eq!(output.purpose.text, "false-target");
}

#[test]
fn selected_branch_can_rejoin_after_unselected_branch_is_skipped() {
    let mut graph = Graph::new();
    let source = graph.add_accelerator("source", common::primitive("done"));
    let true_target = graph.add_accelerator("true", common::primitive("true-target"));
    let false_target = graph.add_accelerator("false", common::primitive("false-target"));
    let join = graph.add_accelerator("join", common::primitive("joined"));
    let condition = graph.add_condition(
        "route",
        Predicate::Purpose(PurposePredicate::Contains("done".into())),
    );

    graph.wire(source.done(), condition.condition_in());
    graph.wire(
        condition.condition_out(ConditionBranch::True),
        true_target.trigger(),
    );
    graph.wire(
        condition.condition_out(ConditionBranch::False),
        false_target.trigger(),
    );
    graph.wire(true_target.done(), join.trigger());
    graph.wire(false_target.done(), join.trigger());
    graph.wire(
        join.purpose(),
        Graph::output(Endpoint::State(Channel::Purpose)),
    );

    let output = run(graph);
    assert_eq!(output.purpose.text, "joined");
}

#[test]
fn skipped_branch_contributes_empty_flux_slot() {
    let mut graph = Graph::new();
    let source = graph.add_accelerator("source", common::primitive("done"));
    let true_target = graph.add_accelerator("true", primitive_with_context("true"));
    let false_target = graph.add_accelerator("false", primitive_with_context("false"));
    let join = graph.add_flux("join", FluxMode::Context(ContextFlux::Append), 2);
    let condition = graph.add_condition(
        "route",
        Predicate::Purpose(PurposePredicate::Contains("done".into())),
    );

    graph.wire(source.done(), condition.condition_in());
    graph.wire(
        condition.condition_out(ConditionBranch::True),
        true_target.trigger(),
    );
    graph.wire(
        condition.condition_out(ConditionBranch::False),
        false_target.trigger(),
    );
    graph.wire(true_target.context(), join.slot(0, Channel::Context));
    graph.wire(false_target.context(), join.slot(1, Channel::Context));
    graph.wire(
        join.flux_out(Channel::Context),
        Graph::output(Endpoint::State(Channel::Context)),
    );

    let output = run(graph);
    assert_eq!(output.context.fragments().len(), 1);
}
