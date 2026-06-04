use serde::{Deserialize, Serialize};

use crate::fragment::Fragment;
use crate::policy::Action;
use crate::usage::Usage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyResult {
    pub done: bool,
    pub event: MachineEvent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MachineEvent {
    pub step: u64,
    pub action: Action,
    pub outcome: ActionOutcome,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionOutcome {
    StateOnly,
    Reactor {
        fragments: Vec<Fragment>,
        usage: Usage,
    },
}

impl MachineEvent {
    pub fn state_only(step: u64, action: Action) -> Self {
        Self {
            step,
            action,
            outcome: ActionOutcome::StateOnly,
        }
    }

    pub fn reactor(step: u64, fragments: Vec<Fragment>, usage: Usage) -> Self {
        Self {
            step,
            action: Action::Halt,
            outcome: ActionOutcome::Reactor { fragments, usage },
        }
    }
}
