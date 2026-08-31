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

    /// Pop the first item whose ToolResult content matches `call_id`.
    /// Document-order FIFO among matches; items without a matching call id
    /// stay queued. `None` when nothing matches (or the inbox is empty).
    pub fn pop_by_call_id(&mut self, call_id: &str) -> Option<InboxItem> {
        let index = self.items.iter().position(|item| {
            matches!(
                &item.fragment.content,
                crate::fragment::Content::ToolResult(result) if result.call_id == call_id
            )
        })?;
        self.items.remove(index)
    }

    /// Peek (clone, no pop) the item a consume-op would take: the first
    /// ToolResult matching `call_id`, or the front item when `call_id` is
    /// None. Resolution reads the inbox without mutating it — the actual
    /// pop happens when `InboxConsumed` is applied.
    pub fn find_item(&self, call_id: Option<&str>) -> Option<InboxItem> {
        match call_id {
            Some(call_id) => self.items.iter().find(|item| {
                matches!(
                    &item.fragment.content,
                    crate::fragment::Content::ToolResult(result) if result.call_id == call_id
                )
            }),
            None => self.items.front(),
        }
        .cloned()
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
