//! File system tool — unified read/write/edit/list/stat.
//!
//! A single `fs` tool that dispatches to focused sub-modules by action.
//! This keeps the LLM interface simple (one tool instead of five) while
//! keeping the implementation modular.

mod edit;
mod list;
mod read;
mod stat;
mod write;

use std::pin::Pin;
use std::sync::LazyLock;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;

// Re-export shared helpers so sub-modules can `use super::*`.
pub(crate) use super::{relative_path, resolve_path};

/// Maximum total bytes returned by any single tool call.
pub(crate) const OUTPUT_CAP_BYTES: usize = 512 * 1024;

/// Maximum line length before truncation.
pub(crate) const MAX_LINE_LENGTH: usize = 2000;

/// Default read limit (lines).
pub(crate) const DEFAULT_READ_LIMIT: usize = 2000;

/// Max entries returned by list.
pub(crate) const MAX_LIST_ENTRIES: usize = 200;

/// Default execution timeout for fs operations.
const DEFAULT_TIMEOUT_SECS: u64 = 60;

static DESCRIPTION: LazyLock<String> = LazyLock::new(|| {
    include_str!("mod.txt")
        .replace("{OUTPUT_CAP_KB}", &(OUTPUT_CAP_BYTES / 1024).to_string())
        .replace("{DEFAULT_READ_LIMIT}", &DEFAULT_READ_LIMIT.to_string())
        .replace("{MAX_LINE_LENGTH}", &MAX_LINE_LENGTH.to_string())
        .replace("{MAX_LIST_ENTRIES}", &MAX_LIST_ENTRIES.to_string())
});

pub struct FsTool;

impl Tool for FsTool {
    fn name(&self) -> &str {
        "fs"
    }

    fn description(&self) -> &str {
        &DESCRIPTION
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["read", "write", "edit", "list", "stat"],
                    "description": "Which filesystem operation to perform: read (read a file with line numbers), write (overwrite or create a file), edit (targeted string replacement), list (list directory contents), stat (file metadata)."
                },
                "filePath": {
                    "type": "string",
                    "description": "Path to the target file or directory. Absolute paths are preferred; relative paths are resolved against the current working directory."
                },
                "offset": {
                    "type": "integer",
                    "description": "(read) 0-based line number to start reading from. Default: 0 (beginning of file). Omit to read from the start."
                },
                "limit": {
                    "type": "integer",
                    "description": "(read, list) Maximum entries to return. For read: max lines (default 2000). For list: total rendered entries across the tree (default 200). Omit for the default."
                },
                "maxDepth": {
                    "type": "integer",
                    "description": "(list) Maximum directory depth to traverse. Default: 4. Use a smaller value for a quick project overview."
                },
                "perDirectoryLimit": {
                    "type": "integer",
                    "description": "(list) Maximum entries rendered from each directory before showing an omitted-count line. Default: 20. Prevents one large directory from consuming the entire listing."
                },
                "showHidden": {
                    "type": "boolean",
                    "description": "(list) Whether to include hidden files and directories such as .git, .github, .vscode. Default: false. Set true only when you explicitly need hidden project metadata."
                },
                "content": {
                    "type": "string",
                    "description": "(write) The complete text content to write to the file. Required for write."
                },
                "oldString": {
                    "type": "string",
                    "description": "(edit) The exact text to find in the file. Include enough surrounding lines to make the match unique. Required for edit."
                },
                "newString": {
                    "type": "string",
                    "description": "(edit) The replacement text. Must differ from oldString. Required for edit."
                },
                "replaceAll": {
                    "type": "boolean",
                    "description": "(edit) When true, replaces ALL occurrences of oldString rather than just one. Use for batch renames. Default: false."
                }
            },
            "required": ["action", "filePath"]
        })
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(DEFAULT_TIMEOUT_SECS)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let action = args["action"]
                .as_str()
                .ok_or("missing required parameter 'action'")?;

            match action {
                "read" => read::execute(&args, env).await,
                "write" => write::execute(&args, env).await,
                "edit" => edit::execute(&args, env).await,
                "list" => list::execute(&args, env).await,
                "stat" => stat::execute(&args, env).await,
                other => Err(format!("unknown action '{other}'")),
            }
        })
    }
}
