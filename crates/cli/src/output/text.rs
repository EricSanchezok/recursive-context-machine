use machine::{Content, Context, Role};

use super::Summary;

pub fn print(ctx: &Context, _summary: &Summary, full_context: bool) {
    if full_context {
        print_context(ctx);
    } else if let Some(text) = final_message(ctx) {
        println!("{}", text.trim_end());
    }
}

fn print_context(ctx: &Context) {
    println!("── Context ─────────────────────────────────────");
    for frag in ctx.fragments() {
        print_fragment(frag.role, &frag.content);
    }
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
            let status = code.map(|code| format!(" HTTP {code}")).unwrap_or_default();
            println!("[{tag}] hitch{}{} {message}", status, retry);
        }
    }
}
