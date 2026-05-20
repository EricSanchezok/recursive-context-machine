use std::pin::Pin;

use machine::{Environment, ToolResult};
use serde_json::Value;

use super::{guard, relative_path, resolve_path};

pub(crate) fn execute<'a>(
    args: &'a Value,
    env: &'a Environment,
) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
    Box::pin(async move {
        let file_path = args["filePath"]
            .as_str()
            .ok_or("missing required parameter 'filePath'")?;

        let content = args["content"]
            .as_str()
            .ok_or("missing required parameter 'content'")?;

        let path = resolve_path(file_path, &env.cwd);
        let relative = relative_path(&path, &env.cwd);
        let _lock = guard::acquire_write_lock(&path).await;

        // Guard: existing files must have been read first.
        let exists = tokio::fs::try_exists(&path).await.unwrap_or(false);
        if exists {
            guard::require_read(env.name.as_str(), &path)?;
        }

        if tokio::fs::metadata(&path)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
        {
            return Err(format!(
                "Path is a directory, not a file: {path}",
                path = path.display()
            ));
        }

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|error| format!("failed to create parent directory: {error}"))?;
        }

        let before = crate::lsp::snapshot(env, &path).await;

        tokio::fs::write(&path, content)
            .await
            .map_err(|error| format!("failed to write file: {error}"))?;

        guard::mark_read(env.name.as_str(), &path);
        let diagnostics = crate::lsp::touch_file_with_text(env, &path, content, true).await;
        let new_errors = crate::lsp::new_error_diagnostics(&before, &diagnostics);
        let mut result = relative.clone();
        result.push_str(&crate::lsp::format_file_diagnostics(&path, &new_errors));

        Ok(ToolResult {
            call_id: String::new(),
            content: result,
            title: Some(format!("wrote {relative}")),
        })
    })
}
