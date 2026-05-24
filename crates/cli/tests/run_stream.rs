use cli::cmd::run::is_terminal_event;
use cli::hook::{GraphEvent, HookEvent, HookKind, MachineEvent};

fn event(kind: HookKind) -> HookEvent {
    HookEvent { source: None, kind }
}

#[test]
fn graph_done_terminates_regardless_of_graph_seen() {
    let done = event(HookKind::Graph(GraphEvent::Done {
        graph: "main".into(),
    }));
    assert!(is_terminal_event(&done, true));
    assert!(is_terminal_event(&done, false));
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
