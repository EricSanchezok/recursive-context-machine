//! Core primitives for the Recursive Context Machine.
//!
//! | Component | Role |
//! |-----------|------|
//! | [`Fragment`] | Tape symbol |
//! | [`Context`] | Tape |
//! | [`Purpose`] | Steering intention |
//! | [`Environment`] | External world |
//! | [`Resources`] | Available pool |
//! | [`Inbox`] | Pending queue |
//! | [`Policy`] | Owned by caller |
//! | [`Machine`] | Interpreter that turns actions into recorded effects |

pub mod completion;
pub mod context;
pub mod env;
pub mod event;
pub mod fragment;
pub mod hook;
pub mod inbox;
pub mod machine;
pub mod model;
pub mod obs;
pub mod overlay;
pub mod policy;
pub mod purpose;
pub(crate) mod reactor;
pub mod record;
pub mod resources;
pub mod tool;
pub mod usage;

pub use context::{Context, ContextIdNotFound};
pub use env::Environment;
pub use fragment::{
    Audio, Content, DataSource, Document, Fragment, Image, Role, Text, ToolCall, ToolResult, Video,
};
pub use inbox::{Inbox, InboxItem};
pub use machine::{ExecutionMode, Machine, MachineFrame, MachineState, MachineStatus, RunState};
pub use model::{Cost, DEFAULT_MODEL_TIMEOUT_SECS, Limit, Modalities, Modality, Model, Protocol};
pub use obs::{
    Budget, LedgerDigest, LedgerDigestEntry, LedgerTransition, Obs, OverlayStatus, RegistryEvent,
    ResourceDigest, ledger_transitions_in, registry_events_in,
};
pub use overlay::Overlay;
pub use policy::{ACTION_VERBS, Action, Policy, PolicyView};
pub use purpose::Purpose;
pub use record::{Effect, StepResult, StoredEvent};
pub use resources::{LookupResult, ModelNotRegistered, Resources, ToolNotRegistered};
pub use tool::{DEFAULT_TOOL_TIMEOUT_SECS, Tool, ToolDefinition, ToolRuntime};
pub use usage::{CompletionId, CompletionRecord, Telemetry, TokenUsage};
