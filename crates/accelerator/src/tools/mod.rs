//! Built-in tools for the accelerator.

mod arxiv;
mod find;
mod fs;
mod git;
mod lsp;
mod shell;
mod wait;

pub use arxiv::{ArxivDownloadTool, ArxivSearchTool};
pub use find::FindTool;
pub use fs::FsTool;
pub use git::{check_safety as check_git_safety, tokenize as tokenize_git, GitTool};
pub use lsp::LspTool;
pub use shell::ShellTool;
pub use wait::WaitTool;

use std::path::{Path, PathBuf};

use crate::catalog::Catalog;

/// Register all built-in tools in the catalog.
pub fn register(catalog: &mut Catalog) {
    for tool in builtin_tools() {
        let name = tool.name().to_string();
        catalog.tools.insert(name, tool);
    }
}

/// All built-in tools.
pub fn builtin_tools() -> Vec<std::sync::Arc<dyn machine::Tool>> {
    vec![
        std::sync::Arc::new(ArxivSearchTool),
        std::sync::Arc::new(ArxivDownloadTool),
        std::sync::Arc::new(FindTool),
        std::sync::Arc::new(FsTool),
        std::sync::Arc::new(GitTool),
        std::sync::Arc::new(LspTool),
        std::sync::Arc::new(ShellTool),
        std::sync::Arc::new(WaitTool),
    ]
}

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
