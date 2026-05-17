//! mode=files — find files by glob pattern.
//!
//! Uses `ignore::WalkBuilder` for file traversal (respects .gitignore).
//! Pattern is matched against relative paths using `globset::Glob`.
//! Results sorted by modification time, newest first, returned as absolute paths.

use std::pin::Pin;

use globset::{Glob, GlobBuilder};
use ignore::WalkBuilder;
use machine::{Environment, ToolResult};
use serde_json::Value;

use super::{MAX_FILES_RESULTS, resolve_path};

pub(crate) fn execute<'a>(
    args: &'a Value,
    env: &'a Environment,
) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
    Box::pin(async move {
        let pattern = args["pattern"]
            .as_str()
            .ok_or("missing required parameter 'pattern'")?;

        let search_path = if let Some(path) = args["path"].as_str() {
            resolve_path(path, &env.cwd)
        } else {
            env.cwd.clone()
        };

        let glob: Glob = GlobBuilder::new(pattern)
            .literal_separator(false)
            .build()
            .map_err(|e| format!("invalid glob pattern: {e}"))?;
        let matcher = glob.compile_matcher();

        let walker = WalkBuilder::new(&search_path)
            .hidden(false)
            .follow_links(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .ignore(true)
            .build();

        let mut matched: Vec<(std::path::PathBuf, Option<std::time::SystemTime>)> = Vec::new();

        for result in walker {
            let entry = match result {
                Ok(e) => e,
                Err(e) => {
                    tracing::warn!(?e, "find: walk error");
                    continue;
                }
            };

            if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                continue;
            }
            if let Ok(rel) = entry.path().strip_prefix(&search_path) {
                if matcher.is_match(rel) {
                    // entry.metadata() reuses the stat from the walker — no extra syscall.
                    let mtime = entry.metadata().ok().and_then(|m| m.modified().ok());
                    matched.push((entry.path().to_path_buf(), mtime));
                }
            }

            if matched.len() >= MAX_FILES_RESULTS {
                break;
            }
        }

        matched.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        let count = matched.len();
        let output = if matched.is_empty() {
            "No files found.\n".to_string()
        } else {
            let mut lines = String::new();
            for (path, _) in &matched {
                lines.push_str(&path.display().to_string());
                lines.push('\n');
            }
            if count >= MAX_FILES_RESULTS {
                lines.push_str(&format!(
                    "\n(truncated at {MAX_FILES_RESULTS} results — narrow the search with 'path')\n"
                ));
            }
            lines
        };

        Ok(ToolResult {
            call_id: String::new(),
            content: output,
            title: Some(format!("find files '{}'", pattern)),
        })
    })
}
