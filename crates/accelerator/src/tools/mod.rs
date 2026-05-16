//! Built-in tools for the accelerator.
//!
//! Each tool implements the [`Tool`] trait from the `machine` crate.
//! Tools are registered into [`Resources`] before the machine runs.

mod add;
mod find;

pub use add::AddTool;
pub use find::FindTool;

/// All built-in tools registered by default.
pub fn builtin_tools() -> Vec<std::sync::Arc<dyn machine::Tool>> {
    vec![std::sync::Arc::new(AddTool), std::sync::Arc::new(FindTool)]
}
