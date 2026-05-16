//! Core primitives for the Context Machine.
//!
//! | Component | Role |
//! |-----------|------|
//! | [`Fragment`] | Tape symbol |
//! | [`Context`] | Tape |
//! | [`Purpose`] | Steering intention |
//! | [`Environment`] | External world |
//! | [`Resources`] | Available pool |
//! | [`Inbox`] | Pending queue |
//! | [`Policy`] | Context engineering |
//! | [`reactor`] | Environment transition |
//! | [`Machine`] | Policy + Reactor composition |

pub mod completion;
pub mod context;
pub mod env;
pub mod fragment;
pub mod inbox;
pub mod logging;
pub mod machine;
pub mod model;
pub mod policy;
pub mod purpose;
pub mod reactor;
pub mod resources;
pub mod tool;

pub use context::Context;
pub use env::Environment;
pub use fragment::{
    Audio, Content, DataSource, Document, Fragment, Image, Role, Text, ToolCall, ToolResult, Video,
};
pub use inbox::Inbox;
pub use machine::Machine;
pub use model::{Cost, Limit, Modalities, Modality, Model, Protocol};
pub use policy::{Action, Policy};
pub use purpose::Purpose;
pub use resources::Resources;
pub use tool::Tool;
