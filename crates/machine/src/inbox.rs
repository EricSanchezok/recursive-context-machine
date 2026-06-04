use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::fragment::Fragment;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inbox {
    fragments: VecDeque<Fragment>,
}

impl Inbox {
    pub fn new() -> Self {
        Self {
            fragments: VecDeque::new(),
        }
    }

    pub fn push(&mut self, fragment: Fragment) {
        self.fragments.push_back(fragment);
    }

    pub fn extend(&mut self, fragments: impl IntoIterator<Item = Fragment>) {
        self.fragments.extend(fragments);
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

    pub fn fragments(&self) -> impl Iterator<Item = &Fragment> {
        self.fragments.iter()
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
    fn from_iter<Items: IntoIterator<Item = Fragment>>(items: Items) -> Self {
        Self {
            fragments: VecDeque::from_iter(items),
        }
    }
}
