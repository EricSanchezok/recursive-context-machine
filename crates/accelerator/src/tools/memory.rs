//! Persistent agent memory (C3 v1): run-scoped key-value notes.
//!
//! `memory.write` / `memory.search` / `memory.format` operate on
//! `<run_dir>/memory.json` with a process-global cache keyed by run
//! directory — the same single-process semantics as the ledger and the
//! resource registry. Search is naive keyword scoring (R2 upgrades it to
//! an embedding index; v1 keeps zero new dependencies). The tool name is
//! the namespace: `memory.*`.

use std::collections::HashMap;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Mutex, OnceLock};

use machine::{Environment, Tool, ToolResult};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::warn;

const MEMORY_FILE_NAME: &str = "memory.json";
/// How many search hits to return; keyword scoring, best first.
const SEARCH_RESULT_LIMIT: usize = 8;
const SNIPPET_CHARS: usize = 160;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct MemoryEntry {
    content: String,
    updated_at: String,
}

fn memories() -> &'static Mutex<HashMap<PathBuf, HashMap<String, MemoryEntry>>> {
    static MEMORIES: OnceLock<Mutex<HashMap<PathBuf, HashMap<String, MemoryEntry>>>> =
        OnceLock::new();
    MEMORIES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn memory_path(run_dir: Option<&Path>) -> PathBuf {
    run_dir
        .map(|dir| dir.join(MEMORY_FILE_NAME))
        .unwrap_or_else(|| PathBuf::from(MEMORY_FILE_NAME))
}

fn cache_key(run_dir: Option<&Path>) -> PathBuf {
    run_dir.map(Path::to_path_buf).unwrap_or_default()
}

fn load_entries(run_dir: Option<&Path>) -> HashMap<String, MemoryEntry> {
    let Some(dir) = run_dir else {
        return HashMap::new();
    };
    match std::fs::read_to_string(memory_path(Some(dir))) {
        Ok(raw) => serde_json::from_str(&raw).unwrap_or_else(|error| {
            warn!(
                path = %dir.display(),
                ?error,
                "memory file unreadable; starting empty"
            );
            HashMap::new()
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => HashMap::new(),
        Err(error) => {
            warn!(
                path = %dir.display(),
                ?error,
                "memory file access failed; starting empty"
            );
            HashMap::new()
        }
    }
}

fn persist_entries(run_dir: Option<&Path>, entries: &HashMap<String, MemoryEntry>) {
    let Some(dir) = run_dir else {
        return;
    };
    match serde_json::to_string_pretty(entries) {
        Ok(raw) => {
            if let Err(error) = std::fs::write(memory_path(Some(dir)), raw) {
                warn!(
                    path = %dir.display(),
                    ?error,
                    "memory persist failed; in-memory copy stays authoritative"
                );
            }
        }
        Err(error) => warn!(?error, "memory serialization failed"),
    }
}

fn with_entries<T>(
    run_dir: Option<&Path>,
    reader: impl FnOnce(&mut HashMap<String, MemoryEntry>) -> T,
) -> T {
    let mut memories = memories().lock().expect("memory lock poisoned");
    let entries = memories.entry(cache_key(run_dir)).or_insert_with(|| {
        let loaded = load_entries(run_dir);
        if run_dir.is_some() && !loaded.is_empty() {
            persist_entries(run_dir, &loaded);
        }
        loaded
    });
    reader(entries)
}

fn now_timestamp() -> String {
    chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
}

/// Score weight for one matched query term. Must dominate any achievable
/// frequency bonus so a match on more distinct terms always outranks
/// repetition of fewer terms.
const PRESENCE_WEIGHT: u64 = 1_000;

/// Naive keyword score: count of matched query terms (dominant) plus total
/// term occurrences (tiebreaker). Case-insensitive, term-split on
/// whitespace.
fn keyword_score(query: &str, content: &str) -> u64 {
    let content_lower = content.to_lowercase();
    let mut present_terms: u64 = 0;
    let mut occurrences: u64 = 0;
    for term in query.split_whitespace().map(str::to_lowercase) {
        let count: u64 = content_lower
            .match_indices(&term)
            .count()
            .try_into()
            .unwrap_or(u64::MAX);
        if count > 0 {
            present_terms += 1;
            occurrences = occurrences.saturating_add(count.min(u64::from(u16::MAX)));
        }
    }
    present_terms
        .saturating_mul(PRESENCE_WEIGHT)
        .saturating_add(occurrences)
}

/// Upsert one note under a key.
pub struct MemoryWriteTool;

impl Tool for MemoryWriteTool {
    fn name(&self) -> &str {
        "memory.write"
    }

    fn description(&self) -> &str {
        "Persist a note under a key (upsert). Notes survive context edits \
and compaction; storage is <run_dir>/memory.json."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "key": {"type": "string"},
                "content": {"type": "string"}
            },
            "required": ["key", "content"]
        })
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        let args = args.clone();
        let env = env.clone();
        Box::pin(async move {
            let key = args["key"]
                .as_str()
                .ok_or("memory.write requires 'key'")?
                .to_string();
            if key.trim().is_empty() {
                return Err("memory key must not be empty".into());
            }
            let content = args["content"]
                .as_str()
                .ok_or("memory.write requires 'content'")?
                .to_string();
            let updated_at = now_timestamp();
            with_entries(env.run_dir.as_deref(), |entries| {
                entries.insert(
                    key.clone(),
                    MemoryEntry {
                        content,
                        updated_at,
                    },
                );
                persist_entries(env.run_dir.as_deref(), entries);
            });
            Ok(ToolResult {
                call_id: String::new(),
                content: json!({
                    "tool": "memory.write",
                    "key": key,
                })
                .to_string(),
                title: Some("memory.write".into()),
            })
        })
    }
}

/// Keyword search over stored notes, best match first.
pub struct MemorySearchTool;

impl Tool for MemorySearchTool {
    fn name(&self) -> &str {
        "memory.search"
    }

    fn description(&self) -> &str {
        "Search stored notes by keywords (case-insensitive; more matched \
terms ranks higher). Returns up to 8 matches as {key, snippet}."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {"type": "string"}
            },
            "required": ["query"]
        })
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        let args = args.clone();
        let env = env.clone();
        Box::pin(async move {
            let query = args["query"]
                .as_str()
                .ok_or("memory.search requires 'query'")?
                .to_string();
            // Clone out of the lock scope: ranked results must outlive the
            // closure that borrowed the entry map.
            let mut ranked = with_entries(env.run_dir.as_deref(), |entries| {
                entries
                    .iter()
                    .map(|(key, entry)| {
                        (
                            keyword_score(&query, &entry.content),
                            key.clone(),
                            entry.clone(),
                        )
                    })
                    .filter(|(score, _, _)| *score > 0)
                    .collect::<Vec<_>>()
            });
            ranked.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| left.1.cmp(&right.1)));
            let matches: Vec<Value> = ranked
                .into_iter()
                .take(SEARCH_RESULT_LIMIT)
                .map(|(_, key, entry)| {
                    json!({
                        "key": key,
                        "snippet": entry
                            .content
                            .chars()
                            .take(SNIPPET_CHARS)
                            .collect::<String>(),
                    })
                })
                .collect();
            Ok(ToolResult {
                call_id: String::new(),
                content: json!({
                    "tool": "memory.search",
                    "query": query,
                    "matches": matches,
                })
                .to_string(),
                title: Some("memory.search".into()),
            })
        })
    }
}

/// Render every stored note as one markdown document.
pub struct MemoryFormatTool;

impl Tool for MemoryFormatTool {
    fn name(&self) -> &str {
        "memory.format"
    }

    fn description(&self) -> &str {
        "Return all stored notes as one markdown document (## key sections)."
    }

    fn parameters(&self) -> Value {
        json!({"type": "object", "properties": {}})
    }

    fn execute<'a>(
        &'a self,
        _args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        let env = env.clone();
        Box::pin(async move {
            let lines = with_entries(env.run_dir.as_deref(), |entries| {
                let mut lines: Vec<(String, String)> = entries
                    .iter()
                    .map(|(key, entry)| (key.clone(), entry.content.clone()))
                    .collect();
                lines.sort_by(|left, right| left.0.cmp(&right.0));
                lines
            });
            let notes = if lines.is_empty() {
                "memory is empty".to_string()
            } else {
                let mut notes = String::from("# Memory\n\n");
                for (key, content) in lines {
                    notes.push_str(&format!("## {key}\n\n{content}\n\n"));
                }
                notes
            };
            Ok(ToolResult {
                call_id: String::new(),
                content: json!({
                    "tool": "memory.format",
                    "notes": notes,
                })
                .to_string(),
                title: Some("memory.format".into()),
            })
        })
    }
}

/// Exported for tests: drop the in-process memory cache so a fresh run_dir
/// view starts clean (nextest runs each test in its own process, but tests
/// within one process may reuse directories).
pub fn reset_for_tests() {
    if let Ok(mut memories) = memories().lock() {
        memories.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score(query: &str, content: &str) -> u64 {
        keyword_score(query, content)
    }

    #[test]
    fn score_ranks_multi_term_presence_above_frequency() {
        let both_terms = score("alpha beta", "alpha beta");
        let one_term_repeated = score("alpha beta", "alpha alpha alpha");
        assert!(both_terms > one_term_repeated);
    }

    #[test]
    fn score_is_case_insensitive_and_zero_without_terms() {
        assert!(score("ALPHA", "the alpha file") > 0);
        assert_eq!(score("alpha", "unrelated"), 0);
    }
}
