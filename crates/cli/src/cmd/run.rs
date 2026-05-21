use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use tracing_subscriber::prelude::*;

use crate::args::{Format, RunArgs};
use crate::hook::{self, HookKind};
use crate::output;

pub async fn run(args: RunArgs) -> anyhow::Result<()> {
    let (hook_tx, hook_rx) = mpsc::channel();
    init_tracing(hook_tx);

    let accelerator = crate::rcm::compile::compile_file(&args.file)
        .await
        .map_err(anyhow::Error::msg)?;

    if args.stream {
        return stream_run(accelerator).await;
    }

    let start = Instant::now();
    let (ctx_tx, ctx_rx) = mpsc::channel();

    thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");
        runtime.block_on(async {
            let output = accelerator.run_with(accelerator::State::default()).await;
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

async fn stream_run(accelerator: accelerator::Accelerator) -> anyhow::Result<()> {
    let (ctx_tx, ctx_rx) = mpsc::channel();

    thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");
        runtime.block_on(async {
            let output = accelerator.run_with(accelerator::State::default()).await;
            let _ = ctx_tx.send(output.ctx);
        });
    });

    for event in hook_rx() {
        let line = match &event.kind {
            HookKind::Graph(hook::GraphEvent::Start { graph }) => {
                json_line("graph_start", serde_json::json!({ "graph": graph }))
            }
            HookKind::Graph(hook::GraphEvent::Done { graph }) => {
                json_line("graph_done", serde_json::json!({ "graph": graph }))
            }
            HookKind::Graph(hook::GraphEvent::FrontierStart {
                graph,
                frontier,
                count,
            }) => json_line(
                "frontier_start",
                serde_json::json!({ "graph": graph, "frontier": frontier, "count": count }),
            ),
            HookKind::Graph(hook::GraphEvent::FrontierDone {
                graph,
                frontier,
                count,
            }) => json_line(
                "frontier_done",
                serde_json::json!({ "graph": graph, "frontier": frontier, "count": count }),
            ),
            HookKind::Component(hook::ComponentEvent::Start(meta)) => {
                json_line("component_start", component_meta_json(meta))
            }
            HookKind::Component(hook::ComponentEvent::Done(meta)) => {
                json_line("component_done", component_meta_json(meta))
            }
            HookKind::Component(hook::ComponentEvent::Skipped(meta)) => {
                json_line("component_skipped", component_meta_json(meta))
            }
            HookKind::Machine(hook::MachineEvent::Start) => {
                json_line("machine_start", serde_json::json!({}))
            }
            HookKind::Machine(hook::MachineEvent::Halt { step }) => {
                json_line("halt", serde_json::json!({ "step": step }))
            }
            HookKind::Machine(hook::MachineEvent::Done) => {
                json_line("machine_done", serde_json::json!({}))
            }
            HookKind::Completion(hook::CompletionEvent::Start) => {
                json_line("completion_start", serde_json::json!({}))
            }
            HookKind::Completion(hook::CompletionEvent::End { fragments }) => json_line(
                "completion_end",
                serde_json::json!({ "fragments": fragments }),
            ),
            HookKind::Tool(hook::ToolEvent::Call { tool, arguments }) => json_line(
                "tool_call",
                serde_json::json!({ "tool": tool, "arguments": arguments }),
            ),
            HookKind::Tool(hook::ToolEvent::Result {
                tool,
                result_len,
                duration,
            }) => json_line(
                "tool_result",
                serde_json::json!({ "tool": tool, "result_len": result_len, "duration": duration }),
            ),
            HookKind::Tool(hook::ToolEvent::Error {
                tool,
                error,
                retryable,
            }) => json_line(
                "tool_error",
                serde_json::json!({ "tool": tool, "error": error, "retryable": retryable }),
            ),
            HookKind::Fragment(hook::FragmentEvent::Appended(meta)) => {
                json_line("appended", fragment_json(meta))
            }
            HookKind::Fragment(hook::FragmentEvent::Taken(meta)) => {
                json_line("taken", fragment_json(meta))
            }
            HookKind::Fragment(hook::FragmentEvent::Inserted(meta)) => {
                json_line("inserted", fragment_json(meta))
            }
            HookKind::Fragment(hook::FragmentEvent::Replaced(meta)) => {
                json_line("replaced", fragment_json(meta))
            }
            HookKind::Fragment(hook::FragmentEvent::Removed { id }) => {
                json_line("removed", serde_json::json!({ "id": id }))
            }
            HookKind::Fragment(hook::FragmentEvent::Swapped { first, second }) => json_line(
                "swapped",
                serde_json::json!({ "first": first, "second": second }),
            ),
            HookKind::Resource(hook::ResourceEvent::Model { name }) => {
                json_line("model", serde_json::json!({ "name": name }))
            }
            HookKind::Resource(hook::ResourceEvent::Activate { name }) => {
                json_line("activate", serde_json::json!({ "name": name }))
            }
            HookKind::Resource(hook::ResourceEvent::Deactivate { name }) => {
                json_line("deactivate", serde_json::json!({ "name": name }))
            }
        };
        println!("{line}");
    }

    drop(ctx_rx);
    Ok(())
}

fn json_line(event_type: &str, payload: serde_json::Value) -> String {
    let mut obj = payload.as_object().cloned().unwrap_or_default();
    obj.insert(
        "type".to_string(),
        serde_json::Value::String(event_type.to_string()),
    );
    serde_json::to_string(&obj).unwrap_or_default()
}

fn component_meta_json(meta: &hook::ComponentMeta) -> serde_json::Value {
    serde_json::json!({
        "name": meta.name,
        "kind": meta.kind,
        "index": meta.index,
        "graph": meta.graph,
        "frontier": meta.frontier,
    })
}

fn fragment_json(meta: &hook::FragmentMeta) -> serde_json::Value {
    serde_json::json!({
        "id": meta.id,
        "role": meta.role,
        "kind": meta.kind,
        "preview": meta.preview,
    })
}

fn hook_rx() -> mpsc::Receiver<crate::hook::HookEvent> {
    let (tx, rx) = mpsc::channel();
    let subscriber = tracing_subscriber::registry().with(crate::hook::hook_layer(tx));
    tracing::subscriber::set_global_default(subscriber).ok();
    rx
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
