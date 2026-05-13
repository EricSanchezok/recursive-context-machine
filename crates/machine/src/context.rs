use crate::fragment::Fragment;

/// The machine's context — an ordered sequence of fragments.
///
/// Each fragment is assigned a unique `id` when stored. The context
/// is pure data: it has no position, no movement. Operations are
/// id-based, not position-based.
pub struct Context {
    cells: Vec<Fragment>,
    next_id: u64,
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
        let id = self.next_id;
        fragment.id = id;
        self.next_id += 1;
        self.cells.push(fragment);
        id
    }

    /// Insert a fragment after the cell with the given id.
    ///
    /// Returns the assigned id of the new fragment.
    ///
    /// # Panics
    ///
    /// Panics if `id` is not found in the context.
    pub fn insert(&mut self, id: u64, mut fragment: Fragment) -> u64 {
        let pos = self
            .find(id)
            .unwrap_or_else(|| panic!("id {} not found in context", id));
        let new_id = self.next_id;
        fragment.id = new_id;
        self.next_id += 1;
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
            .find(id)
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
            .find(id)
            .unwrap_or_else(|| panic!("id {} not found in context", id));
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

    /// Get a mutable reference to a fragment by id.
    pub fn get_mut(&mut self, id: u64) -> Option<&mut Fragment> {
        self.cells.iter_mut().find(|f| f.id == id)
    }

    // ── Query ──

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

    /// The next id that will be assigned.
    pub fn next_id(&self) -> u64 {
        self.next_id
    }
}
