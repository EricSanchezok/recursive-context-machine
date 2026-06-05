use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::fragment::Fragment;
use crate::usage::CompletionId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InboxItem {
    pub fragment: Fragment,
    pub source_completion: Option<CompletionId>,
}

impl InboxItem {
    pub fn new(fragment: Fragment, source_completion: Option<CompletionId>) -> Self {
        Self {
            fragment,
            source_completion,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inbox {
    items: VecDeque<InboxItem>,
}

impl Inbox {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn push(&mut self, fragment: Fragment) {
        self.push_item(InboxItem::new(fragment, None));
    }

    pub fn push_item(&mut self, item: InboxItem) {
        self.items.push_back(item);
    }

    pub fn extend_items(&mut self, items: impl IntoIterator<Item = InboxItem>) {
        self.items.extend(items);
    }

    pub fn pop(&mut self) -> Option<InboxItem> {
        self.items.pop_front()
    }

    pub fn peek(&self) -> Option<&InboxItem> {
        self.items.front()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn items(&self) -> impl Iterator<Item = &InboxItem> {
        self.items.iter()
    }
}

impl IntoIterator for Inbox {
    type Item = InboxItem;
    type IntoIter = std::collections::vec_deque::IntoIter<InboxItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl FromIterator<InboxItem> for Inbox {
    fn from_iter<Items: IntoIterator<Item = InboxItem>>(items: Items) -> Self {
        Self {
            items: VecDeque::from_iter(items),
        }
    }
}
