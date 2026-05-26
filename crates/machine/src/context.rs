use crate::fragment::Fragment;

/// The machine's context — an ordered sequence of fragments.
///
/// Each fragment is assigned a unique `id` when stored.
#[derive(Clone, Debug)]
pub struct Context {
    cells: Vec<Fragment>,
    next_id: u64,
}

/// Returned by mutating Context methods when the targeted id does not exist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextIdNotFound(pub u64);

impl std::fmt::Display for ContextIdNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fragment id {} not found in context", self.0)
    }
}

impl std::error::Error for ContextIdNotFound {}

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

    /// Insert a fragment after the cell with the given id, assigning a fresh
    /// id to the new fragment.
    ///
    /// Returns the assigned id, or [`ContextIdNotFound`] if `after` is stale.
    pub fn insert(&mut self, after: u64, mut fragment: Fragment) -> Result<u64, ContextIdNotFound> {
        let pos = self.position_of(after).ok_or(ContextIdNotFound(after))?;
        let new_id = self.assign_id(&mut fragment);
        self.cells.insert(pos + 1, fragment);
        Ok(new_id)
    }

    /// Replace the fragment at the given id, preserving the id. Returns
    /// [`ContextIdNotFound`] if the id is stale.
    pub fn replace(&mut self, id: u64, mut fragment: Fragment) -> Result<(), ContextIdNotFound> {
        let pos = self.position_of(id).ok_or(ContextIdNotFound(id))?;
        fragment.id = id;
        self.cells[pos] = fragment;
        Ok(())
    }

    /// Remove the fragment with the given id. Returns [`ContextIdNotFound`]
    /// if the id is stale.
    pub fn remove(&mut self, id: u64) -> Result<(), ContextIdNotFound> {
        let pos = self.position_of(id).ok_or(ContextIdNotFound(id))?;
        self.cells.remove(pos);
        Ok(())
    }

    /// Swap the positions of two fragments by id. Returns
    /// [`ContextIdNotFound`] if either id is stale; on error the context is
    /// unchanged.
    pub fn swap(&mut self, id1: u64, id2: u64) -> Result<(), ContextIdNotFound> {
        let i = self.position_of(id1).ok_or(ContextIdNotFound(id1))?;
        let j = self.position_of(id2).ok_or(ContextIdNotFound(id2))?;
        self.cells.swap(i, j);
        Ok(())
    }

    /// Clear all fragments. Id allocation continues (avoids collisions with external references).
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// All fragments in insertion order.
    pub fn fragments(&self) -> &[Fragment] {
        &self.cells
    }

    /// Find the index of a fragment by id.
    pub fn position_of(&self, id: u64) -> Option<usize> {
        self.cells.iter().position(|cell| cell.id == id)
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

    fn assign_id(&mut self, fragment: &mut Fragment) -> u64 {
        let id = self.next_id;
        fragment.id = id;
        self.next_id += 1;
        id
    }
}
