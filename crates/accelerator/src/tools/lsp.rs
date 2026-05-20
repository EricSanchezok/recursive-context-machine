//! Agent-facing LSP tool for code intelligence queries.

use std::path::Path;
use std::pin::Pin;
use std::str::FromStr;
use std::time::Duration;

use lsp_types::request::{DocumentSymbolRequest, HoverRequest};
use lsp_types::{
    GotoDefinitionParams, GotoDefinitionResponse, HoverParams, PartialResultParams, Position,
    TextDocumentIdentifier, TextDocumentPositionParams, WorkDoneProgressParams,
};
use machine::{Environment, Tool, ToolResult};
use serde_json::Value;
use tracing::warn;

use super::resolve_path;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct LspTool;

fn to_lsp_uri(s: &str) -> Result<lsp_types::Uri, String> {
    lsp_types::Uri::from_str(s).map_err(|e| format!("invalid URI '{}': {e}", s))
}

impl Tool for LspTool {
    fn name(&self) -> &str {
        "lsp"
    }

    fn description(&self) -> &str {
        "Query a language server for code intelligence. Requires a compatible LSP server (e.g. rust-analyzer)."
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["diagnostics", "documentSymbol", "hover", "definition"],
                    "description": "What to query: diagnostics (errors/warnings for a file), documentSymbol (symbols in a file), hover (type/docs at a position), definition (go to definition)."
                },
                "filePath": {
                    "type": "string",
                    "description": "Path to the file to query."
                },
                "line": {
                    "type": "integer",
                    "description": "1-based line number. Required for hover and definition."
                },
                "character": {
                    "type": "integer",
                    "description": "1-based character offset. Required for hover and definition."
                }
            },
            "required": ["operation", "filePath"]
        })
    }

    fn timeout(&self) -> Duration {
        DEFAULT_TIMEOUT
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let operation = args["operation"]
                .as_str()
                .ok_or("missing required parameter 'operation'")?;
            let file_path = args["filePath"]
                .as_str()
                .ok_or("missing required parameter 'filePath'")?;
            let resolved = resolve_path(file_path, &env.cwd);

            match operation {
                "diagnostics" => query_diagnostics(&resolved, env).await,
                "documentSymbol" => query_symbols(&resolved, env).await,
                "hover" => {
                    let line = args["line"].as_u64().ok_or("missing 'line'")?;
                    let character = args["character"].as_u64().ok_or("missing 'character'")?;
                    query_hover(&resolved, line, character, env).await
                }
                "definition" => {
                    let line = args["line"].as_u64().ok_or("missing 'line'")?;
                    let character = args["character"].as_u64().ok_or("missing 'character'")?;
                    query_definition(&resolved, line, character, env).await
                }
                other => Err(format!("unknown operation '{other}'")),
            }
        })
    }
}

fn document_identifier(path: &Path) -> Result<TextDocumentIdentifier, String> {
    let raw = url::Url::from_file_path(path)
        .map_err(|_| format!("cannot convert path to file URI: {}", path.display()))?;
    let uri = to_lsp_uri(raw.as_str())?;
    Ok(TextDocumentIdentifier { uri })
}

// ── diagnostics ────────────────────────────────────────────────────────

async fn query_diagnostics(path: &Path, env: &Environment) -> Result<ToolResult, String> {
    let snapshot = crate::lsp::snapshot(env, path).await;
    let output = crate::lsp::format_file_diagnostics(path, snapshot.diagnostics());
    Ok(ToolResult {
        call_id: String::new(),
        content: if output.is_empty() {
            "No diagnostics found.\n".to_string()
        } else {
            output
        },
        title: Some(format!("lsp diagnostics {}", path.display())),
    })
}

// ── documentSymbol ─────────────────────────────────────────────────────

async fn query_symbols(path: &Path, env: &Environment) -> Result<ToolResult, String> {
    let response = crate::lsp::query::<DocumentSymbolRequest>(
        env,
        path,
        lsp_types::DocumentSymbolParams {
            text_document: document_identifier(path)?,
            partial_result_params: PartialResultParams::default(),
            work_done_progress_params: WorkDoneProgressParams::default(),
        },
    )
    .await
    .map_err(|error| {
        warn!(%error, "lsp documentSymbol failed");
        error
    })?;

    let names: Vec<String> = match response {
        Some(lsp_types::DocumentSymbolResponse::Flat(flat)) => flat
            .into_iter()
            .map(|sym| format!("ℹ  {}\n", sym.name))
            .collect(),
        Some(lsp_types::DocumentSymbolResponse::Nested(nested)) => nested
            .into_iter()
            .map(|sym| format!("ℹ  {}\n", sym.name))
            .collect(),
        None => vec![],
    };

    if names.is_empty() {
        return Ok(ToolResult {
            call_id: String::new(),
            content: "No symbols found.\n".to_string(),
            title: Some(format!("lsp symbols {}", path.display())),
        });
    }

    let mut output = String::new();
    for name in names.iter().take(100) {
        output.push_str(name);
    }
    Ok(ToolResult {
        call_id: String::new(),
        content: output,
        title: Some(format!("lsp symbols {}", path.display())),
    })
}

// ── hover ──────────────────────────────────────────────────────────────

async fn query_hover(
    path: &Path,
    line: u64,
    character: u64,
    env: &Environment,
) -> Result<ToolResult, String> {
    let hover = crate::lsp::query::<HoverRequest>(
        env,
        path,
        HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: document_identifier(path)?,
                position: Position {
                    line: (line - 1) as u32,
                    character: (character - 1) as u32,
                },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        },
    )
    .await?;

    let content = match hover {
        Some(h) => format_hover_contents(&h.contents),
        None => "No type information available.".to_string(),
    };

    Ok(ToolResult {
        call_id: String::new(),
        content,
        title: Some(format!(
            "lsp hover {}:{}:{}",
            path.display(),
            line,
            character
        )),
    })
}

fn format_hover_contents(contents: &lsp_types::HoverContents) -> String {
    match contents {
        lsp_types::HoverContents::Scalar(text) => match text {
            lsp_types::MarkedString::String(s) => s.clone(),
            lsp_types::MarkedString::LanguageString(ls) => ls.value.clone(),
        },
        lsp_types::HoverContents::Markup(markup) => markup.value.clone(),
        lsp_types::HoverContents::Array(parts) => parts
            .iter()
            .filter_map(|part| match part {
                lsp_types::MarkedString::String(s) => Some(s.as_str()),
                lsp_types::MarkedString::LanguageString(ls) => Some(ls.value.as_str()),
            })
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

// ── definition ─────────────────────────────────────────────────────────

async fn query_definition(
    path: &Path,
    line: u64,
    character: u64,
    env: &Environment,
) -> Result<ToolResult, String> {
    let definition = crate::lsp::query::<lsp_types::request::GotoDefinition>(
        env,
        path,
        GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: document_identifier(path)?,
                position: Position {
                    line: (line - 1) as u32,
                    character: (character - 1) as u32,
                },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
        },
    )
    .await?;

    let output = format_definition_response(definition);
    Ok(ToolResult {
        call_id: String::new(),
        content: output,
        title: Some(format!(
            "lsp definition {}:{}:{}",
            path.display(),
            line,
            character
        )),
    })
}

fn format_definition_response(response: Option<GotoDefinitionResponse>) -> String {
    match response {
        Some(GotoDefinitionResponse::Scalar(location)) => {
            let start = location.range.start;
            format!(
                "{}:{}:{}\n",
                location.uri.as_str(),
                start.line + 1,
                start.character + 1
            )
        }
        Some(GotoDefinitionResponse::Array(locations)) => {
            let mut output = String::new();
            for location in &locations {
                let start = location.range.start;
                output.push_str(&format!(
                    "{}:{}:{}\n",
                    location.uri.as_str(),
                    start.line + 1,
                    start.character + 1
                ));
            }
            output
        }
        Some(GotoDefinitionResponse::Link(_)) => {
            "Definition: external link (not a local file).\n".to_string()
        }
        None => "No definition found.\n".to_string(),
    }
}
