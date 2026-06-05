//! `SpawnTool` — fan out a worker accelerator over a runtime list of items.
//!
//! A planner accelerator declares workers via the `spawns = [...]` DSL field.
//! At compile time a `SpawnTool` is created for each worker alias and injected
//! into the planner's tool runtime. The planner LLM calls the tool explicitly,
//! controls concurrency via `max_parallel`, and receives a structured summary
//! that it can use to retry failed items.

use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use futures_util::StreamExt;
use futures_util::stream::FuturesUnordered;
use machine::ToolResult;
use serde_json::Value;
use tokio::sync::Semaphore;

use crate::accelerator::Accelerator;
use crate::state::State;

/// Tool that spawns many worker accelerator instances concurrently.
///
/// Signature visible to the LLM:
/// - `items`: JSON array of work item objects (each becomes one worker input)
/// - `max_parallel`: concurrency cap (default 5)
///
/// Returns a per-item outcome summary. Failed items are surfaced for retry.
#[derive(Clone)]
pub struct SpawnTool {
    name: String,
    description: String,
    worker: Accelerator,
}

impl SpawnTool {
    pub fn new(name: impl Into<String>, worker: Accelerator) -> Self {
        let name = name.into();
        let description = format!(
            "Spawn a `{name}` worker for each item in a batch.\n\n\
             - `items`: JSON array of work item objects. Each item must contain \
             all fields the worker needs (e.g. `id`, `title`, `why`, `run_dir`).\n\
             - `max_parallel`: optional concurrency limit (default 5).\n\n\
             Returns a per-item outcome summary. Inspect failed items and retry \
             them by calling this tool again with a smaller items list."
        );
        Self {
            name,
            description,
            worker,
        }
    }
}

impl SpawnTool {
    /// Extract the status from a worker's output state by scanning handoff text.
    fn extract_status(output: &State) -> &'static str {
        for fragment in output.ctx.fragments() {
            let Some(text) = fragment.as_text() else {
                continue;
            };
            if text.contains("status: ok") {
                return "ok";
            }
            if text.contains("evidence: abstract_only") {
                return "abstract_only";
            }
            if text.contains("blocked") {
                return "blocked";
            }
            if text.contains("status: failed") {
                return "failed";
            }
        }
        "unknown"
    }
}

impl machine::Tool for SpawnTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "items": {
                    "type": "array",
                    "description": "JSON array of work items. Each item is passed as the work item to one worker instance.",
                    "items": { "type": "object" }
                },
                "max_parallel": {
                    "type": "integer",
                    "description": "Maximum concurrent workers (default 5).",
                    "default": 5
                }
            },
            "required": ["items"]
        })
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(600)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        _env: &'a machine::Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let items = args
                .get("items")
                .and_then(|v| v.as_array())
                .ok_or_else(|| "spawn requires 'items' as a JSON array".to_string())?;
            let max_parallel = args
                .get("max_parallel")
                .and_then(|v| v.as_u64())
                .unwrap_or(5)
                .max(1) as usize;

            let semaphore = Arc::new(Semaphore::new(max_parallel));
            let total = items.len();

            // Pre-extract item IDs for the summary report.
            let item_ids: Vec<String> = items
                .iter()
                .map(|item| {
                    item.get("id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "?".to_string())
                })
                .collect();

            // Fire each worker as a spawned task, bounded by the semaphore.
            let mut futures: FuturesUnordered<_> = FuturesUnordered::new();
            for (i, item) in items.iter().enumerate() {
                let permit = semaphore
                    .clone()
                    .acquire_owned()
                    .await
                    .map_err(|_| "spawn concurrency interrupted".to_string())?;
                let worker = self.worker.clone();
                let item_text =
                    serde_json::to_string_pretty(item).unwrap_or_else(|_| item.to_string());
                let mut worker_state = State::default();
                worker_state.ctx.append(
                    machine::Fragment::user(format!("Your assigned work item:\n{item_text}"))
                        .with_tag("item"),
                );

                futures.push(tokio::spawn(async move {
                    let _permit = permit;
                    let output = worker.run_with(worker_state).await;
                    let status = SpawnTool::extract_status(&output);
                    (i, status.to_string())
                }));
            }

            // Collect all outcomes.
            let mut outcomes = Vec::with_capacity(total);
            while let Some(result) = futures.next().await {
                match result {
                    Ok((i, status)) => outcomes.push((i, status)),
                    Err(e) => outcomes.push((total, format!("task_panic: {e}"))),
                }
            }

            // Build summary report.
            let mut ok = 0u32;
            let mut failures = Vec::new();
            for (i, status) in &outcomes {
                if status == "ok" {
                    ok += 1;
                } else if let Some(id) = item_ids.get(*i) {
                    failures.push(id.clone());
                }
            }

            let tool_name = self.name.strip_prefix("spawn_").unwrap_or(&self.name);
            let mut report = format!(
                "spawn_{tool_name} results ({total} items, max_parallel={max_parallel}):\nok={ok}"
            );
            if failures.is_empty() {
                report.push_str("\nAll items completed successfully.");
            } else {
                report.push_str(&format!("\nfailed={}", failures.len()));
                for id in &failures {
                    report.push_str(&format!("\n  ✗ {id}"));
                }
                report.push_str(
                    "\n\nYou may retry failed items by calling this tool again with only those items.",
                );
            }

            Ok(ToolResult {
                content: report,
                call_id: String::new(),
                title: Some(format!("spawn_{tool_name}: {ok}/{} ok", outcomes.len())),
            })
        })
    }
}
