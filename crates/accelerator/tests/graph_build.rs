use accelerator::{BuildError, ContextFlux, FluxMode, Graph, State};

#[test]
fn unwired_flux_slot_rejected() {
    let mut graph = Graph::new();
    let agent = graph.spawn(State::default());
    let flux = graph.weave(2, FluxMode::Context(ContextFlux::Append));

    graph.wire(agent.ctx_out(), flux.slot(0));

    let result = graph.build();
    assert!(matches!(result, Err(BuildError::UnwiredFluxSlot { .. })));
}

#[test]
fn fully_wired_flux_builds() {
    let mut graph = Graph::new();
    let upstream = graph.spawn(State::default());
    let downstream = graph.spawn(State::default());
    let flux = graph.weave(1, FluxMode::Context(ContextFlux::Append));

    graph.wire(upstream.done(), downstream.trigger());
    graph.wire(upstream.ctx_out(), flux.slot(0));
    graph.wire(flux.out(), downstream.ctx_in());

    let result = graph.build();
    assert!(result.is_ok());
}
