use cli::cmd::run::is_terminal_event;
use cli::hook::{ComponentMeta, GraphEvent, HookEvent, HookKind, MachineEvent};

fn event(kind: HookKind) -> HookEvent {
    HookEvent { source: None, kind }
}

fn nested_event(kind: HookKind) -> HookEvent {
    HookEvent {
        source: Some(ComponentMeta {
            graph: "survey_pipeline".into(),
            name: "discovery".into(),
            index: 1,
            kind: "accelerator".into(),
            frontier: Some(2),
        }),
        kind,
    }
}

#[test]
fn root_graph_done_terminates_after_root_graph_start() {
    let done = event(HookKind::Graph(GraphEvent::Done {
        graph: "main".into(),
    }));
    assert!(is_terminal_event(&done, true));
}

#[test]
fn nested_graph_done_does_not_terminate_root_stream() {
    let done = nested_event(HookKind::Graph(GraphEvent::Done {
        graph: "discovery".into(),
    }));

    assert!(!is_terminal_event(&done, true));
}

#[test]
fn machine_done_terminates_only_when_no_graph_was_seen() {
    let done = event(HookKind::Machine(MachineEvent::Done));
    assert!(
        is_terminal_event(&done, false),
        "primitive accelerator should terminate on Machine::Done"
    );
    assert!(
        !is_terminal_event(&done, true),
        "graph-based run must wait for Graph::Done; Machine::Done fires per-machine"
    );
}

#[test]
fn nested_machine_done_never_terminates_root_stream() {
    let done = nested_event(HookKind::Machine(MachineEvent::Done));

    assert!(!is_terminal_event(&done, false));
    assert!(!is_terminal_event(&done, true));
}

#[test]
fn machine_start_never_terminates() {
    let start = event(HookKind::Machine(MachineEvent::Start));
    assert!(!is_terminal_event(&start, false));
    assert!(!is_terminal_event(&start, true));
}

#[test]
fn machine_halt_never_terminates() {
    let halt = event(HookKind::Machine(MachineEvent::Halt { step: 3 }));
    assert!(!is_terminal_event(&halt, false));
    assert!(!is_terminal_event(&halt, true));
}

#[test]
fn graph_start_never_terminates() {
    let start = event(HookKind::Graph(GraphEvent::Start {
        graph: "main".into(),
    }));
    assert!(!is_terminal_event(&start, false));
    assert!(!is_terminal_event(&start, true));
}

#[test]
fn frontier_events_never_terminate() {
    let frontier_start = event(HookKind::Graph(GraphEvent::FrontierStart {
        graph: "main".into(),
        frontier: 1,
        count: 2,
    }));
    let frontier_done = event(HookKind::Graph(GraphEvent::FrontierDone {
        graph: "main".into(),
        frontier: 1,
        count: 2,
    }));
    assert!(!is_terminal_event(&frontier_start, true));
    assert!(!is_terminal_event(&frontier_done, true));
}
