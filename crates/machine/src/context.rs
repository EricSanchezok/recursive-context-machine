use crate::fragment::Fragment;

/// The machine's context — an ordered sequence of fragments.
///
/// Each fragment is assigned a unique `id` when stored.
pub struct Context {
    cells: Vec<Fragment>,
    next_id: u64,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            cells: self.cells.clone(),
            next_id: self.next_id,
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    /// Create an empty context. The first assigned id will be 1.
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            next_id: 1,
        }
    }

    /// Append a fragment to the end, assigning it a fresh id.
    ///
    /// Returns the assigned id.
    pub fn append(&mut self, mut fragment: Fragment) -> u64 {
        let id = self.assign_id(&mut fragment);
        self.cells.push(fragment);
        id
    }

    /// Insert a fragment after the cell with the given id.
    ///
    /// Returns the assigned id of the new fragment.
    ///
    /// # Panics
    ///
    /// Panics if `after` is not found in the context.
    pub fn insert(&mut self, after: u64, mut fragment: Fragment) -> u64 {
        let pos = self
            .position_of(after)
            .unwrap_or_else(|| panic!("id {} not found in context", after));
        let new_id = self.assign_id(&mut fragment);
        self.cells.insert(pos + 1, fragment);
        new_id
    }

    /// Replace the fragment at the given id, preserving the id.
    ///
    /// # Panics
    ///
    /// Panics if `id` is not found in the context.
    pub fn replace(&mut self, id: u64, mut fragment: Fragment) {
        let pos = self
            .position_of(id)
            .unwrap_or_else(|| panic!("id {} not found in context", id));
        fragment.id = id;
        self.cells[pos] = fragment;
    }

    /// Remove the fragment with the given id.
    ///
    /// # Panics
    ///
    /// Panics if `id` is not found in the context.
    pub fn remove(&mut self, id: u64) {
        let pos = self
            .position_of(id)
            .unwrap_or_else(|| panic!("id {} not found in context", id));
        self.cells.remove(pos);
    }

    /// Swap the positions of two fragments by id.
    ///
    /// # Panics
    ///
    /// Panics if either id is not found in the context.
    pub fn swap(&mut self, id1: u64, id2: u64) {
        let i = self
            .position_of(id1)
            .unwrap_or_else(|| panic!("id {} not found in context", id1));
        let j = self
            .position_of(id2)
            .unwrap_or_else(|| panic!("id {} not found in context", id2));
        self.cells.swap(i, j);
    }

    /// Clear all fragments without resetting id allocation.
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Number of fragments in the context.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Whether the context is empty.
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// All fragments in order.
    pub fn fragments(&self) -> &[Fragment] {
        &self.cells
    }

    /// Find the index of a fragment by id.
    pub fn position_of(&self, id: u64) -> Option<usize> {
        self.cells.iter().position(|f| f.id == id)
    }

    /// Get a fragment by id.
    pub fn get(&self, id: u64) -> Option<&Fragment> {
        self.position_of(id).map(|i| &self.cells[i])
    }

    /// Get a mutable reference to a fragment by id.
    pub fn get_mut(&mut self, id: u64) -> Option<&mut Fragment> {
        self.position_of(id).map(|i| &mut self.cells[i])
    }

    /// The next id that will be assigned.
    pub fn next_id(&self) -> u64 {
        self.next_id
    }

    // ── Private ──

    fn assign_id(&mut self, fragment: &mut Fragment) -> u64 {
        let id = self.next_id;
        fragment.id = id;
        self.next_id += 1;
        id
    }
}
