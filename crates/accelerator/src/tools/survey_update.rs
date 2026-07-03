use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;
use tracing::{info, warn};

use super::{url_encode, validate_path_id};

const TIMEOUT_SECS: u64 = 30;
const DEFAULT_RDC_URL: &str = "http://localhost:3000";

pub struct SurveyUpdateTool;

impl Tool for SurveyUpdateTool {
    fn name(&self) -> &str {
        "survey_update"
    }

    fn description(&self) -> &str {
        "Update the RDC survey run record during a pipeline execution.\n\n\
         Actions:\n\
         - update_run: PATCH current_phase, progress, or status of a survey run\n\
         - create_event: POST a component event (component_start, component_done, component_error)\n\
         - create_asset: POST a stage asset (markdown content or file path) for the survey run\n\
         - check_status: GET the current survey run status (useful for pause/resume checks)\n\n\
         Examples:\n\
         - update_run: { action=\"update_run\", data={ current_phase=\"discovery\", progress=15 } }\n\
         - create_event: { action=\"create_event\", data={ type=\"component_start\", stage=\"discovery\" } }\n\
         - create_asset: { action=\"create_asset\", data={ stage=\"discovery\", name=\"03_expansion.md\", path=\"03_expansion.md\" } }\n\
         - check_status: { action=\"check_status\" }"
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["update_run", "create_event", "create_asset", "check_status"],
                    "description": "Which survey operation to perform"
                },
                "data": {
                    "type": "object",
                    "description": "Payload for the operation (see action descriptions)"
                }
            },
            "required": ["action", "data"]
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
        Box::pin(async move { execute_survey_update(args, env).await })
    }
}

fn rdc_config(env: &Environment) -> (String, String, String, Option<String>) {
    let url = env
        .vars
        .get("RDC_URL")
        .cloned()
        .unwrap_or_else(|| DEFAULT_RDC_URL.to_string());
    let research_id = env.vars.get("RDC_RESEARCH_ID").cloned().unwrap_or_default();
    let run_id = env
        .vars
        .get("RDC_SURVEY_RUN_ID")
        .cloned()
        .unwrap_or_default();
    let token = env.vars.get("RDC_TOKEN").cloned().filter(|t| !t.is_empty());
    (url, research_id, run_id, token)
}

async fn execute_survey_update(args: Value, env: &Environment) -> Result<ToolResult, String> {
    let action = args["action"]
        .as_str()
        .ok_or("survey_update requires 'action' parameter")?;
    let data = args
        .get("data")
        .ok_or("survey_update requires 'data' parameter")?;

    let (url, research_id, run_id, token) = rdc_config(env);
    if research_id.is_empty() {
        return Err(
            "RDC_RESEARCH_ID is not set. Set it via env or in the pipeline environment."
                .to_string(),
        );
    }
    if run_id.is_empty() {
        return Err(
            "RDC_SURVEY_RUN_ID is not set. Set it via env or in the pipeline environment."
                .to_string(),
        );
    }
    validate_path_id(&research_id)?;
    validate_path_id(&run_id)?;

    let research_id_enc = url_encode(&research_id);
    let run_id_enc = url_encode(&run_id);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()
        .map_err(|e| format!("failed to create HTTP client: {e}"))?;

    if token.is_none() {
        warn!(
            target: "survey_update",
            "RDC_TOKEN is not set; RDC will reject this request with HTTP 401"
        );
    }

    match action {
        "update_run" => {
            let payload = build_update_payload(data)?;
            let endpoint =
                format!("{url}/api/v1/research/{research_id_enc}/survey-runs/{run_id_enc}");
            info!(
                target: "survey_update",
                url = %endpoint,
                "updating survey run"
            );
            let mut request = client.patch(&endpoint);
            if let Some(t) = token.as_ref() {
                request = request.bearer_auth(t);
            }
            let response = request
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await
                .map_err(|e| format!("survey_update request failed: {e}"))?;
            if !response.status().is_success() {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                return Err(format!("survey_update failed: HTTP {status} — {text}"));
            }
            Ok(ToolResult {
                call_id: String::new(),
                content: "Survey run updated".to_string(),
                title: Some("Survey update".to_string()),
            })
        }
        "create_event" => {
            let payload = build_event_payload(data)?;
            let endpoint =
                format!("{url}/api/v1/research/{research_id_enc}/survey-runs/{run_id_enc}/events");
            info!(
                target: "survey_update",
                url = %endpoint,
                "creating survey event"
            );
            let mut request = client.post(&endpoint);
            if let Some(t) = token.as_ref() {
                request = request.bearer_auth(t);
            }
            let response = request
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await
                .map_err(|e| format!("survey_update request failed: {e}"))?;
            if !response.status().is_success() {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                return Err(format!("survey_update failed: HTTP {status} — {text}"));
            }
            Ok(ToolResult {
                call_id: String::new(),
                content: format!(
                    "Event {} for stage {} recorded",
                    payload["type"], payload["stage"]
                ),
                title: Some("Survey event".to_string()),
            })
        }
        "create_asset" => {
            let payload = build_asset_payload(data, env).await?;
            let endpoint =
                format!("{url}/api/v1/research/{research_id_enc}/survey-runs/{run_id_enc}/assets");
            info!(
                target: "survey_update",
                url = %endpoint,
                "creating survey asset"
            );
            let mut request = client.post(&endpoint);
            if let Some(t) = token.as_ref() {
                request = request.bearer_auth(t);
            }
            let response = request
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await
                .map_err(|e| format!("survey_update request failed: {e}"))?;
            if !response.status().is_success() {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                return Err(format!("survey_update failed: HTTP {status} — {text}"));
            }
            Ok(ToolResult {
                call_id: String::new(),
                content: format!(
                    "Asset {} for stage {} uploaded",
                    payload["name"], payload["stage"]
                ),
                title: Some("Survey asset".to_string()),
            })
        }
        "check_status" => {
            let endpoint =
                format!("{url}/api/v1/research/{research_id_enc}/survey-runs/{run_id_enc}");
            info!(
                target: "survey_update",
                url = %endpoint,
                "checking survey run status"
            );
            let mut request = client.get(&endpoint);
            if let Some(t) = token.as_ref() {
                request = request.bearer_auth(t);
            }
            let response = request
                .header("Content-Type", "application/json")
                .send()
                .await
                .map_err(|e| format!("survey_update request failed: {e}"))?;
            if !response.status().is_success() {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                return Err(format!("survey_update failed: HTTP {status} — {text}"));
            }
            let run: Value = response
                .json()
                .await
                .map_err(|e| format!("survey_update response parse failed: {e}"))?;
            let status = run["status"].as_str().unwrap_or("unknown");
            let current_phase = run["currentPhase"].as_str().unwrap_or("-");
            let progress = run["progress"].as_u64().unwrap_or(0);
            Ok(ToolResult {
                call_id: String::new(),
                content: format!(
                    "status={status}, current_phase={current_phase}, progress={progress}%"
                ),
                title: Some("Survey status".to_string()),
            })
        }
        _ => Err(format!("survey_update: unknown action '{action}'")),
    }
}

fn build_update_payload(data: &Value) -> Result<Value, String> {
    let mut payload = serde_json::Map::new();
    if let Some(status) = data.get("status").and_then(|v| v.as_str()) {
        payload.insert(
            "status".to_string(),
            serde_json::Value::String(status.to_string()),
        );
    }
    if let Some(current_phase) = data.get("current_phase").and_then(|v| v.as_str()) {
        payload.insert(
            "current_phase".to_string(),
            serde_json::Value::String(current_phase.to_string()),
        );
    }
    if let Some(progress) = data.get("progress").and_then(|v| v.as_u64()) {
        payload.insert(
            "progress".to_string(),
            serde_json::Value::Number(progress.into()),
        );
    }
    if payload.is_empty() {
        return Err(
            "survey_update update_run requires at least one of status, current_phase, progress"
                .to_string(),
        );
    }
    Ok(Value::Object(payload))
}

fn build_event_payload(data: &Value) -> Result<HashMap<String, Value>, String> {
    let event_type = data["type"]
        .as_str()
        .ok_or("survey_update create_event requires 'type'")?;
    if !matches!(
        event_type,
        "component_start" | "component_done" | "component_error"
    ) {
        return Err(format!(
            "survey_update event type must be one of component_start, component_done, component_error; got {event_type}"
        ));
    }
    let stage = data["stage"]
        .as_str()
        .ok_or("survey_update create_event requires 'stage'")?;
    let mut payload = HashMap::new();
    payload.insert("type".to_string(), Value::String(event_type.to_string()));
    payload.insert("stage".to_string(), Value::String(stage.to_string()));
    if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
        payload.insert("message".to_string(), Value::String(message.to_string()));
    }
    Ok(payload)
}

async fn build_asset_payload(
    data: &Value,
    env: &Environment,
) -> Result<HashMap<String, Value>, String> {
    let stage = data["stage"]
        .as_str()
        .ok_or("survey_update create_asset requires 'stage'")?;
    let name = data["name"]
        .as_str()
        .ok_or("survey_update create_asset requires 'name'")?;

    let mut payload = HashMap::new();
    payload.insert("stage".to_string(), Value::String(stage.to_string()));
    payload.insert("name".to_string(), Value::String(name.to_string()));

    if let Some(content) = data.get("content_md").and_then(|v| v.as_str()) {
        payload.insert("content_md".to_string(), Value::String(content.to_string()));
    } else if let Some(path) = data.get("path").and_then(|v| v.as_str()) {
        let resolved = resolve_path(path, env);
        if !resolved.exists() {
            return Err(format!(
                "survey_update asset path does not exist: {resolved:?}"
            ));
        }
        let content = tokio::fs::read_to_string(&resolved)
            .await
            .map_err(|e| format!("survey_update failed to read asset {resolved:?}: {e}"))?;
        payload.insert("content_md".to_string(), Value::String(content));
    } else {
        return Err(
            "survey_update create_asset requires either 'content_md' or 'path'".to_string(),
        );
    }

    Ok(payload)
}

fn resolve_path(path: &str, env: &Environment) -> std::path::PathBuf {
    let p = std::path::Path::new(path);
    if p.is_absolute() {
        p.to_path_buf()
    } else if let Some(ref run_dir) = env.run_dir {
        run_dir.join(path)
    } else if !env.cwd.as_os_str().is_empty() {
        env.cwd.join(path)
    } else {
        p.to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_update_payload_requires_a_field() {
        let data = serde_json::json!({});
        assert!(build_update_payload(&data).is_err());
    }

    #[test]
    fn build_update_payload_accepts_all_fields() {
        let data = serde_json::json!({
            "status": "running",
            "current_phase": "discovery",
            "progress": 15
        });
        let payload = build_update_payload(&data).unwrap();
        assert_eq!(payload["status"], "running");
        assert_eq!(payload["current_phase"], "discovery");
        assert_eq!(payload["progress"], 15);
    }

    #[test]
    fn build_event_payload_validates_type() {
        let data = serde_json::json!({"type": "component_done", "stage": "discovery"});
        let payload = build_event_payload(&data).unwrap();
        assert_eq!(payload["type"], "component_done");
        assert_eq!(payload["stage"], "discovery");
    }

    #[test]
    fn build_event_payload_rejects_unknown_type() {
        let data = serde_json::json!({"type": "unknown", "stage": "discovery"});
        assert!(build_event_payload(&data).is_err());
    }
}
