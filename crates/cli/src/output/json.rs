use machine::{Content, Context, Role};
use serde_json::json;

use super::Summary;

pub fn print(ctx: &Context, summary: &Summary) -> anyhow::Result<()> {
    let fragments = ctx
        .fragments()
        .iter()
        .map(|frag| {
            json!({
                "role": role_name(frag.role),
                "content": content_json(&frag.content),
            })
        })
        .collect::<Vec<_>>();

    let output = json!({
        "fragments": fragments,
        "summary": {
            "duration_s": summary.duration_s,
            "fragments": summary.fragments,
            "tool_calls": summary.tool_calls,
        }
    });
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
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
        Content::Hitch {
            message,
            retryable,
            code,
        } => json!({
            "type": "hitch",
            "message": message,
            "retryable": retryable,
            "code": code,
        }),
    }
}
