//! Completion assistant — the metered gateway tools use to generate content.
//!
//! Generative tools (context.compact, …) never call models directly: they
//! request one completion through this trait. The production implementation
//! (accelerator) wraps the active model with two hard guards:
//! a per-tool-call cap of [`MAX_COMPLETIONS_PER_TOOL_CALL`] and token
//! accounting that surfaces in the WAL. The machine crate defines the
//! boundary only — it performs no IO.

use std::future::Future;
use std::pin::Pin;

use crate::edit::Selector;
use crate::usage::TokenUsage;

/// Hard cap on embedded completions per single tool invocation. Compact
/// needs one; the cap leaves headroom for a retrieve-then-refine pattern
/// without letting a tool turn into a chat loop.
pub const MAX_COMPLETIONS_PER_TOOL_CALL: u32 = 2;

/// One generated-content request from a tool.
pub struct AssistantRequest {
    /// Instruction for the summarizer persona (what to produce).
    pub instruction: String,
    /// Cells to draw source material from; resolved against the live
    /// document snapshot, with the same semantics as a Delete selector.
    pub source: Selector,
}

/// Boxed completion future returned by [`CompletionAssistant::complete`].
pub type AssistantFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(String, TokenUsage), String>> + Send + 'a>>;

/// Metered gateway for tool-initiated completions.
pub trait CompletionAssistant: Send + Sync {
    /// Reset the per-tool-call completion meter. Tools call this at the
    /// start of `execute` so the cap in [`Self::complete`] bounds embedded
    /// completions per tool invocation, not per run.
    fn begin_tool_call(&self);

    /// Generate content for one tool request. Returns the text and the
    /// measured token usage (recorded into the step's WAL effects).
    fn complete(&self, request: AssistantRequest) -> AssistantFuture<'_>;
}
