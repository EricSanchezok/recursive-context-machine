use machine::{Content, Context, Role};

use super::Summary;

pub fn print(ctx: &Context, summary: &Summary) {
    println!("── Result ──────────────────────────────────────");
    for frag in ctx.fragments() {
        print_fragment(frag.role, &frag.content);
    }
    println!(
        "── done ({:.1}s, {} fragments, {} tool calls) ──",
        summary.duration_s, summary.fragments, summary.tool_calls
    );
}

fn print_fragment(role: Role, content: &Content) {
    let tag = match role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    };

    match content {
        Content::Text(text) => println!("[{tag}] {}", text.text),
        Content::ToolCall(call) => {
            let args = serde_json::to_string(&call.arguments).unwrap_or_default();
            println!("[{tag}] → {} {}", call.name, args);
        }
        Content::ToolResult(result) => {
            let first = result.content.lines().next().unwrap_or("");
            let lines = result.content.lines().count();
            if first.len() > 120 {
                println!("[{tag}] ({lines} lines)");
            } else {
                println!("[{tag}] {first} ({lines} lines)");
            }
        }
        Content::Image(image) => println!("[{tag}] <image {:?}>", image.media_type),
        Content::Audio(audio) => println!("[{tag}] <audio {:?}>", audio.media_type),
        Content::Video(video) => println!("[{tag}] <video {:?}>", video.media_type),
        Content::Document(document) => println!("[{tag}] <document {:?}>", document.media_type),
        Content::Hitch {
            message,
            retryable,
            code,
        } => {
            let retry = if *retryable { " (retryable)" } else { "" };
            let status = code.map(|c| format!(" HTTP {c}")).unwrap_or_default();
            println!("[{tag}] hitch{}{} {message}", status, retry);
        }
    }
}
