mod common;

use std::sync::Arc;
use std::time::Duration;

use accelerator::{
    Accelerator, BridgeKind, Channel, ComponentKind, ContextFlux, Endpoint, FluxMode, Graph,
    ResFlux,
};
use machine::{
    Fragment, Model, Policy, PolicyView, Purpose, Resources, RunState, ToolDefinition, ToolRuntime,
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
        _view: PolicyView<'a>,
    ) -> Pin<Box<dyn Future<Output = machine::Action> + Send + 'a>> {
        Box::pin(async move {
            self.barrier.wait().await;
            machine::Action::Done
        })
    }
}

fn primitive_with_policy(purpose: &str, policy: Box<dyn Policy>) -> Accelerator {
    Accelerator::primitive(
        RunState {
            purpose: Purpose::new(purpose),
            ..RunState::default()
        },
        policy,
        ToolRuntime::new(),
        purpose,
    )
}

fn run(accelerator: Accelerator) -> RunState {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async { accelerator.run_with(RunState::default()).await })
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
    let mut source_state = RunState {
        purpose: Purpose::new("source"),
        ..RunState::default()
    };
    source_state.context.append(Fragment::assistant("done"));

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

    assert_eq!(output.context.fragments().len(), 1);
}

#[test]
fn resource_flux_preserves_model_order_and_tool_pool() {
    let first_state = RunState {
        purpose: Purpose::new("first"),
        resources: Resources::named("first")
            .with_model(Model {
                name: "fast".into(),
                ..Default::default()
            })
            .with_tool_definition(ToolDefinition::from_tool(&accelerator::tools::FindTool)),
        ..RunState::default()
    };
    let second_state = RunState {
        purpose: Purpose::new("second"),
        resources: Resources::named("second")
            .with_model(Model {
                name: "careful".into(),
                ..Default::default()
            })
            .with_tool_definition(ToolDefinition::from_tool(&accelerator::tools::ShellTool)),
        ..RunState::default()
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

    assert_eq!(output.resources.model_order, vec!["fast", "careful"]);
    assert!(output.resources.tool_definitions.contains_key("find"));
    assert!(output.resources.tool_definitions.contains_key("shell"));
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
            Accelerator::composite_named("parallel", graph).run_with(RunState::default()),
        )
        .await
        .is_ok()
    });

    assert!(completed);
}

#[test]
fn downstream_waits_for_parallel_sources() {
    let mut first_state = RunState {
        purpose: Purpose::new("first"),
        ..RunState::default()
    };
    first_state.context.append(Fragment::assistant("first"));
    let mut second_state = RunState {
        purpose: Purpose::new("second"),
        ..RunState::default()
    };
    second_state.context.append(Fragment::assistant("second"));

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

    assert_eq!(output.context.fragments().len(), 2);
}

#[test]
fn context_last_keeps_only_last_fragment_per_slot() {
    let mut first_state = RunState {
        purpose: Purpose::new("first"),
        ..RunState::default()
    };
    first_state.context.append(Fragment::assistant("first-a"));
    first_state.context.append(Fragment::assistant("first-b"));

    let mut second_state = RunState {
        purpose: Purpose::new("second"),
        ..RunState::default()
    };
    second_state.context.append(Fragment::assistant("second-a"));
    second_state.context.append(Fragment::assistant("second-b"));

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

    assert_eq!(output.context.fragments().len(), 2);
    assert_eq!(output.context.fragments()[0].as_text(), Some("first-b"));
    assert_eq!(output.context.fragments()[1].as_text(), Some("second-b"));
}

#[test]
fn context_digest_extracts_key_segments() {
    let mut state = RunState {
        purpose: Purpose::new("search"),
        ..RunState::default()
    };
    // System prompt (dropped)
    state
        .context
        .append(Fragment::system("You are a search agent"));
    // Tool call (dropped)
    state.context.append(Fragment::tool_call(
        "tc1",
        "arxiv_search",
        serde_json::json!({"query": "quantum"}),
    ));
    // Tool result (kept)
    state.context.append(Fragment::tool_result(
        "tc1",
        "Found 3 papers on quantum computing",
        None,
    ));
    // Final answer (kept)
    state
        .context
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

    assert_eq!(output.context.fragments().len(), 1);
    let text = output.context.fragments()[0].as_text().unwrap();
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
    let mut first_state = RunState {
        purpose: Purpose::new("search papers"),
        ..RunState::default()
    };
    first_state
        .context
        .append(Fragment::assistant("Found 3 papers on quantum computing"));

    let mut second_state = RunState {
        purpose: Purpose::new("download best"),
        ..RunState::default()
    };
    second_state
        .context
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
    assert_eq!(output.context.fragments().len(), 4);

    // Slot 0: user question
    assert_eq!(output.context.fragments()[0].role, machine::Role::User);
    assert!(
        output.context.fragments()[0]
            .as_text()
            .unwrap()
            .contains("Task 1")
    );
    // Slot 0: answer
    assert_eq!(output.context.fragments()[1].role, machine::Role::Assistant);
    assert_eq!(
        output.context.fragments()[1].as_text(),
        Some("Found 3 papers on quantum computing")
    );

    // Slot 1: user question
    assert_eq!(output.context.fragments()[2].role, machine::Role::User);
    assert!(
        output.context.fragments()[2]
            .as_text()
            .unwrap()
            .contains("Task 2")
    );
    // Slot 1: answer
    assert_eq!(output.context.fragments()[3].role, machine::Role::Assistant);
    assert_eq!(
        output.context.fragments()[3].as_text(),
        Some("Downloaded arxiv:2401.12345.pdf")
    );
}

// ── FluxMode::Bridge tests ──

#[test]
fn bridge_flattens_context_after_last_filter() {
    // Flux(Last) → Bridge(Context→Purpose): Last extracts the final
    // fragment per slot, Bridge flattens it into a purpose string.
    let mut state = RunState {
        purpose: Purpose::new("search"),
        ..RunState::default()
    };
    state.context.append(Fragment::assistant("first message"));
    state.context.append(Fragment::assistant("second message"));

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
    let last = graph.add_flux("last", FluxMode::Context(ContextFlux::Last), 1);
    let bridge = graph.add_flux(
        "bridge",
        FluxMode::Bridge {
            from: Channel::Context,
            to: Channel::Purpose,
            kind: BridgeKind::ContextToPurpose,
        },
        1,
    );

    graph.wire(source.context(), last.slot(0, Channel::Context));
    graph.wire(
        last.flux_out(Channel::Context),
        bridge.slot(0, Channel::Context),
    );
    graph.wire(
        bridge.flux_out(Channel::Purpose),
        Graph::output(Endpoint::State(Channel::Purpose)),
    );

    let output = run(Accelerator::composite_named("bridge-last", graph));

    assert_eq!(output.purpose.text, "second message");
    assert!(
        !output.purpose.text.contains("first message"),
        "Last should have dropped earlier fragments"
    );
}

#[test]
fn bridge_flattens_full_context_after_append() {
    // Flux(Append) → Bridge(Context→Purpose): all fragments pass through,
    // Bridge flattens them into one purpose string.
    let mut state = RunState {
        purpose: Purpose::new("search"),
        ..RunState::default()
    };
    state.context.append(Fragment::assistant("alpha"));
    state.context.append(Fragment::assistant("beta"));

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
    let append = graph.add_flux("append", FluxMode::Context(ContextFlux::Append), 1);
    let bridge = graph.add_flux(
        "bridge",
        FluxMode::Bridge {
            from: Channel::Context,
            to: Channel::Purpose,
            kind: BridgeKind::ContextToPurpose,
        },
        1,
    );

    graph.wire(source.context(), append.slot(0, Channel::Context));
    graph.wire(
        append.flux_out(Channel::Context),
        bridge.slot(0, Channel::Context),
    );
    graph.wire(
        bridge.flux_out(Channel::Purpose),
        Graph::output(Endpoint::State(Channel::Purpose)),
    );

    let output = run(Accelerator::composite_named("bridge-append", graph));
    assert_eq!(output.purpose.text, "alpha\n\nbeta");
}

#[test]
fn bridge_flattens_digested_context_to_purpose() {
    // Flux(Digest) → Bridge(Context→Purpose): Digest strips scaffolding,
    // Bridge flattens the remaining content.
    let mut state = RunState {
        purpose: Purpose::new("search"),
        ..RunState::default()
    };
    state
        .context
        .append(Fragment::system("You are a helpful assistant"));
    state.context.append(Fragment::assistant("final answer"));

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
    let bridge = graph.add_flux(
        "bridge",
        FluxMode::Bridge {
            from: Channel::Context,
            to: Channel::Purpose,
            kind: BridgeKind::ContextToPurpose,
        },
        1,
    );

    graph.wire(source.context(), digest.slot(0, Channel::Context));
    graph.wire(
        digest.flux_out(Channel::Context),
        bridge.slot(0, Channel::Context),
    );
    graph.wire(
        bridge.flux_out(Channel::Purpose),
        Graph::output(Endpoint::State(Channel::Purpose)),
    );

    let output = run(Accelerator::composite_named("bridge-digest", graph));
    assert_eq!(
        output.purpose.text, "final answer",
        "Digest strips system fragments, Bridge flattens what remains"
    );
}

#[test]
fn bridge_flattens_multi_slot_context_after_last() {
    // Two sources → Flux(Last) → Bridge(Context→Purpose).
    let mut first = RunState {
        purpose: Purpose::new("search"),
        ..RunState::default()
    };
    first.context.append(Fragment::assistant("Found 3 papers"));

    let mut second = RunState {
        purpose: Purpose::new("download"),
        ..RunState::default()
    };
    second
        .context
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
    let last = graph.add_flux("last", FluxMode::Context(ContextFlux::Last), 2);
    let bridge = graph.add_flux(
        "bridge",
        FluxMode::Bridge {
            from: Channel::Context,
            to: Channel::Purpose,
            kind: BridgeKind::ContextToPurpose,
        },
        1,
    );

    graph.wire(first_acc.context(), last.slot(0, Channel::Context));
    graph.wire(second_acc.context(), last.slot(1, Channel::Context));
    graph.wire(
        last.flux_out(Channel::Context),
        bridge.slot(0, Channel::Context),
    );
    graph.wire(
        bridge.flux_out(Channel::Purpose),
        Graph::output(Endpoint::State(Channel::Purpose)),
    );

    let output = run(Accelerator::composite_named("bridge-multi", graph));
    assert!(
        output.purpose.text.contains("Found 3 papers"),
        "should include first slot"
    );
    assert!(
        output.purpose.text.contains("Downloaded arxiv:2401.12345"),
        "should include second slot"
    );
}
