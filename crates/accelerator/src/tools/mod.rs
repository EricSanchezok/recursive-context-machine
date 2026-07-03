//! Built-in tools for the accelerator.

mod arxiv;
mod find;
mod fs;
mod git;
mod image_gen;
mod lsp;
mod rdc_read;
mod rdc_write;
pub mod shell;
mod spawn;
mod survey_update;
mod wait;
mod webfetch;

pub use arxiv::{ArxivDownloadTool, ArxivSearchTool};
pub use find::FindTool;
pub use fs::FsTool;
pub use git::{GitTool, check_safety as check_git_safety, tokenize as tokenize_git};
pub use image_gen::ImageGenTool;
pub use lsp::LspTool;
pub use rdc_read::RdcReadTool;
pub use rdc_write::RdcWriteTool;
pub use shell::{OUTPUT_CAP_BYTES, ShellTool, build_result, collect_output};
pub use spawn::SpawnTool;
pub use survey_update::SurveyUpdateTool;
pub use wait::WaitTool;
pub use webfetch::WebFetchTool;

use std::path::{Path, PathBuf};

use crate::catalog::Catalog;

pub fn register(catalog: &mut Catalog) {
    for tool in builtin_tools() {
        catalog
            .register_tool(tool)
            .expect("built-in tool names must be unique");
    }
}

/// All built-in tools.
pub fn builtin_tools() -> Vec<std::sync::Arc<dyn machine::Tool>> {
    vec![
        std::sync::Arc::new(ArxivSearchTool),
        std::sync::Arc::new(ArxivDownloadTool),
        std::sync::Arc::new(FindTool),
        std::sync::Arc::new(FsTool),
        std::sync::Arc::new(GitTool),
        std::sync::Arc::new(ImageGenTool),
        std::sync::Arc::new(LspTool),
        std::sync::Arc::new(RdcReadTool),
        std::sync::Arc::new(RdcWriteTool),
        std::sync::Arc::new(ShellTool),
        std::sync::Arc::new(SurveyUpdateTool),
        std::sync::Arc::new(WaitTool),
        std::sync::Arc::new(WebFetchTool),
    ]
}

/// Resolve a path string against a working directory.
pub(crate) fn resolve_path(raw: &str, cwd: &Path) -> PathBuf {
    let path = Path::new(raw);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}

/// Compute the display path relative to a working directory.
pub(crate) fn relative_path(path: &Path, cwd: &Path) -> String {
    path.strip_prefix(cwd)
        .map(|relative| {
            if relative.as_os_str().is_empty() {
                ".".to_string()
            } else {
                relative.display().to_string()
            }
        })
        .unwrap_or_else(|_| path.display().to_string())
}

// ── URL helpers (shared by rdc_read / rdc_write) ─────────────────────────────

/// Percent-encode a string for use as a query parameter value or URL path
/// segment. Encodes everything outside RFC 3986's unreserved set
/// `[A-Za-z0-9-._~]`.
///
/// For URL path segments that carry user-controlled IDs (entity_id,
/// research_id), prefer [`validate_path_id`] first — it enforces a tighter
/// `[A-Za-z0-9_-]` whitelist that rejects path-traversal payloads like `../`
/// or encoded slashes outright.
pub(crate) fn url_encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            _ => {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    result
}

/// Validate that an entity/research ID is safe to interpolate into a URL path
/// segment. Allows only `[A-Za-z0-9_-]` (non-empty) to prevent path traversal
/// (`../`), query-string injection (`?`, `&`), fragment injection (`#`), and
/// encoded-slash bypasses (`%2F`) from reaching the RDC REST API.
///
/// Returns `Ok(())` when the ID is safe, or an `Err` with a descriptive
/// message suitable for surfacing to the caller.
pub(crate) fn validate_path_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("entity_id must not be empty".to_string());
    }
    if let Some(bad) = id
        .bytes()
        .find(|&b| !b.is_ascii_alphanumeric() && b != b'-' && b != b'_')
    {
        return Err(format!(
            "entity_id contains illegal character {:?}; allowed: [A-Za-z0-9_-]",
            bad as char
        ));
    }
    Ok(())
}
/// Map an entity_type (singular or plural) to its REST plural path segment.
///
/// Shared by `rdc_read` and `rdc_write` so the two tools always agree on URL
/// construction. Previously each tool had its own copy, and they had drifted:
/// `rdc_write` knew about `papers`/`reviews` but not `story_spines`/
/// `gate_records`; `rdc_read` had the opposite gap. A missing mapping makes
/// the tool fall through to the catch-all arm (`_ => entity_type`), which
/// produces a wrong URL and a silent 404.
pub(crate) fn pluralize(entity_type: &str) -> &str {
    match entity_type {
        "research" => "research",
        "ideas" | "idea" => "ideas",
        "claims" | "claim" => "claims",
        "experiments" | "experiment" => "experiments",
        "papers" | "paper" => "papers",
        "paper_spines" | "paper_spine" => "paper-spines",
        "tech_reports" | "tech_report" => "tech-reports",
        "story_spines" | "story_spine" => "story-spines",
        "positionings" | "positioning" => "positionings",
        "reviews" | "review" => "reviews",
        // RDC exposes gate records at /gates (not /gate_records). Both the
        // canonical /gates route and the /gate_records alias (rdc commit
        // feb7f31) accept these writes; keeping the mapping here means RCM
        // is correct even without the alias.
        "gate_records" | "gate_record" => "gates",
        "lit_papers" | "lit_search" => "literature",
        _ => entity_type,
    }
}

#[cfg(test)]
mod url_tests {
    use super::*;

    #[test]
    fn url_encode_alphanumeric_only() {
        assert_eq!(url_encode("hello123"), "hello123");
    }

    #[test]
    fn url_encode_encodes_space() {
        assert_eq!(url_encode("attention mechanism"), "attention%20mechanism");
    }

    #[test]
    fn url_encode_encodes_special_chars() {
        assert_eq!(url_encode("a&b=c"), "a%26b%3Dc");
    }

    #[test]
    fn url_encode_preserves_unreserved_chars() {
        assert_eq!(url_encode("test-_.~name"), "test-_.~name");
    }

    #[test]
    fn url_encode_empty_string() {
        assert_eq!(url_encode(""), "");
    }

    // ── path-segment encoding (P0-13 regression coverage) ──

    #[test]
    fn url_encode_path_segment_encodes_slash() {
        // A bare slash in an entity_id would escape the path segment; encoding
        // it to %2F is the defense-in-depth layer behind validate_path_id.
        assert_eq!(url_encode("../secret"), "..%2Fsecret");
        assert_eq!(url_encode("a/b"), "a%2Fb");
    }

    #[test]
    fn url_encode_path_segment_encodes_dotdot_only_format() {
        // ".." itself is unreserved-compatible (dots are unreserved), so
        // url_encode leaves it intact. The path-traversal guard is
        // validate_path_id, which rejects the slash that would make ".."
        // dangerous. This test documents that split.
        assert_eq!(url_encode(".."), "..");
    }

    #[test]
    fn url_encode_path_segment_encodes_question_and_hash() {
        assert_eq!(url_encode("a?b=c"), "a%3Fb%3Dc");
        assert_eq!(url_encode("a#b"), "a%23b");
    }

    #[test]
    fn validate_path_id_accepts_safe_ids() {
        // Whitelist is [A-Za-z0-9_-]. Note: '.' is intentionally NOT allowed
        // for path segments (tighter than unreserved) so IDs like 'a.b' are
        // rejected.
        assert!(validate_path_id("idea_001").is_ok());
        assert!(validate_path_id("CL-2024-001").is_ok());
        assert!(validate_path_id("abcDEF012_--").is_ok());
        assert!(validate_path_id("a").is_ok());
    }

    #[test]
    fn validate_path_id_rejects_dot() {
        // '.' is outside the [A-Za-z0-9_-] whitelist for path segments.
        assert!(validate_path_id("a.b").is_err());
    }

    #[test]
    fn validate_path_id_rejects_empty() {
        let err = validate_path_id("").unwrap_err();
        assert!(err.contains("must not be empty"));
    }

    #[test]
    fn validate_path_id_rejects_slash() {
        // 'a/b' contains a slash that would escape the path segment. The
        // first offending byte is '/', reported via Debug formatting as `'/'`.
        let err = validate_path_id("a/b").unwrap_err();
        assert!(err.contains("illegal character"), "got: {err}");
        assert!(
            err.contains("'/'"),
            "expected the slash to be quoted in: {err}"
        );
    }

    #[test]
    fn validate_path_id_rejects_encoded_slash() {
        // %2F must be rejected character-by-character (% is not allowed).
        let err = validate_path_id("a%2Fb").unwrap_err();
        assert!(err.contains("illegal character"));
    }

    #[test]
    fn validate_path_id_rejects_space_and_query_chars() {
        assert!(validate_path_id("a b").is_err());
        assert!(validate_path_id("a?b").is_err());
        assert!(validate_path_id("a&b").is_err());
        assert!(validate_path_id("a#b").is_err());
    }

    #[test]
    fn validate_path_id_rejects_unicode() {
        // Non-ASCII must be rejected; the RDC ID space is ASCII-only.
        assert!(validate_path_id("café").is_err());
        assert!(validate_path_id("id-é").is_err());
    }
}
