use std::future::Future;
use std::path::Path;
use std::pin::Pin;

use chrono::{TimeZone, Utc};
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

        let resolved = if Path::new(file_path).is_absolute() {
            Path::new(file_path).to_path_buf()
        } else {
            env.cwd.join(file_path)
        };

        let metadata = tokio::fs::metadata(&resolved).await.map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                format!("File not found: {}", resolved.display())
            } else {
                format!(
                    "failed to read metadata for {}: {error}",
                    resolved.display()
                )
            }
        })?;

        let relative = if resolved.starts_with(&env.cwd) {
            resolved
                .strip_prefix(&env.cwd)
                .unwrap()
                .to_string_lossy()
                .to_string()
        } else {
            resolved.to_string_lossy().to_string()
        };

        let file_type = if metadata.is_dir() {
            "directory"
        } else if metadata.is_symlink() {
            "symlink"
        } else {
            "file"
        };

        let size_human = human_size(metadata.len());
        let raw_bytes = metadata.len();

        let modified_str = format_modified(metadata.modified().ok());

        let content = format!(
            "Path:  {}\nType:  {}\nSize:  {} ({} bytes)\nModified: {modified_str}",
            relative, file_type, size_human, raw_bytes,
        );

        Ok(ToolResult {
            call_id: String::new(),
            content,
            title: Some(format!("stat {relative}")),
        })
    })
}

fn human_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["bytes", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{size:.1} {}", UNITS[unit_index])
}

fn format_modified(modified: Option<std::time::SystemTime>) -> String {
    let sys_time = match modified {
        Some(st) => st,
        None => return "unknown".to_string(),
    };

    let duration = match sys_time.duration_since(std::time::UNIX_EPOCH) {
        Ok(dur) => dur,
        Err(_) => return "unknown".to_string(),
    };

    match Utc.timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos()) {
        chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        _ => "unknown".to_string(),
    }
}
