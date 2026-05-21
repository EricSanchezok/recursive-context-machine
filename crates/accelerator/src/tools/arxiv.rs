use std::future::Future;
use std::pin::Pin;
use std::sync::LazyLock;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;
use tracing::info;

const MAX_TIMEOUT_SECS: u64 = 120;
const MAX_RESULTS: usize = 100;
const DEFAULT_RESULTS: usize = 10;

static DESCRIPTION: LazyLock<String> = LazyLock::new(|| include_str!("arxiv.txt").to_string());

pub struct ArxivTool;

impl Tool for ArxivTool {
    fn name(&self) -> &str {
        "arxiv"
    }

    fn description(&self) -> &str {
        &DESCRIPTION
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "mode": {
                    "type": "string",
                    "enum": ["search", "download"],
                    "description": "Action: search arXiv papers or download a paper PDF."
                },
                "query": {
                    "type": "string",
                    "description": "(search) Natural language search query."
                },
                "authors": {
                    "type": "string",
                    "description": "(search) Comma-separated author names (OR logic)."
                },
                "categories": {
                    "type": "string",
                    "description": "(search) Comma-separated arXiv categories, e.g. 'cs.AI,cs.LG'."
                },
                "startDate": {
                    "type": "string",
                    "description": "(search) Start date (YYYY-MM-DD, inclusive)."
                },
                "endDate": {
                    "type": "string",
                    "description": "(search) End date (YYYY-MM-DD, inclusive)."
                },
                "titleKeywords": {
                    "type": "string",
                    "description": "(search) Comma-separated title keywords (AND logic)."
                },
                "topK": {
                    "type": "integer",
                    "description": "(search) Number of results (1-100, default 10)."
                },
                "id": {
                    "type": "string",
                    "description": "(download) arXiv paper ID, e.g. '2401.12345'."
                },
                "downloadDir": {
                    "type": "string",
                    "description": "(download) Directory to save PDF. Defaults to project root."
                },
                "overwrite": {
                    "type": "boolean",
                    "description": "(download) Overwrite if file exists. Default false."
                }
            },
            "required": ["mode"]
        })
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(MAX_TIMEOUT_SECS)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let mode = args["mode"]
                .as_str()
                .ok_or("arxiv requires 'mode' parameter")?;

            match mode {
                "search" => execute_search(args).await,
                "download" => execute_download(args, env).await,
                _ => Err(format!("unknown arxiv mode: {}", mode)),
            }
        })
    }
}

async fn execute_search(args: Value) -> Result<ToolResult, String> {
    let query = args["query"]
        .as_str()
        .ok_or("arxiv search requires 'query' parameter")?;
    let top_k = args["topK"]
        .as_u64()
        .unwrap_or(DEFAULT_RESULTS as u64)
        .min(MAX_RESULTS as u64) as usize;

    info!(target: "arxiv", query, top_k, "searching arXiv");

    let mut search_parts: Vec<String> = Vec::new();

    search_parts.push(format!("all:{}", urlencode(query)));

    if let Some(cats) = args["categories"].as_str().filter(|s| !s.is_empty()) {
        let cat_or: Vec<String> = cats
            .split(',')
            .map(|c| format!("cat:{}", c.trim()))
            .collect();
        let combined = cat_or.join("+OR+");
        search_parts.push(format!("({})", combined));
    }

    if let Some(authors) = args["authors"].as_str().filter(|s| !s.is_empty()) {
        for author in authors
            .split(',')
            .map(|a| a.trim())
            .filter(|a| !a.is_empty())
        {
            search_parts.push(format!("au:{}", urlencode(author)));
        }
    }

    if let Some(titles) = args["titleKeywords"].as_str().filter(|s| !s.is_empty()) {
        for kw in titles
            .split(',')
            .map(|k| k.trim())
            .filter(|k| !k.is_empty())
        {
            search_parts.push(format!("ti:{}", urlencode(kw)));
        }
    }

    let search_query = search_parts.join("+AND+");

    let mut url = format!(
        "http://export.arxiv.org/api/query?search_query={}&start=0&max_results={}&sortBy=submittedDate&sortOrder=descending",
        search_query, top_k
    );

    if let Some(date) = args["startDate"].as_str().filter(|s| !s.is_empty()) {
        url.push_str(&format!("&start_date={}", urlencode(date)));
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| format!("failed to create HTTP client: {e}"))?;

    let response = client
        .get(&url)
        .header("User-Agent", "RICA/0.1 (mailto:research@rica.dev)")
        .send()
        .await
        .map_err(|e| format!("arXiv API request failed: {e}"))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("arXiv response read failed: {e}"))?;

    let mut papers = parse_atom_feed(&text)?;
    let total = papers.len();

    // Client-side date filtering (end date)
    if let Some(date) = args["endDate"].as_str().filter(|s| !s.is_empty()) {
        papers.retain(|p| p.published.as_str() <= date);
    }

    if papers.is_empty() {
        return Ok(ToolResult {
            call_id: String::new(),
            content: "No papers found matching your search criteria.".to_string(),
            title: Some("No results found".to_string()),
        });
    }

    info!(target: "arxiv", total, shown = papers.len(), "arXiv search complete");

    let mut lines = vec![
        format!("Found {total} papers (showing {}):", papers.len()),
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
        let published = paper.published.chars().take(10).collect::<String>();
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
        lines.push(format!("**Published:** {}", paper.published));
        lines.push(format!("**PDF:** https://arxiv.org/pdf/{}.pdf", paper.id));
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

async fn execute_download(args: Value, env: &Environment) -> Result<ToolResult, String> {
    let id = args["id"]
        .as_str()
        .ok_or("arxiv download requires 'id' parameter")?;

    // Strip common URL prefixes to get bare ID
    let arxiv_id = id
        .strip_prefix("https://arxiv.org/pdf/")
        .or_else(|| id.strip_prefix("https://arxiv.org/abs/"))
        .or_else(|| id.strip_prefix("http://arxiv.org/pdf/"))
        .or_else(|| id.strip_prefix("http://arxiv.org/abs/"))
        .unwrap_or(id);

    let download_dir = args["downloadDir"]
        .as_str()
        .map(|d| crate::tools::resolve_path(d, &env.cwd))
        .unwrap_or_else(|| env.cwd.clone());

    let base_id = arxiv_id.split('v').next().unwrap_or(arxiv_id);
    let filename = format!("{}.pdf", base_id.replace('/', "_"));
    let output_path = download_dir.join(&filename);

    if output_path.exists() && !args["overwrite"].as_bool().unwrap_or(false) {
        return Ok(ToolResult {
            call_id: String::new(),
            content: format!("File already exists at {}", output_path.display()),
            title: Some("arxiv download".to_string()),
        });
    }

    let pdf_url = format!("https://arxiv.org/pdf/{}.pdf", arxiv_id);

    info!(target: "arxiv", id = arxiv_id, url = %pdf_url, path = ?output_path, "downloading arXiv paper");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| format!("failed to create HTTP client: {e}"))?;

    let response = client
        .get(&pdf_url)
        .header("User-Agent", "RICA/0.1")
        .send()
        .await
        .map_err(|e| format!("arXiv download failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!(
            "arXiv download returned HTTP {}",
            response.status()
        ));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("arXiv download read failed: {e}"))?;

    tokio::fs::create_dir_all(output_path.parent().unwrap())
        .await
        .map_err(|e| format!("failed to create download directory: {e}"))?;

    tokio::fs::write(&output_path, &bytes)
        .await
        .map_err(|e| format!("failed to write PDF: {e}"))?;

    let size_str = if bytes.len() > 1024 * 1024 {
        format!("{:.2} MB", bytes.len() as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} KB", bytes.len() as f64 / 1024.0)
    };

    let relative = crate::tools::relative_path(&output_path, &env.cwd);
    info!(target: "arxiv", bytes = bytes.len(), path = ?output_path, "arXiv paper downloaded");

    Ok(ToolResult {
        call_id: String::new(),
        content: format!(
            "Successfully downloaded arXiv paper {arxiv_id} to {relative} ({size_str})"
        ),
        title: Some("arxiv download".to_string()),
    })
}

struct PaperEntry {
    id: String,
    title: String,
    authors: Vec<String>,
    categories: Vec<String>,
    published: String,
    summary: String,
}

fn parse_atom_feed(xml: &str) -> Result<Vec<PaperEntry>, String> {
    let mut entries = Vec::new();
    let mut pos = 0;
    let bytes = xml.as_bytes();

    while let Some(entry_start) = find(bytes, b"<entry>", pos) {
        let entry_end =
            find(bytes, b"</entry>", entry_start + 7).ok_or_else(|| "unclosed <entry> tag")?;
        let chunk = &xml[entry_start..entry_end + 8];

        let id = extract(chunk, "id").unwrap_or_default();
        let title = extract(chunk, "title").unwrap_or_default();
        let published = extract(chunk, "published")
            .or_else(|| extract(chunk, "updated"))
            .unwrap_or_default();
        let summary = extract(chunk, "summary").unwrap_or_default();

        let arxiv_id = id
            .strip_prefix("http://arxiv.org/abs/")
            .or_else(|| id.strip_prefix("http://arxiv.org/pdf/"))
            .unwrap_or(&id);

        let mut authors = Vec::new();
        let mut auth_pos = 0;
        let chunk_bytes = chunk.as_bytes();
        while let Some(a_start) = find(chunk_bytes, b"<author>", auth_pos) {
            let a_end =
                find(chunk_bytes, b"</author>", a_start + 8).unwrap_or(chunk.len() - a_start);
            let a_chunk = &chunk[a_start..a_end + 9];
            if let Some(name) = extract(a_chunk, "name") {
                authors.push(name);
            }
            auth_pos = a_end + 9;
        }

        let mut categories = Vec::new();
        let mut cat_pos = 0;
        let cat_bytes = chunk.as_bytes();
        while let Some(c_start) = find(cat_bytes, b"<category", cat_pos) {
            let c_end = find(cat_bytes, b"/>", c_start + 9)
                .or_else(|| find(cat_bytes, b">", c_start + 9))
                .unwrap_or(chunk.len() - c_start);
            let c_str = &chunk[c_start..c_end + 1];
            if let Some(term) = extract_attr(c_str, "term") {
                categories.push(term);
            }
            cat_pos = c_end + 1;
        }

        entries.push(PaperEntry {
            id: arxiv_id.to_string(),
            title: html_unescape(&title),
            authors,
            categories,
            published,
            summary: html_unescape(&summary),
        });

        pos = entry_end + 8;
    }

    Ok(entries)
}

fn find(content: &[u8], tag: &[u8], start: usize) -> Option<usize> {
    content[start..]
        .windows(tag.len())
        .position(|w| w == tag)
        .map(|i| start + i)
}

fn extract(content: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let start = content.find(&open)?;
    let vstart = start + open.len();
    let end = content[vstart..].find(&close)?;
    Some(content[vstart..vstart + end].to_string())
}

fn extract_attr(content: &str, attr: &str) -> Option<String> {
    let search = format!("{}=\"", attr);
    let start = content.find(&search)?;
    let vstart = start + search.len();
    let end = content[vstart..].find('"')?;
    Some(content[vstart..vstart + end].to_string())
}

fn html_unescape(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
}

fn urlencode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for byte in s.bytes() {
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
