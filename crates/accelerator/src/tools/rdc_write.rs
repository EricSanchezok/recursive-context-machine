use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;
use tracing::{info, warn};

use super::{pluralize, url_encode, validate_path_id};

const TIMEOUT_SECS: u64 = 30;
const DEFAULT_RDC_URL: &str = "http://localhost:3000";

// ── rdc_write ──

pub struct RdcWriteTool;

impl Tool for RdcWriteTool {
    fn name(&self) -> &str {
        "rdc_write"
    }

    fn description(&self) -> &str {
        "Write data to the shared Research Data Center (RDC).\n\n\
         Available entity_types: ideas, claims, papers, paper_spines, tech_reports, \
         positionings, reviews, lit_papers\n\
         Available actions: create, update, select (for ideas), finalize (for claims)\n\n\
         Examples:\n\
         - Create a new idea: entity_type=\"ideas\", action=\"create\", data={title: \"...\", content: \"...\"}\n\
         - Select an idea: entity_type=\"ideas\", action=\"select\", entity_id=\"idea_001\"\n\
         - Create a claim: entity_type=\"claims\", action=\"create\", data={title: \"...\", statement: \"...\"}\n\
         - Create a review: entity_type=\"reviews\", action=\"create\", data={entityType: \"paper\", entityId: \"...\", ...}\n\n\
         IMPORTANT: Only write entities that belong to THIS phase's ownership."
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "entity_type": {
                    "type": "string",
                    "description": "Type of entity: ideas, claims, papers, paper_spines, tech_reports, positionings, reviews, lit_papers"
                },
                "action": {
                    "type": "string",
                    "description": "Action to perform: create, update, select, finalize"
                },
                "data": {
                    "type": "object",
                    "description": "The entity data to write (JSON object)"
                },
                "entity_id": {
                    "type": "string",
                    "description": "Entity ID for update/select/finalize actions"
                }
            },
            "required": ["entity_type", "action", "data"]
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
        Box::pin(async move { execute_write(args, env).await })
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

/// Build the REST endpoint for a given entity_type, action, and optional entity_id.
///
/// `research_id` and `entity_id` are interpolated directly into the URL path,
/// so both are validated against a tight `[A-Za-z0-9_-]` whitelist and
/// percent-encoded as defense-in-depth. This prevents path traversal
/// (`../`), query-string injection (`?`/`&`), and fragment injection (`#`)
/// from reaching the RDC server. See [`super::validate_path_id`].
///
/// # Concurrency caveat for the `update` action
///
/// When a `spawn`/map planner fans out workers that share one `Environment`
/// (and thus one `RDC_RESEARCH_ID`), concurrent `rdc_write` calls with
/// `action="update"` against the **same `entity_id`** race in the RDC: the
/// last PATCH wins and earlier writes are silently lost. This tool performs
/// no client-side locking. Workers emitted by a `map`/`spawn` fan-out should
/// therefore only `create` their own entities or `update` distinct entity IDs;
/// never have two workers `update` the same RDC entity in the same wave.
/// Coalescing writes to a shared entity must happen in a single serial
/// component downstream of the fan-out.
pub(crate) fn build_endpoint(
    entity_type: &str,
    action: &str,
    entity_id: Option<&str>,
    research_id: &str,
) -> Result<(String, &'static str), String> {
    validate_path_id(research_id)?;
    let research_id_enc = url_encode(research_id);
    let entity_plural = pluralize(entity_type);
    let base = format!("/api/v1/research/{research_id_enc}/{entity_plural}");

    match (action, entity_id) {
        // Create: POST /research/{id}/{entity_plural}
        ("create", _) => Ok((base, "POST")),

        // Select (ideas): POST /research/{id}/ideas/{eid}/select
        ("select", Some(eid)) if entity_type == "ideas" || entity_type == "idea" => {
            validate_path_id(eid)?;
            let eid_enc = url_encode(eid);
            Ok((format!("{base}/{eid_enc}/select"), "POST"))
        }

        // Finalize (claims): POST /research/{id}/claims/{eid}/finalize
        ("finalize", Some(eid)) if entity_type == "claims" || entity_type == "claim" => {
            validate_path_id(eid)?;
            let eid_enc = url_encode(eid);
            Ok((format!("{base}/{eid_enc}/finalize"), "POST"))
        }

        // Update: PATCH /research/{id}/{entity_plural}/{eid}
        ("update", Some(eid)) => {
            validate_path_id(eid)?;
            let eid_enc = url_encode(eid);
            Ok((format!("{base}/{eid_enc}"), "PATCH"))
        }

        // Update without entity_id
        ("update", None) => Err("rdc_write 'update' action requires 'entity_id'".to_string()),

        // Select/finalize without entity_id
        ("select", None) => Err("rdc_write 'select' action requires 'entity_id'".to_string()),
        ("finalize", None) => Err("rdc_write 'finalize' action requires 'entity_id'".to_string()),

        // Select on non-idea entities
        ("select", _) => {
            Err("rdc_write 'select' is only valid for entity_type='ideas'".to_string())
        }

        // Finalize on non-claim entities
        ("finalize", _) => {
            Err("rdc_write 'finalize' is only valid for entity_type='claims'".to_string())
        }

        _ => Err(format!(
            "rdc_write: unknown action '{action}' for entity_type '{entity_type}'"
        )),
    }
}

async fn execute_write(args: Value, env: &Environment) -> Result<ToolResult, String> {
    let entity_type = args["entity_type"]
        .as_str()
        .ok_or("rdc_write requires 'entity_type' parameter")?;

    let action = args["action"]
        .as_str()
        .ok_or("rdc_write requires 'action' parameter")?;

    let data = args
        .get("data")
        .ok_or("rdc_write requires 'data' parameter")?;

    let entity_id = args["entity_id"].as_str();

    let (url, research_id, token) = rdc_config(env);

    if research_id.is_empty() {
        return Err(
            "RDC_RESEARCH_ID is not set. Set it via env or in the pipeline environment."
                .to_string(),
        );
    }
    if token.is_none() {
        warn!(
            target: "rdc_write",
            "RDC_TOKEN is not set; RDC will reject this request with HTTP 401"
        );
    }

    let (endpoint, method) = build_endpoint(entity_type, action, entity_id, &research_id)?;

    let request_url = format!("{url}{endpoint}");

    info!(
        target: "rdc_write",
        url = %request_url,
        method,
        entity_type,
        action,
        auth = if token.is_some() { "bearer" } else { "none" },
        "writing to RDC"
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()
        .map_err(|e| format!("failed to create HTTP client: {e}"))?;

    let request = match method {
        "POST" => client.post(&request_url),
        "PATCH" => client.patch(&request_url),
        _ => {
            return Err(format!(
                "rdc_write: internal error — unsupported HTTP method '{method}'"
            ));
        }
    };

    // P0-12: forward RDC_TOKEN as a Bearer Authorization header. The RDC
    // REST API rejects every request without it (HTTP 401). When the token is
    // absent we still send the request so the caller sees the 401 and knows
    // to set RDC_TOKEN, but we warn above to make the root cause obvious.
    let request = match token.as_ref() {
        Some(t) => request.bearer_auth(t),
        None => request,
    };

    let response = request
        .header("Content-Type", "application/json")
        .json(data)
        .send()
        .await
        .map_err(|e| format!("rdc_write request failed: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(format!("rdc_write failed: HTTP {status} — {text}"));
    }

    let result_data: Value = response
        .json()
        .await
        .map_err(|e| format!("rdc_write response parse failed: {e}"))?;

    let created_id = result_data["id"].as_str().map(|s| s.to_string());
    let success_msg = match (action, &created_id) {
        ("create", Some(id)) => format!("Successfully created {entity_type} #{id}",),
        ("create", None) => format!("Successfully created {entity_type}"),
        ("update", Some(id)) => format!("Successfully updated {entity_type} #{id}"),
        ("update", None) => format!("Successfully updated {entity_type}"),
        _ => format!(
            "Successfully executed {action} on {entity_type}{}",
            entity_id.map_or(String::new(), |eid| format!(" #{eid}"))
        ),
    };

    info!(target: "rdc_write", entity_type, action, "rdc_write complete");

    Ok(ToolResult {
        call_id: String::new(),
        content: success_msg,
        title: Some(format!("RDC {action} {entity_type}")),
    })
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── pluralize ──

    #[test]
    fn pluralize_idea_singular() {
        assert_eq!(pluralize("idea"), "ideas");
    }

    #[test]
    fn pluralize_claim_singular() {
        assert_eq!(pluralize("claim"), "claims");
    }

    #[test]
    fn pluralize_paper_singular() {
        assert_eq!(pluralize("paper"), "papers");
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
    fn pluralize_positioning_singular() {
        assert_eq!(pluralize("positioning"), "positionings");
    }

    #[test]
    fn pluralize_lit_papers_to_literature() {
        assert_eq!(pluralize("lit_papers"), "literature");
    }

    #[test]
    fn pluralize_research_invariant() {
        assert_eq!(pluralize("research"), "research");
    }

    // ── build_endpoint: create ──

    #[test]
    fn create_idea_endpoint() {
        let result = build_endpoint("ideas", "create", None, "res_1");
        assert!(result.is_ok());
        let (endpoint, method) = result.unwrap();
        assert_eq!(endpoint, "/api/v1/research/res_1/ideas");
        assert_eq!(method, "POST");
    }

    #[test]
    fn create_positioning_endpoint() {
        let result = build_endpoint("positionings", "create", None, "res_1");
        let (endpoint, method) = result.unwrap();
        assert_eq!(endpoint, "/api/v1/research/res_1/positionings");
        assert_eq!(method, "POST");
    }

    #[test]
    fn create_claim_endpoint() {
        let result = build_endpoint("claims", "create", None, "res_1");
        let (endpoint, method) = result.unwrap();
        assert_eq!(endpoint, "/api/v1/research/res_1/claims");
        assert_eq!(method, "POST");
    }

    // ── build_endpoint: update ──

    #[test]
    fn update_idea_endpoint() {
        let result = build_endpoint("ideas", "update", Some("idea_001"), "res_1");
        let (endpoint, method) = result.unwrap();
        assert_eq!(endpoint, "/api/v1/research/res_1/ideas/idea_001");
        assert_eq!(method, "PATCH");
    }

    #[test]
    fn update_without_entity_id_is_error() {
        let result = build_endpoint("ideas", "update", None, "res_1");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("'update' action requires 'entity_id'")
        );
    }

    // ── build_endpoint: select ──

    #[test]
    fn select_idea_endpoint() {
        let result = build_endpoint("ideas", "select", Some("idea_001"), "res_1");
        let (endpoint, method) = result.unwrap();
        assert_eq!(endpoint, "/api/v1/research/res_1/ideas/idea_001/select");
        assert_eq!(method, "POST");
    }

    #[test]
    fn select_without_entity_id_is_error() {
        let result = build_endpoint("ideas", "select", None, "res_1");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("'select' action requires 'entity_id'")
        );
    }

    #[test]
    fn select_on_non_idea_is_error() {
        let result = build_endpoint("claims", "select", Some("c1"), "res_1");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("only valid for entity_type='ideas'")
        );
    }

    // ── build_endpoint: finalize ──

    #[test]
    fn finalize_claim_endpoint() {
        let result = build_endpoint("claims", "finalize", Some("claim_001"), "res_1");
        let (endpoint, method) = result.unwrap();
        assert_eq!(endpoint, "/api/v1/research/res_1/claims/claim_001/finalize");
        assert_eq!(method, "POST");
    }

    #[test]
    fn finalize_without_entity_id_is_error() {
        let result = build_endpoint("claims", "finalize", None, "res_1");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("'finalize' action requires 'entity_id'")
        );
    }

    #[test]
    fn finalize_on_non_claim_is_error() {
        let result = build_endpoint("ideas", "finalize", Some("i1"), "res_1");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("only valid for entity_type='claims'")
        );
    }

    // ── build_endpoint: unknown action ──

    #[test]
    fn unknown_action_is_error() {
        let result = build_endpoint("ideas", "delete", Some("i1"), "res_1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown action"));
    }

    // ── build_endpoint: kebab-case plurals ──

    #[test]
    fn create_paper_spine_uses_kebab_path() {
        let result = build_endpoint("paper_spines", "create", None, "res_1");
        let (endpoint, _) = result.unwrap();
        assert_eq!(endpoint, "/api/v1/research/res_1/paper-spines");
    }

    #[test]
    fn create_tech_report_uses_kebab_path() {
        let result = build_endpoint("tech_reports", "create", None, "res_1");
        let (endpoint, _) = result.unwrap();
        assert_eq!(endpoint, "/api/v1/research/res_1/tech-reports");
    }

    // ── build_endpoint: path-segment validation (P0-13) ──

    #[test]
    fn rejects_research_id_with_slash() {
        // A research_id containing '/' must not reach the URL builder.
        let result = build_endpoint("ideas", "create", None, "../evil");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("illegal character"));
    }

    #[test]
    fn rejects_entity_id_with_path_traversal() {
        let result = build_endpoint("ideas", "update", Some("../etc/passwd"), "res_1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("illegal character"));
    }

    #[test]
    fn rejects_entity_id_with_encoded_slash() {
        // %2F must be rejected character-by-character (the % is illegal).
        let result = build_endpoint("ideas", "update", Some("a%2Fb"), "res_1");
        assert!(result.is_err());
    }

    #[test]
    fn rejects_select_entity_id_with_query_injection() {
        let result = build_endpoint("ideas", "select", Some("evil?admin=1"), "res_1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("illegal character"));
    }

    #[test]
    fn rejects_finalize_entity_id_with_fragment() {
        let result = build_endpoint("claims", "finalize", Some("c#frag"), "res_1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("illegal character"));
    }

    #[test]
    fn build_endpoint_safe_ids_still_match_pre_fix_urls() {
        // Regression: the encoding must be a no-op for the IDs the test
        // suite already used, so existing happy-path expectations still hold.
        let (ep_create, _) = build_endpoint("ideas", "create", None, "res_1").unwrap();
        assert_eq!(ep_create, "/api/v1/research/res_1/ideas");
        let (ep_update, _) = build_endpoint("ideas", "update", Some("idea_001"), "res_1").unwrap();
        assert_eq!(ep_update, "/api/v1/research/res_1/ideas/idea_001");
    }
}
