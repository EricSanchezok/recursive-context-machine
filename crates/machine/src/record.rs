use serde::{Deserialize, Serialize};

use crate::edit::EditOp;
use crate::fragment::Fragment;
use crate::inbox::InboxItem;
use crate::policy::Action;
use crate::usage::CompletionRecord;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepResult {
    pub done: bool,
    pub event: StoredEvent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoredEvent {
    pub step: u64,
    pub action: Action,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Effect {
    ActionCounted {
        action: String,
    },
    /// Idempotent named-slot write (Set op): the resolved cell id, and the
    /// completion provenance when the content came from an inbox item.
    ContextSet {
        id: u64,
        anchor: String,
        fragment: Fragment,
        source_completion: Option<crate::usage::CompletionId>,
    },
    ContextInserted {
        id: u64,
        after: Option<u64>,
        fragment: Fragment,
        source_completion: Option<crate::usage::CompletionId>,
    },
    ContextReplaced {
        id: u64,
        fragment: Fragment,
    },
    ContextRemoved {
        id: u64,
    },
    ContextMoved {
        id: u64,
        after: u64,
    },
    ModelSelected {
        name: String,
    },
    ToolActivated {
        name: String,
    },
    ToolDeactivated {
        name: String,
    },
    InboxPushed {
        item: InboxItem,
    },
    CompletionRecorded {
        record: CompletionRecord,
        inbox_items: Vec<InboxItem>,
    },
    /// One inbox item consumed by an Edit op (Inbox content source).
    /// Replay pops by the same rule (call_id match, else FIFO).
    InboxConsumed {
        call_id: Option<String>,
        item: InboxItem,
    },
    StatusChanged {
        status: crate::machine::MachineStatus,
    },
    /// A tool invoked via `Action::Tool` completed; token spend attributed.
    ToolCompleted {
        name: String,
        call_id: String,
        tokens: Option<crate::usage::TokenUsage>,
    },
    /// Edits applied through the drain channel (tool-returned edit
    /// payloads), replayed as nested effects without a step increment.
    DrainEdits {
        ops: Vec<EditOp>,
        effects: Vec<Effect>,
    },
}

impl StoredEvent {
    pub fn new(step: u64, action: Action, effects: Vec<Effect>) -> Self {
        Self {
            step,
            action,
            effects,
        }
    }
}
