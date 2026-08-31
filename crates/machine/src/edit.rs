//! Document-edit action space (v2.1 design, `context-document-model.md`).
//!
//! [`EditOp`] is the single structured verb over the context document; its
//! content sources are deliberately minimal (`literal` or `inbox`): every
//! generative or retrieval capability lives in the tool registry, and
//! policy-emitted tool calls are the [`crate::Action::Tool`] verb. Effects
//! carry resolved numeric ids so replay never re-resolves selectors.

use serde::{Deserialize, Serialize};

use crate::fragment::{Content, Fragment, Role, Text};

/// One structural edit operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EditOp {
    /// Replace or create the named slot (idempotent).
    Set {
        anchor: String,
        content: ContentSpec,
    },
    /// Insert new content at a position; anchors the cell when given.
    Insert {
        position: Position,
        content: ContentSpec,
        anchor: Option<String>,
    },
    /// Remove every cell the selector matches (empty match = no-op).
    Delete { selector: Selector },
    /// Move one cell to sit immediately after another.
    Move { anchor: String, after: Position },
}

/// Where an insert lands.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Position {
    Anchor(String),
    Id(u64),
    End,
}

/// What a delete targets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Selector {
    Anchor(String),
    /// One cell by its numeric id (single-cell structural delete).
    Id(u64),
    Range {
        from: Position,
        to: Position,
    },
    Where(CellPredicate),
}

/// Structural predicate over directory rows; fields AND together.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CellPredicate {
    pub role: Option<String>,
    pub tag: Option<String>,
    pub kind: Option<String>,
    /// Drop the newest `k` matching cells from the match set.
    pub skip_newest: Option<u64>,
    pub bytes_gt: Option<u64>,
}

/// Content source for set/insert ops.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContentSpec {
    Literal {
        text: String,
        role: Role,
        tag: Option<String>,
    },
    /// Consume one inbox item; `None` pops the oldest (FIFO).
    Inbox { call_id: Option<String> },
}

impl ContentSpec {
    /// Build the fragment this spec resolves to, given the inbox item it
    /// may consume. `None` inbox + `Inbox` spec is a caller-side error
    /// handled by the machine (hitch), not here.
    pub fn resolve(&self, inbox_item: Option<Fragment>) -> Option<Fragment> {
        match self {
            ContentSpec::Literal { text, role, tag } => {
                let fragment = Fragment {
                    id: 0,
                    role: *role,
                    tag: tag.clone().unwrap_or_else(|| "literal".into()),
                    anchor: None,
                    content: Content::Text(Text { text: text.clone() }),
                };
                Some(fragment)
            }
            ContentSpec::Inbox { .. } => inbox_item,
        }
    }
}

impl EditOp {
    /// Stable op label for hitches and audit strings.
    pub fn label(&self) -> &'static str {
        match self {
            EditOp::Set { .. } => "set",
            EditOp::Insert { .. } => "insert",
            EditOp::Delete { .. } => "delete",
            EditOp::Move { .. } => "move",
        }
    }
}
