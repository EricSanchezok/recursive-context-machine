use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::fragment::Fragment;
use crate::inbox::Inbox;
use crate::machine::{MachineStatus, RunState};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Append(Fragment),
    Insert { after: u64, fragment: Fragment },
    Replace { id: u64, fragment: Fragment },
    Remove(u64),
    Swap(u64, u64),
    Model(String),
    Activate(String),
    Deactivate(String),
    Take,
    Halt,
    Done,
}

impl Action {
    pub fn name(&self) -> &'static str {
        match self {
            Action::Append(_) => "append",
            Action::Insert { .. } => "insert",
            Action::Replace { .. } => "replace",
            Action::Remove(_) => "remove",
            Action::Swap(..) => "swap",
            Action::Model(_) => "model",
            Action::Activate(_) => "activate",
            Action::Deactivate(_) => "deactivate",
            Action::Take => "take",
            Action::Halt => "halt",
            Action::Done => "done",
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(self, Action::Done)
    }

    pub fn verb(&self) -> &'static str {
        match self {
            Action::Append(_) => "Append",
            Action::Insert { .. } => "Insert",
            Action::Replace { .. } => "Replace",
            Action::Remove(_) => "Remove",
            Action::Swap(..) => "Swap",
            Action::Model(_) => "Model",
            Action::Activate(_) => "Activate",
            Action::Deactivate(_) => "Deactivate",
            Action::Take => "Take",
            Action::Halt => "Halt",
            Action::Done => "Done",
        }
    }
}

pub const ACTION_VERBS: &[&str] = &[
    "Append",
    "Insert",
    "Replace",
    "Remove",
    "Swap",
    "Model",
    "Activate",
    "Deactivate",
    "Take",
    "Halt",
    "Done",
];

pub struct PolicyView<'a> {
    pub run: &'a RunState,
    pub inbox: &'a Inbox,
    pub step: u64,
    pub status: MachineStatus,
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
}

impl Clone for Box<dyn Policy> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
