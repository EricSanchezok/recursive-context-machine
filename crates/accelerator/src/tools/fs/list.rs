//! Directory listing with recursive tree traversal.
//!
//! Uses the `ignore` crate to walk directories, automatically respecting
//! `.gitignore` rules and common ignore files.

use std::pin::Pin;

use ignore::WalkBuilder;
use machine::{Environment, ToolResult};
use serde_json::Value;
use tracing::warn;

use super::{MAX_LIST_ENTRIES, relative_path, resolve_path};

/// Maximum recursion depth for directory listing.
const MAX_DEPTH: usize = 4;

pub(crate) fn execute<'a>(
    args: &'a Value,
    env: &'a Environment,
) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
    Box::pin(async move {
        let file_path = args["filePath"]
            .as_str()
            .ok_or("missing required parameter 'filePath'")?;

        let resolved = resolve_path(file_path, &env.cwd);

        let meta = tokio::fs::metadata(&resolved)
            .await
            .map_err(|e| format!("cannot access '{}': {e}", resolved.display()))?;

        if !meta.is_dir() {
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

        let walker = WalkBuilder::new(&resolved)
            .max_depth(Some(MAX_DEPTH))
            .hidden(false)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .ignore(true)
            .sort_by_file_name(|a, b| a.cmp(b))
            .build();

        // Collect entries: (depth, name, is_dir)
        let mut entries: Vec<(usize, String, bool)> = Vec::new();

        for result in walker {
            if entries.len() >= max_entries {
                break;
            }

            let entry = match result {
                Ok(e) => e,
                Err(e) => {
                    warn!(?e, "list: walk error");
                    continue;
                }
            };

            // Skip the root directory itself.
            if entry.depth() == 0 {
                continue;
            }

            let name = entry.file_name().to_string_lossy().to_string();
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            entries.push((entry.depth(), name, is_dir));
        }

        if entries.is_empty() {
            return Ok(ToolResult {
                call_id: String::new(),
                content: "Directory is empty.\n".to_string(),
                title: Some(format!("list {relative}")),
            });
        }

        // Render a tree. WalkBuilder's sort_by_file_name gives us DFS order
        // sorted at each level, so siblings appear grouped with their
        // children nested between them. We render with depth-based indentation
        // and directory markers.
        let mut output = String::new();
        for (depth, name, is_dir) in &entries {
            let indent = "  ".repeat(*depth - 1);
            let prefix = if *is_dir { "📁 " } else { "📄 " };
            output.push_str(&format!("{indent}{prefix}{name}\n"));
        }

        if entries.len() >= max_entries {
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
