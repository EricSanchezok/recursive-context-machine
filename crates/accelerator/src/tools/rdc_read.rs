use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;
use tracing::info;

const TIMEOUT_SECS: u64 = 30;
const DEFAULT_RDC_URL: &str = "http://localhost:3000";

// ── rdc_read ──

pub struct RdcReadTool;

impl Tool for RdcReadTool {
    fn name(&self) -> &str {
        "rdc_read"
    }

    fn description(&self) -> &str {
        "Read data from the shared Research Data Center (RDC).\n\n\
         Available entity_types:\n\
         - research: Current research project state (anchor, config, counters)\n\
         - ideas: All research ideas with statuses\n\
         - claims: Research claims with evidence links\n\
         - experiments: Experiment records with results\n\
         - lit_papers: Literature paper knowledge base\n\
         - lit_search: Search literature by query\n\n\
         Examples:\n\
         - To get the current research anchor: entity_type=\"research\"\n\
         - To list selected ideas: entity_type=\"ideas\", filter_status=\"selected\"\n\
         - To search literature: entity_type=\"lit_search\", search_query=\"attention mechanism\""
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "entity_type": {
                    "type": "string",
                    "description": "Type of entity to read: research, ideas, claims, experiments, lit_papers, lit_search"
                },
                "entity_id": {
                    "type": "string",
                    "description": "Specific entity ID to fetch (optional)"
                },
                "filter_status": {
                    "type": "string",
                    "description": "Filter by status (optional, e.g. 'selected', 'active', 'finalized')"
                },
                "filter_limit": {
                    "type": "integer",
                    "description": "Maximum number of results to return (optional, default 20)"
                },
                "search_query": {
                    "type": "string",
                    "description": "Search query string (for lit_search entity_type)"
                }
            },
            "required": ["entity_type"]
        })
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(TIMEOUT_SECS)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move { execute_read(args, env).await })
    }
}

fn rdc_config(env: &Environment) -> (String, String) {
    let url = env
        .vars
        .get("RDC_URL")
        .cloned()
        .unwrap_or_else(|| DEFAULT_RDC_URL.to_string());
    let research_id = env
        .vars
        .get("RDC_RESEARCH_ID")
        .cloned()
        .unwrap_or_default();
    (url, research_id)
}

async fn execute_read(args: Value, env: &Environment) -> Result<ToolResult, String> {
    let entity_type = args["entity_type"]
        .as_str()
        .ok_or("rdc_read requires 'entity_type' parameter")?;

    let (url, research_id) = rdc_config(env);

    if research_id.is_empty() {
        return Err(
            "RDC_RESEARCH_ID is not set. Set it via env or in the pipeline environment."
                .to_string(),
        );
    }

    let entity_id = args["entity_id"].as_str();
    let filter_status = args["filter_status"].as_str();
    let filter_limit = args["filter_limit"].as_u64().unwrap_or(20);
    let search_query = args["search_query"].as_str();

    // Build the URL with query parameters.
    let endpoint = if entity_type == "lit_search" {
        format!("/api/v1/research/{research_id}/literature/search")
    } else if let Some(eid) = entity_id {
        let entity_plural = pluralize(entity_type);
        format!("/api/v1/research/{research_id}/{entity_plural}/{eid}")
    } else {
        let entity_plural = pluralize(entity_type);
        format!("/api/v1/research/{research_id}/{entity_plural}")
    };

    let mut query_parts: Vec<String> = Vec::new();
    if let Some(status) = filter_status {
        query_parts.push(format!("status={}", url_encode(status)));
    }
    query_parts.push(format!("limit={filter_limit}"));
    if let Some(query) = search_query {
        query_parts.push(format!("q={}", url_encode(query)));
    }

    let request_url = if query_parts.is_empty() {
        format!("{url}{endpoint}")
    } else {
        format!("{url}{endpoint}?{}", query_parts.join("&"))
    };

    info!(target: "rdc_read", url = %request_url, entity_type, "reading from RDC");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()
        .map_err(|e| format!("failed to create HTTP client: {e}"))?;

    let response = client
        .get(&request_url)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("rdc_read request failed: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(format!("rdc_read failed: HTTP {status} — {text}"));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("rdc_read response parse failed: {e}"))?;

    // Format the response for the LLM.
    let formatted = format_response(entity_type, &data);

    info!(target: "rdc_read", entity_type, "rdc_read complete");

    Ok(ToolResult {
        call_id: String::new(),
        content: formatted,
        title: Some(format!(
            "RDC {} {}",
            entity_type,
            entity_id.map_or(String::new(), |eid| format!("({eid})"))
        )),
    })
}

/// Simple percent-encode for query parameter values.
fn url_encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for byte in input.bytes() {
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

/// Map entity_type singular to its REST plural form.
fn pluralize(entity_type: &str) -> &str {
    match entity_type {
        "research" => "research",
        "ideas" | "idea" => "ideas",
        "claims" | "claim" => "claims",
        "experiments" | "experiment" => "experiments",
        "lit_papers" | "lit_search" => "literature",
        _ => entity_type,
    }
}

/// Format the JSON response into readable text for the LLM.
fn format_response(entity_type: &str, data: &Value) -> String {
    match entity_type {
        "research" => {
            let anchor = data
                .get("anchor")
                .and_then(|v| v.as_str())
                .unwrap_or("(not set)");
            let status = data
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            format!(
                "Research State:\n  Status: {status}\n  Anchor: {anchor}"
            )
        }
        "ideas" | "idea" | "claims" | "claim" | "experiments" | "experiment" => {
            let results = data.get("items").unwrap_or(data);
            let array = results.as_array();
            match array {
                Some(items) if !items.is_empty() => {
                    let mut lines = vec![format!("{} results:\n", items.len())];
                    for item in items {
                        let id = item["id"].as_str().unwrap_or("?");
                        let title = item["title"].as_str().unwrap_or(
                            item["statement"].as_str().unwrap_or("(untitled)"),
                        );
                        let status = item["status"].as_str().unwrap_or("-");
                        lines.push(format!("  [{status}] {id}: {title}"));
                    }
                    lines.join("\n")
                }
                _ => {
                    let total = data["total"].as_u64().unwrap_or(0);
                    format!("No {} found (total: {total}).", entity_type)
                }
            }
        }
        "lit_papers" | "lit_search" => {
            let results = data.get("items").unwrap_or(data);
            let array = results.as_array();
            match array {
                Some(items) if !items.is_empty() => {
                    let mut lines = vec![format!("{} papers:\n", items.len())];
                    for item in items {
                        let id = item["id"].as_str().unwrap_or("?");
                        let title = item["title"].as_str().unwrap_or("(untitled)");
                        let authors = item["authors"]
                            .as_str()
                            .map(|s| s.to_string())
                            .or_else(|| {
                                item["authors"].as_array().map(|a| {
                                    a.iter()
                                        .filter_map(|v| v.as_str())
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                })
                            })
                            .unwrap_or_else(|| "(unknown)".to_string());
                        let year = item["year"]
                            .as_u64()
                            .map(|y| y.to_string())
                            .unwrap_or_default();
                        lines.push(format!("  {id}: {title} ({authors}, {year})"));
                    }
                    lines.join("\n")
                }
                _ => "No papers found.".to_string(),
            }
        }
        _ => {
            // Fallback: pretty-print the JSON.
            serde_json::to_string_pretty(data)
                .unwrap_or_else(|_| "Invalid response".to_string())
        }
    }
}
