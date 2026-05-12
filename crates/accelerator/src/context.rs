use crate::fragment::Fragment;

/// The machine's context — an ordered sequence of fragments.
///
/// Each fragment is assigned a unique `id` when stored.
/// The context is pure data: it has no position, no movement.
/// Position and movement belong to the machine (Head), not the context.
pub struct Context {
    cells: Vec<Fragment>,
    next_id: u64,
}

impl Context {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            next_id: 1,
        }
    }

    /// Append a fragment to the end, assigning it a fresh id.
    pub fn append(&mut self, mut fragment: Fragment) {
        fragment.id = self.next_id;
        self.next_id += 1;
        self.cells.push(fragment);
    }

    /// Insert a fragment after the cell with the given id.
    pub fn insert(&mut self, id: u64, mut fragment: Fragment) {
        let pos = self
            .find(id)
            .unwrap_or_else(|| panic!("id {} not found", id));
        fragment.id = self.next_id;
        self.next_id += 1;
        self.cells.insert(pos + 1, fragment);
    }

    /// Replace the fragment at the given id, preserving the id.
    pub fn replace(&mut self, id: u64, mut fragment: Fragment) {
        let pos = self
            .find(id)
            .unwrap_or_else(|| panic!("id {} not found", id));
        fragment.id = id;
        self.cells[pos] = fragment;
    }

    /// Remove the fragment with the given id.
    pub fn remove(&mut self, id: u64) {
        let pos = self
            .find(id)
            .unwrap_or_else(|| panic!("id {} not found", id));
        self.cells.remove(pos);
    }

    /// Find the position of a fragment by id.
    pub fn find(&self, id: u64) -> Option<usize> {
        self.cells.iter().position(|f| f.id == id)
    }

    /// Get a fragment by id.
    pub fn get(&self, id: u64) -> Option<&Fragment> {
        self.cells.iter().find(|f| f.id == id)
    }

    // ── Query ──

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// All fragments in order.
    pub fn fragments(&self) -> &[Fragment] {
        &self.cells
    }
}
