use serde::{Deserialize, Serialize};

use crate::fragment::Fragment;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    cells: Vec<Fragment>,
    next_id: u64,
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
        }
    }

    pub fn append(&mut self, mut fragment: Fragment) -> u64 {
        let id = self.assign_id(&mut fragment);
        self.cells.push(fragment);
        id
    }

    pub fn insert(&mut self, after: u64, mut fragment: Fragment) -> Result<u64, ContextIdNotFound> {
        let position = self.position_of(after).ok_or(ContextIdNotFound(after))?;
        let new_id = self.assign_id(&mut fragment);
        self.cells.insert(position + 1, fragment);
        Ok(new_id)
    }

    pub fn replace(&mut self, id: u64, mut fragment: Fragment) -> Result<(), ContextIdNotFound> {
        let position = self.position_of(id).ok_or(ContextIdNotFound(id))?;
        fragment.id = id;
        self.cells[position] = fragment;
        Ok(())
    }

    pub fn remove(&mut self, id: u64) -> Result<(), ContextIdNotFound> {
        let position = self.position_of(id).ok_or(ContextIdNotFound(id))?;
        self.cells.remove(position);
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

    pub fn clear(&mut self) {
        self.cells.clear();
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

    fn assign_id(&mut self, fragment: &mut Fragment) -> u64 {
        let id = self.next_id;
        fragment.id = id;
        self.next_id += 1;
        id
    }
}
