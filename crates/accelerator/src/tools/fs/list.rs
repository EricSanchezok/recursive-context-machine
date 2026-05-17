//! Directory listing with recursive tree traversal.
//!
//! Walks the directory tree up to a configurable depth, applying built-in
//! ignore rules (node_modules, .git, target, etc.) and an entry limit.

use std::pin::Pin;

use machine::{Environment, ToolResult};
use serde_json::Value;

use super::{MAX_LIST_ENTRIES, relative_path, resolve_path};

/// Built-in ignore patterns — directories and files excluded from listing.
/// These match Synergy's standard ignore list.
const IGNORE_PATTERNS: &[&str] = &[
    "node_modules",
    "__pycache__",
    ".git",
    ".svn",
    ".hg",
    "dist",
    "build",
    "target",
    "vendor",
    "bin",
    "obj",
    ".idea",
    ".vscode",
    ".zig-cache",
    "zig-out",
    ".coverage",
    "coverage",
    "tmp",
    "temp",
    ".cache",
    "cache",
    "logs",
    ".venv",
    "venv",
    "env",
    ".DS_Store",
    "Thumbs.db",
];

/// Returns true if the entry name matches any ignore pattern.
fn is_ignored(name: &str) -> bool {
    IGNORE_PATTERNS.contains(&name) || name.starts_with('.')
}

pub(crate) fn execute<'a>(
    args: &'a Value,
    env: &'a Environment,
) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
    Box::pin(async move {
        let file_path = args["filePath"]
            .as_str()
            .ok_or("missing required parameter 'filePath'")?;

        let resolved = resolve_path(file_path, &env.cwd);

        let metadata = tokio::fs::metadata(&resolved)
            .await
            .map_err(|error| format!("cannot access '{}': {error}", resolved.display()))?;

        if !metadata.is_dir() {
            return Err(format!("not a directory: '{}'", resolved.display()));
        }

        let relative = relative_path(&resolved, &env.cwd);

        let max_entries = args["limit"].as_u64().unwrap_or(MAX_LIST_ENTRIES as u64) as usize;
        if max_entries == 0 {
            return Ok(ToolResult {
                call_id: String::new(),
                content: String::new(),
                title: Some(format!("list {relative}")),
            });
        }
        let max_depth = 4;

        let mut output = String::new();
        let mut count = 0;
        let mut stack = vec![(resolved.clone(), 0)];

        while let Some((dir, depth)) = stack.pop() {
            if depth > max_depth || count >= max_entries {
                continue;
            }

            let mut reader = match tokio::fs::read_dir(&dir).await {
                Ok(r) => r,
                Err(_) => continue,
            };

            let mut entries: Vec<(String, bool)> = Vec::new();
            loop {
                let entry = match reader.next_entry().await {
                    Ok(Some(e)) => e,
                    _ => break,
                };
                let name = entry.file_name().to_string_lossy().to_string();
                if is_ignored(&name) {
                    continue;
                }
                let is_dir = entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false);
                entries.push((name, is_dir));
            }

            // Sort: directories first, then files, each alphabetically
            entries.sort_by(|a, b| {
                if a.1 != b.1 {
                    b.1.cmp(&a.1) // dirs first
                } else {
                    a.0.cmp(&b.0)
                }
            });

            let indent = "  ".repeat(depth);

            for (name, is_directory) in &entries {
                if count >= max_entries {
                    break;
                }
                count += 1;

                let prefix = if *is_directory { "📁 " } else { "📄 " };
                output.push_str(&format!("{indent}{prefix}{name}\n"));
            }

            // Push subdirectories for continued traversal (reversed to
            // maintain alphabetical order with LIFO popping).
            if count < max_entries {
                let dirs: Vec<_> = entries
                    .iter()
                    .rev()
                    .filter(|(_, is_dir)| *is_dir)
                    .map(|(name, _)| dir.join(name))
                    .collect();
                stack.extend(dirs.into_iter().map(|p| (p, depth + 1)));
            }
        }

        if count == 0 {
            output.push_str("Directory is empty.\n");
        }

        if count >= max_entries {
            output.push_str(&format!(
                "\n(truncated at {max_entries} entries — directory may have more)"
            ));
        }

        Ok(ToolResult {
            call_id: String::new(),
            content: output,
            title: Some(format!("list {relative}")),
        })
    })
}
