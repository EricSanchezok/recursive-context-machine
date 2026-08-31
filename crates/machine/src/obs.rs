//! Controller observation channel.
//!
//! [`Obs`] is the policy-facing sensory layer derived from [`RunState`]
//! immediately before every `decide` call. It is never stored on the machine
//! state: every read is freshly computed, so it can never drift from the
//! state it summarizes.
//!
//! Budget estimation uses the chars-per-token heuristic shared with
//! production harnesses (synergy's prompt budgeter):
//! `(Σ text chars + Σ active tool manifest chars) / 4 + 500 per media
//! fragment + 4 per fragment`. Consumers that need exact numbers should
//! prefer `last_actual_input` — the most recent API-measured input token
//! count — over the estimate.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::fragment::Content;
use crate::machine::RunState;

/// Fraction (in percent) of the context window at which compaction-style
/// action becomes advisable; mirrors synergy's production soft threshold
/// of 0.85. Integer percent keeps the math exact for realistic limits.
const SOFT_THRESHOLD_PERCENT: u64 = 85;
/// Heuristic chars-per-token divisor for plain-text estimates.
const CHARS_PER_TOKEN: u64 = 4;
/// Fixed token estimate for one media (image/audio/video/document) fragment.
const MEDIA_FRAGMENT_TOKEN_ESTIMATE: u64 = 500;
/// Per-fragment framing overhead: role tags, separators, tool-call ids.
const PER_FRAGMENT_OVERHEAD: u64 = 4;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Budget {
    /// Context window of the active model; 0 when no model is active.
    pub context_limit: u64,
    /// Heuristic input-token estimate of the next completion request.
    pub estimated_input: u64,
    /// `context_limit × 0.85` — the soft compaction threshold.
    pub soft_threshold: u64,
    /// `context_limit − estimated_input`, saturating at zero.
    pub headroom: u64,
    /// Most recent API-measured input tokens, when a completion succeeded.
    pub last_actual_input: Option<u64>,
}

/// Digest of ledger state. Filled by the accelerator once the ledger tool
/// reports it — the machine itself performs no IO.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerDigest {
    pub total: u64,
    pub by_status: HashMap<String, u64>,
    pub current_entry: Option<LedgerDigestEntry>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerDigestEntry {
    pub id: String,
    pub title: String,
    pub status: String,
}

/// One ledger state migration, recorded into the trajectory envelope at
/// the step whose tool call caused it. Future per-node credit-assignment
/// anchor for offline policy learning.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerTransition {
    pub entry_id: String,
    pub from_status: String,
    pub to_status: String,
}

/// Extract ledger transitions from a step's effects by parsing tool-result
/// fragments emitted by the ledger tool (results whose JSON body carries
/// `"tool": "ledger"`). Kept here so both the accelerator loop and the
/// gRPC server share one parser; the machine itself does no IO.
pub fn ledger_transitions_in(effects: &[crate::record::Effect]) -> Vec<LedgerTransition> {
    let mut transitions = Vec::new();
    for effect in effects {
        match effect {
            crate::record::Effect::CompletionRecorded { inbox_items, .. } => {
                for item in inbox_items {
                    collect_from_tool_result(&item.fragment, &mut transitions);
                }
            }
            crate::record::Effect::InboxPushed { item } => {
                collect_from_tool_result(&item.fragment, &mut transitions);
            }
            _ => {}
        }
    }
    transitions
}

fn collect_from_tool_result(
    fragment: &crate::fragment::Fragment,
    sink: &mut Vec<LedgerTransition>,
) {
    let crate::fragment::Content::ToolResult(tool_result) = &fragment.content else {
        return;
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&tool_result.content) else {
        return;
    };
    if value.get("tool").and_then(|tool| tool.as_str()) != Some("ledger") {
        return;
    }
    let Some(entries) = value.get("transitions").and_then(|list| list.as_array()) else {
        return;
    };
    for entry in entries {
        let (Some(entry_id), Some(from_status), Some(to_status)) = (
            entry.get("id").and_then(|field| field.as_str()),
            entry.get("from").and_then(|field| field.as_str()),
            entry.get("to").and_then(|field| field.as_str()),
        ) else {
            continue;
        };
        sink.push(LedgerTransition {
            entry_id: entry_id.to_string(),
            from_status: from_status.to_string(),
            to_status: to_status.to_string(),
        });
    }
}

/// One resource-registry mutation caused by the `resources` tool, lifted
/// into the trajectory envelope. Self-evolution provenance: what the agent
/// changed about its own harness, and when.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryEvent {
    pub op: String,
    pub kind: String,
    pub name: String,
}

/// Extract registry events from a step's effects by parsing tool-result
/// fragments emitted by the resources tool (`"tool": "resources"` bodies).
/// Shared parser for the accelerator loop and the gRPC server; the machine
/// itself does no IO.
pub fn registry_events_in(effects: &[crate::record::Effect]) -> Vec<RegistryEvent> {
    let mut events = Vec::new();
    for effect in effects {
        match effect {
            crate::record::Effect::CompletionRecorded { inbox_items, .. } => {
                for item in inbox_items {
                    collect_registry_from_tool_result(&item.fragment, &mut events);
                }
            }
            crate::record::Effect::InboxPushed { item } => {
                collect_registry_from_tool_result(&item.fragment, &mut events);
            }
            _ => {}
        }
    }
    events
}

fn collect_registry_from_tool_result(
    fragment: &crate::fragment::Fragment,
    sink: &mut Vec<RegistryEvent>,
) {
    let crate::fragment::Content::ToolResult(tool_result) = &fragment.content else {
        return;
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&tool_result.content) else {
        return;
    };
    if value.get("tool").and_then(|tool| tool.as_str()) != Some("resources") {
        return;
    }
    let Some(entries) = value.get("events").and_then(|list| list.as_array()) else {
        return;
    };
    for entry in entries {
        let (Some(op), Some(kind), Some(name)) = (
            entry.get("op").and_then(|field| field.as_str()),
            entry.get("kind").and_then(|field| field.as_str()),
            entry.get("name").and_then(|field| field.as_str()),
        ) else {
            continue;
        };
        sink.push(RegistryEvent {
            op: op.to_string(),
            kind: kind.to_string(),
            name: name.to_string(),
        });
    }
}

/// Digest of the policy-declared overlay for this turn. Counts only —
/// projected content itself never enters observation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverlayStatus {
    pub declared: bool,
    pub system_prefix_count: u64,
    pub tail_count: u64,
}

/// Digest of the runtime resource registry. `None` until the agent (or the
/// harness) registers a resource — no phantom "empty registry" in every
/// observation. Enriched by the accelerator; the machine performs no IO.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceDigest {
    pub total: u64,
    pub by_kind: HashMap<String, u64>,
    /// Registered resource names, sorted for stable serialization.
    pub names: Vec<String>,
}

/// One row of the context directory — the policy/tool-facing read-only
/// view of a document cell. Metadata and a bounded preview, never content.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellDirEntry {
    pub id: u64,
    pub anchor: Option<String>,
    pub role: String,
    pub kind: String,
    pub tag: String,
    pub bytes: u64,
    pub created_step: u64,
    pub last_seen_step: u64,
    pub preview: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Obs {
    pub budget: Budget,
    pub ledger_digest: Option<LedgerDigest>,
    pub overlay_status: OverlayStatus,
    #[serde(default)]
    pub resources_digest: Option<ResourceDigest>,
    #[serde(default)]
    pub context_directory: Vec<CellDirEntry>,
}

/// Derive a fresh observation from the run state. Pure: no IO, no caching.
pub fn measure(run: &RunState) -> Obs {
    Obs {
        budget: measure_budget(run),
        // The ledger lives in accelerator tool state, not RunState; the
        // accelerator enriches this field after calling measure.
        ledger_digest: None,
        // The overlay is policy-declared and cannot be derived from state;
        // the caller fills this once the declaration is known.
        overlay_status: OverlayStatus::default(),
        // The resource registry is likewise accelerator-side state; the
        // fire loop enriches this digest after calling measure.
        resources_digest: None,
        // The directory is derived from the document itself.
        context_directory: directory_of(run),
    }
}

/// Directory rows for every cell, document order, preview capped.
fn directory_of(run: &RunState) -> Vec<CellDirEntry> {
    const PREVIEW_CHARS: usize = 80;
    run.context
        .fragments()
        .iter()
        .map(|cell| {
            let meta = run.context.meta(cell.id());
            let full_text = cell.content_as_text();
            let preview: String = full_text.chars().take(PREVIEW_CHARS).collect();
            CellDirEntry {
                id: cell.id(),
                anchor: cell.anchor.clone(),
                role: format!("{:?}", cell.role).to_lowercase(),
                kind: kind_of(cell).to_string(),
                tag: cell.tag.clone(),
                bytes: crate::context::Context::cell_bytes(cell),
                created_step: meta.created_step,
                last_seen_step: meta.last_seen_step,
                preview,
            }
        })
        .collect()
}

fn kind_of(cell: &crate::fragment::Fragment) -> &'static str {
    match &cell.content {
        crate::fragment::Content::Text(_) => "text",
        crate::fragment::Content::Image(_) => "image",
        crate::fragment::Content::Audio(_) => "audio",
        crate::fragment::Content::Video(_) => "video",
        crate::fragment::Content::Document(_) => "document",
        crate::fragment::Content::ToolCall(_) => "tool_call",
        crate::fragment::Content::ToolResult(_) => "tool_result",
        crate::fragment::Content::Hitch { .. } => "hitch",
    }
}

fn measure_budget(run: &RunState) -> Budget {
    let context_limit = run
        .resources
        .active_model()
        .and_then(|model| model.limit.as_ref())
        .map(|limit| limit.context)
        .unwrap_or(0);
    let estimated_input = estimate_input_tokens(run);
    Budget {
        context_limit,
        estimated_input,
        soft_threshold: context_limit.saturating_mul(SOFT_THRESHOLD_PERCENT) / 100,
        headroom: context_limit.saturating_sub(estimated_input),
        last_actual_input: last_measured_input(run),
    }
}

fn estimate_input_tokens(run: &RunState) -> u64 {
    let fragments = run.context.fragments();
    let mut text_chars: u64 = 0;
    let mut media_fragments: u64 = 0;
    for fragment in fragments {
        match &fragment.content {
            Content::Text(text) => text_chars = text_chars.saturating_add(text.text.len() as u64),
            Content::Hitch { message, .. } => {
                text_chars = text_chars.saturating_add(message.len() as u64);
            }
            Content::ToolResult(result) => {
                text_chars = text_chars.saturating_add(result.content.len() as u64);
            }
            Content::ToolCall(tool_call) => {
                let arguments_len = serde_json::to_string(&tool_call.arguments)
                    .map(|encoded| encoded.len() as u64)
                    .unwrap_or(0);
                text_chars = text_chars
                    .saturating_add(tool_call.name.len() as u64)
                    .saturating_add(arguments_len);
            }
            Content::Image(_) | Content::Audio(_) | Content::Video(_) | Content::Document(_) => {
                media_fragments = media_fragments.saturating_add(1);
            }
        }
    }
    let tool_manifest_chars =
        run.resources
            .active_tool_definitions()
            .iter()
            .fold(0u64, |accumulator, definition| {
                let parameters_len = serde_json::to_string(&definition.parameters)
                    .map(|encoded| encoded.len() as u64)
                    .unwrap_or(0);
                accumulator
                    .saturating_add(definition.name.len() as u64)
                    .saturating_add(definition.description.len() as u64)
                    .saturating_add(parameters_len)
            });
    text_chars
        .saturating_add(tool_manifest_chars)
        .saturating_div(CHARS_PER_TOKEN)
        .saturating_add(media_fragments.saturating_mul(MEDIA_FRAGMENT_TOKEN_ESTIMATE))
        .saturating_add((fragments.len() as u64).saturating_mul(PER_FRAGMENT_OVERHEAD))
}

fn last_measured_input(run: &RunState) -> Option<u64> {
    // Failed completions are recorded with empty usage; a zero reading is
    // not a measurement, so walk back to the newest record carrying one.
    run.telemetry
        .completions
        .iter()
        .rev()
        .map(|record| record.tokens.input_tokens)
        .find(|input_tokens| *input_tokens > 0)
}
