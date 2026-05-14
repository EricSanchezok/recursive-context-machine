//! Built-in tools for the accelerator.
//!
//! Each tool implements the [`Tool`] trait from the `machine` crate.
//! Tools are registered into [`Resources`] before the machine runs.

mod add;

pub use add::AddTool;

/// All built-in tools registered by default.
pub fn builtin_tools() -> Vec<Box<dyn machine::Tool>> {
    vec![Box::new(AddTool)]
}
