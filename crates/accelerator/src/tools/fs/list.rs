use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use machine::{Environment, ToolResult};
use serde_json::Value;

use super::MAX_LIST_ENTRIES;

pub(crate) fn execute<'a>(
    args: &'a Value,
    env: &'a Environment,
) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
    Box::pin(async move {
        let raw = args["filePath"]
            .as_str()
            .ok_or("missing required parameter 'filePath'")?;

        let resolved = if PathBuf::from(raw).is_absolute() {
            PathBuf::from(raw)
        } else {
            env.cwd.join(raw)
        };

        let metadata = tokio::fs::metadata(&resolved)
            .await
            .map_err(|error| format!("cannot access '{}': {error}", raw))?;

        if !metadata.is_dir() {
            return Err(format!("not a directory: '{}'", raw));
        }

        let relative = if resolved.starts_with(&env.cwd) {
            resolved
                .strip_prefix(&env.cwd)
                .unwrap()
                .to_string_lossy()
                .to_string()
        } else {
            resolved.to_string_lossy().to_string()
        };

        let mut reader = tokio::fs::read_dir(&resolved)
            .await
            .map_err(|error| format!("cannot read directory '{}': {error}", raw))?;

        let mut dirs: Vec<String> = Vec::new();
        let mut files: Vec<String> = Vec::new();

        while let Some(entry) = reader
            .next_entry()
            .await
            .map_err(|error| format!("error reading directory '{}': {error}", raw))?
        {
            let name = entry.file_name().to_string_lossy().to_string();
            let file_type = entry
                .file_type()
                .await
                .map_err(|error| format!("error reading entry type for '{name}': {error}"))?;

            if file_type.is_dir() {
                dirs.push(name);
            } else {
                files.push(name);
            }
        }

        dirs.sort();
        files.sort();

        if dirs.is_empty() && files.is_empty() {
            return Ok(ToolResult {
                call_id: String::new(),
                content: "Directory is empty.".to_string(),
                title: Some(format!("list {relative}")),
            });
        }

        let total = dirs.len() + files.len();
        let omitted = total.saturating_sub(MAX_LIST_ENTRIES);

        let mut output = String::with_capacity(total * 64);

        for name in dirs.iter().take(MAX_LIST_ENTRIES) {
            output.push_str(&format!("📁 {}/\n", name));
        }
        for name in files
            .iter()
            .take(MAX_LIST_ENTRIES.saturating_sub(dirs.len()))
        {
            output.push_str(&format!("📄 {}\n", name));
        }

        if omitted > 0 {
            output.push_str(&format!(
                "\n[{} entr{} omitted — showing first {} of {}]\n",
                omitted,
                if omitted == 1 { "y" } else { "ies" },
                MAX_LIST_ENTRIES,
                total
            ));
        }

        Ok(ToolResult {
            call_id: String::new(),
            content: output,
            title: Some(format!("list {relative}")),
        })
    })
}
