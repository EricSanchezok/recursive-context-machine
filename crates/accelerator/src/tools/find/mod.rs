//! Unified find tool — files / text / ast.
//!
//! A single `find` tool that dispatches to focused sub-modules by mode.
//! All modes use the `ignore` crate for file traversal, respecting
//! `.gitignore` and other ignore files automatically.

mod ast;
mod files;
mod list;
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

/// Max entries rendered by list mode.
pub(crate) const MAX_LIST_ENTRIES: usize = 200;

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
                    "enum": ["files", "text", "ast", "list"],
                    "description": "What to find: files (by glob pattern on file names), text (by regex in file contents), ast (by AST pattern in code structure), list (browse a directory as a recursive tree)."
                },
                "pattern": {
                    "type": "string",
                    "description": "The search pattern. Required for files/text/ast; ignored for list. For files: a glob like '**/*.rs'. For text: a regex like 'fn\\s+\\w+'. For ast: an AST pattern with meta-variables ($VAR for one node, $$$ for zero or more)."
                },
                "path": {
                    "type": "string",
                    "description": "Directory to search inside (files/text/ast) or to list (list). Omit to use the project root."
                },
                "include": {
                    "type": "string",
                    "description": "(text only) Glob to filter which files to search, e.g. '*.rs' or '*.{ts,tsx}'. Files not matching this glob are skipped."
                },
                "showHidden": {
                    "type": "boolean",
                    "description": "(files, text) Whether to include hidden files and directories such as .git, .github, .vscode. Default: false. Set true only when you explicitly need hidden project metadata."
                },
                "lang": {
                    "type": "string",
                    "description": "(ast only) Language identifier for the code: rust, python, typescript, go, java, c, cpp, csharp, javascript, bash, css, elixir, haskell, html, json, kotlin, lua, nix, php, ruby, scala, solidity, swift, tsx, yaml."
                },
                "context": {
                    "type": "integer",
                    "description": "(ast only) Number of extra lines to show around each match. Default: 0."
                },
                "maxDepth": {
                    "type": "integer",
                    "description": "(list only) Maximum directory depth to traverse. Default: 4. Use a smaller value for a quick overview."
                },
                "perDirectoryLimit": {
                    "type": "integer",
                    "description": "(list only) Maximum entries rendered from each directory before showing an omitted-count line. Default: 20."
                },
                "limit": {
                    "type": "integer",
                    "description": "(list only) Maximum total entries rendered across the whole tree. Default: 200."
                }
            },
            "required": ["mode"]
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
                "list" => list::execute(&args, env).await,
                other => Err(format!("unknown mode '{other}'")),
            }
        })
    }
}
