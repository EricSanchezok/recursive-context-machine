use accelerator::{ContextFlux, FluxMode, Graph, State};

#[test]
#[should_panic(expected = "name 'graph' already exists")]
fn graph_and_nodes_share_one_name_namespace() {
    let mut graph = Graph::new();
    graph.spawn_named("graph", State::default());
}

#[test]
#[should_panic(expected = "name 'agent' already exists")]
fn duplicate_accelerator_names_are_rejected() {
    let mut graph = Graph::new();
    graph.spawn_named("agent", State::default());
    graph.spawn_named("agent", State::default());
}

#[test]
#[should_panic(expected = "name 'shared' already exists")]
fn accelerator_and_flux_share_one_name_namespace() {
    let mut graph = Graph::new();
    graph.spawn_named("shared", State::default());
    graph.weave_named("shared", 1, FluxMode::Context(ContextFlux::Append));
}

#[test]
#[should_panic(expected = "name 'agent' already exists")]
fn graph_cannot_rename_to_existing_node_name() {
    let mut graph = Graph::new();
    graph.spawn_named("agent", State::default());
    graph.rename("agent");
}

#[test]
#[should_panic(expected = "name 'graph' already exists")]
fn accelerator_cannot_rename_to_graph_name() {
    let mut graph = Graph::new();
    let agent = graph.spawn_named("agent", State::default());
    graph.rename_accelerator(agent, "graph");
}
