//! `SpawnTool` — fan out a worker accelerator over a runtime list of items.
//!
//! A planner accelerator declares workers via the `spawns = [...]` DSL field.
//! At compile time a `SpawnTool` is created for each worker alias and injected
//! into the planner's tool runtime. The planner LLM calls the tool explicitly,
//! controls concurrency via `max_parallel`, and receives a structured summary
//! that it can use to retry failed items.

use std::pin::Pin;
use std::time::Duration;

use machine::{Environment, RunState, ToolResult};
use serde_json::Value;

use crate::accelerator::Accelerator;

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

    /// Extract a handoff status from a worker's output.
    /// Scans context fragments first, then falls back to the purpose field
    /// (status text set in the worker's base purpose survives merge_input).
    /// Returns one of `ok` / `partial` / `blocked` / `failed` / `unknown`.
    fn extract_status(output: &RunState) -> &'static str {
        // 1. Check context text fragments (handoff messages from real workers).
        for fragment in output.context.fragments() {
            let Some(text) = fragment.as_text() else {
                continue;
            };
            for line in text.lines() {
                if let Some(status) = Self::classify_status_line(line) {
                    return status;
                }
            }
        }
        // 2. Fallback: scan the purpose (workers with halt-only policies
        //    may encode status in purpose, which survives merge_input).
        for line in output.purpose.text.lines() {
            if let Some(status) = Self::classify_status_line(line) {
                return status;
            }
        }
        "unknown"
    }

    /// Classify a single line as a handoff status, if it is one. Matching is
    /// lenient on purpose: handoffs come from an LLM, so we accept the
    /// schema's `status: <value>` case-insensitively, ignore leading list /
    /// markdown markers, and look only at the first word after the colon. This
    /// keeps `- status: ok`, `Status: partial (image-only PDF)` etc. from being
    /// misread as failures.
    fn classify_status_line(line: &str) -> Option<&'static str> {
        let trimmed = line
            .trim()
            .trim_start_matches(['-', '*', '•', '#', '>', ' '])
            .trim()
            .to_ascii_lowercase();
        let value = trimmed.strip_prefix("status:")?;
        match value.split_whitespace().next()? {
            "ok" => Some("ok"),
            "partial" => Some("partial"),
            "blocked" => Some("blocked"),
            "failed" => Some("failed"),
            _ => None,
        }
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
        env: &'a Environment,
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

            let total = items.len();
            if total == 0 {
                return Ok(ToolResult {
                    content: "spawn called with empty items list, nothing to do.".into(),
                    call_id: String::new(),
                    title: Some("spawn: empty".into()),
                });
            }

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

            // Run workers in bounded waves. Use chunks so we never run more
            // than max_parallel futures concurrently. join_all works on any
            // tokio runtime (no tokio::spawn needed).
            let mut outcomes = vec![None; total];
            let mut start = 0;
            for chunk in items.chunks(max_parallel) {
                let wave: Vec<_> = chunk
                    .iter()
                    .enumerate()
                    .map(|(offset, item)| {
                        let worker = self.worker.clone();
                        let item_text =
                            serde_json::to_string_pretty(item).unwrap_or_else(|_| item.to_string());
                        // NOTE: every worker in the wave shares the SAME
                        // Environment (same RDC_TOKEN, RDC_RESEARCH_ID, cwd).
                        // Workers therefore MUST NOT rdc_write `update` the
                        // same entity_id concurrently — concurrent PATCHes to
                        // one RDC entity race (last write wins, earlier
                        // writes silently lost). Have each worker `create` its
                        // own entity, or coalesce shared writes in a serial
                        // component downstream of this fan-out.
                        let mut worker_state = RunState {
                            environment: env.clone(),
                            ..RunState::default()
                        };
                        worker_state.context.append(
                            machine::Fragment::user(format!(
                                "Your assigned work item:\n{item_text}"
                            ))
                            .with_tag("item"),
                        );
                        let idx = start + offset;
                        async move {
                            let output = worker.run_with(worker_state).await;
                            let status = SpawnTool::extract_status(&output);
                            (idx, status.to_string())
                        }
                    })
                    .collect();
                for (idx, status) in futures_util::future::join_all(wave).await {
                    outcomes[idx] = Some(status);
                }
                start += chunk.len();
            }

            // Build summary report. `ok` and `partial` both mean the worker
            // produced its artifact, so neither should be retried; only
            // blocked / failed / unknown are surfaced as failures.
            let mut ok = 0u32;
            let mut partial = 0u32;
            let mut failures = Vec::new();
            for (i, slot) in outcomes.iter().enumerate() {
                match slot.as_deref() {
                    Some("ok") => ok += 1,
                    Some("partial") => partial += 1,
                    _ => {
                        if let Some(id) = item_ids.get(i) {
                            failures.push(id.clone());
                        }
                    }
                }
            }

            let tool_name = self.name.strip_prefix("spawn_").unwrap_or(&self.name);
            let mut report = format!(
                "spawn_{tool_name} results ({total} items, max_parallel={max_parallel}):\nok={ok}"
            );
            if partial > 0 {
                report.push_str(&format!("\npartial={partial}"));
            }
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
                title: Some(format!("spawn_{tool_name}: {}/{} ok", ok + partial, total)),
            })
        })
    }
}
