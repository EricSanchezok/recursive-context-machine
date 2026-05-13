use crate::fragment::Fragment;

/// The machine's context — an ordered sequence of fragments plus selected resources.
///
/// Each fragment is assigned a unique `id` when stored. The context also
/// tracks which model and tools the Policy has selected for the next
/// Reactor invocation.
pub struct Context {
    cells: Vec<Fragment>,
    model: Option<String>,
    tools: Vec<String>,
    next_id: u64,
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
            model: None,
            tools: Vec::new(),
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
    /// Panics if `id` is not found in the context.
    pub fn insert(&mut self, id: u64, mut fragment: Fragment) -> u64 {
        let pos = self
            .find(id)
            .unwrap_or_else(|| panic!("id {} not found in context", id));
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

    /// Swap the positions of two fragments by id.
    ///
    /// # Panics
    ///
    /// Panics if either id is not found in the context.
    pub fn swap(&mut self, id1: u64, id2: u64) {
        let i = self
            .find(id1)
            .unwrap_or_else(|| panic!("id {} not found in context", id1));
        let j = self
            .find(id2)
            .unwrap_or_else(|| panic!("id {} not found in context", id2));
        self.cells.swap(i, j);
    }

    /// Clear all fragments without resetting id allocation.
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Set the selected model name.
    pub fn set_model(&mut self, name: impl Into<String>) {
        self.model = Some(name.into());
    }

    /// Add a tool name if not already present.
    pub fn add_tool(&mut self, name: impl Into<String>) {
        let name = name.into();
        if !self.tools.contains(&name) {
            self.tools.push(name);
        }
    }

    /// Remove a tool name.
    pub fn remove_tool(&mut self, name: impl AsRef<str>) {
        self.tools.retain(|t| t != name.as_ref());
    }

    // ── Query ──

    /// Selected model name, if any.
    pub fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }

    /// Selected tool names.
    pub fn tools(&self) -> &[String] {
        &self.tools
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

    /// Find the position of a fragment by id.
    pub fn find(&self, id: u64) -> Option<usize> {
        self.cells.iter().position(|f| f.id == id)
    }

    /// Get a fragment by id.
    pub fn get(&self, id: u64) -> Option<&Fragment> {
        self.find(id).map(|i| &self.cells[i])
    }

    /// Get a mutable reference to a fragment by id.
    pub fn get_mut(&mut self, id: u64) -> Option<&mut Fragment> {
        self.find(id).map(|i| &mut self.cells[i])
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
