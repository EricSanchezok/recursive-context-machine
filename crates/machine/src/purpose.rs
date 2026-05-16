use serde::{Deserialize, Serialize};

/// The steering intention — what the agent should do.
///
/// Set by the creator of a [`Machine`](crate::Machine) and passed to
/// [`Policy::decide`](crate::Policy::decide) on every step.
/// Purpose is **read-only** — the machine never modifies it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Purpose {
    pub text: String,
}

impl Purpose {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl<T: Into<String>> From<T> for Purpose {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
