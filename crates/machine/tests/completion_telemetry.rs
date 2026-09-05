use machine::event::{
    CompletionDiagnostics, ProviderFailureDiagnostics, completion_telemetry,
    completion_telemetry_with_diagnostics,
};
use machine::{Fragment, Role};

#[test]
fn successful_completion_has_no_failure_metadata() {
    let telemetry = completion_telemetry(&[Fragment::assistant("done")]);

    assert_eq!(telemetry.outcome, "success");
    assert_eq!(telemetry.http_status, None);
    assert_eq!(telemetry.failure_kind, None);
    assert_eq!(telemetry.retryable, None);
}

#[test]
fn http_failures_are_stably_classified() {
    let cases = [
        (400, "invalid_request", false),
        (401, "authentication", false),
        (403, "authentication", false),
        (408, "timeout", true),
        (425, "provider_unavailable", true),
        (429, "rate_limited", true),
        (503, "provider_error", true),
    ];

    for (status, expected_kind, expected_retryable) in cases {
        let telemetry = completion_telemetry(&[Fragment::hitch(
            "provider response omitted from telemetry",
            Some(status),
            Role::Assistant,
            None::<&str>,
        )]);

        assert_eq!(telemetry.outcome, "failure");
        assert_eq!(telemetry.http_status, Some(status));
        assert_eq!(telemetry.failure_kind, Some(expected_kind));
        assert_eq!(telemetry.retryable, Some(expected_retryable));
    }
}

#[test]
fn statusless_transport_failures_are_classified_without_exposing_the_message() {
    let timeout = completion_telemetry(&[Fragment::hitch(
        "request timed out after 240s",
        None,
        Role::Assistant,
        None::<&str>,
    )]);
    assert_eq!(timeout.failure_kind, Some("timeout"));
    assert_eq!(timeout.retryable, Some(true));

    let network = completion_telemetry(&[Fragment::hitch(
        "error sending request: connection reset",
        None,
        Role::Assistant,
        None::<&str>,
    )]);
    assert_eq!(network.failure_kind, Some("network"));
    assert_eq!(network.retryable, Some(true));

    let configuration = completion_telemetry(&[Fragment::hitch(
        "no active model set",
        None,
        Role::System,
        None::<&str>,
    )]);
    assert_eq!(configuration.failure_kind, Some("configuration"));
    assert_eq!(configuration.retryable, Some(false));
}

#[test]
fn provider_failure_metadata_is_additive_and_content_free() {
    let diagnostics = CompletionDiagnostics {
        provider: ProviderFailureDiagnostics {
            provider_code: Some("invalid_request_error".into()),
            provider_type: Some("invalid_request_error".into()),
            request_id: Some("req-123".into()),
            request_class: Some("thinking_tool_history"),
        },
        ..CompletionDiagnostics::default()
    };
    let telemetry = completion_telemetry_with_diagnostics(
        &[Fragment::hitch(
            "PRIVATE PAPER BODY reasoning_content is missing",
            Some(400),
            Role::Assistant,
            None::<&str>,
        )],
        &diagnostics,
    );

    assert_eq!(
        telemetry.provider_code.as_deref(),
        Some("invalid_request_error")
    );
    assert_eq!(
        telemetry.provider_type.as_deref(),
        Some("invalid_request_error")
    );
    assert_eq!(telemetry.request_id.as_deref(), Some("req-123"));
    assert_eq!(telemetry.request_class, Some("thinking_tool_history"));
    assert!(!format!("{telemetry:?}").contains("PRIVATE PAPER BODY"));
}
