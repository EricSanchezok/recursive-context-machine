use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use accelerator::Graph;
use accelerator::mcp::{McpRegistry, McpServerConfig};
use tracing_subscriber::prelude::*;

use crate::args::{Format, RunArgs};
use crate::hook;
use crate::output;

pub async fn run(args: RunArgs) -> anyhow::Result<()> {
    let (hook_tx, hook_rx) = mpsc::channel();
    init_tracing(hook_tx);

    let prompt = args.prompt_text();

    // Detect `.rcm` files by extension.
    let graph = if prompt.ends_with(".rcm") {
        let source = std::fs::read_to_string(&prompt)
            .map_err(|e| anyhow::anyhow!("failed to read '{}': {}", prompt, e))?;
        let file = crate::rcm::parse(&source).map_err(anyhow::Error::msg)?;
        crate::rcm::compile::compile(&file).map_err(anyhow::Error::msg)?
    } else {
        let mut graph = Graph::new();
        graph.spawn(accelerator::State {
            purpose: prompt,
            ..Default::default()
        });
        graph
    };

    // ── MCP setup ──
    let configs: Vec<McpServerConfig> = args
        .mcp_servers
        .iter()
        .map(|s| McpServerConfig::parse(s))
        .collect::<Result<Vec<_>, _>>()
        .map_err(anyhow::Error::msg)?;

    let _mcp_tools: Vec<Arc<dyn machine::Tool>> = if configs.is_empty() {
        Vec::new()
    } else {
        let registry: McpRegistry = McpRegistry::start(&configs)
            .await
            .map_err(anyhow::Error::msg)?;
        registry.tools()
    };

    // ── Accelerator ──
    let start = Instant::now();
    let (ctx_tx, ctx_rx) = mpsc::channel();

    thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");
        runtime.block_on(async {
            let outputs = graph.build().expect("assembly").run().await;
            let output = outputs.into_iter().next().expect("agent output");
            let _ = ctx_tx.send(output.ctx);
        });
    });

    let summary = output::tape::run_animation(hook_rx, args.speed, start);
    let ctx = ctx_rx.recv()?;

    match args.format {
        Format::Text => output::text::print(&ctx, &summary, args.context),
        Format::Json => output::json::print(&ctx, &summary, args.context)?,
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
