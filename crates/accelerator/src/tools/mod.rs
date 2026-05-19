//! Built-in tools for the accelerator.
//!
//! Each tool implements the [`Tool`] trait from the `machine` crate.
//! Tools are registered into [`Resources`] before the machine runs.

mod find;
mod fs;
mod shell;
mod wait;

pub use find::FindTool;
pub use fs::FsTool;
pub use shell::ShellTool;
pub use wait::WaitTool;

use std::path::{Path, PathBuf};

/// Resolve a path string against a working directory.
pub(crate) fn resolve_path(raw: &str, cwd: &Path) -> PathBuf {
    let path = Path::new(raw);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}

/// Compute the display path relative to a working directory.
pub(crate) fn relative_path(path: &Path, cwd: &Path) -> String {
    path.strip_prefix(cwd)
        .map(|relative| {
            if relative.as_os_str().is_empty() {
                ".".to_string()
            } else {
                relative.display().to_string()
            }
        })
        .unwrap_or_else(|_| path.display().to_string())
}

/// All built-in tools registered by default.
pub fn builtin_tools() -> Vec<std::sync::Arc<dyn machine::Tool>> {
    vec![
        std::sync::Arc::new(FindTool),
        std::sync::Arc::new(FindTool),
        std::sync::Arc::new(FsTool),
        std::sync::Arc::new(ShellTool),
        std::sync::Arc::new(WaitTool),
    ]
}
