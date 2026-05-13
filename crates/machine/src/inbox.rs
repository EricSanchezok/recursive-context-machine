use crate::fragment::Fragment;

/// Inbox — the pending queue between $\omega$ and $\pi$.
///
/// The Reactor produces fragments (LLM output, tool results) and places
/// them into the inbox. The Policy consumes them one at a time, deciding
/// where to place each fragment in the context.
///
/// Fragments must be consumed in FIFO order — only the head is accessible.
#[derive(Debug, Clone)]
pub struct Inbox {
    fragments: Vec<Fragment>,
}

impl Inbox {
    pub fn new() -> Self {
        Self {
            fragments: Vec::new(),
        }
    }

    /// Push a fragment onto the queue.
    pub fn push(&mut self, f: Fragment) {
        self.fragments.push(f);
    }

    /// Pop the head of the queue.
    pub fn pop(&mut self) -> Option<Fragment> {
        if self.fragments.is_empty() {
            None
        } else {
            Some(self.fragments.remove(0))
        }
    }

    /// Peek at the head without removing it.
    pub fn peek(&self) -> Option<&Fragment> {
        self.fragments.first()
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
