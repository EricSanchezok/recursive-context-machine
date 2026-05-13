//! CM — Context Machine
//!
//! A context machine is a triple $\mathcal{M} = (\mathcal{C}, \mathcal{E}, \Phi)$
//! where $\Phi(c, e) = (c', e')$ with $c' = \pi(c, e)$ and $e' = \omega(c', e)$.
//!
//! This crate provides the core primitives:
//!
//! | Component | Math | Role |
//! |-----------|------|------|
//! | [`Fragment`] | $\mathcal{F}$ | Tape symbol |
//! | [`Context`] | $\mathcal{C}$ | Tape |
//! | [`Environment`] | $\mathcal{E}$ | External world |
//! | [`Inbox`] | $p$ | Pending queue |
//! | [`Policy`] | $\pi$ | Context engineering |
//! | [`Reactor`] | $\omega$ | Environment transition |
//! | [`Machine`] | $\mathcal{M}$ | Composition of $\pi$ and $\omega$ |

pub mod context;
pub mod env;
pub mod fragment;
pub mod inbox;
pub mod machine;
pub mod policy;
pub mod reactor;

pub use context::Context;
pub use env::{Config, Environment};
pub use fragment::{
    Audio, Content, DataSource, Document, Fragment, Image, Role, Text, ToolCall, ToolDef,
    ToolResult, Video,
};
pub use inbox::Inbox;
pub use machine::Machine;
pub use policy::{Action, Policy};
pub use reactor::Reactor;
