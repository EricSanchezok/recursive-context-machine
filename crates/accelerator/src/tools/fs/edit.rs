//! Edit tool — fuzzy-matching string replacement with 9 replacer strategies.
//!
//! The edit tool performs in-place text replacement in files. When an exact match
//! can't be found, it tries a cascade of fuzzy-matching replacers that handle
//! whitespace variation, indentation differences, escape sequences, and boundary
//! trimming. If all strategies fail, a Levenshtein-based similarity hint suggests
//! the closest lines in the file.
//!
//! ## Replacer cascade (tried in order)
//!
//! 1. **SimpleReplacer** — exact string match.
//! 2. **LineTrimmedReplacer** — per-line trimming within a sliding window.
//! 3. **BlockAnchorReplacer** — first/last line anchors with Levenshtein similarity on middle lines.
//! 4. **WhitespaceNormalizedReplacer** — collapse all whitespace to a single space.
//! 5. **IndentationFlexibleReplacer** — strip common leading indentation.
//! 6. **EscapeNormalizedReplacer** — unescape `\n`, `\t`, etc.
//! 7. **TrimmedBoundaryReplacer** — trim leading/trailing whitespace.
//! 8. **ContextAwareReplacer** — anchor-based with ≥50% middle-line match threshold.
//! 9. **MultiOccurrenceReplacer** — literal oldString; triggers multi-match error when matches are ambiguous.

use std::pin::Pin;

use machine::{Environment, ToolResult};
use serde_json::Value;

use super::{guard, relative_path, resolve_path};

// ── Replacer type ──────────────────────────────────────────────────────────

/// A replacer yields candidate substrings from `content` that could substitute
/// for `find`. Returns an empty vec when no candidates are available.
type Replacer = fn(content: &str, find: &str) -> Vec<String>;

/// Ordered replacer cascade — tried in sequence until one succeeds.
static REPLACERS: &[(&str, Replacer)] = &[
    ("simple", simple_replacer),
    ("line_trimmed", line_trimmed_replacer),
    ("block_anchor", block_anchor_replacer),
    ("whitespace_normalized", whitespace_normalized_replacer),
    ("indentation_flexible", indentation_flexible_replacer),
    ("escape_normalized", escape_normalized_replacer),
    ("trimmed_boundary", trimmed_boundary_replacer),
    ("context_aware", context_aware_replacer),
    ("multi_occurrence", multi_occurrence_replacer),
];

// ── Levenshtein distance ───────────────────────────────────────────────────

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut prev: Vec<usize> = (0..=b_len).collect();
    let mut curr = vec![0; b_len + 1];

    for i in 1..=a_len {
        curr[0] = i;
        for j in 1..=b_len {
            let cost = usize::from(a_chars[i - 1] != b_chars[j - 1]);
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[b_len]
}

fn levenshtein_similarity(a: &str, b: &str) -> f64 {
    let max_len = a.len().max(b.len());
    if max_len == 0 {
        return 1.0;
    }
    1.0 - (levenshtein_distance(a, b) as f64 / max_len as f64)
}

fn average_levenshtein_similarity(find_lines: &[&str], content_lines: &[&str]) -> f64 {
    if find_lines.is_empty() {
        return 0.0;
    }
    let total: f64 = find_lines
        .iter()
        .zip(content_lines.iter())
        .map(|(find_line, content_line)| {
            levenshtein_similarity(find_line.trim(), content_line.trim())
        })
        .sum();
    total / find_lines.len() as f64
}

// ── Match helpers ──────────────────────────────────────────────────────────

fn find_all_occurrences(haystack: &str, needle: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut start = 0;
    while let Some(pos) = haystack[start..].find(needle) {
        positions.push(start + pos);
        start += pos + needle.len();
    }
    positions
}

fn line_number_at(content: &str, byte_pos: usize) -> usize {
    content[..byte_pos].lines().count() + 1
}

// ── Indentation helpers ────────────────────────────────────────────────────

fn lines_differ_only_by_indentation(a: &str, b: &str) -> bool {
    let a_lines: Vec<&str> = a.lines().collect();
    let b_lines: Vec<&str> = b.lines().collect();
    if a_lines.len() != b_lines.len() {
        return false;
    }
    a_lines
        .iter()
        .zip(&b_lines)
        .all(|(la, lb)| la.trim() == lb.trim())
}

fn leading_indent(line: &str) -> &str {
    let trimmed_start = line.len() - line.trim_start().len();
    &line[..trimmed_start]
}

fn adjust_indentation(matched: &str, new: &str) -> String {
    let matched_lines: Vec<&str> = matched.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();

    let matched_indents: Vec<&str> = matched_lines
        .iter()
        .map(|line| leading_indent(line))
        .collect();

    let mut result = String::new();
    for (i, new_line) in new_lines.iter().enumerate() {
        if i > 0 {
            result.push('\n');
        }
        let trimmed = new_line.trim_start();
        let indent = matched_indents
            .get(i)
            .copied()
            .or_else(|| matched_indents.last().copied())
            .unwrap_or("");
        result.push_str(indent);
        result.push_str(trimmed);
    }
    result
}

fn min_indent(lines: &[&str]) -> usize {
    lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0)
}

fn strip_common_indent(lines: &[&str]) -> String {
    let indent = min_indent(lines);
    lines
        .iter()
        .map(|l| if l.len() >= indent { &l[indent..] } else { l })
        .collect::<Vec<&str>>()
        .join("\n")
}

// ── Normalization helpers ──────────────────────────────────────────────────

fn normalize_whitespace(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut in_whitespace = false;
    for ch in s.chars() {
        if ch.is_whitespace() {
            if !in_whitespace {
                result.push(' ');
                in_whitespace = true;
            }
        } else {
            result.push(ch);
            in_whitespace = false;
        }
    }
    result.trim().to_string()
}

fn unescape(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('\'') => result.push('\''),
                Some('"') => result.push('"'),
                Some('`') => result.push('`'),
                Some('\\') => result.push('\\'),
                Some('$') => result.push('$'),
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }
    result
}

// ── 1. SimpleReplacer ──────────────────────────────────────────────────────

/// Yields the exact `find` string when it exists verbatim in content.
fn simple_replacer(content: &str, find: &str) -> Vec<String> {
    if content.contains(find) {
        vec![find.to_string()]
    } else {
        vec![]
    }
}

// ── 2. LineTrimmedReplacer ─────────────────────────────────────────────────

/// Splits content and find into lines. For each window of `find.len()` lines,
/// checks whether all lines match when both sides are trimmed. Returns the
/// actual matching substrings from content.
fn line_trimmed_replacer(content: &str, find: &str) -> Vec<String> {
    let find_lines: Vec<&str> = find.lines().collect();
    let content_lines: Vec<&str> = content.lines().collect();

    if find_lines.is_empty() || find_lines.len() > content_lines.len() {
        return vec![];
    }

    let mut results = Vec::new();
    for start in 0..=content_lines.len() - find_lines.len() {
        let all_match = find_lines
            .iter()
            .enumerate()
            .all(|(i, find_line)| find_line.trim() == content_lines[start + i].trim());
        if all_match {
            let matched = content_lines[start..start + find_lines.len()].join("\n");
            results.push(matched);
        }
    }
    results
}

// ── 3. BlockAnchorReplacer ─────────────────────────────────────────────────

/// Requires ≥3 find lines. Uses the first and last (trimmed) lines as anchors.
/// For each candidate block found in content, computes Levenshtein similarity
/// on the middle lines and accepts candidates with similarity ≥0.3.
///
/// Unlike simpler strategies, the end anchor is searched at any position
/// after the start anchor (not necessarily at a fixed offset). This means
/// the matched block can have a different number of lines than the find
/// string — similarity is evaluated over min(both_middle_counts) lines.
fn block_anchor_replacer(content: &str, find: &str) -> Vec<String> {
    let find_lines: Vec<&str> = find.lines().collect();
    if find_lines.len() < 3 {
        return vec![];
    }

    // Strip trailing empty line from find (common when copying multi-line text)
    let find_lines = if find_lines.last().copied() == Some("") {
        &find_lines[..find_lines.len() - 1]
    } else {
        find_lines.as_slice()
    };
    if find_lines.len() < 3 {
        return vec![];
    }

    let content_lines: Vec<&str> = content.lines().collect();
    let first = find_lines[0].trim();
    let last = find_lines[find_lines.len() - 1].trim();
    let find_block_size = find_lines.len();

    // Collect candidates where both anchors match in content.
    // The end anchor is searched after the start anchor + 2 lines.
    let mut candidates: Vec<(usize, usize)> = Vec::new();
    for i in 0..content_lines.len() {
        if content_lines[i].trim() != first {
            continue;
        }
        for j in (i + 2)..content_lines.len() {
            if content_lines[j].trim() == last {
                candidates.push((i, j));
                break; // first occurrence of last anchor after this start
            }
        }
    }

    if candidates.is_empty() {
        return vec![];
    }

    /// Evaluate similarity between find middle lines and content middle lines.
    /// Only compares up to `min(find_middle, content_middle)` lines.
    fn evaluate(
        content_lines: &[&str],
        find_lines: &[&str],
        start: usize,
        end: usize,
        find_block_size: usize,
    ) -> f64 {
        let actual_size = end - start + 1;
        let lines_to_check = std::cmp::min(find_block_size - 2, actual_size - 2);
        if lines_to_check == 0 {
            return 1.0;
        }
        let mut sum = 0.0_f64;
        let max_j = std::cmp::min(find_block_size - 1, actual_size - 1);
        for j in 1..max_j {
            let cl = content_lines[start + j].trim();
            let fl = find_lines[j].trim();
            let max_len = std::cmp::max(cl.len(), fl.len());
            if max_len == 0 {
                continue;
            }
            let sim = 1.0 - levenshtein_distance(cl, fl) as f64 / max_len as f64;
            sum += sim / lines_to_check as f64;
        }
        sum
    }

    // Single candidate: accept if similarity ≥ 0.3
    if candidates.len() == 1 {
        let (start, end) = candidates[0];
        let sim = evaluate(&content_lines, find_lines, start, end, find_block_size);
        if sim >= 0.3 {
            return vec![content_lines[start..=end].join("\n")];
        }
        return vec![];
    }

    // Multiple candidates: pick the best with highest similarity
    let mut best: Option<(usize, usize, f64)> = None;
    for &(start, end) in &candidates {
        let sim = evaluate(&content_lines, find_lines, start, end, find_block_size);
        if sim >= 0.3 {
            match best {
                Some((_, _, bs)) if sim <= bs => {}
                _ => best = Some((start, end, sim)),
            }
        }
    }

    if let Some((start, end, _)) = best {
        vec![content_lines[start..=end].join("\n")]
    } else {
        vec![]
    }
}

// ── 4. WhitespaceNormalizedReplacer ────────────────────────────────────────

/// Collapses all whitespace runs to a single space and trims. For single-line
/// find, compares each content line after normalization. For multi-line find,
/// normalizes entire blocks and compares.
fn whitespace_normalized_replacer(content: &str, find: &str) -> Vec<String> {
    let normalized_find = normalize_whitespace(find);
    let find_lines: Vec<&str> = find.lines().collect();
    let content_lines: Vec<&str> = content.lines().collect();

    let mut results = Vec::new();

    if find_lines.len() == 1 {
        for line in &content_lines {
            if normalize_whitespace(line) == normalized_find {
                results.push(line.to_string());
            }
        }
    } else if find_lines.len() <= content_lines.len() {
        for start in 0..=content_lines.len() - find_lines.len() {
            let block = content_lines[start..start + find_lines.len()].join("\n");
            if normalize_whitespace(&block) == normalized_find {
                results.push(block);
            }
        }
    }

    results
}

// ── 5. IndentationFlexibleReplacer ─────────────────────────────────────────

/// Strips the common leading indentation from the find string (minimum indent
/// across non-empty lines). Searches content for blocks whose de-indented form
/// matches. This handles cases where the LLM pasted more or less indentation
/// than the actual file has.
fn indentation_flexible_replacer(content: &str, find: &str) -> Vec<String> {
    let find_lines: Vec<&str> = find.lines().collect();
    if find_lines.is_empty() {
        return vec![];
    }
    let dedented_find = strip_common_indent(&find_lines);
    let content_lines: Vec<&str> = content.lines().collect();

    let mut results = Vec::new();
    for start in 0..=content_lines.len().saturating_sub(find_lines.len()) {
        let window = &content_lines[start..start + find_lines.len()];
        if strip_common_indent(window) == dedented_find {
            results.push(window.join("\n"));
        }
    }
    results
}

// ── 6. EscapeNormalizedReplacer ────────────────────────────────────────────

/// Unescapes common escape sequences (`\n` → newline, `\t` → tab, etc.) in
/// both the find string and content. Attempts two strategies: matching the
/// unescaped find against raw content, and matching per-line after unescaping
/// both sides.
fn escape_normalized_replacer(content: &str, find: &str) -> Vec<String> {
    let unescaped_find = unescape(find);
    if unescaped_find == find {
        return vec![];
    }

    let mut results = Vec::new();

    // Strategy 1: unescaped find matches raw content verbatim
    if content.contains(&unescaped_find) {
        results.push(unescaped_find.clone());
    }

    // Strategy 2: unescape both sides per-line and compare
    let find_lines: Vec<&str> = find.lines().collect();
    let content_lines: Vec<&str> = content.lines().collect();
    let content_unescaped: Vec<String> = content_lines.iter().map(|line| unescape(line)).collect();

    if find_lines.len() <= content_lines.len() {
        for start in 0..=content_lines.len() - find_lines.len() {
            let block_matches = find_lines
                .iter()
                .enumerate()
                .all(|(i, find_line)| unescape(find_line) == content_unescaped[start + i]);
            if block_matches {
                results.push(content_lines[start..start + find_lines.len()].join("\n"));
            }
        }
    }

    results
}

// ── 7. TrimmedBoundaryReplacer ─────────────────────────────────────────────

/// If the find string has leading or trailing whitespace that differs from the
/// actual file content, tries the trimmed version. Also checks whether any
/// content block's trimmed form matches the find string's trimmed form.
fn trimmed_boundary_replacer(content: &str, find: &str) -> Vec<String> {
    let trimmed = find.trim();
    if trimmed == find {
        return vec![];
    }

    let mut results = Vec::new();

    if content.contains(trimmed) {
        results.push(trimmed.to_string());
    }

    let find_lines: Vec<&str> = find.lines().collect();
    let content_lines: Vec<&str> = content.lines().collect();

    if find_lines.len() <= content_lines.len() {
        for start in 0..=content_lines.len() - find_lines.len() {
            let block = content_lines[start..start + find_lines.len()].join("\n");
            if block.trim() == trimmed {
                results.push(block);
            }
        }
    }

    results
}

// ── 8. ContextAwareReplacer ────────────────────────────────────────────────

/// Requires ≥3 lines. Uses the first and last lines as trimmed anchors and
/// checks that at least 50% of the middle lines match (after trimming).
/// Returns the first matching block — useful when the LLM's middle lines
/// have minor differences from the actual file.
fn context_aware_replacer(content: &str, find: &str) -> Vec<String> {
    let find_lines: Vec<&str> = find.lines().collect();
    if find_lines.len() < 3 {
        return vec![];
    }
    let content_lines: Vec<&str> = content.lines().collect();

    let first = find_lines[0].trim();
    let last = find_lines[find_lines.len() - 1].trim();

    for start in 0..content_lines.len() {
        let end = start + find_lines.len() - 1;
        if end >= content_lines.len() {
            break;
        }
        if content_lines[start].trim() != first || content_lines[end].trim() != last {
            continue;
        }

        let middle_len = find_lines.len() - 2;
        if middle_len == 0 {
            continue;
        }

        let matching_middle = (1..find_lines.len() - 1)
            .filter(|&i| content_lines[start + i].trim() == find_lines[i].trim())
            .count();

        if matching_middle as f64 / middle_len as f64 >= 0.5 {
            let matched = content_lines[start..start + find_lines.len()].join("\n");
            return vec![matched];
        }
    }

    vec![]
}

// ── 9. MultiOccurrenceReplacer ─────────────────────────────────────────────

/// Yields the literal oldString. In non-replaceAll mode, if the exact string
/// appears multiple times, the caller skips this replacer and later reports
/// all match locations so the user can provide more surrounding context.
fn multi_occurrence_replacer(_content: &str, find: &str) -> Vec<String> {
    vec![find.to_string()]
}

// ── Error formatting ───────────────────────────────────────────────────────

/// Formats a list of line numbers and trimmed line content when multiple exact
/// matches are found for the literal oldString.
fn format_match_locations(content: &str, positions: &[usize]) -> String {
    let content_lines: Vec<&str> = content.lines().collect();
    let mut lines = vec![
        "Multiple matches found. Provide more surrounding context to disambiguate:\n".to_string(),
    ];
    for &pos in positions {
        let line_num = line_number_at(content, pos);
        let line_text = content_lines.get(line_num.saturating_sub(1)).unwrap_or(&"");
        lines.push(format!("  line {}: {}\n", line_num, line_text.trim()));
    }
    lines.join("")
}

/// Searches for the most similar window of lines in content using a sliding
/// window and per-line Levenshtein similarity. Skips large files (>5000 lines).
/// If the best similarity is <0.3, the hint is suppressed.
fn similar_lines_hint(content: &str, find: &str) -> String {
    let content_lines: Vec<&str> = content.lines().collect();
    let find_lines: Vec<&str> = find.lines().collect();

    if content_lines.len() > 5000 || find_lines.is_empty() {
        return "No match found.".to_string();
    }

    let mut best_start = 0;
    let mut best_similarity = 0.0_f64;

    let max_start = content_lines.len().saturating_sub(find_lines.len());
    for start in 0..=max_start {
        let window = &content_lines[start..start + find_lines.len()];
        let similarity = average_levenshtein_similarity(&find_lines, window);
        if similarity > best_similarity {
            best_similarity = similarity;
            best_start = start;
        }
    }

    if best_similarity < 0.3 {
        return "No match found.".to_string();
    }

    let context_before = 2;
    let context_after = 2;
    let display_start = best_start.saturating_sub(context_before);
    let display_end = (best_start + find_lines.len() + context_after).min(content_lines.len());

    let mut result = String::from("No match found. Similar lines:\n\n");
    for i in display_start..display_end {
        let line_num = i + 1; // 1-based
        let in_match = i >= best_start && i < best_start + find_lines.len();
        let marker = if in_match { ">" } else { " " };
        result.push_str(&format!(
            "{marker} {:05} | {}\n",
            line_num, content_lines[i]
        ));
    }
    result
}

// ── Replace orchestrator ───────────────────────────────────────────────────

/// Tries each replacer strategy in order. For each candidate, applies the
/// replacement if the candidate appears exactly once (or always, in
/// replaceAll mode). When all strategies fail:
/// - If the literal oldString appears multiple times, reports all match locations.
/// - Otherwise, reports the most similar lines in the file.
fn replace(
    content: &str,
    old_str: &str,
    new_str: &str,
    replace_all: bool,
) -> Result<String, String> {
    // Track exact literal matches for error reporting if all replacers fail.
    let exact_positions = find_all_occurrences(content, old_str);

    for replacer in REPLACERS {
        let candidates = replacer.1(content, old_str);

        for candidate in &candidates {
            let positions = find_all_occurrences(content, candidate);

            if positions.is_empty() {
                continue;
            }

            let final_new = if lines_differ_only_by_indentation(candidate, old_str) {
                adjust_indentation(candidate, new_str)
            } else {
                new_str.to_string()
            };

            if replace_all {
                return Ok(content.replace(candidate, &final_new));
            }

            if positions.len() == 1 {
                let pos = positions[0];
                let mut result = String::with_capacity(content.len() + final_new.len());
                result.push_str(&content[..pos]);
                result.push_str(&final_new);
                result.push_str(&content[pos + candidate.len()..]);
                return Ok(result);
            }
        }
    }

    // All replacers failed. Report either multiple exact matches or a
    // similarity hint.
    if exact_positions.len() > 1 {
        return Err(format_match_locations(content, &exact_positions));
    }

    Err(similar_lines_hint(content, old_str))
}

// ── Entry point ────────────────────────────────────────────────────────────

pub(crate) fn execute<'a>(
    args: &'a Value,
    env: &'a Environment,
) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
    Box::pin(async move {
        let file_path_str = args["filePath"]
            .as_str()
            .ok_or("missing required parameter 'filePath'")?;

        let old_string = args["oldString"]
            .as_str()
            .ok_or("missing required parameter 'oldString'")?;

        let new_string = args["newString"]
            .as_str()
            .ok_or("missing required parameter 'newString'")?;

        let replace_all = args["replaceAll"].as_bool().unwrap_or(false);

        if old_string == new_string {
            return Err("oldString and newString must differ".to_string());
        }

        let resolved = resolve_path(file_path_str, &env.cwd);

        // Guard: existing files must have been read first.
        // (old_string.is_empty() creates a new file, skip guard.)
        if !old_string.is_empty() {
            guard::require_read(env.name.as_str(), &resolved)?;
        }

        if old_string.is_empty() {
            let parent = resolved
                .parent()
                .ok_or_else(|| format!("cannot write to root directory: {}", resolved.display()))?;
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("failed to create parent directory: {e}"))?;
            tokio::fs::write(&resolved, new_string)
                .await
                .map_err(|e| format!("failed to write {}: {e}", resolved.display()))?;
            guard::mark_read(env.name.as_str(), &resolved);
            let title = relative_path(&resolved, &env.cwd);
            let diagnostics = crate::lsp::touch_file(env, &resolved, true).await;
            let mut result = format!("Wrote {}", resolved.display());
            result.push_str(&crate::lsp::format_file_diagnostics(
                &resolved,
                &diagnostics,
            ));
            return Ok(ToolResult {
                call_id: String::new(),
                content: result,
                title: Some(title),
            });
        }

        let content = tokio::fs::read_to_string(&resolved)
            .await
            .map_err(|e| format!("failed to read {}: {e}", resolved.display()))?;

        let _lock = guard::acquire_write_lock(&resolved).await;
        let new_content = replace(&content, old_string, new_string, replace_all)?;

        tokio::fs::write(&resolved, &new_content)
            .await
            .map_err(|e| format!("failed to write {}: {e}", resolved.display()))?;

        guard::mark_read(env.name.as_str(), &resolved);

        let title = relative_path(&resolved, &env.cwd);
        let diagnostics = crate::lsp::touch_file(env, &resolved, true).await;
        let mut result = format!("Successfully modified {}", resolved.display());
        result.push_str(&crate::lsp::format_file_diagnostics(
            &resolved,
            &diagnostics,
        ));
        Ok(ToolResult {
            call_id: String::new(),
            content: result,
            title: Some(title),
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Levenshtein ──

    #[test]
    fn levenshtein_identical() {
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
    }

    #[test]
    fn levenshtein_empty() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("abc", ""), 3);
        assert_eq!(levenshtein_distance("", "abc"), 3);
    }

    #[test]
    fn levenshtein_substitution() {
        assert_eq!(levenshtein_distance("kitten", "sitten"), 1);
        assert_eq!(levenshtein_distance("flaw", "lawn"), 2);
    }

    #[test]
    fn levenshtein_similarity_perfect() {
        assert!((levenshtein_similarity("abc", "abc") - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn levenshtein_similarity_different() {
        assert!(levenshtein_similarity("abc", "xyz") < 0.5);
    }

    // ── SimpleReplacer ──

    #[test]
    fn simple_replacer_finds_exact() {
        let result = simple_replacer("hello world", "hello");
        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn simple_replacer_no_match() {
        let result = simple_replacer("hello world", "xyz");
        assert!(result.is_empty());
    }

    // ── LineTrimmedReplacer ──

    #[test]
    fn line_trimmed_replacer_collapses_leading_spaces() {
        let content = "  foo\n  bar";
        let find = "foo\nbar";
        let result = line_trimmed_replacer(content, find);
        assert!(!result.is_empty());
    }

    #[test]
    fn line_trimmed_replacer_no_match() {
        let content = "foo\nbar";
        let find = "baz\nqux";
        let result = line_trimmed_replacer(content, find);
        assert!(result.is_empty());
    }

    // ── WhitespaceNormalizedReplacer ──

    #[test]
    fn whitespace_normalized_single_line() {
        let content = "hello    world";
        let find = "hello world";
        let result = whitespace_normalized_replacer(content, find);
        assert_eq!(result, vec!["hello    world"]);
    }

    // ── IndentationFlexibleReplacer ──

    #[test]
    fn indentation_flexible_replacer_strips_indent() {
        let content = "  foo\n  bar";
        let find = "    foo\n    bar";
        let result = indentation_flexible_replacer(content, find);
        assert!(!result.is_empty());
    }

    // ── EscapeNormalizedReplacer ──

    #[test]
    fn escape_normalized_replacer_newline() {
        let content = "foo\nbar";
        let find = "foo\\nbar";
        let result = escape_normalized_replacer(content, find);
        assert!(!result.is_empty());
    }

    #[test]
    fn escape_normalized_no_escapes_is_noop() {
        let content = "foo bar";
        let find = "foo bar";
        let result = escape_normalized_replacer(content, find);
        assert!(result.is_empty());
    }

    // ── TrimmedBoundaryReplacer ──

    #[test]
    fn trimmed_boundary_replacer_handles_leading_whitespace() {
        let content = "foo";
        let find = "  foo  ";
        let result = trimmed_boundary_replacer(content, find);
        assert!(!result.is_empty());
    }

    #[test]
    fn trimmed_boundary_noop_when_already_trimmed() {
        let content = "foo";
        let find = "foo";
        let result = trimmed_boundary_replacer(content, find);
        assert!(result.is_empty());
    }

    // ── ContextAwareReplacer ──

    #[test]
    fn context_aware_replacer_matches_with_anchors() {
        let content = "pub struct Foo {\n    field: i32,\n    other: bool,\n}";
        let find = "pub struct Foo {\n    field: i32,\n    other: String,\n}";
        let result = context_aware_replacer(content, find);
        assert!(!result.is_empty());
    }

    #[test]
    fn context_aware_too_few_lines() {
        let result = context_aware_replacer("a\nb", "a\nb");
        assert!(result.is_empty());
    }

    // ── MultiOccurrenceReplacer ──

    #[test]
    fn multi_occurrence_replacer_always_yields() {
        let result = multi_occurrence_replacer("content", "find");
        assert_eq!(result, vec!["find"]);
    }

    // ── replace orchestrator ──

    #[test]
    fn replace_single_exact() {
        let result = replace("hello world", "hello", "hi", false);
        assert_eq!(result, Ok("hi world".to_string()));
    }

    #[test]
    fn replace_all() {
        let result = replace("x x x", "x", "y", true);
        assert_eq!(result, Ok("y y y".to_string()));
    }

    #[test]
    fn replace_all_non_replace_all_multi_skips() {
        let result = replace("x x x", "x", "y", false);
        assert!(result.is_err());
    }

    #[test]
    fn replace_trimmed_match() {
        let result = replace("  foo\n  bar", "foo\nbar", "baz\nqux", false);
        assert_eq!(result, Ok("  baz\n  qux".to_string()));
    }

    #[test]
    fn replace_no_match() {
        let result = replace("hello", "xyz", "abc", false);
        assert!(result.is_err());
    }

    // ── Indentation adjustment ──

    #[test]
    fn adjust_indentation_preserves_content_indent() {
        let matched = "  foo\n  bar";
        let new = "baz\nqux";
        let result = adjust_indentation(matched, new);
        assert_eq!(result, "  baz\n  qux");
    }

    #[test]
    fn adjust_indentation_extra_new_lines() {
        let matched = "  foo";
        let new = "baz\nqux\nzot";
        let result = adjust_indentation(matched, new);
        assert_eq!(result, "  baz\n  qux\n  zot");
    }

    // ── Norm helpers ──

    #[test]
    fn normalize_whitespace_collapses_runs() {
        assert_eq!(normalize_whitespace("a   b\t\tc"), "a b c");
    }

    #[test]
    fn unescape_handles_all_sequences() {
        assert_eq!(unescape("a\\nb\\tc"), "a\nb\tc");
        assert_eq!(unescape("\\'\\\"\\`\\\\\\$"), "'\"`\\$");
    }

    // ── find_all_occurrences ──

    #[test]
    fn find_all_occurrences_basic() {
        assert_eq!(find_all_occurrences("x x x", "x"), vec![0, 2, 4]);
    }

    #[test]
    fn find_all_occurrences_none() {
        assert_eq!(find_all_occurrences("hello", "x").len(), 0);
    }

    // ── line_number_at ──

    #[test]
    fn line_number_at_start_of_line() {
        let content = "line1\nline2\nline3";
        assert_eq!(line_number_at(content, 0), 1);
        assert_eq!(line_number_at(content, 6), 2);
    }

    // ── min_indent / strip_common_indent ──

    #[test]
    fn min_indent_skips_empty() {
        let lines = ["  foo", "", "    bar"];
        assert_eq!(min_indent(&lines), 2);
    }

    #[test]
    fn strip_common_indent_basic() {
        let lines = ["    foo", "    bar"];
        assert_eq!(strip_common_indent(&lines), "foo\nbar");
    }
}
