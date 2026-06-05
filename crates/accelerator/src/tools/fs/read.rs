use std::path::Path;
use std::pin::Pin;

use machine::{Environment, ToolResult};
use serde_json::Value;

use super::{
    DEFAULT_READ_LIMIT, MAX_LINE_LENGTH, OUTPUT_CAP_BYTES, guard, relative_path, resolve_path,
};

use chrono::{TimeZone, Utc};

/// File extensions known to be binary — no content is readable.
const BINARY_EXTENSIONS: &[&str] = &[
    "zip", "tar", "gz", "exe", "dll", "so", "class", "jar", "war", "7z", "doc", "xls", "ppt",
    "odt", "ods", "odp", "bin", "dat", "obj", "o", "a", "lib", "wasm", "pyc", "pyo",
];

/// Document extensions requiring external extraction.
const DOCUMENT_EXTENSIONS: &[&str] = &["pdf", "docx", "xlsx", "pptx"];

/// Leading bytes sampled for binary-content detection.
const BINARY_SAMPLE: usize = 4096;

pub(crate) fn execute<'a>(
    args: &'a Value,
    env: &'a Environment,
) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
    Box::pin(async move {
        let raw_path = args["filePath"]
            .as_str()
            .ok_or("missing required parameter 'filePath'")?;

        let resolved = resolve_path(raw_path, &env.cwd);

        // Stat the file (handles not-found with suggestions).
        let metadata = match tokio::fs::metadata(&resolved).await {
            Ok(m) => m,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Err(build_not_found_message(&resolved).await);
            }
            Err(error) => {
                return Err(format!("cannot stat '{}': {}", resolved.display(), error));
            }
        };

        if !metadata.is_file() {
            return Err(format!("'{}' is not a regular file", resolved.display()));
        }

        // Classification by extension.
        if let Some(ext) = resolved.extension().and_then(|e| e.to_str()) {
            let lower = ext.to_ascii_lowercase();
            if BINARY_EXTENSIONS.contains(&lower.as_str()) {
                return Err(format!("Cannot read binary file: {}", resolved.display()));
            }
            if lower == "pdf" {
                // PDFs are read by extracting text with a safe pdftotext wrapper
                // (no shell is exposed to the model).
                let text = super::pdf::extract_pdf_text(&resolved).await?;
                let offset = args["offset"].as_u64().unwrap_or(0) as usize;
                let limit = args["limit"].as_u64().unwrap_or(DEFAULT_READ_LIMIT as u64) as usize;
                let path_str = relative_path(&resolved, &env.cwd);
                let mut output = stat_header(&metadata, &path_str);
                output.push('\n');
                output.push_str(&format_lines(&text, offset, limit, &path_str));
                guard::mark_read(env.name.as_str(), &resolved);
                return Ok(ToolResult {
                    call_id: String::new(),
                    content: output,
                    title: Some(path_str),
                });
            }
            if DOCUMENT_EXTENSIONS.contains(&lower.as_str()) {
                return Err(format!(
                    "Cannot read document file: {}. {} files require external extraction.",
                    resolved.display(),
                    ext.to_uppercase()
                ));
            }
        }

        let raw = tokio::fs::read(&resolved)
            .await
            .map_err(|error| format!("cannot read '{}': {}", resolved.display(), error))?;

        if is_binary_content(&raw) {
            return Err(format!("Cannot read binary file: {}", resolved.display()));
        }

        let text = String::from_utf8(raw).map_err(|_| {
            format!(
                "Cannot read '{}': file is not valid UTF-8 text",
                resolved.display()
            )
        })?;

        let offset = args["offset"].as_u64().unwrap_or(0) as usize;
        let limit = args["limit"].as_u64().unwrap_or(DEFAULT_READ_LIMIT as u64) as usize;

        let path_str = relative_path(&resolved, &env.cwd);
        let mut output = stat_header(&metadata, &path_str);
        output.push('\n');
        let content = format_lines(&text, offset, limit, &path_str);
        output.push_str(&content);

        guard::mark_read(env.name.as_str(), &resolved);
        let lsp_env = env.clone();
        let lsp_path = resolved.clone();
        tokio::spawn(async move {
            crate::lsp::touch_file_from_disk(&lsp_env, &lsp_path, false).await;
        });

        Ok(ToolResult {
            call_id: String::new(),
            content: output,
            title: Some(path_str),
        })
    })
}

/// True when the byte slice looks like binary (null bytes or >30% ASCII
/// control characters in the leading sample).
fn is_binary_content(bytes: &[u8]) -> bool {
    let sample = &bytes[..bytes.len().min(BINARY_SAMPLE)];
    if sample.is_empty() {
        return false;
    }

    let mut non_printable = 0usize;
    for &byte in sample {
        if byte == 0 {
            return true;
        }
        // Control chars excluding tab (9), lf (10), cr (13).
        if byte < 9 || (byte > 13 && byte < 32) {
            non_printable += 1;
        }
    }

    non_printable * 100 > sample.len() * 30
}

/// Scan the parent directory and build a "Did you mean?" error message.
async fn build_not_found_message(resolved: &Path) -> String {
    let mut message = format!("File not found: '{}'", resolved.display());

    let (Some(parent), Some(name)) = (resolved.parent(), resolved.file_name()) else {
        return message;
    };
    let name_str = name.to_string_lossy().to_ascii_lowercase();

    let mut entries = match tokio::fs::read_dir(parent).await {
        Ok(e) => e,
        Err(_) => return message,
    };

    let mut suggestions: Vec<String> = Vec::new();
    loop {
        let entry = match entries.next_entry().await {
            Ok(Some(e)) => e,
            _ => break,
        };
        let entry_lower = entry.file_name().to_string_lossy().to_ascii_lowercase();
        if entry_lower.contains(&name_str) && entry_lower != name_str {
            suggestions.push(entry.file_name().to_string_lossy().to_string());
        }
        if suggestions.len() >= 3 {
            break;
        }
    }

    if !suggestions.is_empty() {
        message.push_str("\nDid you mean?");
        for suggestion in &suggestions {
            message.push_str(&format!("\n  - {}", suggestion));
        }
    }

    message
}

/// Build the stat header for a file.
fn stat_header(metadata: &std::fs::Metadata, title: &str) -> String {
    let file_type = if metadata.is_dir() {
        "directory"
    } else if metadata.is_symlink() {
        "symlink"
    } else {
        "file"
    };
    let bytes = metadata.len();
    let size = if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    };
    let modified = metadata.modified().ok().map(|t| {
        let dur = t.duration_since(std::time::UNIX_EPOCH).ok();
        dur.and_then(|d| {
            Utc.timestamp_opt(d.as_secs() as i64, d.subsec_nanos())
                .single()
        })
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| "unknown".to_string())
    });
    let modified = modified.as_deref().unwrap_or("unknown");

    format!(
        "Path: {title}\nType: {file_type}\nSize: {size} ({bytes} bytes)\nModified: {modified}\n\n"
    )
}

/// Build the `<file>`-wrapped line-numbered output.
fn format_lines(text: &str, offset: usize, limit: usize, title: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let total = lines.len();

    let mut output = format!("<file path=\"{}\">\n", title);

    if offset >= total {
        output.push_str(&format!("\n(End of file - total {} lines)\n</file>", total));
        return output;
    }

    // Account for header bytes in the output cap.
    let mut byte_count: usize = output.len();

    let end = (offset + limit).min(total);

    for (index, line) in lines[offset..end].iter().enumerate() {
        let line_num = offset + index + 1;
        let content = truncate_line(line, MAX_LINE_LENGTH);

        let formatted = format!("{:05}| {}\n", line_num, content);
        byte_count += formatted.len();

        if byte_count > OUTPUT_CAP_BYTES {
            output.push_str(&format!(
                "\n(Output truncated at {} bytes. Use 'offset' and 'limit' to narrow the range.)\n</file>",
                OUTPUT_CAP_BYTES
            ));
            return output;
        }

        output.push_str(&formatted);
    }

    if end < total {
        output.push_str(&format!(
            "\n(File has more lines. Use 'offset' parameter to read beyond line {})\n</file>",
            end
        ));
    } else {
        output.push_str(&format!("\n(End of file - total {} lines)\n</file>", total));
    }

    output
}

fn truncate_line(line: &str, max_chars: usize) -> String {
    if line.chars().count() <= max_chars {
        return line.to_string();
    }

    let mut truncated: String = line.chars().take(max_chars).collect();
    truncated.push_str("...");
    truncated
}
