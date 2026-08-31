use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use tokio::io::AsyncReadExt;
use tracing_subscriber::prelude::*;

use super::report;
use crate::args::{Format, RunArgs};
use crate::hook::{self, HookKind};
use crate::output;

const RUNTIME_POLL_INTERVAL: Duration = Duration::from_millis(50);

pub async fn run(args: RunArgs) -> anyhow::Result<()> {
    let (hook_tx, hook_rx) = mpsc::channel();
    init_tracing(hook_tx);

    let accelerator = crate::rcm::compile::compile_file(&args.file)
        .await
        .map_err(anyhow::Error::msg)?;

    let purpose = if args.purpose_stdin {
        let mut purpose = String::new();
        tokio::io::stdin().read_to_string(&mut purpose).await?;
        purpose
    } else {
        args.purpose.unwrap_or_default()
    };
    let run_dir = prepare_run_dir(args.run_dir)?;

    if args.stream {
        return stream_run(accelerator, hook_rx, purpose, run_dir, args.label).await;
    }

    let start = Instant::now();
    let (runtime_thread, ctx_rx) = spawn_runtime(accelerator, purpose.clone(), run_dir.clone());

    let summary = output::tape::run_animation(hook_rx, args.speed, start, &runtime_thread);
    let ctx = finish_runtime(runtime_thread, ctx_rx)?;

    if let Some(ref dir) = run_dir {
        let measures = report::RunMeasures {
            completions: summary.completions,
            tool_calls: summary.tool_calls as u64,
            input_tokens: summary.input_tokens,
            output_tokens: summary.output_tokens,
        };
        report::write(dir, &args.label, &purpose, &ctx, measures, start.elapsed());
    }

    match args.format {
        Format::Text => output::text::print(&ctx, &summary, args.context),
        Format::Json => output::json::print(&ctx, &summary, args.context)?,
    }

    Ok(())
}

// ===========================================================================
// Stream JSON Line Format — EXTERNAL FROZEN CONTRACT
//
// This JSON output format is consumed by portal-gateway's parse-rcm.ts and
// must NOT be changed without synchronizing with the gateway codebase.
//
// Format: {"type":"<event_type>","field1":"value1",...}
//
// Event types:
//   graph_start, graph_done, frontier_start, frontier_done
//   component_start, component_done, component_skipped
//   machine_start, machine_done, halt, completion_start, completion_end
//   tool_call, tool_result, tool_error
//   appended, taken, inserted, replaced, removed, swapped
//   moved, consumed (document-model v2: Move op, inbox consumption)
//   model, activate, deactivate, resource
//
// All field names use snake_case.
// Consumer: portal-gateway/src/hub/parse-rcm.ts (gateway sync waived for
// the v2 additions; the gateway ignores unrecognized event types)
// ===========================================================================
async fn stream_run(
    accelerator: accelerator::Accelerator,
    hook_rx: mpsc::Receiver<hook::HookEvent>,
    purpose: String,
    run_dir: Option<std::path::PathBuf>,
    label: String,
) -> anyhow::Result<()> {
    let (runtime_thread, ctx_rx) = spawn_runtime(accelerator, purpose.clone(), run_dir.clone());

    let mut graph_seen = false;
    let mut measures = report::RunMeasures::default();
    let start = Instant::now();
    loop {
        let event = match hook_rx.recv_timeout(RUNTIME_POLL_INTERVAL) {
            Ok(event) => event,
            Err(mpsc::RecvTimeoutError::Timeout) if runtime_thread.is_finished() => break,
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        };
        if let HookKind::Completion(hook::CompletionEvent::End {
            input_tokens,
            output_tokens,
            ..
        }) = &event.kind
        {
            measures.completions += 1;
            measures.input_tokens = measures.input_tokens.saturating_add(*input_tokens);
            measures.output_tokens = measures.output_tokens.saturating_add(*output_tokens);
        }
        if matches!(&event.kind, HookKind::Tool(hook::ToolEvent::Result { .. })) {
            measures.tool_calls += 1;
        }
        if event.source.is_none()
            && matches!(event.kind, HookKind::Graph(hook::GraphEvent::Start { .. }))
        {
            graph_seen = true;
        }
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
            HookKind::Completion(completion @ hook::CompletionEvent::End { .. }) => {
                json_line("completion_end", completion_event_json(completion))
            }
            HookKind::Tool(hook::ToolEvent::Call {
                call_id,
                tool,
                arguments,
            }) => json_line(
                "tool_call",
                serde_json::json!({ "call_id": call_id, "tool": tool, "arguments": arguments }),
            ),
            HookKind::Tool(hook::ToolEvent::Result {
                call_id,
                tool,
                result_len,
                duration,
            }) => json_line(
                "tool_result",
                serde_json::json!({ "call_id": call_id, "tool": tool, "result_len": result_len, "duration": duration }),
            ),
            HookKind::Tool(hook::ToolEvent::Error {
                call_id,
                tool,
                error,
                retryable,
            }) => json_line(
                "tool_error",
                serde_json::json!({ "call_id": call_id, "tool": tool, "error": error, "retryable": retryable }),
            ),
            HookKind::Fragment(hook::FragmentEvent::Appended(meta)) => {
                json_line("appended", fragment_json(meta))
            }
            HookKind::Fragment(hook::FragmentEvent::Taken(meta)) => {
                json_line("taken", fragment_json(meta))
            }
            HookKind::Fragment(hook::FragmentEvent::Inserted { meta, after }) => {
                let mut payload = fragment_json(meta);
                payload["after"] = serde_json::json!(after);
                json_line("inserted", payload)
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
            HookKind::Fragment(hook::FragmentEvent::Moved { id, after }) => {
                json_line("moved", serde_json::json!({ "id": id, "after": after }))
            }
            HookKind::Fragment(hook::FragmentEvent::Consumed { call_id }) => {
                json_line("consumed", serde_json::json!({ "call_id": call_id }))
            }
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
        if is_terminal_event(&event, graph_seen) {
            break;
        }
    }

    let ctx = finish_runtime(runtime_thread, ctx_rx)?;
    if let Some(ref dir) = run_dir {
        report::write(dir, &label, &purpose, &ctx, measures, start.elapsed());
    }
    Ok(())
}

fn spawn_runtime(
    accelerator: accelerator::Accelerator,
    purpose: String,
    run_dir: Option<std::path::PathBuf>,
) -> (thread::JoinHandle<()>, mpsc::Receiver<machine::Context>) {
    let (ctx_tx, ctx_rx) = mpsc::channel();
    let runtime_thread = thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");
        runtime.block_on(async {
            let mut state = machine::RunState {
                purpose: machine::Purpose::new(purpose),
                run_dir: run_dir.clone(),
                ..machine::RunState::default()
            };
            if let Some(ref dir) = run_dir {
                state.environment.cwd = dir.clone();
                state.environment.root = Some(dir.clone());
                state.environment.run_dir = Some(dir.clone());
                state
                    .environment
                    .vars
                    .insert("RCM_RUN_DIR".to_string(), dir.display().to_string());
            }
            let output = accelerator.run_with(state).await;
            let _ = ctx_tx.send(output.context);
        });
    });
    (runtime_thread, ctx_rx)
}

fn finish_runtime(
    runtime_thread: thread::JoinHandle<()>,
    ctx_rx: mpsc::Receiver<machine::Context>,
) -> anyhow::Result<machine::Context> {
    if let Err(payload) = runtime_thread.join() {
        let message = payload
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| payload.downcast_ref::<&str>().copied())
            .unwrap_or("unknown panic");
        return Err(anyhow::anyhow!("RCM runtime panicked: {message}"));
    }
    ctx_rx
        .recv()
        .map_err(|error| anyhow::anyhow!("RCM runtime finished without context: {error}"))
}

fn prepare_run_dir(
    run_dir: Option<std::path::PathBuf>,
) -> anyhow::Result<Option<std::path::PathBuf>> {
    let Some(run_dir) = run_dir else {
        return Ok(None);
    };
    std::fs::create_dir_all(&run_dir)?;
    Ok(Some(run_dir.canonicalize()?))
}

pub fn is_terminal_event(event: &hook::HookEvent, graph_seen: bool) -> bool {
    match &event.kind {
        HookKind::Graph(hook::GraphEvent::Done { .. }) => event.source.is_none(),
        HookKind::Machine(hook::MachineEvent::Done) => !graph_seen && event.source.is_none(),
        _ => false,
    }
}

fn json_line(event_type: &str, payload: serde_json::Value) -> String {
    let mut obj = payload.as_object().cloned().unwrap_or_default();
    obj.insert(
        "type".to_string(),
        serde_json::Value::String(event_type.to_string()),
    );
    serde_json::to_string(&obj).unwrap_or_default()
}

/// Serialize a completion event using only the additive, sanitized stream fields.
pub fn completion_event_json(event: &hook::CompletionEvent) -> serde_json::Value {
    let hook::CompletionEvent::End {
        fragments,
        input_tokens,
        output_tokens,
        total_tokens,
        cached_input_tokens,
        cache_creation_input_tokens,
        outcome,
        http_status,
        failure_kind,
        retryable,
        duration_ms,
    } = event
    else {
        return serde_json::json!({});
    };

    let mut payload = serde_json::json!({
        "fragments": fragments,
        "input_tokens": input_tokens,
        "output_tokens": output_tokens,
        "total_tokens": total_tokens,
        "cached_input_tokens": cached_input_tokens,
        "cache_creation_input_tokens": cache_creation_input_tokens,
    });
    let object = payload
        .as_object_mut()
        .expect("completion payload is an object");
    if let Some(value) = outcome {
        object.insert("outcome".into(), serde_json::json!(value));
    }
    if let Some(value) = http_status {
        object.insert("http_status".into(), serde_json::json!(value));
    }
    if let Some(value) = failure_kind {
        object.insert("failure_kind".into(), serde_json::json!(value));
    }
    if let Some(value) = retryable {
        object.insert("retryable".into(), serde_json::json!(value));
    }
    if let Some(value) = duration_ms {
        object.insert("duration_ms".into(), serde_json::json!(value));
    }
    payload
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
        "tag": meta.tag,
        "preview": meta.preview,
    })
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
