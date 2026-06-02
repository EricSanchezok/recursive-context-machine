//! Versioned diagnostic storage and formatting.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub use lsp_types::Diagnostic;
use lsp_types::DiagnosticSeverity;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub struct DiagnosticEvent {
    pub path: PathBuf,
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct DiagnosticSnapshot {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticSnapshot {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

#[derive(Debug, Clone)]
struct StoredDiagnostics {
    diagnostics: Vec<Diagnostic>,
}

#[derive(Clone)]
pub struct DiagnosticStore {
    inner: Arc<Mutex<HashMap<PathBuf, StoredDiagnostics>>>,
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

    pub fn clear(&self, path: &Path) {
        self.inner.lock().unwrap().remove(path);
    }

    pub fn snapshot(&self, path: &Path) -> DiagnosticSnapshot {
        let diagnostics = self
            .inner
            .lock()
            .unwrap()
            .get(path)
            .map(|stored| stored.diagnostics.clone())
            .unwrap_or_default();
        DiagnosticSnapshot { diagnostics }
    }

    pub fn get(&self, path: &Path) -> Vec<Diagnostic> {
        self.snapshot(path).diagnostics
    }

    pub fn update(
        &self,
        path: PathBuf,
        version: Option<i32>,
        diagnostics: Vec<Diagnostic>,
        current_version: Option<i32>,
    ) -> bool {
        if let (Some(published), Some(current)) = (version, current_version)
            && published != current
        {
            return false;
        }

        {
            let mut inner = self.inner.lock().unwrap();
            inner.insert(path.clone(), StoredDiagnostics { diagnostics });
        }

        let _ = self.tx.send(DiagnosticEvent { path, version });
        true
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DiagnosticEvent> {
        self.tx.subscribe()
    }
}

pub fn new_error_diagnostics(before: &DiagnosticSnapshot, after: &[Diagnostic]) -> Vec<Diagnostic> {
    let before_errors: HashSet<String> = before
        .diagnostics
        .iter()
        .filter(|diagnostic| is_error(diagnostic))
        .map(diagnostic_key)
        .collect();

    after
        .iter()
        .filter(|diagnostic| is_error(diagnostic))
        .filter(|diagnostic| !before_errors.contains(&diagnostic_key(diagnostic)))
        .cloned()
        .collect()
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

fn is_error(diagnostic: &Diagnostic) -> bool {
    diagnostic.severity == Some(DiagnosticSeverity::ERROR)
}

fn diagnostic_key(diagnostic: &Diagnostic) -> String {
    serde_json::to_string(diagnostic).unwrap_or_else(|_| diagnostic.message.clone())
}

fn severity_label(severity: Option<DiagnosticSeverity>) -> &'static str {
    match severity {
        Some(DiagnosticSeverity::ERROR) => "ERROR",
        Some(DiagnosticSeverity::WARNING) => "WARN",
        Some(DiagnosticSeverity::INFORMATION) => "INFO",
        Some(DiagnosticSeverity::HINT) => "HINT",
        _ => "DIAG",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lsp_types::{Position, Range};

    fn diagnostic(message: &str, severity: DiagnosticSeverity, line: u32) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position { line, character: 2 },
                end: Position { line, character: 3 },
            },
            severity: Some(severity),
            message: message.to_string(),
            ..Diagnostic::default()
        }
    }

    #[test]
    fn formats_one_based_line_column() {
        let diagnostic = diagnostic("bad", DiagnosticSeverity::ERROR, 0);
        let output = format_file_diagnostics(Path::new("src/lib.rs"), &[diagnostic]);
        assert!(output.contains("ERROR [1:3] bad"));
    }

    #[test]
    fn drops_stale_versioned_diagnostics() {
        let store = DiagnosticStore::new();
        let path = PathBuf::from("src/lib.rs");
        let accepted = store.update(
            path.clone(),
            Some(1),
            vec![diagnostic("old", DiagnosticSeverity::ERROR, 0)],
            Some(2),
        );
        assert!(!accepted);
        assert!(store.get(&path).is_empty());
    }

    #[test]
    fn accepts_matching_versioned_diagnostics() {
        let store = DiagnosticStore::new();
        let path = PathBuf::from("src/lib.rs");
        let accepted = store.update(
            path.clone(),
            Some(2),
            vec![diagnostic("new", DiagnosticSeverity::ERROR, 0)],
            Some(2),
        );
        assert!(accepted);
        assert_eq!(store.get(&path).len(), 1);
    }

    #[test]
    fn diff_returns_only_new_errors() {
        let before = DiagnosticSnapshot {
            diagnostics: vec![
                diagnostic("old error", DiagnosticSeverity::ERROR, 0),
                diagnostic("old warning", DiagnosticSeverity::WARNING, 1),
            ],
        };
        let after = vec![
            diagnostic("old error", DiagnosticSeverity::ERROR, 0),
            diagnostic("new warning", DiagnosticSeverity::WARNING, 2),
            diagnostic("new error", DiagnosticSeverity::ERROR, 3),
        ];
        let new_errors = new_error_diagnostics(&before, &after);
        assert_eq!(new_errors.len(), 1);
        assert_eq!(new_errors[0].message, "new error");
    }
}
