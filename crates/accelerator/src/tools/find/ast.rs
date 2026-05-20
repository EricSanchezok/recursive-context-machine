//! mode=ast — find code structure via ast-grep CLI.
//!
//! Spawns `sg` (or `ast-grep`) with `--json=compact` and parses the output.
//! Discovers the binary through PATH, Homebrew, and common system paths.

use std::pin::Pin;

use machine::{Environment, ToolResult};
use serde_json::Value;

use super::{MAX_AST_RESULTS, resolve_path};

/// Try each candidate binary name / path until a valid one is found.
/// Validates that the binary exists and is a file (not a directory or symlink
/// to nothing).
fn find_sg_binary() -> Option<std::path::PathBuf> {
    let candidates: &[&str] = &[
        "sg",
        "ast-grep",
        "/opt/homebrew/bin/sg",
        "/opt/homebrew/bin/ast-grep",
        "/usr/local/bin/sg",
        "/usr/local/bin/ast-grep",
        "/usr/bin/sg",
    ];

    for name in candidates {
        let path = std::path::PathBuf::from(name);
        if path.is_file() {
            return Some(path);
        }
        if name.starts_with('/') {
            continue;
        }
        // PATH lookup.
        if let Ok(output) = std::process::Command::new("which").arg(name).output() {
            if output.status.success() {
                let found = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let found_path = std::path::PathBuf::from(&found);
                if found_path.is_file() {
                    return Some(found_path);
                }
            }
        }
    }
    None
}

/// Parse compact JSON output from ast-grep.
/// Note: ast-grep uses 0-based line and column numbers; we add 1 before display.
#[derive(serde::Deserialize)]
struct AstGrepMatch {
    #[serde(rename = "file")]
    file: String,
    #[serde(rename = "lines")]
    lines: String,
    #[serde(rename = "range")]
    range: AstGrepRange,
}

#[derive(serde::Deserialize)]
struct AstGrepRange {
    start: AstGrepPos,
}

#[derive(serde::Deserialize)]
struct AstGrepPos {
    line: usize,
    column: usize,
}

pub(crate) fn execute<'a>(
    args: &'a Value,
    env: &'a Environment,
) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
    Box::pin(async move {
        let pattern = args["pattern"]
            .as_str()
            .ok_or("missing required parameter 'pattern'")?;

        let lang = args["lang"]
            .as_str()
            .ok_or("missing required parameter 'lang' (ast mode requires a language)")?;

        let search_path = if let Some(path) = args["path"].as_str() {
            resolve_path(path, &env.cwd)
        } else {
            env.cwd.clone()
        };

        let context = args["context"].as_u64().unwrap_or(0);

        let sg = tokio::task::spawn_blocking(find_sg_binary)
            .await
            .map_err(|e| format!("find_sg_binary panicked: {e}"))?
            .ok_or(
                "ast-grep CLI not found. Install it with: brew install ast-grep, or cargo install ast-grep --locked",
            )?;

        let mut cmd = tokio::process::Command::new(&sg);
        cmd.args(["run", "-p", pattern, "--lang", lang, "--json=compact"]);

        if context > 0 {
            cmd.arg(format!("-C{context}"));
        }

        cmd.arg(search_path.to_string_lossy().as_ref());

        let output = tokio::time::timeout(std::time::Duration::from_secs(60), cmd.output())
            .await
            .map_err(|_| "ast-grep timed out after 60 seconds".to_string())?
            .map_err(|e| format!("failed to spawn ast-grep: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("ast-grep failed: {}", stderr.trim()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.trim().is_empty() {
            return Ok(ToolResult {
                call_id: String::new(),
                content: format!(
                    "No matches found.\n\nCheck:\n  - Pattern must be a complete AST node.\n  - Python: no trailing colons (use `def foo($$$)` not `def foo($$$):`).\n  - Functions: include params and body (use `function $N($$$) {{ $$$ }}` not `function $N`)."
                ),
                title: Some(format!("ast {lang}: {pattern}")),
            });
        }

        let matches: Vec<AstGrepMatch> = serde_json::from_str(&stdout)
            .map_err(|e| format!("failed to parse ast-grep output: {e}"))?;

        let total = matches.len();
        let matches: Vec<_> = matches.into_iter().take(MAX_AST_RESULTS).collect();
        let truncated = total > MAX_AST_RESULTS;

        let mut output = String::new();
        output.push_str(&format!(
            "Found {} match{}\n\n",
            matches.len(),
            if matches.len() == 1 { "" } else { "es" }
        ));

        for m in &matches {
            // ast-grep uses 0-based indexing; convert to 1-based for display.
            output.push_str(&format!(
                "{}:{}:{}:{}\n",
                m.file,
                m.range.start.line + 1,
                m.range.start.column + 1,
                m.lines.trim_end()
            ));
        }

        if truncated {
            output.push_str(&format!(
                "\n(truncated at {MAX_AST_RESULTS} of {total} matches — narrow the search)\n"
            ));
        }

        Ok(ToolResult {
            call_id: String::new(),
            content: output,
            title: Some(format!("ast {lang}: {pattern}")),
        })
    })
}
