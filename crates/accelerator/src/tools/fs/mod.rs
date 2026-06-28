//! File tools — read, write, and edit individual files.
//!
//! Three focused single-purpose tools that share one implementation module.
//! Each maps to exactly one operation with a small, unambiguous parameter set,
//! rather than one aggregated tool dispatching on an `action` enum. Directory
//! listing lives in the `find` tool (mode=list); these tools deal only with
//! individual files.

mod edit;
mod guard;
mod pdf;
mod read;
mod write;

use std::pin::Pin;
use std::sync::LazyLock;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;

// Re-export shared helpers so sub-modules can `use super::*`.
pub(crate) use super::{relative_path, resolve_path};

/// Maximum total bytes returned by a single read.
pub(crate) const OUTPUT_CAP_BYTES: usize = 512 * 1024;

/// Maximum line length before truncation.
pub(crate) const MAX_LINE_LENGTH: usize = 2000;

/// Default read limit (lines).
pub(crate) const DEFAULT_READ_LIMIT: usize = 2000;

/// Default execution timeout for file operations.
const DEFAULT_TIMEOUT_SECS: u64 = 60;

static READ_DESC: LazyLock<String> = LazyLock::new(|| {
    include_str!("read.txt")
        .replace("{OUTPUT_CAP_KB}", &(OUTPUT_CAP_BYTES / 1024).to_string())
        .replace("{DEFAULT_READ_LIMIT}", &DEFAULT_READ_LIMIT.to_string())
        .replace("{MAX_LINE_LENGTH}", &MAX_LINE_LENGTH.to_string())
});

static EDIT_DESC: LazyLock<String> = LazyLock::new(|| include_str!("edit.txt").to_string());

static WRITE_DESC: LazyLock<String> = LazyLock::new(|| include_str!("write.txt").to_string());

pub struct ReadTool;

impl Tool for ReadTool {
    fn name(&self) -> &str {
        "read"
    }

    fn description(&self) -> &str {
        &READ_DESC
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "filePath": {
                    "type": "string",
                    "description": "Path to the file to read. Absolute paths are preferred; relative paths are resolved against the current working directory."
                },
                "offset": {
                    "type": "integer",
                    "description": "0-based line number to start reading from. Default: 0 (beginning of file). Omit to read from the start."
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of lines to return. Default: 2000. Omit for the default."
                }
            },
            "required": ["filePath"]
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
        Box::pin(async move { read::execute(&args, env).await })
    }
}

pub struct EditTool;

impl Tool for EditTool {
    fn name(&self) -> &str {
        "edit"
    }

    fn description(&self) -> &str {
        &EDIT_DESC
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "filePath": {
                    "type": "string",
                    "description": "Path to the file to edit. The file must already exist and must have been read first."
                },
                "oldString": {
                    "type": "string",
                    "description": "The exact text to find in the file. Include enough surrounding lines to make the match unique."
                },
                "newString": {
                    "type": "string",
                    "description": "The replacement text. Must differ from oldString."
                },
                "replaceAll": {
                    "type": "boolean",
                    "description": "When true, replaces ALL occurrences of oldString rather than requiring a unique match. Use for batch renames. Default: false."
                }
            },
            "required": ["filePath", "oldString", "newString"]
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
        Box::pin(async move { edit::execute(&args, env).await })
    }
}

pub struct WriteTool;

impl Tool for WriteTool {
    fn name(&self) -> &str {
        "write"
    }

    fn description(&self) -> &str {
        &WRITE_DESC
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "filePath": {
                    "type": "string",
                    "description": "Path to the file to write. Parent directories are created automatically. Absolute paths are preferred."
                },
                "content": {
                    "type": "string",
                    "description": "The complete text content to write to the file."
                }
            },
            "required": ["filePath", "content"]
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
        Box::pin(async move { write::execute(&args, env).await })
    }
}
