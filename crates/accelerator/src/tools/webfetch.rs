use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};

use machine::{Environment, Tool, ToolResult};
use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, HeaderMap, HeaderName, HeaderValue,
    USER_AGENT,
};
use serde_json::Value;

const TIMEOUT_SECS: u64 = 30;
const MAX_BYTES: usize = 1_000_000;
const DOMAIN_INTERVAL: Duration = Duration::from_secs(2);

const BROWSER_UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

static CLIENT: std::sync::LazyLock<reqwest::Client> = std::sync::LazyLock::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8",
        ),
    );
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(USER_AGENT, HeaderValue::from_static(BROWSER_UA));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    headers.insert(
        HeaderName::from_static("sec-fetch-dest"),
        HeaderValue::from_static("document"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-mode"),
        HeaderValue::from_static("navigate"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-site"),
        HeaderValue::from_static("none"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-user"),
        HeaderValue::from_static("?1"),
    );
    headers.insert(
        HeaderName::from_static("upgrade-insecure-requests"),
        HeaderValue::from_static("1"),
    );

    reqwest::Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()
        .expect("webfetch client")
});

fn domain(url: &str) -> Option<String> {
    url::Url::parse(url)
        .ok()
        .map(|u| u.host_str().unwrap_or("").to_string())
}

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

            if let Some(d) = domain(url) {
                rate_limit(&d).await;
            }

            let response = CLIENT
                .get(url)
                .send()
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

async fn rate_limit(host: &str) {
    static LAST: std::sync::LazyLock<std::sync::Mutex<HashMap<String, Instant>>> =
        std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

    let elapsed = LAST.lock().ok().and_then(|mut map| {
        let now = Instant::now();
        let elapsed = map.get(host).map(|last| now - *last);
        map.insert(host.to_string(), now);
        elapsed
    });

    if let Some(d) = elapsed
        && d < DOMAIN_INTERVAL
    {
        tokio::time::sleep(DOMAIN_INTERVAL - d).await;
    }
}
