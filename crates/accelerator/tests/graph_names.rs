use accelerator::{ContextFlux, FluxMode, Graph, State};

#[test]
fn duplicate_accelerator_names_are_allowed() {
    let mut graph = Graph::new();
    let first = graph.spawn_named("agent", State::default());
    let second = graph.spawn_named("agent", State::default());

    assert_ne!(first.id(), second.id());
}

#[test]
fn graph_and_nodes_may_share_names() {
    let mut graph = Graph::new();
    let agent = graph.spawn_named("graph", State::default());

    assert_ne!(graph.id().as_str(), agent.id().as_str());
}

#[test]
fn accelerator_and_flux_may_share_names_but_not_ids() {
    let mut graph = Graph::new();
    let agent = graph.spawn_named("shared", State::default());
    let flux = graph.weave_named("shared", 1, FluxMode::Context(ContextFlux::Append));

    assert_ne!(agent.id().as_str(), flux.id().as_str());
}

#[test]
fn rename_does_not_change_id() {
    let mut graph = Graph::new();
    let before = graph.id().clone();

    graph.rename("New Graph Name");

    assert_eq!(graph.id(), &before);
    assert_eq!(graph.name.as_str(), "New Graph Name");
}

#[test]
#[should_panic(expected = "accelerator reference does not belong to this graph")]
fn stale_accelerator_ref_is_rejected() {
    let mut first = Graph::new();
    let mut second = Graph::new();
    let agent = first.spawn_named("agent", State::default());

    second.spawn_named("agent", State::default());
    second.rename_accelerator(agent, "renamed");
}

#[test]
#[should_panic(expected = "flux port does not belong to this graph")]
fn stale_flux_port_is_rejected() {
    let mut first = Graph::new();
    let mut second = Graph::new();
    let agent = second.spawn_named("agent", State::default());
    let flux = first.weave_named("shared", 1, FluxMode::Context(ContextFlux::Append));

    second.wire(agent.ctx_out(), flux.slot(0));
}
