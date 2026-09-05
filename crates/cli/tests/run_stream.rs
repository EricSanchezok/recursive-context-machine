use cli::cmd::run::{completion_event_json, is_terminal_event};
use cli::hook::{
    CompletionDiagnosticFields, CompletionEvent, ComponentMeta, GraphEvent, HookEvent, HookKind,
    MachineEvent,
};
use serde::Deserialize;

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

#[test]
fn completion_failure_json_adds_sanitized_metadata() {
    let payload = completion_event_json(&CompletionEvent::End {
        fragments: 1,
        input_tokens: 0,
        output_tokens: 0,
        total_tokens: 0,
        cached_input_tokens: 0,
        cache_creation_input_tokens: 0,
        outcome: Some("failure".into()),
        http_status: Some(503),
        failure_kind: Some("provider_error".into()),
        retryable: Some(true),
        duration_ms: Some(12_345),
        diagnostics: Some(Box::new(CompletionDiagnosticFields {
            serialized_request_bytes: 524_288,
            estimated_input_tokens: 131_072,
            message_count: 12,
            tool_definition_count: 3,
            tool_call_count: 4,
            tool_result_count: 3,
            thinking_enabled: true,
            reasoning_content_present: true,
            reasoning_content_bytes: 4096,
            unmatched_tool_call_count: 1,
            duplicate_tool_call_count: 0,
            provider_code: Some("server_error".into()),
            provider_type: Some("upstream_error".into()),
            request_id: Some("req-123".into()),
            request_class: Some("thinking_tool_history".into()),
        })),
    });

    assert_eq!(payload["outcome"], "failure");
    assert_eq!(payload["http_status"], 503);
    assert_eq!(payload["failure_kind"], "provider_error");
    assert_eq!(payload["retryable"], true);
    assert_eq!(payload["duration_ms"], 12_345);
    assert_eq!(payload["serialized_request_bytes"], 524_288);
    assert_eq!(payload["estimated_input_tokens"], 131_072);
    assert_eq!(payload["message_count"], 12);
    assert_eq!(payload["tool_definition_count"], 3);
    assert_eq!(payload["tool_call_count"], 4);
    assert_eq!(payload["tool_result_count"], 3);
    assert_eq!(payload["thinking_enabled"], true);
    assert_eq!(payload["reasoning_content_present"], true);
    assert_eq!(payload["reasoning_content_bytes"], 4096);
    assert_eq!(payload["unmatched_tool_call_count"], 1);
    assert_eq!(payload["duplicate_tool_call_count"], 0);
    assert_eq!(payload["provider_code"], "server_error");
    assert_eq!(payload["provider_type"], "upstream_error");
    assert_eq!(payload["request_id"], "req-123");
    assert_eq!(payload["request_class"], "thinking_tool_history");
    assert!(payload.get("error").is_none());
    assert!(payload.get("message").is_none());
}

#[test]
fn legacy_completion_event_omits_new_optional_fields() {
    let payload = completion_event_json(&CompletionEvent::End {
        fragments: 2,
        input_tokens: 10,
        output_tokens: 20,
        total_tokens: 30,
        cached_input_tokens: 0,
        cache_creation_input_tokens: 0,
        outcome: None,
        http_status: None,
        failure_kind: None,
        retryable: None,
        duration_ms: None,
        diagnostics: None,
    });

    assert!(payload.get("outcome").is_none());
    assert!(payload.get("http_status").is_none());
    assert!(payload.get("failure_kind").is_none());
    assert!(payload.get("retryable").is_none());
    assert!(payload.get("duration_ms").is_none());
}

#[derive(Deserialize)]
struct LegacyCompletionConsumer {
    fragments: usize,
    total_tokens: u64,
}

#[test]
fn legacy_consumers_can_ignore_additive_completion_fields() {
    let payload = completion_event_json(&CompletionEvent::End {
        fragments: 1,
        input_tokens: 0,
        output_tokens: 0,
        total_tokens: 0,
        cached_input_tokens: 0,
        cache_creation_input_tokens: 0,
        outcome: Some("failure".into()),
        http_status: Some(429),
        failure_kind: Some("rate_limited".into()),
        retryable: Some(true),
        duration_ms: Some(100),
        diagnostics: Some(Box::new(CompletionDiagnosticFields {
            serialized_request_bytes: 1024,
            estimated_input_tokens: 256,
            message_count: 2,
            tool_definition_count: 0,
            tool_call_count: 0,
            tool_result_count: 0,
            thinking_enabled: false,
            reasoning_content_present: false,
            reasoning_content_bytes: 0,
            unmatched_tool_call_count: 0,
            duplicate_tool_call_count: 0,
            provider_code: Some("rate_limit".into()),
            provider_type: None,
            request_id: Some("req-safe".into()),
            request_class: None,
        })),
    });

    let legacy: LegacyCompletionConsumer = serde_json::from_value(payload).unwrap();
    assert_eq!(legacy.fragments, 1);
    assert_eq!(legacy.total_tokens, 0);
}
