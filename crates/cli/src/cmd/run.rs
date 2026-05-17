use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use accelerator::{Graph, State};
use tracing_subscriber::prelude::*;

use crate::args::{Format, RunArgs};
use crate::hook;
use crate::output;

pub fn run(args: RunArgs) -> anyhow::Result<()> {
    let (hook_tx, hook_rx) = mpsc::channel();
    init_tracing(hook_tx);

    let start = Instant::now();
    let (ctx_tx, ctx_rx) = mpsc::channel();
    let prompt = args.prompt_text();

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
            let outputs = graph.build().expect("assembly").run().await;
            let output = outputs.into_iter().next().expect("agent output");
            let _ = ctx_tx.send(output.ctx);
        });
    });

    let summary = output::tape::run_animation(hook_rx, args.speed, start);
    let ctx = ctx_rx.recv()?;

    match args.format {
        Format::Text => output::text::print(&ctx, &summary),
        Format::Json => output::json::print(&ctx, &summary)?,
    }

    Ok(())
}

fn init_tracing(tx: mpsc::Sender<hook::HookEvent>) {
    use tracing_subscriber::layer::Layer;

    let fmt_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("off"));
    let subscriber = tracing_subscriber::registry()
        .with(hook::hook_layer(tx))
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_writer(std::io::stderr)
                .compact()
                .with_filter(fmt_filter),
        );
    tracing::subscriber::set_global_default(subscriber).ok();
}
