//! Unified find tool — files / text / ast.
//!
//! A single `find` tool that dispatches to focused sub-modules by mode.
//! All modes use the `ignore` crate for file traversal, respecting
//! `.gitignore` and other ignore files automatically.

mod ast;
mod files;
mod text;

use std::pin::Pin;
use std::sync::LazyLock;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;

// Re-export shared helpers so sub-modules can use them.
pub(crate) use super::{relative_path, resolve_path};

/// Max files mode results.
pub(crate) const MAX_FILES_RESULTS: usize = 200;

/// Max text mode results.
pub(crate) const MAX_TEXT_RESULTS: usize = 200;

/// Max ast mode results.
pub(crate) const MAX_AST_RESULTS: usize = 500;

/// Max line length in text mode output.
pub(crate) const MAX_LINE_LENGTH: usize = 2000;

/// Default execution timeout.
const DEFAULT_TIMEOUT_SECS: u64 = 60;

static DESCRIPTION: LazyLock<String> = LazyLock::new(|| include_str!("mod.txt").to_string());

pub struct FindTool;

impl Tool for FindTool {
    fn name(&self) -> &str {
        "find"
    }

    fn description(&self) -> &str {
        &DESCRIPTION
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "mode": {
                    "type": "string",
                    "enum": ["files", "text", "ast"],
                    "description": "What to find: files (by glob pattern on file names), text (by regex in file contents), ast (by AST pattern in code structure)."
                },
                "pattern": {
                    "type": "string",
                    "description": "The search pattern. For files: a glob like '**/*.rs'. For text: a regex like 'fn\\s+\\w+'. For ast: an AST pattern with meta-variables ($VAR for one node, $$$ for zero or more)."
                },
                "path": {
                    "type": "string",
                    "description": "Directory to search inside. Omit to search from the project root."
                },
                "include": {
                    "type": "string",
                    "description": "(text only) Glob to filter which files to search, e.g. '*.rs' or '*.{ts,tsx}'. Files not matching this glob are skipped."
                },
                "lang": {
                    "type": "string",
                    "description": "(ast only) Language identifier for the code: rust, python, typescript, go, java, c, cpp, csharp, javascript, bash, css, elixir, haskell, html, json, kotlin, lua, nix, php, ruby, scala, solidity, swift, tsx, yaml."
                },
                "context": {
                    "type": "integer",
                    "description": "(ast only) Number of extra lines to show around each match. Default: 0."
                }
            },
            "required": ["mode", "pattern"]
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
            let mode = args["mode"]
                .as_str()
                .ok_or("missing required parameter 'mode'")?;

            match mode {
                "files" => files::execute(&args, env).await,
                "text" => text::execute(&args, env).await,
                "ast" => ast::execute(&args, env).await,
                other => Err(format!("unknown mode '{other}'")),
            }
        })
    }
}
