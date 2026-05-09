//! RICA tool pool — dynamic tool linking for RICA cells.
//!
//! Tools live in a ToolPool. Cells link to tools at creation time and release
//! them when done. No global registry, no agent-vs-tool distinction.

pub mod pool;
pub mod tool_def;
