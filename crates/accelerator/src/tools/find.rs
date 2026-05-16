use std::collections::HashMap;
use std::fs;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;

use machine::{Environment, Tool, ToolResult};
use regex::Regex;
use serde_json::Value;
use tracing::{debug, warn};

// ── Constants ──

const MAX_MATCHES_TOTAL: usize = 500;
const MAX_MATCHES_PER_FILE: usize = 50;
const MAX_FILE_SIZE_BYTES: u64 = 10 * 1024 * 1024;
const PREVIEW_CONTEXT_LINES: usize = 5;
const FULL_FALLBACK_CONTEXT_LINES: usize = 15;
const MAX_BLOCK_SEARCH_LINES: usize = 500;
const OUTPUT_MAX_MATCHES_PER_CATEGORY: usize = 30;

// ── Arguments ──

#[derive(Debug, serde::Deserialize)]
struct FindArgs {
    query: String,
    #[serde(default)]
    dir: Option<String>,
    #[serde(default)]
    include: Option<String>,
    #[serde(default)]
    depth: FindDepth,
}

#[derive(Debug, Default, serde::Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
enum FindDepth {
    #[default]
    Preview,
    Names,
    Full,
}

// ── Internal Types ──

#[derive(Debug, Clone)]
struct RawMatch {
    path: PathBuf,
    line_num: usize,
    line_text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MatchCategory {
    FunctionDefinition,
    StructDefinition,
    ClassDefinition,
    EnumDefinition,
    TraitDefinition,
    InterfaceDefinition,
    ImplBlock,
    TypeAlias,
    Constant,
    StaticValue,
    MacroDefinition,
    ModuleDeclaration,
    Import,
    FunctionCall,
    VariableAssignment,
    FieldAccess,
    Comment,
    StringLiteral,
    Attribute,
    GenericParameter,
    Other,
}

#[derive(Debug, Clone)]
struct ClassifiedMatch {
    path: PathBuf,
    line_num: usize,
    line_text: String,
    category: MatchCategory,
    context: Vec<(usize, String)>,
}

// ── Language Detection ──

#[derive(Clone, Copy, Debug)]
enum Lang {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Tsx,
    Go,
    Java,
    Unknown,
}

impl Lang {
    fn from_ext(ext: &str) -> Self {
        match ext {
            "rs" => Lang::Rust,
            "py" | "pyi" => Lang::Python,
            "js" | "jsx" | "mjs" | "cjs" => Lang::JavaScript,
            "ts" | "cts" | "mts" => Lang::TypeScript,
            "tsx" => Lang::Tsx,
            "go" => Lang::Go,
            "java" => Lang::Java,
            _ => Lang::Unknown,
        }
    }

    fn parser(self) -> Option<tree_sitter::Parser> {
        let mut parser = tree_sitter::Parser::new();
        let language: tree_sitter::Language = match self {
            Lang::Rust => tree_sitter_rust::LANGUAGE.into(),
            Lang::Python => tree_sitter_python::LANGUAGE.into(),
            Lang::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
            Lang::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Lang::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
            Lang::Go => tree_sitter_go::LANGUAGE.into(),
            Lang::Java => tree_sitter_java::LANGUAGE.into(),
            Lang::Unknown => return None,
        };
        parser.set_language(&language).ok()?;
        Some(parser)
    }
}

// ── Tool Implementation ──

pub struct FindTool;

impl Tool for FindTool {
    fn name(&self) -> &str {
        "find"
    }

    fn description(&self) -> &str {
        "Search for text across files in a directory. \
         Returns matches grouped by semantic type — function definitions, call sites, comments, string literals, etc. — \
         with contextual previews. Classification uses AST parsing for supported languages (Rust, Python, JS/TS, Go, Java) \
         and falls back to text heuristics for others. Use this to locate content before reading it in detail with the `read` tool."
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Text to search for. Treated as a literal string (regex special characters are escaped). Matches anywhere in a line, not whole-word."
                },
                "dir": {
                    "type": "string",
                    "description": "Absolute path of the directory to search in. Defaults to the current working directory. Must be within the project root boundary."
                },
                "include": {
                    "type": "string",
                    "description": "File extension filter. Supported patterns: '*.rs', '*.{ts,tsx}'. Only these exact forms are supported; complex glob patterns are ignored."
                },
                "depth": {
                    "type": "string",
                    "enum": ["names", "preview", "full"],
                    "description": "How much detail to return. 'names' returns file:line with no context. 'preview' (default) returns matches with surrounding lines. 'full' returns the complete semantic block when detectable (e.g. an entire function body). 'full' may fall back to 'preview' for indentation-based languages like Python."
                }
            },
            "required": ["query"]
        })
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move { do_find(args, env).await })
    }
}

// ── Main Execution ──

async fn do_find(args: Value, env: &Environment) -> Result<ToolResult, String> {
    let args: FindArgs =
        serde_json::from_value(args).map_err(|e| format!("invalid arguments: {e}"))?;

    if args.query.is_empty() {
        return Err("query cannot be empty".into());
    }

    let dir = resolve_dir(args.dir.as_deref(), &env.cwd, env.root.as_deref())?;
    let regex = build_regex(&args.query)?;
    let include_filter = args
        .include
        .as_deref()
        .map(build_include_filter)
        .transpose()?;

    debug!(
        query = %args.query,
        dir = %dir.display(),
        depth = ?args.depth,
        "find start"
    );

    let (matches, truncated) = search_directory(&dir, &regex, include_filter.as_ref())?;

    if matches.is_empty() {
        return Ok(ToolResult {
            call_id: String::new(),
            content: format!("No matches found for '{}' in {}", args.query, dir.display()),
            title: Some(format!("find: {}", args.query)),
        });
    }

    let classified = classify_all(matches, &args.query, args.depth)?;
    let output = format_output(&classified, truncated, args.depth, &dir);

    Ok(ToolResult {
        call_id: String::new(),
        content: output,
        title: Some(format!("find: {}", args.query)),
    })
}

// ── Directory Resolution ──

fn resolve_dir(dir: Option<&str>, cwd: &Path, root: Option<&Path>) -> Result<PathBuf, String> {
    let path = match dir {
        Some(s) => {
            let p = PathBuf::from(s);
            if p.is_absolute() { p } else { cwd.join(p) }
        }
        None => cwd.to_path_buf(),
    };

    if !path.exists() {
        return Err(format!("dir does not exist: {}", path.display()));
    }

    let canonical = path
        .canonicalize()
        .map_err(|e| format!("failed to resolve dir '{}': {e}", path.display()))?;

    if let Some(root) = root {
        let root_canonical = root
            .canonicalize()
            .map_err(|e| format!("failed to resolve root '{}': {e}", root.display()))?;
        if !canonical.starts_with(&root_canonical) {
            return Err(format!(
                "dir {} is outside the allowed root {}",
                canonical.display(),
                root_canonical.display()
            ));
        }
    }

    Ok(canonical)
}

// ── Regex Builder ──

fn build_regex(query: &str) -> Result<Regex, String> {
    let escaped = regex::escape(query);
    Regex::new(&escaped).map_err(|e| format!("failed to build regex: {e}"))
}

// ── Include Filter ──

fn build_include_filter(pattern: &str) -> Result<Arc<dyn Fn(&Path) -> bool + Send + Sync>, String> {
    let pattern = pattern.trim();

    if pattern.starts_with("*.") {
        let ext_part = &pattern[2..];

        if ext_part.starts_with('{') && ext_part.ends_with('}') {
            let inner = &ext_part[1..ext_part.len() - 1];
            let exts: Vec<String> = inner.split(',').map(|s| s.trim().to_lowercase()).collect();
            return Ok(Arc::new(move |p: &Path| {
                p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| exts.contains(&e.to_lowercase()))
                    .unwrap_or(false)
            }));
        }

        let ext = ext_part.to_lowercase();
        return Ok(Arc::new(move |p: &Path| {
            p.extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case(&ext))
                .unwrap_or(false)
        }));
    }

    warn!("complex include pattern '{pattern}' not supported, allowing all files");
    Ok(Arc::new(|_| true))
}

// ── Directory Search ──

fn search_directory(
    dir: &Path,
    regex: &Regex,
    include_filter: Option<&Arc<dyn Fn(&Path) -> bool + Send + Sync>>,
) -> Result<(Vec<RawMatch>, bool), String> {
    let mut matches = Vec::new();
    let mut truncated = false;

    let mut builder = ignore::WalkBuilder::new(dir);
    builder
        .hidden(false)
        .parents(true)
        .git_global(true)
        .git_ignore(true)
        .ignore(true)
        .max_filesize(Some(MAX_FILE_SIZE_BYTES));

    for result in builder.build() {
        let entry = match result {
            Ok(e) => e,
            Err(e) => {
                warn!("walk error: {e}");
                continue;
            }
        };

        let path = entry.path();

        if !entry.file_type().map_or(false, |ft| ft.is_file()) {
            continue;
        }

        if let Some(filter) = include_filter {
            if !filter(path) {
                continue;
            }
        }

        search_file(path, regex, &mut matches)?;
        if matches.len() > MAX_MATCHES_TOTAL {
            truncated = true;
            break;
        }
    }

    Ok((matches, truncated))
}

fn search_file(path: &Path, regex: &Regex, matches: &mut Vec<RawMatch>) -> Result<(), String> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            warn!("skipping unreadable file {}: {e}", path.display());
            return Ok(());
        }
    };

    if content.as_bytes().contains(&0) {
        warn!("skipping binary file {}", path.display());
        return Ok(());
    }

    let mut file_match_count = 0;
    for (idx, line) in content.lines().enumerate() {
        if regex.is_match(line) {
            file_match_count += 1;
            if file_match_count > MAX_MATCHES_PER_FILE {
                break;
            }
            matches.push(RawMatch {
                path: path.to_path_buf(),
                line_num: idx + 1,
                line_text: line.to_string(),
            });
            if matches.len() > MAX_MATCHES_TOTAL {
                return Ok(());
            }
        }
    }

    Ok(())
}

// ── Semantic Classification ──

fn classify_all(
    matches: Vec<RawMatch>,
    query: &str,
    depth: FindDepth,
) -> Result<Vec<ClassifiedMatch>, String> {
    let mut by_file: HashMap<PathBuf, Vec<RawMatch>> = HashMap::new();
    for m in matches {
        by_file.entry(m.path.clone()).or_default().push(m);
    }

    let mut classified = Vec::new();

    for (path, file_matches) in by_file {
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let lines: Vec<&str> = content.lines().collect();

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let lang = Lang::from_ext(ext);
        let ast_tree = parse_file(&content, lang);

        for m in file_matches {
            let category = match &ast_tree {
                Some(tree) => deepest_node_at_line(tree.root_node(), m.line_num)
                    .and_then(|node| classify_node(node, lang))
                    .unwrap_or_else(|| classify_line_fallback(&m.line_text, query)),
                None => classify_line_fallback(&m.line_text, query),
            };

            let context = extract_context(&lines, m.line_num, depth, category, lang);

            classified.push(ClassifiedMatch {
                path: path.clone(),
                line_num: m.line_num,
                line_text: m.line_text,
                category,
                context,
            });
        }
    }

    Ok(classified)
}

fn parse_file(source: &str, lang: Lang) -> Option<tree_sitter::Tree> {
    let mut parser = lang.parser()?;
    parser.parse(source, None)
}

fn deepest_node_at_line(node: tree_sitter::Node, line: usize) -> Option<tree_sitter::Node> {
    let target = line.saturating_sub(1);
    if node.start_position().row > target || node.end_position().row < target {
        return None;
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i as u32) {
            if child.start_position().row <= target && child.end_position().row >= target {
                return deepest_node_at_line(child, line).or(Some(node));
            }
        }
    }
    Some(node)
}

fn classify_node(node: tree_sitter::Node, lang: Lang) -> Option<MatchCategory> {
    let mut current = node;
    loop {
        if let Some(cat) = category_for_kind(current.kind(), lang) {
            return Some(cat);
        }
        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            return None;
        }
    }
}

fn category_for_kind(kind: &str, lang: Lang) -> Option<MatchCategory> {
    match lang {
        Lang::Rust => rust_category(kind),
        Lang::Python => python_category(kind),
        Lang::JavaScript | Lang::TypeScript | Lang::Tsx => js_ts_category(kind),
        Lang::Go => go_category(kind),
        Lang::Java => java_category(kind),
        Lang::Unknown => None,
    }
}

fn rust_category(kind: &str) -> Option<MatchCategory> {
    match kind {
        "function_item" => Some(MatchCategory::FunctionDefinition),
        "struct_item" => Some(MatchCategory::StructDefinition),
        "enum_item" => Some(MatchCategory::EnumDefinition),
        "trait_item" => Some(MatchCategory::TraitDefinition),
        "impl_item" => Some(MatchCategory::ImplBlock),
        "type_item" => Some(MatchCategory::TypeAlias),
        "const_item" => Some(MatchCategory::Constant),
        "static_item" => Some(MatchCategory::StaticValue),
        "macro_definition" | "macro_rules_definition" => Some(MatchCategory::MacroDefinition),
        "mod_item" => Some(MatchCategory::ModuleDeclaration),
        "use_declaration" | "extern_crate_declaration" => Some(MatchCategory::Import),
        "call_expression" => Some(MatchCategory::FunctionCall),
        "field_expression" => Some(MatchCategory::FieldAccess),
        "let_declaration" => Some(MatchCategory::VariableAssignment),
        "line_comment" | "block_comment" | "doc_comment" => Some(MatchCategory::Comment),
        "string_literal" | "raw_string_literal" => Some(MatchCategory::StringLiteral),
        "attribute_item" => Some(MatchCategory::Attribute),
        "closure_expression" => Some(MatchCategory::FunctionDefinition),
        "macro_invocation" => Some(MatchCategory::FunctionCall),
        _ => None,
    }
}

fn python_category(kind: &str) -> Option<MatchCategory> {
    match kind {
        "function_definition" => Some(MatchCategory::FunctionDefinition),
        "class_definition" => Some(MatchCategory::ClassDefinition),
        "import_statement" | "import_from_statement" => Some(MatchCategory::Import),
        "call" => Some(MatchCategory::FunctionCall),
        "comment" => Some(MatchCategory::Comment),
        "string" | "string_content" => Some(MatchCategory::StringLiteral),
        "assignment" | "augmented_assignment" => Some(MatchCategory::VariableAssignment),
        "attribute" => Some(MatchCategory::FieldAccess),
        "lambda" => Some(MatchCategory::FunctionDefinition),
        "decorated_definition" => Some(MatchCategory::Attribute),
        _ => None,
    }
}

fn js_ts_category(kind: &str) -> Option<MatchCategory> {
    match kind {
        "function_declaration"
        | "function_expression"
        | "arrow_function"
        | "generator_function_declaration"
        | "generator_function"
        | "method_definition" => Some(MatchCategory::FunctionDefinition),
        "class_declaration" | "class_expression" => Some(MatchCategory::ClassDefinition),
        "interface_declaration" => Some(MatchCategory::InterfaceDefinition),
        "enum_declaration" => Some(MatchCategory::EnumDefinition),
        "import_statement" | "import_declaration" | "export_statement" => {
            Some(MatchCategory::Import)
        }
        "call_expression" | "new_expression" => Some(MatchCategory::FunctionCall),
        "member_expression" | "property_identifier" => Some(MatchCategory::FieldAccess),
        "assignment_expression" => Some(MatchCategory::VariableAssignment),
        "variable_declarator" => Some(MatchCategory::VariableAssignment),
        "comment" | "jsx_text" => Some(MatchCategory::Comment),
        "string" | "string_fragment" | "template_string" => Some(MatchCategory::StringLiteral),
        "decorator" => Some(MatchCategory::Attribute),
        "type_alias_declaration" => Some(MatchCategory::TypeAlias),
        _ => None,
    }
}

fn go_category(kind: &str) -> Option<MatchCategory> {
    match kind {
        "function_declaration" | "method_declaration" => Some(MatchCategory::FunctionDefinition),
        "type_declaration" => Some(MatchCategory::TypeAlias),
        "struct_type" => Some(MatchCategory::StructDefinition),
        "interface_type" | "interface_declaration" => Some(MatchCategory::InterfaceDefinition),
        "import_declaration" => Some(MatchCategory::Import),
        "call_expression" => Some(MatchCategory::FunctionCall),
        "selector_expression" => Some(MatchCategory::FieldAccess),
        "short_var_declaration" | "assignment_statement" => Some(MatchCategory::VariableAssignment),
        "var_declaration" => Some(MatchCategory::VariableAssignment),
        "const_declaration" => Some(MatchCategory::Constant),
        "comment" => Some(MatchCategory::Comment),
        "interpreted_string_literal" | "raw_string_literal" => Some(MatchCategory::StringLiteral),
        _ => None,
    }
}

fn java_category(kind: &str) -> Option<MatchCategory> {
    match kind {
        "method_declaration" | "constructor_declaration" => Some(MatchCategory::FunctionDefinition),
        "class_declaration" => Some(MatchCategory::ClassDefinition),
        "interface_declaration" => Some(MatchCategory::InterfaceDefinition),
        "enum_declaration" => Some(MatchCategory::EnumDefinition),
        "import_declaration" => Some(MatchCategory::Import),
        "method_invocation" => Some(MatchCategory::FunctionCall),
        "field_access" => Some(MatchCategory::FieldAccess),
        "variable_declarator" => Some(MatchCategory::VariableAssignment),
        "field_declaration" | "local_variable_declaration" => {
            Some(MatchCategory::VariableAssignment)
        }
        "line_comment" | "block_comment" => Some(MatchCategory::Comment),
        "string_literal" => Some(MatchCategory::StringLiteral),
        "annotation" => Some(MatchCategory::Attribute),
        _ => None,
    }
}

// ── Text Fallback Classification ──

fn classify_line_fallback(line: &str, query: &str) -> MatchCategory {
    let trimmed = line.trim_start();

    if trimmed.starts_with("///") || trimmed.starts_with("//!") || trimmed.starts_with("/*!") {
        return MatchCategory::Comment;
    }

    if trimmed.starts_with("//")
        || trimmed.starts_with("/*")
        || (trimmed.starts_with("* ") && !trimmed.starts_with("*/"))
    {
        return MatchCategory::Comment;
    }

    if is_in_string_literal(line, query) {
        return MatchCategory::StringLiteral;
    }

    if trimmed.starts_with("#") {
        return MatchCategory::Attribute;
    }

    let Some(pos) = line.find(query) else {
        return MatchCategory::Other;
    };

    let before = &line[..pos];
    let after = &line[pos + query.len()..];

    if after.starts_with('(') {
        let prev_char = before.chars().last();
        if !prev_char.map_or(false, |c| c.is_alphanumeric() || c == '_') {
            return MatchCategory::FunctionCall;
        }
    }

    if before.ends_with('.') {
        let dot_pos = before.len().saturating_sub(2);
        if dot_pos == 0 || before.chars().nth(dot_pos) != Some('.') {
            return MatchCategory::FieldAccess;
        }
    }

    if after.starts_with('.') && !after.starts_with(".(") {
        return MatchCategory::FieldAccess;
    }

    if trimmed.starts_with("let ") && line.contains('=') {
        return MatchCategory::VariableAssignment;
    }

    if before.ends_with('<') || after.starts_with('>') || after.starts_with("::<") {
        return MatchCategory::GenericParameter;
    }

    MatchCategory::Other
}

fn is_in_string_literal(line: &str, query: &str) -> bool {
    let Some(pos) = line.find(query) else {
        return false;
    };

    let before = &line[..pos];
    let mut in_double = false;
    let mut in_single = false;
    let mut in_backtick = false;
    let mut escaped = false;

    for ch in before.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '`' if !in_double && !in_single => in_backtick = !in_backtick,
            '"' if !in_single && !in_backtick => in_double = !in_double,
            '\'' if !in_double && !in_backtick => in_single = !in_single,
            _ => {}
        }
    }

    in_double || in_single || in_backtick
}

// ── Context Extraction ──

fn extract_context(
    lines: &[&str],
    match_line: usize,
    depth: FindDepth,
    category: MatchCategory,
    lang: Lang,
) -> Vec<(usize, String)> {
    match depth {
        FindDepth::Names => Vec::new(),
        FindDepth::Preview => extract_line_window(lines, match_line, PREVIEW_CONTEXT_LINES),
        FindDepth::Full => {
            if let Some(end) = try_extract_block(lines, match_line, category, lang) {
                let start = match_line.saturating_sub(1);
                (start..end)
                    .map(|i| (i + 1, lines[i].to_string()))
                    .collect()
            } else {
                extract_line_window(lines, match_line, FULL_FALLBACK_CONTEXT_LINES)
            }
        }
    }
}

fn extract_line_window(lines: &[&str], match_line: usize, radius: usize) -> Vec<(usize, String)> {
    let idx = match_line.saturating_sub(1);
    let start = idx.saturating_sub(radius);
    let end = (idx + radius + 1).min(lines.len());
    (start..end)
        .map(|i| (i + 1, lines[i].to_string()))
        .collect()
}

fn try_extract_block(
    lines: &[&str],
    match_line: usize,
    category: MatchCategory,
    lang: Lang,
) -> Option<usize> {
    if matches!(lang, Lang::Python) {
        return try_extract_indent_block(lines, match_line);
    }

    if !matches!(
        category,
        MatchCategory::FunctionDefinition
            | MatchCategory::StructDefinition
            | MatchCategory::ClassDefinition
            | MatchCategory::EnumDefinition
            | MatchCategory::TraitDefinition
            | MatchCategory::InterfaceDefinition
            | MatchCategory::ImplBlock
            | MatchCategory::MacroDefinition
    ) {
        return None;
    }

    let start_idx = match_line.saturating_sub(1);
    if start_idx >= lines.len() {
        return None;
    }

    let mut brace_line = None;
    for i in start_idx..lines.len().min(start_idx + 10) {
        if lines[i].contains('{') {
            brace_line = Some(i);
            break;
        }
    }
    let brace_line = brace_line?;

    let mut depth = 0i32;
    let mut in_string = false;
    let mut string_char = '\0';
    let mut escaped = false;

    for i in brace_line..lines.len().min(brace_line + MAX_BLOCK_SEARCH_LINES) {
        for ch in lines[i].chars() {
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                continue;
            }
            if ch == '"' || ch == '\'' || ch == '`' {
                in_string = true;
                string_char = ch;
                continue;
            }
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i + 1);
                    }
                }
                _ => {}
            }
        }
    }

    None
}

fn try_extract_indent_block(lines: &[&str], match_line: usize) -> Option<usize> {
    let start_idx = match_line.saturating_sub(1);
    if start_idx >= lines.len() {
        return None;
    }

    let base_indent = leading_spaces(lines[start_idx]);

    let mut body_start = None;
    for i in (start_idx + 1)..lines.len() {
        let line = lines[i];
        if line.trim().is_empty() {
            continue;
        }
        let indent = leading_spaces(line);
        if indent > base_indent {
            body_start = Some(i);
            break;
        }
    }
    let body_start = body_start?;
    let _body_indent = leading_spaces(lines[body_start]);

    for i in (body_start + 1)..lines.len() {
        let line = lines[i];
        if line.trim().is_empty() {
            continue;
        }
        let indent = leading_spaces(line);
        if indent <= base_indent {
            return Some(i);
        }
    }

    Some(lines.len())
}

fn leading_spaces(line: &str) -> usize {
    line.chars().take_while(|c| c.is_whitespace()).count()
}

// ── Output Formatting ──

fn format_output(
    classified: &[ClassifiedMatch],
    truncated: bool,
    depth: FindDepth,
    dir: &Path,
) -> String {
    let mut groups: HashMap<MatchCategory, Vec<&ClassifiedMatch>> = HashMap::new();
    for m in classified {
        groups.entry(m.category).or_default().push(m);
    }

    let mut lines = Vec::new();
    let mut total_shown = 0;

    for cat in category_priority_order() {
        let Some(matches) = groups.get(&cat) else {
            continue;
        };
        if matches.is_empty() {
            continue;
        }

        lines.push(format!(
            "[{}] {} matches",
            category_label(cat),
            matches.len()
        ));
        total_shown += matches.len();

        let display_count = matches.len().min(OUTPUT_MAX_MATCHES_PER_CATEGORY);
        for m in &matches[..display_count] {
            let rel = make_relative(&m.path, dir);
            lines.push(format!(
                "  {}:{}  {}",
                rel,
                m.line_num,
                truncate_visual(&m.line_text, 100)
            ));

            if depth != FindDepth::Names && !m.context.is_empty() {
                for (ln, text) in &m.context {
                    let marker = if *ln == m.line_num { ">>>" } else { "   " };
                    lines.push(format!(
                        "    {} {:>4}| {}",
                        marker,
                        ln,
                        truncate_visual(text, 88)
                    ));
                }
            }
        }

        if matches.len() > OUTPUT_MAX_MATCHES_PER_CATEGORY {
            lines.push(format!(
                "    ... and {} more",
                matches.len() - OUTPUT_MAX_MATCHES_PER_CATEGORY
            ));
        }

        lines.push(String::new());
    }

    lines.push(format!(
        "Found {} matches across {} categories",
        total_shown,
        groups.len()
    ));

    if truncated {
        lines.push(String::new());
        lines.push("Results truncated. Suggestions:".into());
        lines.push("  - Narrow the dir to a subdirectory".into());
        lines.push("  - Use include to filter by file type".into());
        lines.push("  - Use a more specific query".into());
    }

    lines.join("\n")
}

fn category_priority_order() -> Vec<MatchCategory> {
    vec![
        MatchCategory::FunctionDefinition,
        MatchCategory::StructDefinition,
        MatchCategory::ClassDefinition,
        MatchCategory::EnumDefinition,
        MatchCategory::TraitDefinition,
        MatchCategory::InterfaceDefinition,
        MatchCategory::ImplBlock,
        MatchCategory::TypeAlias,
        MatchCategory::Constant,
        MatchCategory::StaticValue,
        MatchCategory::MacroDefinition,
        MatchCategory::ModuleDeclaration,
        MatchCategory::Import,
        MatchCategory::FunctionCall,
        MatchCategory::VariableAssignment,
        MatchCategory::FieldAccess,
        MatchCategory::Comment,
        MatchCategory::StringLiteral,
        MatchCategory::Attribute,
        MatchCategory::GenericParameter,
        MatchCategory::Other,
    ]
}

fn category_label(cat: MatchCategory) -> &'static str {
    match cat {
        MatchCategory::FunctionDefinition => "Function Definition",
        MatchCategory::StructDefinition => "Struct Definition",
        MatchCategory::ClassDefinition => "Class Definition",
        MatchCategory::EnumDefinition => "Enum Definition",
        MatchCategory::TraitDefinition => "Trait Definition",
        MatchCategory::InterfaceDefinition => "Interface Definition",
        MatchCategory::ImplBlock => "Impl Block",
        MatchCategory::TypeAlias => "Type Alias",
        MatchCategory::Constant => "Constant",
        MatchCategory::StaticValue => "Static",
        MatchCategory::MacroDefinition => "Macro Definition",
        MatchCategory::ModuleDeclaration => "Module Declaration",
        MatchCategory::Import => "Import",
        MatchCategory::FunctionCall => "Function Call",
        MatchCategory::VariableAssignment => "Variable Assignment",
        MatchCategory::FieldAccess => "Field Access",
        MatchCategory::Comment => "Comment",
        MatchCategory::StringLiteral => "String Literal",
        MatchCategory::Attribute => "Attribute",
        MatchCategory::GenericParameter => "Generic Parameter",
        MatchCategory::Other => "Other",
    }
}

fn make_relative(path: &Path, base: &Path) -> String {
    path.strip_prefix(base)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

fn truncate_visual(s: &str, max_chars: usize) -> String {
    let mut count = 0;
    let mut cutoff = s.len();
    for (i, _ch) in s.char_indices() {
        count += 1;
        if count > max_chars {
            cutoff = i;
            break;
        }
    }
    if cutoff < s.len() {
        format!("{}...", &s[..cutoff])
    } else {
        s.to_string()
    }
}
