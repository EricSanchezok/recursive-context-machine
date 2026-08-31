use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::edit::EditOp;
use crate::inbox::Inbox;
use crate::machine::{MachineStatus, RunState};
use crate::obs::Obs;
use crate::overlay::Overlay;

/// Document-model action space (v2.1): one structured edit verb, one tool
/// verb, resource shaping, and pacing. Generative/retrieval capability lives
/// in the tool registry, not in verbs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Structural document edit; one action commits a batch of ops.
    Edit {
        ops: Vec<EditOp>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        because: Option<String>,
    },
    /// Invoke a registered tool from the policy side; the result lands in
    /// the inbox as a ToolResult fragment.
    Tool {
        name: String,
        args: Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        because: Option<String>,
    },
    Model(String),
    Activate(String),
    Deactivate(String),
    Halt,
    Done,
}

impl Action {
    pub fn name(&self) -> &'static str {
        match self {
            Action::Edit { .. } => "edit",
            Action::Tool { .. } => "tool",
            Action::Model(_) => "model",
            Action::Activate(_) => "activate",
            Action::Deactivate(_) => "deactivate",
            Action::Halt => "halt",
            Action::Done => "done",
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(self, Action::Done)
    }

    pub fn verb(&self) -> &'static str {
        match self {
            Action::Edit { .. } => "Edit",
            Action::Tool { .. } => "Tool",
            Action::Model(_) => "Model",
            Action::Activate(_) => "Activate",
            Action::Deactivate(_) => "Deactivate",
            Action::Halt => "Halt",
            Action::Done => "Done",
        }
    }
}

pub const ACTION_VERBS: &[&str] = &[
    "Edit",
    "Tool",
    "Model",
    "Activate",
    "Deactivate",
    "Halt",
    "Done",
];

pub struct PolicyView<'a> {
    pub run: &'a RunState,
    pub inbox: &'a Inbox,
    pub step: u64,
    pub status: MachineStatus,
    pub obs: &'a Obs,
}

pub trait Policy: Send + Sync {
    fn clone_box(&self) -> Box<dyn Policy>;

    fn name(&self) -> &str {
        "policy"
    }

    fn decide<'a>(
        &'a self,
        view: PolicyView<'a>,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>>;

    /// Projection declared for the next completion request. Consumed only
    /// at Halt; never materialized on the tape; re-derived every turn.
    /// Empty by default — policies that do not opt in produce requests
    /// byte-identical to pre-overlay behavior.
    fn overlay(&self, _view: &PolicyView<'_>) -> Overlay {
        Overlay::default()
    }
}

impl Clone for Box<dyn Policy> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
