use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;
use tracing::{info, warn};

use super::{pluralize, url_encode, validate_path_id};

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

/// Resolve RDC connection settings from the environment.
///
/// Returns `(url, research_id, token)` where `token` is `Some` only when
/// `RDC_TOKEN` is set. The token is forwarded to the RDC REST API as a
/// `Bearer` Authorization header; without it the API rejects every call
/// with HTTP 401.
fn rdc_config(env: &Environment) -> (String, String, Option<String>) {
    let url = env
        .vars
        .get("RDC_URL")
        .cloned()
        .unwrap_or_else(|| DEFAULT_RDC_URL.to_string());
    let research_id = env.vars.get("RDC_RESEARCH_ID").cloned().unwrap_or_default();
    let token = env.vars.get("RDC_TOKEN").cloned().filter(|t| !t.is_empty());
    (url, research_id, token)
}

async fn execute_read(args: Value, env: &Environment) -> Result<ToolResult, String> {
    let entity_type = args["entity_type"]
        .as_str()
        .ok_or("rdc_read requires 'entity_type' parameter")?;

    let (url, research_id, token) = rdc_config(env);

    if research_id.is_empty() {
        return Err(
            "RDC_RESEARCH_ID is not set. Set it via env or in the pipeline environment."
                .to_string(),
        );
    }
    // P0-13: research_id is interpolated into the URL path; validate it
    // against a tight whitelist before encoding so traversal payloads never
    // reach the RDC server.
    validate_path_id(&research_id)?;
    let research_id_enc = url_encode(&research_id);
    if token.is_none() {
        warn!(
            target: "rdc_read",
            "RDC_TOKEN is not set; RDC will reject this request with HTTP 401"
        );
    }

    let entity_id = args["entity_id"].as_str();
    let filter_status = args["filter_status"].as_str();
    let filter_limit = args["filter_limit"].as_u64().unwrap_or(20);
    let search_query = args["search_query"].as_str();

    // Build the endpoint URL. entity_id is also validated + percent-encoded
    // when present (P0-13). Special cases: research uses the base path;
    // lit_search uses /literature/search.
    let (endpoint, skip_query) = if entity_type == "lit_search" {
        (
            format!("/api/v1/research/{research_id_enc}/literature/search"),
            false,
        )
    } else if entity_type == "research" {
        // Research is the base endpoint itself — no query params needed
        (format!("/api/v1/research/{research_id_enc}"), true)
    } else if let Some(eid) = entity_id {
        validate_path_id(eid)?;
        let eid_enc = url_encode(eid);
        let entity_plural = pluralize(entity_type);
        (
            format!("/api/v1/research/{research_id_enc}/{entity_plural}/{eid_enc}"),
            false,
        )
    } else {
        let entity_plural = pluralize(entity_type);
        (
            format!("/api/v1/research/{research_id_enc}/{entity_plural}"),
            false,
        )
    };

    let request_url = if skip_query {
        format!("{url}{endpoint}")
    } else {
        let mut query_parts: Vec<String> = Vec::new();
        if let Some(status) = filter_status {
            query_parts.push(format!("status={}", url_encode(status)));
        }
        query_parts.push(format!("limit={filter_limit}"));
        if let Some(query) = search_query {
            query_parts.push(format!("q={}", url_encode(query)));
        }
        if query_parts.is_empty() {
            format!("{url}{endpoint}")
        } else {
            format!("{url}{endpoint}?{}", query_parts.join("&"))
        }
    };

    info!(
        target: "rdc_read",
        url = %request_url,
        entity_type,
        auth = if token.is_some() { "bearer" } else { "none" },
        "reading from RDC"
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()
        .map_err(|e| format!("failed to create HTTP client: {e}"))?;

    // P0-12: forward RDC_TOKEN as a Bearer Authorization header. The RDC
    // REST API rejects every request without it (HTTP 401). When the token is
    // absent we still send the request so the caller sees the 401 and knows
    // to set RDC_TOKEN, but we warn above to make the root cause obvious.
    let request = match token.as_ref() {
        Some(t) => client.get(&request_url).bearer_auth(t),
        None => client.get(&request_url),
    };

    let response = request
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

/// Format the JSON response into readable text for the LLM.
fn format_response(entity_type: &str, data: &Value) -> String {
    match entity_type {
        "research" => {
            let project = data
                .get("project")
                .and_then(|v| v.as_str())
                .unwrap_or("(unknown)");
            let anchor = data
                .get("anchor")
                .and_then(|v| v.as_str())
                .unwrap_or("(not set)");
            let counters = data
                .get("counters")
                .and_then(|v| v.as_object())
                .map(|o| {
                    o.iter()
                        .map(|(k, v)| format!("  {k}: {v}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .unwrap_or_else(|| "  (none)".to_string());
            format!(
                "Research State:\n  Project: {project}\n  Anchor: {anchor}\n\nCounters:\n{counters}"
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
                        let title = item["title"]
                            .as_str()
                            .unwrap_or(item["statement"].as_str().unwrap_or("(untitled)"));
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
                        let slug = item["slug"].as_str().or(item["id"].as_str()).unwrap_or("?");
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
                        lines.push(format!("  {slug}: {title} ({authors}, {year})"));
                    }
                    lines.join("\n")
                }
                _ => "No papers found.".to_string(),
            }
        }
        _ => {
            // Fallback: pretty-print the JSON.
            serde_json::to_string_pretty(data).unwrap_or_else(|_| "Invalid response".to_string())
        }
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pluralize_idea_singular() {
        assert_eq!(pluralize("idea"), "ideas");
    }

    #[test]
    fn pluralize_ideas_already_plural() {
        assert_eq!(pluralize("ideas"), "ideas");
    }

    #[test]
    fn pluralize_claim_singular() {
        assert_eq!(pluralize("claim"), "claims");
    }

    #[test]
    fn pluralize_claims_already_plural() {
        assert_eq!(pluralize("claims"), "claims");
    }

    #[test]
    fn pluralize_experiment_singular() {
        assert_eq!(pluralize("experiment"), "experiments");
    }

    #[test]
    fn pluralize_experiments_already_plural() {
        assert_eq!(pluralize("experiments"), "experiments");
    }

    #[test]
    fn pluralize_research() {
        assert_eq!(pluralize("research"), "research");
    }

    #[test]
    fn pluralize_lit_papers_to_literature() {
        assert_eq!(pluralize("lit_papers"), "literature");
    }

    #[test]
    fn pluralize_lit_search_to_literature() {
        assert_eq!(pluralize("lit_search"), "literature");
    }

    #[test]
    fn pluralize_unknown_entity_is_identity() {
        assert_eq!(pluralize("unknown"), "unknown");
    }

    #[test]
    fn url_encode_alphanumeric_only() {
        assert_eq!(url_encode("hello123"), "hello123");
    }

    #[test]
    fn url_encode_encodes_space() {
        assert_eq!(url_encode("attention mechanism"), "attention%20mechanism");
    }

    #[test]
    fn url_encode_encodes_special_chars() {
        assert_eq!(url_encode("a&b=c"), "a%26b%3Dc");
    }

    #[test]
    fn url_encode_preserves_unreserved_chars() {
        assert_eq!(url_encode("test-_.~name"), "test-_.~name");
    }

    #[test]
    fn url_encode_empty_string() {
        assert_eq!(url_encode(""), "");
    }

    #[test]
    fn pluralize_paper_spine_kebab() {
        assert_eq!(pluralize("paper_spine"), "paper-spines");
    }

    #[test]
    fn pluralize_tech_report_kebab() {
        assert_eq!(pluralize("tech_report"), "tech-reports");
    }

    #[test]
    fn pluralize_story_spine_to_kebab() {
        assert_eq!(pluralize("story_spine"), "story-spines");
    }

    #[test]
    fn pluralize_positioning_singular() {
        assert_eq!(pluralize("positioning"), "positionings");
    }

    #[test]
    fn pluralize_gate_record_to_gates() {
        assert_eq!(pluralize("gate_record"), "gates");
    }
}
