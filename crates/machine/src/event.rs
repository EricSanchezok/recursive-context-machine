use crate::fragment::{Content, Fragment};

/// Generate a short preview of a fragment's content (max 60 chars).
pub(crate) fn preview(frag: &Fragment) -> String {
    match &frag.content {
        Content::Text(t) => t.text.chars().take(60).collect(),
        Content::ToolCall(tc) => format!("call {}()", tc.name),
        Content::ToolResult(tr) => tr.content.chars().take(60).collect(),
        Content::Hitch { message, .. } => message.chars().take(60).collect(),
        _ => format!("<{:?}>", frag.content),
    }
}
