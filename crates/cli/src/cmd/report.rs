//! Run report — the external feedback surface for optimizers.
//!
//! When a run directory exists, `accelerate run` finishes by writing
//! `report.json` there: identity (label, purpose preview), outcome (final
//! answer), cost (tokens, tool calls, wall time), and pointers to the
//! trajectory/registry/ledger artifacts. An external optimizer (e.g. a
//! harness-manager agent) reads exactly this file to decide how to revise
//! the harness for the next iteration — see
//! `docs/design/optimizer-loop.md` for the loop protocol.

use std::path::Path;
use std::time::Duration;

use machine::Context;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RunReport {
    pub schema: &'static str,
    pub label: String,
    pub purpose: String,
    pub answer: String,
    pub steps: RunSteps,
    pub cost: RunCost,
    pub artifacts: RunArtifacts,
}

#[derive(Debug, Serialize)]
pub struct RunSteps {
    /// Completion calls (LLM turns) taken.
    pub completions: u64,
    /// Tool calls that returned a result.
    pub tool_calls: u64,
}

#[derive(Debug, Serialize)]
pub struct RunCost {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub total_tokens: u64,
    pub wall_ms: u128,
}

#[derive(Debug, Serialize)]
pub struct RunArtifacts {
    /// Trajectory WAL directories under this run, relative form.
    pub trajectory_dirs: Vec<String>,
    /// Resource registry file, when the agent registered resources.
    pub registry: Option<String>,
    /// Working ledger file, when the agent used the ledger tool.
    pub ledger: Option<String>,
}

/// Aggregated run measurements fed by the CLI's hook-stream accounting.
#[derive(Debug, Clone, Copy, Default)]
pub struct RunMeasures {
    pub completions: u64,
    pub tool_calls: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

/// Assemble and persist the report for one finished run. Failures are
/// logged, never fatal — a missing report must not fail a successful run.
pub fn write(
    run_dir: &Path,
    label: &str,
    purpose: &str,
    context: &Context,
    measures: RunMeasures,
    wall: Duration,
) -> Option<std::path::PathBuf> {
    let answer = context
        .fragments()
        .iter()
        .rev()
        .find(|fragment| fragment.role == machine::Role::Assistant)
        .and_then(|fragment| fragment.as_text().map(String::from))
        .unwrap_or_default();

    let report = RunReport {
        schema: "rcm.run.report/v1",
        label: label.to_string(),
        purpose: preview(purpose, 2_000),
        answer: preview(&answer, 8_000),
        steps: RunSteps {
            completions: measures.completions,
            tool_calls: measures.tool_calls,
        },
        cost: RunCost {
            input_tokens: measures.input_tokens,
            output_tokens: measures.output_tokens,
            total_tokens: measures.input_tokens.saturating_add(measures.output_tokens),
            wall_ms: wall.as_millis(),
        },
        artifacts: scan_artifacts(run_dir),
    };
    persist(run_dir, &report)
}

fn preview(text: &str, limit_chars: usize) -> String {
    if text.chars().count() <= limit_chars {
        text.to_string()
    } else {
        let truncated: String = text.chars().take(limit_chars).collect();
        format!("{truncated}\n[truncated for report preview]")
    }
}

fn scan_artifacts(run_dir: &Path) -> RunArtifacts {
    let mut trajectory_dirs = Vec::new();
    let trajectory_root = run_dir.join("trajectory");
    if let Ok(entries) = std::fs::read_dir(&trajectory_root) {
        for entry in entries.flatten() {
            if entry.path().is_dir()
                && let Some(name) = entry.file_name().to_str()
            {
                trajectory_dirs.push(format!("trajectory/{name}"));
            }
        }
    }
    trajectory_dirs.sort();

    let registry = run_dir
        .join("resources")
        .join("registry.json")
        .is_file()
        .then(|| "resources/registry.json".to_string());
    let ledger = run_dir
        .join("ledger.json")
        .is_file()
        .then(|| "ledger.json".to_string());

    RunArtifacts {
        trajectory_dirs,
        registry,
        ledger,
    }
}

fn persist(run_dir: &Path, report: &RunReport) -> Option<std::path::PathBuf> {
    let path = run_dir.join("report.json");
    match serde_json::to_string_pretty(report) {
        Ok(raw) => {
            if let Err(error) = std::fs::write(&path, raw) {
                tracing::warn!(
                    path = %path.display(),
                    ?error,
                    "run report write failed; run output is unaffected"
                );
                return None;
            }
            Some(path)
        }
        Err(error) => {
            tracing::warn!(?error, "run report serialization failed");
            None
        }
    }
}
