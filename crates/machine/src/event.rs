use crate::fragment::{Content, Fragment, Role};

/// Sanitized metadata describing one model completion attempt.
///
/// The classification intentionally contains no provider response text. It is
/// safe for external stream consumers and operational logs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionTelemetry {
    pub outcome: &'static str,
    pub http_status: Option<u16>,
    pub failure_kind: Option<&'static str>,
    pub retryable: Option<bool>,
}

/// Classify the completion result without exposing hitch content.
pub fn completion_telemetry(fragments: &[Fragment]) -> CompletionTelemetry {
    let failure = fragments
        .iter()
        .find_map(|fragment| match &fragment.content {
            Content::Hitch { message, code, .. } => Some((fragment.role, message.as_str(), *code)),
            _ => None,
        });
    let Some((role, message, http_status)) = failure else {
        return CompletionTelemetry {
            outcome: "success",
            http_status: None,
            failure_kind: None,
            retryable: None,
        };
    };

    let (failure_kind, retryable) = classify_completion_failure(role, message, http_status);
    CompletionTelemetry {
        outcome: "failure",
        http_status,
        failure_kind: Some(failure_kind),
        retryable: Some(retryable),
    }
}

fn classify_completion_failure(
    role: Role,
    message: &str,
    http_status: Option<u16>,
) -> (&'static str, bool) {
    if let Some(status) = http_status {
        return match status {
            401 | 403 => ("authentication", false),
            408 => ("timeout", true),
            425 => ("provider_unavailable", true),
            429 => ("rate_limited", true),
            500..=599 => ("provider_error", true),
            400..=499 => ("invalid_request", false),
            _ => ("http_error", false),
        };
    }

    if role == Role::System {
        return ("configuration", false);
    }

    let normalized = message.to_ascii_lowercase();
    if normalized.contains("timed out") || normalized.contains("timeout") {
        return ("timeout", true);
    }
    if [
        "connection",
        "network",
        "dns",
        "transport",
        "error sending request",
        "request error",
    ]
    .iter()
    .any(|marker| normalized.contains(marker))
    {
        return ("network", true);
    }

    ("unknown", false)
}

/// Generate a short preview of a fragment's content (max 60 chars).
pub(crate) fn preview(frag: &Fragment) -> String {
    match &frag.content {
        Content::Text(t) => t.text.chars().take(60).collect(),
        Content::ToolCall(tc) => format!("call {}()", tc.name),
        Content::ToolResult(tr) => tr.content.chars().take(60).collect(),
        Content::Hitch { message, .. } => message.chars().take(60).collect(),
        Content::Image(_) => "<image>".into(),
        Content::Audio(_) => "<audio>".into(),
        Content::Video(_) => "<video>".into(),
        Content::Document(_) => "<document>".into(),
    }
}

pub fn role_name(role: Role) -> &'static str {
    match role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
}

pub(crate) fn content_kind(frag: &Fragment) -> &'static str {
    match &frag.content {
        Content::Text(_) => "text",
        Content::Image(_) => "image",
        Content::Audio(_) => "audio",
        Content::Video(_) => "video",
        Content::Document(_) => "document",
        Content::ToolCall(_) => "tool_call",
        Content::ToolResult(_) => "tool_result",
        Content::Hitch { .. } => "hitch",
    }
}
