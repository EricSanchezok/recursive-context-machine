//! Policy-declared projection layer for completion requests.
//!
//! An [`Overlay`] never touches the tape: its contents exist only for the
//! duration of one Halt→request cycle and are re-derived by the policy on
//! every turn. `system_prefix` lands before the tape's messages (stable,
//! cache-friendly); `tail` lands after them (volatile, cache-busting).
//! Empty by default — existing policies see byte-identical requests.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Overlay {
    /// System content placed before the encoded context. Stable across
    /// turns for cache hits (synergy's early-system layers).
    pub system_prefix: Vec<String>,
    /// System content placed after the encoded context. Recomputed each
    /// turn by design (synergy's late-system layers).
    pub tail: Vec<String>,
}

impl Overlay {
    pub fn is_empty(&self) -> bool {
        self.system_prefix.is_empty() && self.tail.is_empty()
    }
}
