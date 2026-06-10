//! Safe PDF text extraction for the `fs` read tool.
//!
//! PDF parsing needs an external tool (`pdftotext`, from poppler-utils). We wrap
//! it safely rather than exposing the raw `shell` tool to the model: the binary
//! is invoked with parameterized arguments via `Command` (never through a shell),
//! so nothing in the path is ever interpreted. As defense in depth we also reject
//! any path containing a shell metacharacter, require a `.pdf` extension, and
//! bound both runtime and output size.

use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use tokio::process::Command;

/// How long `pdftotext` may run before we give up.
const PDFTOTEXT_TIMEOUT: Duration = Duration::from_secs(60);

/// Cap on extracted text, to keep tool output bounded.
const MAX_PDF_TEXT_BYTES: usize = 500_000;

/// Extract text from a local PDF using `pdftotext`. The path must already be a
/// resolved filesystem path (the `fs` tool confines it to the workspace).
pub(crate) async fn extract_pdf_text(path: &Path) -> Result<String, String> {
    let path_str = path.to_str().ok_or("PDF path is not valid UTF-8")?;
    if let Some(bad) = path_str.chars().find(|c| is_shell_metacharacter(*c)) {
        return Err(format!(
            "refusing to extract: PDF path contains an unsafe character {bad:?}"
        ));
    }
    if !path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("pdf"))
    {
        return Err("extract_pdf_text expects a .pdf path".to_string());
    }

    let child = Command::new("pdftotext")
        .arg("-q") // quiet — no progress noise on stderr
        .arg(path) // input path — parameterized, never shell-interpolated
        .arg("-") // write extracted text to stdout
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|error| match error.kind() {
            std::io::ErrorKind::NotFound => {
                "pdftotext is not installed (install poppler-utils to read PDFs)".to_string()
            }
            _ => format!("failed to launch pdftotext: {error}"),
        })?;

    let output = match tokio::time::timeout(PDFTOTEXT_TIMEOUT, child.wait_with_output()).await {
        Err(_) => return Err("pdftotext timed out".to_string()),
        Ok(Err(error)) => return Err(format!("pdftotext failed: {error}")),
        Ok(Ok(output)) => output,
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "pdftotext exited unsuccessfully: {}",
            stderr.trim()
        ));
    }

    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    if text.trim().is_empty() {
        return Err(
            "pdftotext produced no text (the PDF may be scanned or image-only)".to_string(),
        );
    }
    if text.len() > MAX_PDF_TEXT_BYTES {
        text.truncate(MAX_PDF_TEXT_BYTES);
        text.push_str("\n\n[... truncated: extracted PDF text exceeded the size cap ...]");
    }
    Ok(text)
}

/// Shell-significant characters refused in a PDF path. The path is never passed
/// through a shell, so this is purely defense in depth; it still allows ordinary
/// path characters (`/`, `.`, `-`, `_`, `:`, spaces, alphanumerics).
fn is_shell_metacharacter(c: char) -> bool {
    matches!(
        c,
        ';' | '|'
            | '&'
            | '$'
            | '`'
            | '('
            | ')'
            | '<'
            | '>'
            | '*'
            | '?'
            | '{'
            | '}'
            | '['
            | ']'
            | '!'
            | '\\'
            | '"'
            | '\''
            | '\n'
            | '\r'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_shell_metacharacters() {
        for bad in [
            "/tmp/a;rm -rf b.pdf",
            "/tmp/$(whoami).pdf",
            "/tmp/a`b`.pdf",
            "/tmp/a|b.pdf",
            "/tmp/a>b.pdf",
            "/tmp/a*.pdf",
        ] {
            assert!(
                bad.chars().any(is_shell_metacharacter),
                "{bad} should be flagged as unsafe"
            );
        }
    }

    #[test]
    fn allows_ordinary_pdf_paths() {
        for ok in [
            "examples/autoresearch-survey/runs/20260603T123058Z/pdfs/2401.12345.pdf",
            "/Users/Jane Doe/RCM/runs/pdfs/hep-th_9901001.pdf",
            "runs/pdfs/2210.03310.pdf",
        ] {
            assert!(
                !ok.chars().any(is_shell_metacharacter),
                "{ok} should be allowed"
            );
        }
    }
}
