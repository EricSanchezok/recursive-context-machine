//! Directory listing with fair recursive tree rendering.
//!
//! Uses the `ignore` crate to walk directories, automatically respecting
//! `.gitignore` rules and common ignore files.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::pin::Pin;

use ignore::WalkBuilder;
use machine::{Environment, ToolResult};
use serde_json::Value;
use tracing::warn;

use super::{MAX_LIST_ENTRIES, relative_path, resolve_path};

const DEFAULT_MAX_DEPTH: usize = 4;
const DEFAULT_PER_DIRECTORY_LIMIT: usize = 20;

#[derive(Debug)]
struct ListedEntry {
    name: String,
    path: PathBuf,
    is_dir: bool,
}

pub(crate) fn execute<'a>(
    args: &'a Value,
    env: &'a Environment,
) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
    Box::pin(async move {
        let resolved = if let Some(path) = args["path"].as_str() {
            resolve_path(path, &env.cwd)
        } else {
            env.cwd.clone()
        };

        let metadata = tokio::fs::metadata(&resolved)
            .await
            .map_err(|error| format!("cannot access '{}': {error}", resolved.display()))?;

        if !metadata.is_dir() {
            return Err(format!("not a directory: '{}'", resolved.display()));
        }

        let relative = relative_path(&resolved, &env.cwd);
        let max_entries = args["limit"].as_u64().unwrap_or(MAX_LIST_ENTRIES as u64) as usize;
        let max_depth = args["maxDepth"]
            .as_u64()
            .unwrap_or(DEFAULT_MAX_DEPTH as u64) as usize;
        let per_directory_limit = args["perDirectoryLimit"]
            .as_u64()
            .unwrap_or(DEFAULT_PER_DIRECTORY_LIMIT as u64)
            as usize;
        let show_hidden = args["showHidden"].as_bool().unwrap_or(false);

        if max_entries == 0 || per_directory_limit == 0 || max_depth == 0 {
            return Ok(ToolResult {
                call_id: String::new(),
                content: String::new(),
                title: Some(format!("list {relative}")),
            });
        }

        let tree = collect_tree(&resolved, max_depth, show_hidden)?;
        let mut output = String::new();
        let mut rendered = 0usize;

        render_dir(
            &resolved,
            &tree,
            0,
            per_directory_limit,
            max_entries,
            &mut rendered,
            &mut output,
        );

        if rendered == 0 {
            output.push_str("Directory is empty.\n");
        }

        if rendered >= max_entries {
            output.push_str(&format!(
                "\n(truncated at {max_entries} entries — narrow with 'path', 'maxDepth', or 'perDirectoryLimit')"
            ));
        }

        Ok(ToolResult {
            call_id: String::new(),
            content: output,
            title: Some(format!("list {relative}")),
        })
    })
}

fn collect_tree(
    root: &Path,
    max_depth: usize,
    show_hidden: bool,
) -> Result<BTreeMap<PathBuf, Vec<ListedEntry>>, String> {
    let walker = WalkBuilder::new(root)
        .max_depth(Some(max_depth))
        .hidden(!show_hidden)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .ignore(true)
        .sort_by_file_name(|left, right| left.cmp(right))
        .build();

    let mut tree: BTreeMap<PathBuf, Vec<ListedEntry>> = BTreeMap::new();

    for result in walker {
        let entry = match result {
            Ok(entry) => entry,
            Err(error) => {
                warn!(?error, "list: walk error");
                continue;
            }
        };

        if entry.depth() == 0 {
            continue;
        }

        let Some(parent) = entry.path().parent() else {
            continue;
        };

        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry
            .file_type()
            .map(|file_type| file_type.is_dir())
            .unwrap_or(false);

        tree.entry(parent.to_path_buf())
            .or_default()
            .push(ListedEntry {
                name,
                path: entry.path().to_path_buf(),
                is_dir,
            });
    }

    Ok(tree)
}

fn render_dir(
    dir: &Path,
    tree: &BTreeMap<PathBuf, Vec<ListedEntry>>,
    depth: usize,
    per_directory_limit: usize,
    max_entries: usize,
    rendered: &mut usize,
    output: &mut String,
) {
    if *rendered >= max_entries {
        return;
    }

    let Some(entries) = tree.get(dir) else {
        return;
    };

    let visible_count = entries.len().min(per_directory_limit);
    for entry in entries.iter().take(visible_count) {
        if *rendered >= max_entries {
            return;
        }

        *rendered += 1;
        let indent = "  ".repeat(depth);
        let prefix = if entry.is_dir { "📁 " } else { "📄 " };
        output.push_str(&format!("{indent}{prefix}{}\n", entry.name));

        if entry.is_dir {
            render_dir(
                &entry.path,
                tree,
                depth + 1,
                per_directory_limit,
                max_entries,
                rendered,
                output,
            );
        }
    }

    if entries.len() > visible_count && *rendered < max_entries {
        let omitted = entries.len() - visible_count;
        let indent = "  ".repeat(depth);
        output.push_str(&format!("{indent}… {omitted} more entries\n"));
    }
}
