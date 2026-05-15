use std::collections::VecDeque;

use crate::fragment::Fragment;

/// Inbox — the pending queue between reactor and policy.
///
/// The reactor produces fragments and places them into the inbox.
/// The machine drains them into the context.
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

    pub fn push(&mut self, f: Fragment) {
        self.fragments.push_back(f);
    }

    pub fn pop(&mut self) -> Option<Fragment> {
        self.fragments.pop_front()
    }

    pub fn peek(&self) -> Option<&Fragment> {
        self.fragments.front()
    }

    pub fn is_empty(&self) -> bool {
        self.fragments.is_empty()
    }

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
