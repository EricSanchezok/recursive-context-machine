mod common;

use std::sync::Arc;
use std::time::Duration;

use accelerator::{
    Accelerator, Captain, Channel, ComponentKind, ContextFlux, Endpoint, FluxMode, Graph, ResFlux,
    State,
};
use machine::{
    Environment, Fragment, Model, Policy, Purpose, Resources, Role, ToolDefinition, ToolRuntime,
};

use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
struct BarrierPolicy {
    barrier: Arc<tokio::sync::Barrier>,
}

impl Policy for BarrierPolicy {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn decide<'a>(
        &'a self,
        _purpose: &'a Purpose,
        _ctx: &'a machine::Context,
        _env: &'a Environment,
        _resources: &'a Resources,
        _inbox: &'a machine::Inbox,
    ) -> Pin<Box<dyn Future<Output = machine::Action> + Send + 'a>> {
        Box::pin(async move {
            self.barrier.wait().await;
            machine::Action::Done
        })
    }
}

fn primitive_with_policy(purpose: &str, policy: Box<dyn Policy>) -> Accelerator {
    Accelerator::primitive(
        State {
            purpose: purpose.to_string(),
            ..State::default()
        },
        policy,
        ToolRuntime::new(),
        purpose,
    )
}

fn run(accelerator: Accelerator) -> State {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async { accelerator.run_with(State::default()).await })
}

#[test]
fn primitive_accelerator_keeps_its_id_when_cloned() {
    let accelerator = common::primitive("review");
    let clone = accelerator.clone();

    assert_eq!(accelerator.id(), clone.id());
}

#[test]
fn graph_components_have_stable_component_ids() {
    let mut graph = Graph::new();
    let first = graph.add_accelerator("first", common::primitive("first"));
    let second = graph.add_accelerator("second", common::primitive("second"));

    assert_ne!(first.id(), second.id());
}

#[test]
fn graph_and_component_names_are_independent() {
    let mut graph = Graph::named("pipeline");
    graph.add_accelerator("pipeline", common::primitive("leaf"));

    assert_eq!(graph.name.as_str(), "pipeline");
}

#[test]
fn flux_and_accelerator_are_distinct_component_kinds() {
    let mut graph = Graph::new();
    graph.add_accelerator("agent", common::primitive("agent"));
    graph.add_flux("join", FluxMode::Context(ContextFlux::Append), 1);

    let accelerator = &graph.components()[0];
    let flux = &graph.components()[1];

    assert!(matches!(accelerator.kind, ComponentKind::Accelerator(_)));
    assert!(matches!(flux.kind, ComponentKind::Flux(_)));
}

#[test]
fn composite_accelerator_routes_context_to_output() {
    let mut source_state = State {
        purpose: "source".into(),
        ..State::default()
    };
    source_state.ctx.append(Fragment::assistant("done"));

    let mut graph = Graph::new();
    let source = graph.add_accelerator(
        "source",
        Accelerator::primitive(
            source_state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "source",
        ),
    );
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
    let first_state = State {
        purpose: "first".into(),
        res: Resources::named("first")
            .with_model(Model {
                name: "fast".into(),
                ..Default::default()
            })
            .with_tool_definition(ToolDefinition::from_tool(&accelerator::tools::FindTool)),
        ..State::default()
    };
    let second_state = State {
        purpose: "second".into(),
        res: Resources::named("second")
            .with_model(Model {
                name: "careful".into(),
                ..Default::default()
            })
            .with_tool_definition(ToolDefinition::from_tool(&accelerator::tools::ShellTool)),
        ..State::default()
    };

    let mut graph = Graph::new();
    let first = graph.add_accelerator(
        "first",
        Accelerator::primitive(
            first_state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "first",
        ),
    );
    let second = graph.add_accelerator(
        "second",
        Accelerator::primitive(
            second_state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "second",
        ),
    );
    let join = graph.add_flux("join", FluxMode::Resources(ResFlux::Merge), 2);

    graph.wire(first.resources(), join.slot(0, Channel::Resources));
    graph.wire(second.resources(), join.slot(1, Channel::Resources));
    graph.wire(
        join.flux_out(Channel::Resources),
        Graph::output(Endpoint::State(Channel::Resources)),
    );

    let output = run(Accelerator::composite_named("resources", graph));

    assert_eq!(output.res.model_order, vec!["fast", "careful"]);
    assert!(output.res.tool_definitions.contains_key("find"));
    assert!(output.res.tool_definitions.contains_key("shell"));
}

#[test]
fn independent_accelerators_run_in_parallel() {
    let barrier = Arc::new(tokio::sync::Barrier::new(2));
    let mut graph = Graph::new();
    graph.add_accelerator(
        "first",
        primitive_with_policy(
            "first",
            Box::new(BarrierPolicy {
                barrier: barrier.clone(),
            }),
        ),
    );
    graph.add_accelerator(
        "second",
        primitive_with_policy("second", Box::new(BarrierPolicy { barrier })),
    );

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let completed = runtime.block_on(async {
        tokio::time::timeout(
            Duration::from_millis(200),
            Accelerator::composite_named("parallel", graph).run_with(State::default()),
        )
        .await
        .is_ok()
    });

    assert!(completed);
}

#[test]
fn downstream_waits_for_parallel_sources() {
    let mut first_state = State {
        purpose: "first".into(),
        ..State::default()
    };
    first_state.ctx.append(Fragment::assistant("first"));
    let mut second_state = State {
        purpose: "second".into(),
        ..State::default()
    };
    second_state.ctx.append(Fragment::assistant("second"));

    let mut graph = Graph::new();
    let first = graph.add_accelerator(
        "first",
        Accelerator::primitive(
            first_state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "first",
        ),
    );
    let second = graph.add_accelerator(
        "second",
        Accelerator::primitive(
            second_state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "second",
        ),
    );
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

#[test]
fn context_last_keeps_only_last_fragment_per_slot() {
    let mut first_state = State {
        purpose: "first".into(),
        ..State::default()
    };
    first_state.ctx.append(Fragment::assistant("first-a"));
    first_state.ctx.append(Fragment::assistant("first-b"));

    let mut second_state = State {
        purpose: "second".into(),
        ..State::default()
    };
    second_state.ctx.append(Fragment::assistant("second-a"));
    second_state.ctx.append(Fragment::assistant("second-b"));

    let mut graph = Graph::new();
    let first = graph.add_accelerator(
        "first",
        Accelerator::primitive(
            first_state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "first",
        ),
    );
    let second = graph.add_accelerator(
        "second",
        Accelerator::primitive(
            second_state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "second",
        ),
    );
    let join = graph.add_flux("join", FluxMode::Context(ContextFlux::Last), 2);

    graph.wire(first.context(), join.slot(0, Channel::Context));
    graph.wire(second.context(), join.slot(1, Channel::Context));
    graph.wire(
        join.flux_out(Channel::Context),
        Graph::output(Endpoint::State(Channel::Context)),
    );

    let output = run(Accelerator::composite_named("last", graph));

    assert_eq!(output.ctx.fragments().len(), 2);
    assert_eq!(output.ctx.fragments()[0].as_text(), Some("first-b"));
    assert_eq!(output.ctx.fragments()[1].as_text(), Some("second-b"));
}

#[test]
fn context_digest_extracts_key_segments() {
    let mut state = State {
        purpose: "search".into(),
        ..State::default()
    };
    // System prompt (dropped)
    state.ctx.append(Fragment::system("You are a search agent"));
    // Tool call (dropped)
    state.ctx.append(Fragment::tool_call(
        "tc1",
        "arxiv_search",
        serde_json::json!({"query": "quantum"}),
    ));
    // Tool result (kept)
    state.ctx.append(Fragment::tool_result(
        "tc1",
        "Found 3 papers on quantum computing",
        None,
    ));
    // Final answer (kept)
    state
        .ctx
        .append(Fragment::assistant("Here are the top papers..."));

    let mut graph = Graph::new();
    let source = graph.add_accelerator(
        "source",
        Accelerator::primitive(
            state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "source",
        ),
    );
    let digest = graph.add_flux("digest", FluxMode::Context(ContextFlux::Digest), 1);

    graph.wire(source.context(), digest.slot(0, Channel::Context));
    graph.wire(
        digest.flux_out(Channel::Context),
        Graph::output(Endpoint::State(Channel::Context)),
    );

    let output = run(Accelerator::composite_named("digest", graph));

    assert_eq!(output.ctx.fragments().len(), 1);
    let text = output.ctx.fragments()[0].as_text().unwrap();
    assert!(
        text.contains("[Tool result]"),
        "digest should include tool result"
    );
    assert!(
        text.contains("Here are the top papers"),
        "digest should include final answer"
    );
    assert!(
        !text.contains("You are a search agent"),
        "digest should drop system prompt"
    );
}

#[test]
fn context_thread_assembles_qa_pairs() {
    let mut first_state = State {
        purpose: "search papers".into(),
        ..State::default()
    };
    first_state
        .ctx
        .append(Fragment::assistant("Found 3 papers on quantum computing"));

    let mut second_state = State {
        purpose: "download best".into(),
        ..State::default()
    };
    second_state
        .ctx
        .append(Fragment::assistant("Downloaded arxiv:2401.12345.pdf"));

    let mut graph = Graph::new();
    let first = graph.add_accelerator(
        "first",
        Accelerator::primitive(
            first_state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "first",
        ),
    );
    let second = graph.add_accelerator(
        "second",
        Accelerator::primitive(
            second_state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "second",
        ),
    );
    let thread = graph.add_flux("thread", FluxMode::Context(ContextFlux::Thread), 2);

    graph.wire(first.context(), thread.slot(0, Channel::Context));
    graph.wire(second.context(), thread.slot(1, Channel::Context));
    graph.wire(
        thread.flux_out(Channel::Context),
        Graph::output(Endpoint::State(Channel::Context)),
    );

    let output = run(Accelerator::composite_named("thread", graph));

    // Each slot contributes 2 fragments: user question + assistant answer.
    assert_eq!(output.ctx.fragments().len(), 4);

    // Slot 0: user question
    assert_eq!(output.ctx.fragments()[0].role, machine::Role::User);
    assert!(
        output.ctx.fragments()[0]
            .as_text()
            .unwrap()
            .contains("Task 1")
    );
    // Slot 0: answer
    assert_eq!(output.ctx.fragments()[1].role, machine::Role::Assistant);
    assert_eq!(
        output.ctx.fragments()[1].as_text(),
        Some("Found 3 papers on quantum computing")
    );

    // Slot 1: user question
    assert_eq!(output.ctx.fragments()[2].role, machine::Role::User);
    assert!(
        output.ctx.fragments()[2]
            .as_text()
            .unwrap()
            .contains("Task 2")
    );
    // Slot 1: answer
    assert_eq!(output.ctx.fragments()[3].role, machine::Role::Assistant);
    assert_eq!(
        output.ctx.fragments()[3].as_text(),
        Some("Downloaded arxiv:2401.12345.pdf")
    );
}

// ── ContextFlux::Fold tests ──

#[test]
fn context_fold_extracts_last_assistant_into_payload() {
    let mut state = State {
        purpose: "search".into(),
        ..State::default()
    };
    state.ctx.append(Fragment::assistant("first message"));
    state.ctx.append(Fragment::assistant("second message"));

    let mut graph = Graph::new();
    let source = graph.add_accelerator(
        "source",
        Accelerator::primitive(
            state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "source",
        ),
    );
    let fold = graph.add_flux("fold", FluxMode::Context(ContextFlux::Fold), 1);

    graph.wire(source.context(), fold.slot(0, Channel::Context));
    graph.wire(
        fold.flux_out(Channel::Context),
        Graph::output(Endpoint::State(Channel::Context)),
    );

    let output = run(Accelerator::composite_named("fold-test", graph));

    assert_eq!(output.ctx.fragments().len(), 0, "fold ctx should be empty");
    assert!(
        output.fold_payload.contains("second message"),
        "fold_payload should contain the last assistant text"
    );
    assert!(
        !output.fold_payload.contains("first message"),
        "fold_payload should NOT contain earlier non-last texts"
    );
}

#[test]
fn context_fold_single_assistant() {
    let mut state = State {
        purpose: "search".into(),
        ..State::default()
    };
    state.ctx.append(Fragment::assistant("single result"));

    let mut graph = Graph::new();
    let source = graph.add_accelerator(
        "source",
        Accelerator::primitive(
            state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "source",
        ),
    );
    let fold = graph.add_flux("fold", FluxMode::Context(ContextFlux::Fold), 1);

    graph.wire(source.context(), fold.slot(0, Channel::Context));
    graph.wire(
        fold.flux_out(Channel::Context),
        Graph::output(Endpoint::State(Channel::Context)),
    );

    let output = run(Accelerator::composite_named("fold-single", graph));
    assert_eq!(output.fold_payload, "single result");
}

#[test]
fn context_fold_ignores_system_and_tool_fragments() {
    let mut state = State {
        purpose: "search".into(),
        ..State::default()
    };
    state
        .ctx
        .append(Fragment::system("You are a helpful assistant"));
    state
        .ctx
        .append(Fragment::tool_result("tc1", "tool output", None));
    state.ctx.append(Fragment::assistant("final answer"));

    let mut graph = Graph::new();
    let source = graph.add_accelerator(
        "source",
        Accelerator::primitive(
            state,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "source",
        ),
    );
    let fold = graph.add_flux("fold", FluxMode::Context(ContextFlux::Fold), 1);

    graph.wire(source.context(), fold.slot(0, Channel::Context));
    graph.wire(
        fold.flux_out(Channel::Context),
        Graph::output(Endpoint::State(Channel::Context)),
    );

    let output = run(Accelerator::composite_named("fold-ignore-system", graph));
    assert_eq!(
        output.fold_payload, "final answer",
        "only the last assistant text, not system or tool results"
    );
}

#[test]
fn context_fold_multi_slot_joins() {
    let mut first = State {
        purpose: "search".into(),
        ..State::default()
    };
    first.ctx.append(Fragment::assistant("Found 3 papers"));

    let mut second = State {
        purpose: "download".into(),
        ..State::default()
    };
    second
        .ctx
        .append(Fragment::assistant("Downloaded arxiv:2401.12345"));

    let mut graph = Graph::new();
    let first_acc = graph.add_accelerator(
        "first",
        Accelerator::primitive(
            first,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "first",
        ),
    );
    let second_acc = graph.add_accelerator(
        "second",
        Accelerator::primitive(
            second,
            Box::new(common::DonePolicy),
            ToolRuntime::new(),
            "second",
        ),
    );
    let fold = graph.add_flux("fold", FluxMode::Context(ContextFlux::Fold), 2);

    graph.wire(first_acc.context(), fold.slot(0, Channel::Context));
    graph.wire(second_acc.context(), fold.slot(1, Channel::Context));
    graph.wire(
        fold.flux_out(Channel::Context),
        Graph::output(Endpoint::State(Channel::Context)),
    );

    let output = run(Accelerator::composite_named("fold-multi", graph));
    assert!(
        output.fold_payload.contains("Found 3 papers"),
        "should include first slot's last assistant"
    );
    assert!(
        output.fold_payload.contains("Downloaded arxiv:2401.12345"),
        "should include second slot's last assistant"
    );
}

#[tokio::test]
async fn last_flux_reorders_upstream_content_after_scaffolding() {
    // Set up resources with a captain prompt so Captain has something to inject.
    let mut captain_resources = Resources::named("captain-test");
    captain_resources
        .prompts
        .insert("captain".into(), "Captain prompt".into());
    captain_resources = captain_resources.with_model(Model {
        name: "fast".into(),
        ..Default::default()
    });

    let state = State {
        purpose: "调查大模型多智能体框架".into(),
        res: captain_resources,
        ..State::default()
    };

    let accelerator = Accelerator::primitive(
        state,
        Box::new(Captain::new()),
        ToolRuntime::new(),
        "last-reorder-test",
    );

    // Feed upstream context through run_with: a handoff from upstream.
    let mut input = State::default();
    input
        .ctx
        .append(Fragment::assistant("handoff: run_dir=xxx, status=ok"));

    let output = accelerator.run_with(input).await;

    // After Captain setup + fire reordering:
    // Expected order: [captain prompt, agenst.md, purpose_initial, env, handoff, purpose_b]
    let frags = output.ctx.fragments();
    assert!(
        frags.len() >= 5,
        "should have scaffolding + upstream + purpose_b: {}",
        frags.len()
    );

    // Find positions.
    let pos_env = frags
        .iter()
        .position(|f| f.role == Role::System && f.tag == "env");
    let pos_handoff = frags.iter().position(|f| f.tag == "assistant");
    let pos_purpose_b = frags.iter().position(|f| f.tag == "purpose_b");

    assert!(pos_env.is_some(), "env fragment must exist");
    assert!(pos_handoff.is_some(), "handoff fragment must exist");
    assert!(pos_purpose_b.is_some(), "purpose_b fragment must exist");

    // Check ordering: env < handoff < purpose_b
    assert!(
        pos_env.unwrap() < pos_handoff.unwrap(),
        "env should come before handoff"
    );
    assert!(
        pos_handoff.unwrap() < pos_purpose_b.unwrap(),
        "handoff should come before purpose_b"
    );
}
