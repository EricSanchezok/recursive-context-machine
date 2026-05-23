use machine::{Content, Context, Role};
use serde_json::json;

use super::Summary;

pub fn print(ctx: &Context, summary: &Summary, full_context: bool) -> anyhow::Result<()> {
    let output = if full_context {
        json!({
            "fragments": fragments_json(ctx),
            "summary": summary_json(summary),
        })
    } else {
        json!({
            "message": final_message(ctx),
            "summary": summary_json(summary),
        })
    };

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn final_message(ctx: &Context) -> Option<&str> {
    ctx.fragments().iter().rev().find_map(|frag| {
        if frag.role != Role::Assistant {
            return None;
        }
        match &frag.content {
            Content::Text(text) => Some(text.text.as_str()),
            _ => None,
        }
    })
}

fn fragments_json(ctx: &Context) -> Vec<serde_json::Value> {
    ctx.fragments()
        .iter()
        .map(|frag| {
            json!({
                "role": role_name(frag.role),
                "content": content_json(&frag.content),
            })
        })
        .collect()
}

fn summary_json(summary: &Summary) -> serde_json::Value {
    json!({
        "duration_s": summary.duration_s,
        "fragments": summary.fragments,
        "tool_calls": summary.tool_calls,
    })
}

fn role_name(role: Role) -> &'static str {
    match role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
}

fn content_json(content: &Content) -> serde_json::Value {
    match content {
        Content::Text(text) => json!({ "type": "text", "text": text.text }),
        Content::ToolCall(call) => json!({
            "type": "tool_call",
            "name": call.name,
            "arguments": call.arguments,
        }),
        Content::ToolResult(result) => json!({
            "type": "tool_result",
            "content": result.content,
            "title": result.title,
        }),
        Content::Image(image) => {
            json!({ "type": "image", "media_type": format!("{:?}", image.media_type) })
        }
        Content::Audio(audio) => {
            json!({ "type": "audio", "media_type": format!("{:?}", audio.media_type) })
        }
        Content::Video(video) => {
            json!({ "type": "video", "media_type": format!("{:?}", video.media_type) })
        }
        Content::Document(document) => {
            json!({ "type": "document", "media_type": format!("{:?}", document.media_type) })
        }
        Content::Hitch { message, code, .. } => json!({
            "type": "hitch",
            "message": message,
            "code": code,
        }),
    }
}
