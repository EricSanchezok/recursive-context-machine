//! Persistent structured working ledger (state-layer Blueprint P5).
//!
//! The model writes structure; this code guards the state machine:
//! `pending → running → completed | failed`, nothing else. When an entry
//! completes, pending entries whose deps are all completed are promoted
//! to running by code, so a stuck planner cannot skip the gate.
//!
//! Memory is the source of truth within a process; every mutation is
//! written through to `<run_dir>/ledger.json` when a run directory exists.
//! Without one, the ledger is ephemeral for the process. Tool results are
//! JSON carrying `"tool": "ledger"` and a `transitions` array so
//! `machine::ledger_transitions_in` can lift them into trajectory records.

use std::collections::HashMap;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Mutex, OnceLock};

use machine::{Environment, LedgerDigest, LedgerDigestEntry, Tool, ToolResult};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::warn;

const LEDGER_FILE_NAME: &str = "ledger.json";
const STATUS_PENDING: &str = "pending";
const STATUS_RUNNING: &str = "running";
const STATUS_COMPLETED: &str = "completed";
const STATUS_FAILED: &str = "failed";

/// One work item. The schema is deliberately flat: the ledger is read by
/// policies projecting result-level content, not by a query engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id: String,
    pub title: String,
    pub status: String,
    #[serde(default)]
    pub deps: Vec<String>,
    #[serde(default)]
    pub result: String,
    pub updated_at: String,
}

/// Recorded state migration, mirrored into tool results.
#[derive(Debug, Clone, Serialize)]
struct TransitionRecord {
    id: String,
    from: String,
    to: String,
}

/// Per-run ledger book. The empty path keys the ephemeral (no run_dir)
/// ledger so concurrent graph components sharing a run_dir also share
/// one book — exactly the semantics a ledger exists for.
fn ledgers() -> &'static Mutex<HashMap<PathBuf, HashMap<String, LedgerEntry>>> {
    static LEDGERS: OnceLock<Mutex<HashMap<PathBuf, HashMap<String, LedgerEntry>>>> =
        OnceLock::new();
    LEDGERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cache_key(run_dir: Option<&Path>) -> PathBuf {
    run_dir.map(Path::to_path_buf).unwrap_or_default()
}
/// Load the book for one run. Without a run directory the ledger is
/// process-ephemeral: no file is read or written, so a bare `accelerate
/// run` never pollutes the working directory.
fn load_book(run_dir: Option<&Path>) -> HashMap<String, LedgerEntry> {
    let Some(dir) = run_dir else {
        return HashMap::new();
    };
    let path = dir.join(LEDGER_FILE_NAME);
    match std::fs::read_to_string(&path) {
        Ok(raw) => serde_json::from_str(&raw).unwrap_or_else(|error| {
            warn!(
                path = %path.display(),
                ?error,
                "ledger file unreadable; starting from an empty ledger"
            );
            HashMap::new()
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => HashMap::new(),
        Err(error) => {
            warn!(
                path = %path.display(),
                ?error,
                "ledger file access failed; starting from an empty ledger"
            );
            HashMap::new()
        }
    }
}

fn persist_book(run_dir: Option<&Path>, book: &HashMap<String, LedgerEntry>) {
    let Some(dir) = run_dir else {
        return;
    };
    let path = dir.join(LEDGER_FILE_NAME);
    match serde_json::to_string_pretty(book) {
        Ok(raw) => {
            if let Err(error) = std::fs::write(&path, raw) {
                warn!(
                    path = %path.display(),
                    ?error,
                    "ledger persist failed; in-memory state stays authoritative"
                );
            }
        }
        Err(error) => warn!(?error, "ledger serialization failed"),
    }
}

fn with_book<T>(
    run_dir: Option<&Path>,
    operation: impl FnOnce(&mut HashMap<String, LedgerEntry>) -> T,
) -> T {
    let mut ledgers = ledgers().lock().expect("ledger lock poisoned");
    let key = cache_key(run_dir);
    let book = ledgers.entry(key).or_insert_with(|| load_book(run_dir));
    operation(book)
}

/// Digest for the observation channel. `None` means the run has never
/// touched a ledger — no phantom "empty ledger" in every observation.
pub fn ledger_digest_for(run_dir: &Path) -> Option<LedgerDigest> {
    let mut ledgers = ledgers().lock().expect("ledger lock poisoned");
    let book = ledgers.get_mut(&cache_key(Some(run_dir)))?;
    Some(digest_of(book))
}

fn digest_of(book: &HashMap<String, LedgerEntry>) -> LedgerDigest {
    let mut by_status: HashMap<String, u64> = HashMap::new();
    for entry in book.values() {
        *by_status.entry(entry.status.clone()).or_default() += 1;
    }
    let mut ordered: Vec<&LedgerEntry> = book.values().collect();
    ordered.sort_by(|left, right| left.id.cmp(&right.id));
    let current = ordered
        .iter()
        .find(|entry| entry.status == STATUS_RUNNING)
        .or_else(|| ordered.iter().find(|entry| entry.status == STATUS_PENDING))
        .map(|entry| LedgerDigestEntry {
            id: entry.id.clone(),
            title: entry.title.clone(),
            status: entry.status.clone(),
        });
    LedgerDigest {
        total: book.len() as u64,
        by_status,
        current_entry: current,
    }
}

fn now_timestamp() -> String {
    chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
}

fn find_entry<'a>(
    book: &'a HashMap<String, LedgerEntry>,
    id: &str,
) -> Result<&'a LedgerEntry, String> {
    book.get(id)
        .ok_or_else(|| format!("ledger entry '{id}' not found"))
}

fn guard_transition(entry: &mut LedgerEntry, to: &str) -> Result<TransitionRecord, String> {
    let allowed = match entry.status.as_str() {
        STATUS_PENDING => to == STATUS_RUNNING,
        STATUS_RUNNING => to == STATUS_COMPLETED || to == STATUS_FAILED,
        _ => false,
    };
    if !allowed {
        return Err(format!(
            "illegal ledger transition: '{}' entry '{}' cannot move to '{to}'",
            entry.status, entry.id
        ));
    }
    let from = entry.status.clone();
    entry.status = to.to_string();
    entry.updated_at = now_timestamp();
    Ok(TransitionRecord {
        id: entry.id.clone(),
        from,
        to: to.to_string(),
    })
}

fn deps_are_completed(book: &HashMap<String, LedgerEntry>, entry: &LedgerEntry) -> bool {
    entry.deps.iter().all(|dep| {
        book.get(dep)
            .is_some_and(|dep_entry| dep_entry.status == STATUS_COMPLETED)
    })
}

/// Promote pending entries whose deps are all completed. Runs after any
/// completion because that is the only status change that can unblock.
fn promote_ready(book: &mut HashMap<String, LedgerEntry>) -> Vec<TransitionRecord> {
    let mut promoted = Vec::new();
    loop {
        let ready: Vec<String> = book
            .values()
            .filter(|entry| entry.status == STATUS_PENDING && deps_are_completed(book, entry))
            .map(|entry| entry.id.clone())
            .collect();
        if ready.is_empty() {
            break;
        }
        for id in ready {
            if let Some(entry) = book.get_mut(&id)
                && let Ok(record) = guard_transition(entry, STATUS_RUNNING)
            {
                promoted.push(record);
            }
        }
    }
    promoted
}

fn entry_json(entry: &LedgerEntry) -> Value {
    serde_json::to_value(entry).unwrap_or_else(|error| {
        warn!(?error, "ledger entry serialization failed");
        Value::Null
    })
}

pub struct LedgerTool;

impl Tool for LedgerTool {
    fn name(&self) -> &str {
        "ledger"
    }

    fn description(&self) -> &str {
        "Structured working ledger that survives context edits. Add entries for planned work \
         (optionally with deps on other entry ids), start/complete/fail them; entries whose deps \
         are all completed are promoted automatically. Use list/get to read. Statuses move only \
         pending→running→completed|failed."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "op": {
                    "type": "string",
                    "enum": ["add", "start", "complete", "fail", "list", "get"],
                    "description": "Ledger operation."
                },
                "id": {"type": "string", "description": "Entry id (add/start/complete/fail/get)."},
                "title": {"type": "string", "description": "Entry title (add)."},
                "deps": {"type": "array", "items": {"type": "string"}, "description": "Entry ids this work depends on (add)."},
                "result": {"type": "string", "description": "Outcome text (complete/fail)."}
            },
            "required": ["op"]
        })
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        let run_dir = env.run_dir.clone();
        Box::pin(async move {
            let operation = args["op"]
                .as_str()
                .ok_or("missing required parameter 'op'")?;
            let id = args["id"].as_str().unwrap_or_default().to_string();

            let (transitions, payload) = with_book(run_dir.as_deref(), |book| match operation {
                "add" => {
                    if id.is_empty() {
                        return Err("ledger add requires an 'id'".to_string());
                    }
                    if book.contains_key(&id) {
                        return Err(format!("ledger entry '{id}' already exists"));
                    }
                    let entry = LedgerEntry {
                        id: id.clone(),
                        title: args["title"].as_str().unwrap_or_default().to_string(),
                        status: STATUS_PENDING.to_string(),
                        deps: args["deps"]
                            .as_array()
                            .map(|deps| {
                                deps.iter()
                                    .filter_map(|dep| dep.as_str().map(String::from))
                                    .collect()
                            })
                            .unwrap_or_default(),
                        result: String::new(),
                        updated_at: now_timestamp(),
                    };
                    let payload = json!({"op": "add", "entry": entry_json(&entry)});
                    book.insert(id, entry);
                    Ok((Vec::new(), payload))
                }
                "start" => {
                    let entry = find_entry(book, &id)?;
                    if !entry.deps.is_empty() && !deps_are_completed(book, entry) {
                        let unmet: Vec<String> = entry
                            .deps
                            .iter()
                            .filter(|dep| {
                                book.get(*dep)
                                    .is_none_or(|dep_entry| dep_entry.status != STATUS_COMPLETED)
                            })
                            .cloned()
                            .collect();
                        return Err(format!(
                            "entry '{id}' has uncompleted deps: {}",
                            unmet.join(", ")
                        ));
                    }
                    let entry = book.get_mut(&id).expect("entry existence checked above");
                    let record = guard_transition(entry, STATUS_RUNNING)?;
                    Ok((
                        vec![record],
                        json!({"op": "start", "entry": entry_json(entry)}),
                    ))
                }
                "complete" | "fail" => {
                    let target = if operation == "complete" {
                        STATUS_COMPLETED
                    } else {
                        STATUS_FAILED
                    };
                    let entry = book
                        .get_mut(&id)
                        .ok_or(format!("ledger entry '{id}' not found"))?;
                    let record = guard_transition(entry, target)?;
                    if let Some(result_text) = args["result"].as_str() {
                        entry.result = result_text.to_string();
                    }
                    let mut transitions = vec![record];
                    transitions.extend(promote_ready(book));
                    let entry = book.get(&id).expect("entry existence checked above");
                    Ok((
                        transitions,
                        json!({"op": operation, "entry": entry_json(entry)}),
                    ))
                }
                "get" => {
                    let entry = find_entry(book, &id)?;
                    Ok((Vec::new(), json!({"op": "get", "entry": entry_json(entry)})))
                }
                "list" => {
                    let mut ordered: Vec<&LedgerEntry> = book.values().collect();
                    ordered.sort_by(|left, right| left.id.cmp(&right.id));
                    Ok((
                        Vec::new(),
                        json!({
                            "op": "list",
                            "entries": ordered.iter().map(|entry| entry_json(entry)).collect::<Vec<_>>(),
                        }),
                    ))
                }
                other => Err(format!("unknown ledger op '{other}'")),
            })?;

            persist_book(
                run_dir.as_deref(),
                &with_book(run_dir.as_deref(), |book| book.clone()),
            );

            let transition_values: Vec<Value> = transitions
                .iter()
                .map(|record| json!({"id": record.id, "from": record.from, "to": record.to}))
                .collect();
            let content = json!({
                "tool": "ledger",
                "op": operation,
                "transitions": transition_values,
                "digest": with_book(run_dir.as_deref(), |book| digest_of(book)),
                "result": payload,
            })
            .to_string();

            Ok(ToolResult {
                call_id: String::new(),
                content,
                title: Some(format!("ledger {operation}")),
            })
        })
    }
}
