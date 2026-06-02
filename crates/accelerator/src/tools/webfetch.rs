use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;

const TIMEOUT_SECS: u64 = 30;
const MAX_BYTES: usize = 1_000_000;

pub struct WebFetchTool;

impl Tool for WebFetchTool {
    fn name(&self) -> &str {
        "webfetch"
    }

    fn description(&self) -> &str {
        "Fetch and read a web page. Given a URL, returns the page content as readable text after stripping navigation, sidebars, and other chrome."
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The full URL to fetch (including scheme)"
                },
                "max_length": {
                    "type": "integer",
                    "default": 5000,
                    "description": "Maximum characters of text to return (default 5000, max 100000)"
                }
            },
            "required": ["url"]
        })
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(TIMEOUT_SECS)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        _env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let url = args["url"]
                .as_str()
                .ok_or("missing required parameter 'url'")?;
            let max_length = args["max_length"].as_u64().unwrap_or(5000).min(100_000) as usize;

            let response = reqwest::get(url)
                .await
                .map_err(|e| format!("failed to fetch {url}: {e}"))?;

            let status = response.status();
            if !status.is_success() {
                return Ok(ToolResult {
                    call_id: String::new(),
                    content: format!("HTTP {status}"),
                    title: Some(format!("HTTP {status}")),
                });
            }

            let content_type = response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();

            let body = response
                .bytes()
                .await
                .map_err(|e| format!("failed to read response body: {e}"))?;

            if body.len() > MAX_BYTES {
                return Ok(ToolResult {
                    call_id: String::new(),
                    content: format!("page too large ({} bytes, max {MAX_BYTES})", body.len()),
                    title: Some("too large".into()),
                });
            }

            let is_html = content_type.contains("text/html")
                || content_type.contains("text/plain")
                || content_type.is_empty();

            let text = if is_html {
                let html = String::from_utf8_lossy(&body);
                let doc = scraper::Html::parse_document(&html);
                let mut lines: Vec<String> = Vec::new();

                for text_node in doc.root_element().text() {
                    let trimmed = text_node.trim();
                    if !trimmed.is_empty() {
                        lines.push(trimmed.to_string());
                    }
                }

                let raw_text = lines.join("\n");
                let title_el = doc
                    .select(&scraper::Selector::parse("title").unwrap())
                    .next()
                    .map(|el| el.text().collect::<String>())
                    .filter(|t| !t.trim().is_empty());

                let mut result = String::new();
                if let Some(ref t) = title_el {
                    result.push_str(&format!("# {}\n\n", t.trim()));
                }
                result.push_str(&if raw_text.len() <= max_length {
                    raw_text
                } else {
                    let truncated = &raw_text[..max_length];
                    format!(
                        "{truncated}\n\n... (truncated, full page is {} chars)",
                        raw_text.len()
                    )
                });
                result
            } else {
                String::from_utf8_lossy(&body)
                    .chars()
                    .take(max_length)
                    .collect()
            };

            Ok(ToolResult {
                call_id: String::new(),
                content: text,
                title: Some(url.to_string()),
            })
        })
    }
}
