use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::fragment::Fragment;

/// Header-slot ordering for named anchors: stable, cache-friendly layout.
/// Custom anchors not listed here land after the listed region, before
/// unanchored cells.
pub const SLOT_ORDER: [&str; 6] = [
    "@agent",
    "@env",
    "@purpose",
    "@plan",
    "@summary",
    "@reflection",
];

/// Anchors that structural edits refuse to delete — the scaffolding.
pub const PROTECTED_ANCHORS: [&str; 3] = ["@agent", "@env", "@purpose"];

/// Per-cell bookkeeping for the context directory. Derived/observed data,
/// never the source of truth for content.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellMeta {
    pub created_step: u64,
    pub last_seen_step: u64,
    /// Completion id whose request last included this cell, when known.
    pub source_completion: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    cells: Vec<Fragment>,
    next_id: u64,
    #[serde(default)]
    metas: HashMap<u64, CellMeta>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextIdNotFound(pub u64);

impl std::fmt::Display for ContextIdNotFound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "fragment id {} not found in context", self.0)
    }
}

impl std::error::Error for ContextIdNotFound {}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            next_id: 1,
            metas: HashMap::new(),
        }
    }

    pub fn append(&mut self, fragment: Fragment) -> u64 {
        let id = self.next_id;
        self.append_with_id(id, fragment);
        id
    }

    pub fn append_with_id(&mut self, id: u64, mut fragment: Fragment) {
        self.assign_specific_id(id, &mut fragment);
        self.cells.push(fragment);
    }

    pub fn insert(&mut self, after: u64, fragment: Fragment) -> Result<u64, ContextIdNotFound> {
        let new_id = self.next_id;
        self.insert_with_id(after, new_id, fragment)?;
        Ok(new_id)
    }

    pub fn insert_with_id(
        &mut self,
        after: u64,
        id: u64,
        mut fragment: Fragment,
    ) -> Result<(), ContextIdNotFound> {
        let position = self.position_of(after).ok_or(ContextIdNotFound(after))?;
        self.assign_specific_id(id, &mut fragment);
        self.cells.insert(position + 1, fragment);
        Ok(())
    }

    pub fn replace(&mut self, id: u64, mut fragment: Fragment) -> Result<(), ContextIdNotFound> {
        let position = self.position_of(id).ok_or(ContextIdNotFound(id))?;
        fragment.id = id;
        // A replace that drops a named anchor frees the name; keep the meta
        // (created_step history) intact — content changed, identity persists.
        self.cells[position] = fragment;
        self.next_id = self.next_id.max(id + 1);
        Ok(())
    }

    pub fn remove(&mut self, id: u64) -> Result<(), ContextIdNotFound> {
        let position = self.position_of(id).ok_or(ContextIdNotFound(id))?;
        self.cells.remove(position);
        self.metas.remove(&id);
        Ok(())
    }

    pub fn swap(&mut self, first_id: u64, second_id: u64) -> Result<(), ContextIdNotFound> {
        let first_position = self
            .position_of(first_id)
            .ok_or(ContextIdNotFound(first_id))?;
        let second_position = self
            .position_of(second_id)
            .ok_or(ContextIdNotFound(second_id))?;
        self.cells.swap(first_position, second_position);
        Ok(())
    }

    /// Move one cell to sit immediately after another. No-op ordering
    /// (moving after itself) is accepted to keep op application uniform.
    pub fn move_after(&mut self, id: u64, after: u64) -> Result<(), ContextIdNotFound> {
        let position = self.position_of(id).ok_or(ContextIdNotFound(id))?;
        let after_position = self.position_of(after).ok_or(ContextIdNotFound(after))?;
        if position == after_position || position == after_position + 1 {
            return Ok(());
        }
        let fragment = self.cells.remove(position);
        // Recompute: removal may have shifted the after position.
        let after_position = self
            .position_of(after)
            .expect("position existed before removal");
        self.cells.insert(after_position + 1, fragment);
        Ok(())
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.metas.clear();
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn fragments(&self) -> &[Fragment] {
        &self.cells
    }

    pub fn position_of(&self, id: u64) -> Option<usize> {
        self.cells.iter().position(|cell| cell.id == id)
    }

    pub fn get(&self, id: u64) -> Option<&Fragment> {
        self.position_of(id).map(|index| &self.cells[index])
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Fragment> {
        self.position_of(id).map(|index| &mut self.cells[index])
    }

    pub fn next_id(&self) -> u64 {
        self.next_id
    }

    /// Resolve a named anchor to its cell id.
    pub fn find_anchor(&self, anchor: &str) -> Option<u64> {
        self.cells
            .iter()
            .find(|cell| cell.anchor.as_deref() == Some(anchor))
            .map(Fragment::id)
    }

    /// Idempotent named-slot write: replace in place when the anchor exists,
    /// otherwise insert at the slot's declared position (SLOT_ORDER keeps the
    /// header stable; unknown anchors land after the anchored region, before
    /// unanchored cells). Returns the cell id.
    pub fn set_named(&mut self, anchor: &str, fragment: Fragment) -> u64 {
        if let Some(id) = self.find_anchor(anchor) {
            let mut fragment = fragment;
            fragment.anchor = Some(anchor.to_string());
            let _ = self.replace(id, fragment);
            return id;
        }
        let id = self.next_id;
        let mut fragment = fragment;
        fragment.anchor = Some(anchor.to_string());
        let insertion = self.slot_position(anchor);
        self.assign_specific_id(id, &mut fragment);
        self.cells.insert(insertion, fragment);
        id
    }

    /// Document-order position for a new named slot: after the last existing
    /// cell whose anchor ranks at-or-before it in SLOT_ORDER, before any
    /// cell that ranks later or is unanchored.
    fn slot_position(&self, anchor: &str) -> usize {
        let target_rank = Self::anchor_rank(anchor);
        let mut insertion = 0;
        for (index, cell) in self.cells.iter().enumerate() {
            match &cell.anchor {
                Some(existing) => {
                    let rank = Self::anchor_rank(existing);
                    if rank <= target_rank {
                        insertion = index + 1;
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        insertion
    }

    fn anchor_rank(anchor: &str) -> usize {
        SLOT_ORDER
            .iter()
            .position(|slot| *slot == anchor)
            .map(|rank| rank + 1)
            .unwrap_or(SLOT_ORDER.len() + 1)
    }

    /// Record creation metadata for a cell (idempiently fills defaults).
    pub fn note_created(&mut self, id: u64, step: u64, source_completion: Option<u64>) {
        let entry = self.metas.entry(id).or_default();
        if entry.created_step == 0 {
            entry.created_step = step;
        }
        if source_completion.is_some() {
            entry.source_completion = source_completion;
        }
    }

    /// Mark every cell that entered a request as seen at this step.
    /// Called at request assembly; never recorded as WAL effects.
    pub fn note_seen(&mut self, ids: &[u64], step: u64, completion: Option<u64>) {
        for id in ids {
            let entry = self.metas.entry(*id).or_default();
            entry.last_seen_step = entry.last_seen_step.max(step);
            if completion.is_some() {
                entry.source_completion = completion;
            }
        }
    }

    pub fn meta(&self, id: u64) -> CellMeta {
        self.metas.get(&id).copied().unwrap_or_default()
    }

    /// Approximate byte size of a cell's content for directory accounting.
    pub fn cell_bytes(fragment: &Fragment) -> u64 {
        match &fragment.content {
            crate::fragment::Content::Text(text) => text.text.len() as u64,
            crate::fragment::Content::ToolCall(call) => {
                call.name.len() as u64
                    + call.arguments.to_string().len() as u64
                    + call.id.len() as u64
            }
            crate::fragment::Content::ToolResult(result) => result.content.len() as u64,
            crate::fragment::Content::Hitch { message, .. } => message.len() as u64,
            crate::fragment::Content::Image(_) | crate::fragment::Content::Audio(_) => 1_000,
            crate::fragment::Content::Video(_) | crate::fragment::Content::Document(_) => 4_000,
        }
    }

    fn assign_specific_id(&mut self, id: u64, fragment: &mut Fragment) {
        fragment.id = id;
        self.next_id = self.next_id.max(id + 1);
    }
}
