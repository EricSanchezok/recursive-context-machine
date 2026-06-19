use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;
use tracing::info;

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

fn rdc_config(env: &Environment) -> (String, String) {
    let url = env
        .vars
        .get("RDC_URL")
        .cloned()
        .unwrap_or_else(|| DEFAULT_RDC_URL.to_string());
    let research_id = env.vars.get("RDC_RESEARCH_ID").cloned().unwrap_or_default();
    (url, research_id)
}

/// Build the REST endpoint for a given entity_type, action, and optional entity_id.
pub(crate) fn build_endpoint(
    entity_type: &str,
    action: &str,
    entity_id: Option<&str>,
    research_id: &str,
) -> Result<(String, &'static str), String> {
    let entity_plural = pluralize(entity_type);
    let base = format!("/api/v1/research/{research_id}/{entity_plural}");

    match (action, entity_id) {
        // Create: POST /research/{id}/{entity_plural}
        ("create", _) => Ok((base, "POST")),

        // Select (ideas): POST /research/{id}/ideas/{eid}/select
        ("select", Some(eid)) if entity_type == "ideas" || entity_type == "idea" => {
            Ok((format!("{base}/{eid}/select"), "POST"))
        }

        // Finalize (claims): POST /research/{id}/claims/{eid}/finalize
        ("finalize", Some(eid)) if entity_type == "claims" || entity_type == "claim" => {
            Ok((format!("{base}/{eid}/finalize"), "POST"))
        }

        // Update: PATCH /research/{id}/{entity_plural}/{eid}
        ("update", Some(eid)) => Ok((format!("{base}/{eid}"), "PATCH")),

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

/// Map entity_type singular to its REST plural form.
pub(crate) fn pluralize(entity_type: &str) -> &str {
    match entity_type {
        "research" => "research",
        "ideas" | "idea" => "ideas",
        "claims" | "claim" => "claims",
        "experiments" | "experiment" => "experiments",
        "papers" | "paper" => "papers",
        "paper_spines" | "paper_spine" => "paper-spines",
        "tech_reports" | "tech_report" => "tech-reports",
        "positionings" | "positioning" => "positionings",
        "reviews" | "review" => "reviews",
        "lit_papers" => "literature",
        "lit_search" => "literature",
        // RDC exposes gate records at /gates (not /gate_records). rdc_read::pluralize
        // already maps this; rdc_write must too, or gate writes 404.
        "gate_records" | "gate_record" => "gates",
        _ => entity_type,
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

    let (url, research_id) = rdc_config(env);
    // Bearer token for RDC instances that enforce auth (RDC_AUTH_ENFORCE=true).
    // Optional: when unset (trusted/local RDC) requests go out unauthenticated.
    let token = env.vars.get("RDC_TOKEN").cloned();

    if research_id.is_empty() {
        return Err(
            "RDC_RESEARCH_ID is not set. Set it via env or in the pipeline environment."
                .to_string(),
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
        "writing to RDC"
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()
        .map_err(|e| format!("failed to create HTTP client: {e}"))?;

    let mut request = match method {
        "POST" => client.post(&request_url),
        "PATCH" => client.patch(&request_url),
        _ => {
            return Err(format!(
                "rdc_write: internal error — unsupported HTTP method '{method}'"
            ));
        }
    };
    if let Some(t) = &token {
        request = request.bearer_auth(t);
    }

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
}
