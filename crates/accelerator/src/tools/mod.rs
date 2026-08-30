//! Built-in tools for the accelerator.

mod arxiv;
mod find;
mod fs;
mod git;
mod image_gen;
mod ledger;
mod lsp;
pub mod shell;
mod spawn;
mod wait;
mod webfetch;

pub use arxiv::{ArxivDownloadTool, ArxivSearchTool};
pub use find::FindTool;
pub use fs::FsTool;
pub use git::{GitTool, check_safety as check_git_safety, tokenize as tokenize_git};
pub use image_gen::{IMAGE_GEN_DIAGNOSTIC_ENV, ImageGenTool};
pub use ledger::{LedgerTool, ledger_digest_for};
pub use lsp::LspTool;
pub use shell::{OUTPUT_CAP_BYTES, ShellTool, build_result, collect_output};
pub use spawn::SpawnTool;
pub use wait::WaitTool;
pub use webfetch::WebFetchTool;

use std::path::{Component, Path, PathBuf};

use crate::catalog::Catalog;
use crate::registry::ResourcesTool;
pub fn register(catalog: &mut Catalog) {
    for tool in builtin_tools() {
        catalog
            .register_tool(tool)
            .expect("built-in tool names must be unique");
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
        std::sync::Arc::new(ImageGenTool),
        std::sync::Arc::new(LedgerTool),
        std::sync::Arc::new(LspTool),
        std::sync::Arc::new(ResourcesTool),
        std::sync::Arc::new(ShellTool),
        std::sync::Arc::new(WaitTool),
        std::sync::Arc::new(WebFetchTool),
    ]
}

/// Resolve a tool path within the run-directory capability, when present.
///
/// `Environment::root` is the capability boundary established by the CLI for
/// `--run-dir`. Every existing path component is checked with
/// `symlink_metadata`, so a model cannot escape through `..`, an absolute path,
/// or a symlink prepared inside the workspace. Generic RCM runs without a root
/// retain the historical cwd-relative behavior.
pub(crate) fn resolve_path(raw: &str, env: &machine::Environment) -> Result<PathBuf, String> {
    let path = Path::new(raw);
    let Some(root) = env.root.as_deref() else {
        return Ok(if path.is_absolute() {
            path.to_path_buf()
        } else {
            env.cwd.join(path)
        });
    };

    if path
        .components()
        .any(|component| component == Component::ParentDir)
    {
        return Err("sandbox violation: parent-directory traversal is not allowed".to_string());
    }

    let relative = if path.is_absolute() {
        path.strip_prefix(root).map_err(|_| {
            "sandbox violation: absolute path is outside the run directory".to_string()
        })?
    } else {
        path
    };
    if relative
        .components()
        .any(|component| matches!(component, Component::RootDir | Component::Prefix(_)))
    {
        return Err("sandbox violation: invalid workspace path".to_string());
    }

    let mut candidate = root.to_path_buf();
    for component in relative.components() {
        match component {
            Component::CurDir => continue,
            Component::Normal(segment) => candidate.push(segment),
            _ => return Err("sandbox violation: invalid workspace path".to_string()),
        }
        match std::fs::symlink_metadata(&candidate) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err("sandbox violation: symbolic links are not allowed".to_string());
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("cannot inspect workspace path: {error}")),
        }
    }

    let existing = candidate
        .ancestors()
        .find(|ancestor| ancestor.exists())
        .ok_or_else(|| "sandbox violation: workspace root is unavailable".to_string())?;
    let canonical_existing = existing
        .canonicalize()
        .map_err(|error| format!("cannot resolve workspace path: {error}"))?;
    if !canonical_existing.starts_with(root) {
        return Err("sandbox violation: path escapes the run directory".to_string());
    }
    Ok(candidate)
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

#[cfg(test)]
mod path_tests {
    use std::fs;
    use std::path::PathBuf;

    use machine::Environment;

    use super::resolve_path;

    fn workspace(name: &str) -> (PathBuf, Environment) {
        let root = std::env::temp_dir().join(format!(
            "rcm-sandbox-{name}-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("workspace");
        let root = root.canonicalize().expect("canonical workspace");
        let mut env = Environment::empty(&root);
        env.root = Some(root.clone());
        env.run_dir = Some(root.clone());
        (root, env)
    }

    #[test]
    fn resolves_relative_path_inside_workspace() {
        let (root, env) = workspace("inside");
        assert_eq!(
            resolve_path("reports/final.md", &env).expect("inside path"),
            root.join("reports/final.md")
        );
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn rejects_parent_directory_escape() {
        let (root, env) = workspace("parent");
        let error = resolve_path("../outside", &env).expect_err("escape must fail");
        assert!(error.contains("sandbox violation"));
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn rejects_absolute_path_outside_workspace() {
        let (root, env) = workspace("absolute");
        let error = resolve_path("/proc/self/environ", &env).expect_err("escape must fail");
        assert!(error.contains("sandbox violation"));
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlink_escape() {
        use std::os::unix::fs::symlink;

        let (root, env) = workspace("symlink");
        symlink("/etc", root.join("link")).expect("symlink");
        let error = resolve_path("link/passwd", &env).expect_err("symlink must fail");
        assert!(error.contains("symbolic links"));
        fs::remove_dir_all(root).expect("cleanup");
    }
}
