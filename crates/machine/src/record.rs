use serde::{Deserialize, Serialize};

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
    ContextAppended {
        id: u64,
        fragment: Fragment,
    },
    ContextInserted {
        id: u64,
        after: u64,
        fragment: Fragment,
    },
    ContextReplaced {
        id: u64,
        fragment: Fragment,
    },
    ContextRemoved {
        id: u64,
    },
    ContextSwapped {
        first: u64,
        second: u64,
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
    InboxTaken {
        source_completion: Option<crate::usage::CompletionId>,
        fragment_id: u64,
    },
    StatusChanged {
        status: crate::machine::MachineStatus,
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
