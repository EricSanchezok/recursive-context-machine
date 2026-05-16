use accelerator::{Graph, State};
use machine::Role;

#[tokio::main]
async fn main() {
    let _guard = machine::logging::init();

    let args: Vec<String> = std::env::args().collect();
    let intent = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        eprintln!("usage: cargo run -p accelerator -- <prompt>");
        std::process::exit(1);
    };

    let mut graph = Graph::new();
    let _agent = graph.spawn(intent, State::default());

    let assembly = graph.build().unwrap();
    let outputs = assembly.run().await;

    let output = outputs.into_iter().next().expect("no output");

    for frag in output.context.fragments() {
        match &frag.content {
            machine::Content::Hitch {
                message,
                retryable,
                code,
            } => {
                let retry = if *retryable { " (retryable)" } else { "" };
                let status = code.map(|c| format!(" HTTP {c}")).unwrap_or_default();
                eprintln!("[hitch]{}{} {message}", status, retry);
            }
            _ => {
                let text = frag.as_text().unwrap_or("");
                match frag.role {
                    Role::System if !text.is_empty() => println!("[system] {}", text),
                    Role::User => println!("[user] {}", text),
                    Role::Assistant => println!("[assistant] {}", text),
                    _ => {}
                }
            }
        }
    }
}
