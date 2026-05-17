//! mode=text — search file contents with regex.
//!
//! Uses `ignore::WalkBuilder` for traversal and `regex::Regex` for matching.
//! Results grouped by file and sorted by modification time (newest first).

use std::pin::Pin;

use ignore::WalkBuilder;
use machine::{Environment, ToolResult};
use regex::Regex;
use serde_json::Value;

use super::{MAX_LINE_LENGTH, MAX_TEXT_RESULTS, relative_path, resolve_path};

/// Maximum file size to read into memory for text search (10 MB).
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

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

        let re = Regex::new(pattern).map_err(|e| format!("invalid regex pattern: {e}"))?;

        let include = args["include"].as_str();

        let mut walker = WalkBuilder::new(&search_path);
        walker
            .hidden(false)
            .follow_links(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .ignore(true);

        if let Some(inc) = include {
            let inc_glob = globset::GlobBuilder::new(inc)
                .literal_separator(false)
                .build()
                .map_err(|e| format!("invalid include glob: {e}"))?;
            let inc_matcher = inc_glob.compile_matcher();
            let cwd = search_path.clone();
            walker.filter_entry(move |entry| {
                if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                    entry
                        .path()
                        .strip_prefix(&cwd)
                        .is_ok_and(|rel| inc_matcher.is_match(rel))
                } else {
                    true // always traverse directories
                }
            });
        }

        struct Match {
            path: std::path::PathBuf,
            mtime: Option<std::time::SystemTime>,
            hits: Vec<(usize, String)>,
        }

        let mut results: Vec<Match> = Vec::new();
        let mut total_hits = 0;

        for result in walker.build() {
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

            // Skip files that are too large to read into memory.
            if entry.metadata().map(|m| m.len()).unwrap_or(0) > MAX_FILE_SIZE {
                continue;
            }

            let content = match tokio::fs::read_to_string(entry.path()).await {
                Ok(c) => c,
                Err(e) => {
                    tracing::warn!(?e, path = %entry.path().display(), "find: cannot read file");
                    continue;
                }
            };

            let mut hits: Vec<(usize, String)> = Vec::new();
            for (index, line) in content.lines().enumerate() {
                let line_num = index + 1;
                if re.is_match(line) {
                    let text = if line.len() > MAX_LINE_LENGTH {
                        format!("{}...", &line[..MAX_LINE_LENGTH])
                    } else {
                        line.to_string()
                    };
                    hits.push((line_num, text));
                    total_hits += 1;
                    if total_hits >= MAX_TEXT_RESULTS {
                        break;
                    }
                }
            }

            if !hits.is_empty() {
                let mtime = entry.metadata().ok().and_then(|m| m.modified().ok());
                results.push(Match {
                    path: entry.path().to_path_buf(),
                    mtime,
                    hits,
                });
            }

            if total_hits >= MAX_TEXT_RESULTS {
                break;
            }
        }

        // Sort results by mtime desc (newest first), then by path for ties.
        results.sort_by(|a, b| b.mtime.cmp(&a.mtime).then_with(|| a.path.cmp(&b.path)));

        if results.is_empty() {
            return Ok(ToolResult {
                call_id: String::new(),
                content: "No matches found.\n".to_string(),
                title: Some(format!("grep '{}'", pattern)),
            });
        }

        let mut output = String::new();
        output.push_str(&format!("Found {} matches\n\n", total_hits));

        for m in &results {
            let display = relative_path(&m.path, &search_path);
            output.push_str(&format!("{}:\n", display));
            for (line_num, text) in &m.hits {
                output.push_str(&format!("{:>6}: {}\n", line_num, text));
            }
            output.push('\n');
        }

        if total_hits >= MAX_TEXT_RESULTS {
            output.push_str(&format!(
                "(truncated at {MAX_TEXT_RESULTS} matches — narrow with 'include' or 'path')\n"
            ));
        }

        Ok(ToolResult {
            call_id: String::new(),
            content: output,
            title: Some(format!("grep '{}'", pattern)),
        })
    })
}
