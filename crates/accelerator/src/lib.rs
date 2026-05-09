//! RICA Core — Recursive Intelligence Creation Accelerator
//!
//! RICA is a Turing-machine model for LLM agents. Every Rica is a read/write head
//! operating on a tape of Fragments, driven by an Engine that decides what action
//! to take next.
//!
//! # Architecture
//!
//! ```text
//! Tape (Vec<Option<Fragment>>)  ←→  Head (DefaultRica)  ←→  Register (cwd, env, tools, state)
//!                                       │
//!                                  Engine decides:
//!                                  · CallLlm  → LLM response → append to tape
//!                                  · Prune    → erase tape cells
//!                                  · Compact  → summarize + replace
//!                                  · Halt     → stop
//! ```
//!
//! # Key types
//!
//! - [`Fragment`] — a symbol on the tape
//! - [`Tape`] — the tape itself (fixed-address cells)
//! - [`Register`] — the machine's state (cwd, env, tools, state)
//! - [`Tool`] — a machine part (can wrap a child Rica)
//! - [`Engine`] — the transition decision function
//! - [`Rica`] — the read/write head trait
//! - [`DefaultRica`] — the standard single-machine implementation
//! - [`Channel`] — communication pipe between Rica instances
//! - [`Trace`] — computation history (audit trail)
//!
//! # Composition
//!
//! - [`Pipeline`] — sequential stages
//! - [`Parallel`] — concurrent branches
//! - [`Ensemble`] — voting aggregation

pub mod channel;
pub mod compose;
pub mod engine;
pub mod fragment;
pub mod register;
pub mod rica;
pub mod tape;
pub mod tool;
pub mod trace;

// Re-export the public API
pub use channel::Channel;
pub use compose::{Ensemble, Parallel, Pipeline};
pub use engine::{Action, BudgetStage, Engine, HaltStage, LlmStage, PipelineEngine, PruneStage};
pub use fragment::{Content, Fragment, Role, ToolCallDef};
pub use register::Register;
pub use rica::{
    DefaultRica, HaltCondition, Intent, LlmBackend, LlmResponse, MaxCycles, Output, Rica,
    RigBackend,
};
pub use tape::Tape;
pub use tool::Tool;
pub use trace::{Cycle, TokenUsage, Trace};
