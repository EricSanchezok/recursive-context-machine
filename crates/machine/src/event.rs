use crate::fragment::{Content, Fragment, Role};

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

pub(crate) fn role_name(role: Role) -> &'static str {
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
