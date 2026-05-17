use std::future::Future;
use std::pin::Pin;

use machine::{Environment, ToolResult};
use serde_json::Value;

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
        let relative = path
            .strip_prefix(&env.cwd)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();

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

        tokio::fs::write(&path, content)
            .await
            .map_err(|error| format!("failed to write file: {error}"))?;

        Ok(ToolResult {
            call_id: String::new(),
            content: relative.clone(),
            title: Some(format!("wrote {relative}")),
        })
    })
}

fn resolve_path(raw: &str, cwd: &std::path::Path) -> std::path::PathBuf {
    let candidate = std::path::Path::new(raw);
    if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        cwd.join(candidate)
    }
}
