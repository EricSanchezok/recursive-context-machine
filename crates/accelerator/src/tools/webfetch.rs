use std::collections::HashMap;
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr};
use std::pin::Pin;
use std::time::{Duration, Instant};

use futures_util::StreamExt;
use machine::{Environment, Tool, ToolResult};
use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, HeaderMap, HeaderName, HeaderValue,
    USER_AGENT,
};
use scraper::Selector;
use serde_json::Value;
use tokio::sync::Mutex;
use url::Url;

const TIMEOUT_SECS: u64 = 30;
const MAX_BYTES: u64 = 1_000_000;
const DOMAIN_INTERVAL: Duration = Duration::from_secs(2);

const BROWSER_UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

const HIDDEN_TAGS: &[&str] = &["script", "style", "noscript", "iframe"];

static TITLE_SELECTOR: std::sync::LazyLock<Selector> =
    std::sync::LazyLock::new(|| Selector::parse("title").expect("title selector"));

static PRIVATE_RANGES: &[(Ipv4Addr, Ipv4Addr)] = &[
    (Ipv4Addr::new(0, 0, 0, 0), Ipv4Addr::new(0, 255, 255, 255)),
    (Ipv4Addr::new(10, 0, 0, 0), Ipv4Addr::new(10, 255, 255, 255)),
    (
        Ipv4Addr::new(100, 64, 0, 0),
        Ipv4Addr::new(100, 127, 255, 255),
    ),
    (
        Ipv4Addr::new(127, 0, 0, 0),
        Ipv4Addr::new(127, 255, 255, 255),
    ),
    (
        Ipv4Addr::new(169, 254, 0, 0),
        Ipv4Addr::new(169, 254, 255, 255),
    ),
    (
        Ipv4Addr::new(172, 16, 0, 0),
        Ipv4Addr::new(172, 31, 255, 255),
    ),
    (
        Ipv4Addr::new(192, 168, 0, 0),
        Ipv4Addr::new(192, 168, 255, 255),
    ),
    (
        Ipv4Addr::new(198, 18, 0, 0),
        Ipv4Addr::new(198, 19, 255, 255),
    ),
    (
        Ipv4Addr::new(203, 0, 113, 0),
        Ipv4Addr::new(203, 0, 113, 255),
    ),
];

static RESTRICTED_HOSTNAMES: &[&str] = &[
    "localhost",
    "localhost.localdomain",
    "localhost6",
    "metadata",
    "metadata.google.internal",
    "169.254.169.254",
];

static RESTRICTED_SUFFIXES: &[&str] = &[".localhost", ".local", ".internal"];

static BLOCK_ELEMENTS: &[&str] = &[
    "article",
    "aside",
    "blockquote",
    "br",
    "dd",
    "div",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hr",
    "li",
    "main",
    "nav",
    "ol",
    "p",
    "pre",
    "section",
    "table",
    "tbody",
    "td",
    "tfoot",
    "th",
    "thead",
    "tr",
    "ul",
];

// ── SSRF protection ──

fn is_internal(domain: &str) -> bool {
    let lower = domain.to_lowercase();
    if RESTRICTED_HOSTNAMES
        .iter()
        .any(|h| lower == *h || lower == format!("{h}."))
    {
        return true;
    }
    RESTRICTED_SUFFIXES.iter().any(|s| lower.ends_with(s))
}

fn is_private(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => PRIVATE_RANGES
            .iter()
            .any(|&(start, end)| v4 >= start && v4 <= end),
        IpAddr::V6(v6) => v6.is_loopback() || v6.is_unspecified() || v6.segments()[0] == 0xfd00,
    }
}

fn validate_url(raw: &str) -> Result<Url, String> {
    let url = Url::parse(raw).map_err(|e| format!("invalid URL: {e}"))?;
    if url.scheme() != "http" && url.scheme() != "https" {
        return Err(format!("unsupported scheme: {}", url.scheme()));
    }
    let Some(host) = url.host() else {
        return Err("URL has no host".into());
    };
    match host {
        url::Host::Ipv4(ip) if is_private(IpAddr::V4(ip)) => {
            return Err(format!("blocked IP: {ip}"));
        }
        url::Host::Ipv6(ip) if is_private(IpAddr::V6(ip)) => {
            return Err(format!("blocked IP: {ip}"));
        }
        url::Host::Domain(domain) if is_internal(domain) => {
            return Err(format!("blocked hostname: {domain}"));
        }
        _ => {}
    }
    Ok(url)
}

// ── WebFetch client ──

static CLIENT: std::sync::LazyLock<reqwest::Client> = std::sync::LazyLock::new(|| {
    let mut default_headers = HeaderMap::new();
    default_headers.insert(
        ACCEPT,
        HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8",
        ),
    );
    default_headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    default_headers.insert(USER_AGENT, HeaderValue::from_static(BROWSER_UA));
    default_headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    default_headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    default_headers.insert(
        HeaderName::from_static("sec-fetch-dest"),
        HeaderValue::from_static("document"),
    );
    default_headers.insert(
        HeaderName::from_static("sec-fetch-mode"),
        HeaderValue::from_static("navigate"),
    );
    default_headers.insert(
        HeaderName::from_static("sec-fetch-site"),
        HeaderValue::from_static("none"),
    );
    default_headers.insert(
        HeaderName::from_static("sec-fetch-user"),
        HeaderValue::from_static("?1"),
    );
    default_headers.insert(
        HeaderName::from_static("upgrade-insecure-requests"),
        HeaderValue::from_static("1"),
    );

    reqwest::Client::builder()
        .default_headers(default_headers)
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            let url_str = attempt.url().as_str();
            match validate_url(url_str) {
                Ok(_) => attempt.follow(),
                Err(e) => attempt.error(e),
            }
        }))
        .build()
        .expect("webfetch client")
});

// ── Rate limiter ──

async fn rate_limit(host: &str) {
    static STATE: std::sync::LazyLock<Mutex<HashMap<String, Instant>>> =
        std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

    let should_wait = {
        let map = STATE.lock().await;
        map.get(host)
            .is_some_and(|last| last.elapsed() < DOMAIN_INTERVAL)
    };

    if should_wait {
        tokio::time::sleep(DOMAIN_INTERVAL).await;
    }

    let mut map = STATE.lock().await;
    if map.len() > 1_000 {
        let cutoff = Instant::now() - Duration::from_secs(60);
        map.retain(|_, last| *last >= cutoff);
    }
    map.insert(host.to_string(), Instant::now());
}

// ── HTML text extraction ──

// ── HTML text extraction ──

fn extract_page_body(doc: &scraper::Html) -> String {
    use std::ops::Deref;

    fn collect(element: &scraper::ElementRef, output: &mut String) {
        let node = element.deref();
        for child in node.children() {
            match child.value() {
                scraper::node::Node::Text(text) => {
                    let trimmed = text.text.trim();
                    if !trimmed.is_empty() {
                        if !output.is_empty() && !output.ends_with('\n') {
                            output.push(' ');
                        }
                        output.push_str(trimmed);
                    }
                }
                scraper::node::Node::Element(el) => {
                    if HIDDEN_TAGS.contains(&el.name()) {
                        continue;
                    }
                    if BLOCK_ELEMENTS.contains(&el.name()) {
                        if !output.is_empty() && !output.ends_with('\n') {
                            output.push('\n');
                        }
                    }
                    if let Some(child_element) = scraper::ElementRef::wrap(child) {
                        collect(&child_element, output);
                    }
                }
                _ => {}
            }
        }
    }

    let mut result = String::new();
    collect(&doc.root_element(), &mut result);
    result
}

/// Truncate a string at a character boundary, appending a notice.
fn truncate_at(text: &str, max_chars: usize) -> String {
    let Some((idx, _)) = text.char_indices().nth(max_chars) else {
        return text.to_string();
    };
    format!(
        "{}\n\n... (truncated, full page is {} chars)",
        &text[..idx],
        text.chars().count(),
    )
}

// ── Tool definition ──

pub struct WebFetchTool;

impl Tool for WebFetchTool {
    fn name(&self) -> &str {
        "webfetch"
    }

    fn description(&self) -> &str {
        "Fetch and read a web page. Given a URL, returns the page content as readable text."
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The full URL to fetch (including scheme, e.g. https://example.com)"
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
            let url_text = args["url"]
                .as_str()
                .ok_or("missing required parameter 'url'")?;
            let max_length = args["max_length"].as_u64().unwrap_or(5000).min(100_000) as usize;

            let validated = validate_url(url_text)?;
            let host = validated.host_str().unwrap_or("");
            if !host.is_empty() {
                rate_limit(host).await;
            }

            let response = CLIENT
                .get(url_text)
                .send()
                .await
                .map_err(|e| format!("failed to fetch {url_text}: {e}"))?;

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
                .and_then(|value| value.to_str().ok())
                .unwrap_or("")
                .to_string();

            let mut body = Vec::new();
            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk.map_err(|e| format!("read error: {e}"))?;
                if body.len() + chunk.len() > MAX_BYTES as usize {
                    return Ok(ToolResult {
                        call_id: String::new(),
                        content: format!("page too large (exceeds {MAX_BYTES} bytes)"),
                        title: Some("page too large".into()),
                    });
                }
                body.extend_from_slice(&chunk);
            }

            let is_html = content_type.contains("text/html") || content_type.is_empty();
            let text = if is_html {
                let html = String::from_utf8_lossy(&body);
                let doc = scraper::Html::parse_document(&html);
                let mut result = String::new();

                if let Some(t) = doc.select(&TITLE_SELECTOR).next().and_then(|el| {
                    let t = el.text().collect::<String>();
                    (!t.trim().is_empty()).then(|| t.trim().to_string())
                }) {
                    result.push_str(&format!("# {t}\n\n"));
                }

                let page_text = extract_page_body(&doc);
                result.push_str(&if page_text.len() <= max_length {
                    page_text
                } else {
                    truncate_at(&page_text, max_length)
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
                title: Some(url_text.to_string()),
            })
        })
    }
}
