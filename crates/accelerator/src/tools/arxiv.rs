use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde::Deserialize;
use serde_json::Value;
use tracing::info;

const SEARCH_TIMEOUT_SECS: u64 = 30;
const DOWNLOAD_TIMEOUT_SECS: u64 = 60;
const MAX_RESULTS: usize = 100;
const DEFAULT_RESULTS: usize = 10;
const ARXIV_API_BASE: &str = "https://arxivsearch.synergy.holosai.io";
const ARXIV_PDF_BASE: &str = "https://arxiv.org/pdf";

// ── arxiv_search ──

pub struct ArxivSearchTool;

impl Tool for ArxivSearchTool {
    fn name(&self) -> &str {
        "arxiv_search"
    }

    fn description(&self) -> &str {
        "Search the arXiv database for academic papers using semantic search and filters.\n\n\
         Use this tool to find research papers on arXiv. You can search using:\n\
         - Natural language queries for semantic search\n\
         - Author names (OR logic between multiple authors)\n\
         - arXiv categories like 'cs.AI', 'hep-ph', 'math.AG' (OR logic)\n\
         - Date ranges (YYYY-MM-DD format)\n\
         - Title keywords (AND logic between keywords)\n\n\
         Returns paper metadata including title, authors, abstract, categories, and arXiv ID."
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Natural language search query for semantic search"
                },
                "authors": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Filter by author names (OR logic)"
                },
                "categories": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Filter by arXiv categories like 'cs.AI', 'hep-ph' (OR logic)"
                },
                "startDate": {
                    "type": "string",
                    "description": "Start date (YYYY-MM-DD, inclusive)"
                },
                "endDate": {
                    "type": "string",
                    "description": "End date (YYYY-MM-DD, inclusive)"
                },
                "titleKeywords": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Keywords in title (AND logic)"
                },
                "topK": {
                    "type": "integer",
                    "default": 10,
                    "description": "Number of results (1-100, default: 10)"
                }
            }
        })
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(SEARCH_TIMEOUT_SECS)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        _env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move { execute_search(args).await })
    }
}

#[derive(Debug, Deserialize)]
struct Paper {
    id: String,
    title: String,
    authors: Vec<String>,
    categories: Vec<String>,
    published_date: String,
    summary: String,
    pdf_url: String,
    arxiv_url: String,
    score: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    papers: Vec<Paper>,
    total: u32,
    query: Option<String>,
    mode: String,
    reranked: bool,
}

async fn execute_search(args: Value) -> Result<ToolResult, String> {
    let top_k = args["topK"]
        .as_u64()
        .unwrap_or(DEFAULT_RESULTS as u64)
        .min(MAX_RESULTS as u64) as usize;

    let body = serde_json::json!({
        "query": args["query"].as_str(),
        "authors": args.get("authors").and_then(|v| v.as_array().cloned()),
        "categories": args.get("categories").and_then(|v| v.as_array().cloned()),
        "start_date": args["startDate"].as_str(),
        "end_date": args["endDate"].as_str(),
        "title_keywords": args.get("titleKeywords").and_then(|v| v.as_array().cloned()),
        "top_k": top_k,
        "mode": "hybrid",
        "rerank": true,
        "include_summary": true,
    });

    info!(target: "arxiv_search", %body, "searching arXiv via Holos API");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(SEARCH_TIMEOUT_SECS))
        .build()
        .map_err(|e| format!("failed to create HTTP client: {e}"))?;

    let response = client
        .post(format!("{}/search", ARXIV_API_BASE))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("arXiv search request failed: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(format!("arXiv search failed: HTTP {status} — {text}"));
    }

    let data: SearchResponse = response
        .json()
        .await
        .map_err(|e| format!("arXiv search response parse failed: {e}"))?;

    let papers = data.papers;

    if papers.is_empty() {
        return Ok(ToolResult {
            call_id: String::new(),
            content: "No papers found matching your search criteria.".to_string(),
            title: Some("No results found".to_string()),
        });
    }

    info!(
        target: "arxiv_search",
        total = data.total,
        shown = papers.len(),
        "arXiv search complete"
    );

    let mut lines = vec![
        format!("Found {} papers (showing {}):", data.total, papers.len()),
        String::new(),
        "| # | arXiv ID | Title | Authors | Categories | Published |".into(),
        "|---|----------|-------|---------|------------|-----------|".into(),
    ];

    for (i, paper) in papers.iter().enumerate() {
        let title_safe = paper.title.replace('|', "\\|").replace('\n', " ");
        let title_trim: String = title_safe.chars().take(60).collect();
        let authors = if paper.authors.len() > 3 {
            format!("{} et al.", paper.authors[..3].join(", "))
        } else {
            paper.authors.join(", ")
        };
        let cats: Vec<&str> = paper
            .categories
            .iter()
            .take(2)
            .map(|s| s.as_str())
            .collect();
        let published = paper.published_date.chars().take(10).collect::<String>();
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            i + 1,
            paper.id,
            title_trim,
            authors,
            cats.join(", "),
            published
        ));
    }

    lines.push(String::new());
    lines.push("**Paper Details:**".into());
    lines.push(String::new());

    for paper in &papers {
        lines.push(format!("### {}: {}", paper.id, paper.title));
        lines.push(format!("**Authors:** {}", paper.authors.join(", ")));
        lines.push(format!("**Categories:** {}", paper.categories.join(", ")));
        lines.push(format!("**Published:** {}", paper.published_date));
        lines.push(format!("**PDF:** {}", paper.pdf_url));
        lines.push(String::new());
        lines.push(format!("**Abstract:** {}", paper.summary));
        lines.push(String::new());
        lines.push("---".into());
        lines.push(String::new());
    }

    Ok(ToolResult {
        call_id: String::new(),
        content: lines.join("\n"),
        title: Some(format!("{} papers found", papers.len())),
    })
}

// ── arxiv_download ──

pub struct ArxivDownloadTool;

impl Tool for ArxivDownloadTool {
    fn name(&self) -> &str {
        "arxiv_download"
    }

    fn description(&self) -> &str {
        "Download an arXiv paper as a PDF file.\n\n\
         Use this tool to download a paper from arXiv given its ID. The paper will be saved as a PDF file to the specified path.\n\n\
         Examples of valid arXiv IDs:\n\
         - 2401.12345\n\
         - 2401.12345v1\n\
         - hep-th/9901001"
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "arxivId": {
                    "type": "string",
                    "description": "The arXiv paper ID (e.g., '2401.12345' or '2401.12345v1')"
                },
                "outputPath": {
                    "type": "string",
                    "description": "The output file path (must end with .pdf)"
                },
                "overwrite": {
                    "type": "boolean",
                    "default": false,
                    "description": "Whether to overwrite if file exists"
                }
            },
            "required": ["arxivId", "outputPath"]
        })
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(DOWNLOAD_TIMEOUT_SECS)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move { execute_download(args, env).await })
    }
}

async fn execute_download(args: Value, env: &Environment) -> Result<ToolResult, String> {
    let id = args["arxivId"]
        .as_str()
        .ok_or("arxiv_download requires 'arxivId' parameter")?;

    let output_path_raw = args["outputPath"]
        .as_str()
        .ok_or("arxiv_download requires 'outputPath' parameter")?;

    if !output_path_raw.to_lowercase().ends_with(".pdf") {
        return Err("Output path must end with .pdf".to_string());
    }

    let output_path = resolve_path(output_path_raw, &env.cwd);

    if output_path.exists() && !args["overwrite"].as_bool().unwrap_or(false) {
        return Ok(ToolResult {
            call_id: String::new(),
            content: format!(
                "File already exists at {}. Set overwrite=true to replace it.",
                output_path.display()
            ),
            title: Some("File exists".to_string()),
        });
    }

    let url = format!("{}/{}.pdf", ARXIV_PDF_BASE, id);

    info!(target: "arxiv_download", id, url = %url, path = ?output_path, "downloading arXiv paper");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(DOWNLOAD_TIMEOUT_SECS))
        .build()
        .map_err(|e| format!("failed to create HTTP client: {e}"))?;

    let response = client
        .get(&url)
        .header("User-Agent", "RCM/0.1 (compatible; Synergy/1.0)")
        .send()
        .await
        .map_err(|e| format!("arXiv download failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to download paper: HTTP {}",
            response.status()
        ));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("arXiv download read failed: {e}"))?;

    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("failed to create download directory: {e}"))?;
    }

    tokio::fs::write(&output_path, &bytes)
        .await
        .map_err(|e| format!("failed to write PDF: {e}"))?;

    let size_str = if bytes.len() > 1024 * 1024 {
        format!("{:.2} MB", bytes.len() as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} KB", bytes.len() as f64 / 1024.0)
    };

    let relative = relative_path(&output_path, &env.cwd);

    info!(
        target: "arxiv_download",
        bytes = bytes.len(),
        path = ?output_path,
        "arXiv paper downloaded"
    );

    Ok(ToolResult {
        call_id: String::new(),
        content: format!("Successfully downloaded arXiv paper {id} to {relative} ({size_str})"),
        title: Some(format!("Downloaded {id}")),
    })
}

fn resolve_path(raw: &str, cwd: &Path) -> PathBuf {
    let path = Path::new(raw);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}

fn relative_path(path: &Path, cwd: &Path) -> String {
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
