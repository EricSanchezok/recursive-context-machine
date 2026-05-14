//! CM — Context Machine
//!
//! A context machine is a triple ℳ = (ℂ, ℰ, Φ)
//! where Φ(c, e) = (c′, e′) with c′ = π(c, e, ρ) and e′ = ω(c, e, τ, m).
//!
//! This crate provides the core primitives:
//!
//! | Component | Math | Role |
//! |-----------|------|------|
//! | [`Fragment`] | ℱ | Tape symbol |
//! | [`Context`] | ℂ | Tape |
//! | [`Environment`] | ℰ | External world |
//! | [`Resources`] | ρ | Available pool |
//! | [`Inbox`] | ℐ | Pending queue |
//! | [`Policy`] | π | Context engineering |
//! | [`reactor`] | ω | Environment transition |
//! | [`Machine`] | ℳ | Composition of π and ω |

pub mod completion;
pub mod context;
pub mod env;
pub mod fragment;
pub mod inbox;
pub mod machine;
pub mod model;
pub mod policy;
pub mod reactor;
pub mod resources;
pub mod tool;

pub use context::Context;
pub use env::Environment;
pub use fragment::{
    Audio, Content, DataSource, Document, Fragment, Image, Role, Text, ToolCall, Video,
};
pub use inbox::Inbox;
pub use machine::Machine;
pub use model::{Cost, Limit, Modalities, Modality, Model, Protocol};
pub use policy::{Action, Policy};
pub use resources::Resources;
pub use tool::{Tool, ToolResult};
