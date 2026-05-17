//! mode=text — search file contents with regex.
//!
//! Uses `ignore::WalkBuilder` for traversal and `regex::Regex` for matching.
//! Results are grouped by file and sorted by modification time (newest first).

use std::pin::Pin;

use ignore::WalkBuilder;
use machine::{Environment, ToolResult};
use regex::Regex;
use serde_json::Value;
use tracing::warn;

use super::{MAX_LINE_LENGTH, MAX_TEXT_RESULTS, relative_path, resolve_path};

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

struct MatchGroup {
    path: std::path::PathBuf,
    modified: Option<std::time::SystemTime>,
    hits: Vec<(usize, String)>,
}

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

        let regex =
            Regex::new(pattern).map_err(|error| format!("invalid regex pattern: {error}"))?;
        let include_matcher = build_include_matcher(args["include"].as_str())?;

        let walker = WalkBuilder::new(&search_path)
            .hidden(!show_hidden)
            .follow_links(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .ignore(true)
            .build();

        let mut results = Vec::new();
        let mut total_hits = 0usize;

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

            if include_matcher
                .as_ref()
                .is_some_and(|matcher| !matcher.is_match(relative))
            {
                continue;
            }

            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(error) => {
                    warn!(?error, path = %entry.path().display(), "find: cannot stat file");
                    continue;
                }
            };

            if metadata.len() > MAX_FILE_SIZE {
                continue;
            }

            let content = match tokio::fs::read_to_string(entry.path()).await {
                Ok(content) => content,
                Err(error) => {
                    warn!(?error, path = %entry.path().display(), "find: cannot read file");
                    continue;
                }
            };

            let mut hits = Vec::new();
            for (index, line) in content.lines().enumerate() {
                if regex.is_match(line) {
                    hits.push((index + 1, truncate_line(line, MAX_LINE_LENGTH)));
                    total_hits += 1;
                }
            }

            if !hits.is_empty() {
                results.push(MatchGroup {
                    path: entry.path().to_path_buf(),
                    modified: metadata.modified().ok(),
                    hits,
                });
            }
        }

        results.sort_by(|left, right| {
            right
                .modified
                .cmp(&left.modified)
                .then_with(|| left.path.cmp(&right.path))
        });

        if results.is_empty() {
            return Ok(ToolResult {
                call_id: String::new(),
                content: "No matches found.\n".to_string(),
                title: Some(format!("grep '{}'", pattern)),
            });
        }

        let mut emitted = 0usize;
        let mut output = String::new();
        output.push_str(&format!("Found {total_hits} matches\n\n"));

        for group in &results {
            if emitted >= MAX_TEXT_RESULTS {
                break;
            }

            let remaining = MAX_TEXT_RESULTS - emitted;
            let hits_to_emit = group.hits.len().min(remaining);
            let display = relative_path(&group.path, &search_path);
            output.push_str(&format!("{}:\n", display));

            for (line_num, text) in group.hits.iter().take(hits_to_emit) {
                output.push_str(&format!("{:>6}: {}\n", line_num, text));
            }
            output.push('\n');
            emitted += hits_to_emit;
        }

        if total_hits > MAX_TEXT_RESULTS {
            output.push_str(&format!(
                "(truncated at {MAX_TEXT_RESULTS} of {total_hits} matches — narrow with 'include' or 'path')\n"
            ));
        }

        Ok(ToolResult {
            call_id: String::new(),
            content: output,
            title: Some(format!("grep '{}'", pattern)),
        })
    })
}

fn build_include_matcher(include: Option<&str>) -> Result<Option<globset::GlobMatcher>, String> {
    let Some(include) = include else {
        return Ok(None);
    };

    let glob = globset::GlobBuilder::new(include)
        .literal_separator(false)
        .build()
        .map_err(|error| format!("invalid include glob: {error}"))?;

    Ok(Some(glob.compile_matcher()))
}

fn truncate_line(line: &str, max_chars: usize) -> String {
    if line.chars().count() <= max_chars {
        return line.to_string();
    }

    let mut truncated: String = line.chars().take(max_chars).collect();
    truncated.push_str("...");
    truncated
}
