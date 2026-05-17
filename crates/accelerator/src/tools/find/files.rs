//! mode=files — find files by glob pattern.
//!
//! Uses `ignore::WalkBuilder` for file traversal and matches patterns against
//! paths relative to the requested search root.

use std::pin::Pin;

use globset::{Glob, GlobBuilder};
use ignore::WalkBuilder;
use machine::{Environment, ToolResult};
use serde_json::Value;
use tracing::warn;

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
        let show_hidden = args["showHidden"].as_bool().unwrap_or(false);

        let glob: Glob = GlobBuilder::new(pattern)
            .literal_separator(false)
            .build()
            .map_err(|error| format!("invalid glob pattern: {error}"))?;
        let matcher = glob.compile_matcher();

        let walker = WalkBuilder::new(&search_path)
            .hidden(!show_hidden)
            .follow_links(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .ignore(true)
            .build();

        let mut matched: Vec<(std::path::PathBuf, Option<std::time::SystemTime>)> = Vec::new();

        for result in walker {
            let entry = match result {
                Ok(entry) => entry,
                Err(error) => {
                    warn!(?error, "find: walk error");
                    continue;
                }
            };

            if !entry
                .file_type()
                .map(|file_type| file_type.is_file())
                .unwrap_or(false)
            {
                continue;
            }

            let Ok(relative) = entry.path().strip_prefix(&search_path) else {
                continue;
            };

            if matcher.is_match(relative) {
                let modified = entry
                    .metadata()
                    .ok()
                    .and_then(|metadata| metadata.modified().ok());
                matched.push((entry.path().to_path_buf(), modified));
            }
        }

        matched.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));

        if matched.is_empty() {
            return Ok(ToolResult {
                call_id: String::new(),
                content: "No files found.\n".to_string(),
                title: Some(format!("find files '{}'", pattern)),
            });
        }

        let total = matched.len();
        let mut output = String::new();
        for (path, _) in matched.iter().take(MAX_FILES_RESULTS) {
            output.push_str(&path.display().to_string());
            output.push('\n');
        }

        if total > MAX_FILES_RESULTS {
            output.push_str(&format!(
                "\n(truncated at {MAX_FILES_RESULTS} of {total} results — narrow the search with 'path')\n"
            ));
        }

        Ok(ToolResult {
            call_id: String::new(),
            content: output,
            title: Some(format!("find files '{}'", pattern)),
        })
    })
}
