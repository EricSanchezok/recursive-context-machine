mod display;
mod hook;

use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use accelerator::{Graph, State};
use machine::Content;
use tracing_subscriber::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let prompt = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        eprintln!("usage: cargo run -p cli -- <prompt> [--speed <ms>]");
        std::process::exit(1);
    };

    let delay_ms: u64 = {
        let mut d = 50;
        for w in args.windows(2) {
            if w[0] == "--speed" {
                d = w[1].parse().unwrap_or(50);
            }
        }
        d
    };

    let (hook_tx, hook_rx) = mpsc::channel();

    // Init tracing — hook layer intercepts target:"hook" for the visualizer,
    // fmt layer prints other logs to stderr only.
    let subscriber = tracing_subscriber::registry()
        .with(hook::hook_layer(hook_tx))
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_writer(std::io::stderr)
                .compact(),
        );
    tracing::subscriber::set_global_default(subscriber).ok();

    let start = Instant::now();

    // Run the machine on the tokio runtime, animation on a separate thread.
    let (result_tx, result_rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");
        runtime.block_on(async {
            let mut graph = Graph::new();
            let _agent = graph.spawn(State {
                purpose: prompt,
                ..State::default()
            });
            let outputs = graph.build().unwrap().run().await;
            let output = outputs.into_iter().next().expect("no output");
            let _ = result_tx.send(output.ctx);
        });
    });

    let summary = display::run_animation(hook_rx, delay_ms, start);

    let ctx = result_rx.recv().expect("output ctx");

    // Clear the two animation lines and print structured output.
    for _ in 0..2 {
        eprintln!();
    }
    println!("── Result ──────────────────────────────────────");
    for frag in ctx.fragments() {
        let tag = match frag.role {
            machine::Role::System => "system",
            machine::Role::User => "user",
            machine::Role::Assistant => "assistant",
            machine::Role::Tool => "tool",
        };
        match &frag.content {
            Content::Text(text) => {
                if frag.role == machine::Role::System
                    && text.text.contains("You are an AI assistant")
                {
                    continue;
                }
                println!("[{tag}] {}", text.text);
            }
            Content::ToolCall(tc) => {
                let args = serde_json::to_string(&tc.arguments).unwrap_or_default();
                println!("[{tag}] → {} {}", tc.name, args);
            }
            Content::ToolResult(tr) => {
                let first = tr.content.lines().next().unwrap_or("");
                let lines = tr.content.lines().count();
                if first.len() > 120 {
                    println!("[{tag}] ({lines} lines)");
                } else {
                    println!("[{tag}] {first} ({lines} lines)");
                }
            }
            Content::Hitch {
                message,
                retryable,
                code,
            } => {
                let retry = if *retryable { " (retryable)" } else { "" };
                let status = code.map(|c| format!(" HTTP {c}")).unwrap_or_default();
                eprintln!("[{tag}] hitch{}{} {message}", status, retry);
            }
            _ => {}
        }
    }
    println!(
        "── done ({:.1}s, {} fragments, {} tool calls) ──",
        summary.duration_s, summary.fragments, summary.tool_calls
    );
}
