use std::collections::VecDeque;

use crate::fragment::Fragment;

/// Inbox — the pending queue between $\omega$ and $\pi$.
///
/// The Reactor produces fragments (LLM output, tool results) and places
/// them into the inbox. The Machine drains them into the context.
#[derive(Debug, Clone, Default)]
pub struct Inbox {
    fragments: VecDeque<Fragment>,
}

impl Inbox {
    pub fn new() -> Self {
        Self {
            fragments: VecDeque::new(),
        }
    }

    /// Push a fragment onto the queue.
    pub fn push(&mut self, f: Fragment) {
        self.fragments.push_back(f);
    }

    /// Pop the head of the queue.
    pub fn pop(&mut self) -> Option<Fragment> {
        self.fragments.pop_front()
    }

    /// Peek at the head without removing it.
    pub fn peek(&self) -> Option<&Fragment> {
        self.fragments.front()
    }

    /// Whether the inbox is empty.
    pub fn is_empty(&self) -> bool {
        self.fragments.is_empty()
    }

    /// Number of fragments in the inbox.
    pub fn len(&self) -> usize {
        self.fragments.len()
    }
}

impl IntoIterator for Inbox {
    type Item = Fragment;
    type IntoIter = std::collections::vec_deque::IntoIter<Fragment>;

    fn into_iter(self) -> Self::IntoIter {
        self.fragments.into_iter()
    }
}

impl FromIterator<Fragment> for Inbox {
    fn from_iter<I: IntoIterator<Item = Fragment>>(iter: I) -> Self {
        Self {
            fragments: VecDeque::from_iter(iter),
        }
    }
}
