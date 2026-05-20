//! Diagnostic storage and formatting.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use super::uri::uri_to_path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<u8>,
    pub message: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DiagnosticEvent {
    pub path: PathBuf,
}

#[derive(Clone)]
pub struct DiagnosticStore {
    inner: Arc<Mutex<HashMap<PathBuf, Vec<Diagnostic>>>>,
    tx: broadcast::Sender<DiagnosticEvent>,
}

impl DiagnosticStore {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(64);
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            tx,
        }
    }

    pub fn update_from_notification(&self, params: &serde_json::Value) -> Result<(), String> {
        let uri = params["uri"]
            .as_str()
            .ok_or("diagnostic notification missing uri")?;
        let path = uri_to_path(uri)?;
        let diagnostics = serde_json::from_value::<Vec<Diagnostic>>(params["diagnostics"].clone())
            .map_err(|error| format!("invalid diagnostics payload: {error}"))?;

        {
            let mut inner = self.inner.lock().unwrap();
            inner.insert(path.clone(), diagnostics);
        }
        let _ = self.tx.send(DiagnosticEvent { path });
        Ok(())
    }

    pub fn get(&self, path: &Path) -> Vec<Diagnostic> {
        self.inner
            .lock()
            .unwrap()
            .get(path)
            .cloned()
            .unwrap_or_default()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DiagnosticEvent> {
        self.tx.subscribe()
    }
}

pub fn format_file_diagnostics(path: &Path, diagnostics: &[Diagnostic]) -> String {
    if diagnostics.is_empty() {
        return String::new();
    }

    let mut output = format!("\n<file_diagnostics path=\"{}\">\n", path.display());
    for diagnostic in diagnostics.iter().take(10) {
        output.push_str(&format!(
            "{} [{}:{}] {}\n",
            severity_label(diagnostic.severity),
            diagnostic.range.start.line + 1,
            diagnostic.range.start.character + 1,
            diagnostic.message.replace('\n', " ")
        ));
    }
    if diagnostics.len() > 10 {
        output.push_str(&format!(
            "... {} more diagnostics\n",
            diagnostics.len() - 10
        ));
    }
    output.push_str("</file_diagnostics>\n");
    output
}

fn severity_label(severity: Option<u8>) -> &'static str {
    match severity {
        Some(1) => "ERROR",
        Some(2) => "WARN",
        Some(3) => "INFO",
        Some(4) => "HINT",
        _ => "DIAG",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_one_based_line_column() {
        let diagnostic = Diagnostic {
            range: Range {
                start: Position {
                    line: 0,
                    character: 2,
                },
                end: Position {
                    line: 0,
                    character: 3,
                },
            },
            severity: Some(1),
            message: "bad".to_string(),
            source: None,
        };
        let output = format_file_diagnostics(Path::new("src/lib.rs"), &[diagnostic]);
        assert!(output.contains("ERROR [1:3] bad"));
    }
}
