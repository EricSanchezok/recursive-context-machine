use machine::{Content, Context, Role};

use super::Summary;

pub fn print(ctx: &Context, _summary: &Summary, full_context: bool) {
    if full_context {
        print_context(ctx);
    } else if let Some(text) = final_message(ctx) {
        print_response(text.trim_end());
    }
}

fn print_response(text: &str) {
    let width = crossterm::terminal::size()
        .map(|(width, _)| width as usize)
        .unwrap_or(100)
        .max(40);

    print_top("Response", width);
    println!();
    println!("{}", text);
    println!();
    print_bottom(width);
}

fn print_top(title: &str, width: usize) {
    let label = format!(" {title} ");
    let right = width.saturating_sub(label.chars().count() + 2);
    println!("╭─{}{}╮", label, "─".repeat(right.saturating_sub(1)));
}

fn print_bottom(width: usize) {
    println!("╰{}╯", "─".repeat(width.saturating_sub(2)));
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
