use std::collections::HashMap;
use std::sync::mpsc;
use std::time::Duration;

use chrono::Utc;
use reqwest::Client;
use tracing::{error, info, warn};

use super::{ComponentEvent, HookEvent, HookKind, MachineEvent};

const TIMEOUT_SECS: u64 = 30;
const DEFAULT_RDC_URL: &str = "http://localhost:3000";

/// Non-blocking RDC reporter that follows a running RCM survey pipeline and
/// mirrors its component lifecycle to the RDC backend:
/// - writes `component_start` / `component_done` events
/// - updates `survey_runs.current_phase` and `progress`
/// - uploads known stage artifacts when a component finishes
///
/// Activation requires these environment variables:
/// - `RDC_URL` (optional, defaults to http://localhost:3000)
/// - `RDC_TOKEN` (optional)
/// - `RDC_RESEARCH_ID`
/// - `RDC_SURVEY_RUN_ID`
/// - `RCM_RUN_DIR` (required for asset upload)
pub struct RdcReporter {
    rx: mpsc::Receiver<HookEvent>,
    client: Client,
    url: String,
    research_id: String,
    run_id: String,
    token: Option<String>,
    run_dir: Option<std::path::PathBuf>,
    stage_order: Vec<&'static str>,
    stage_assets: HashMap<&'static str, &'static str>,
    current_stage: Option<String>,
}

impl RdcReporter {
    pub fn new(
        rx: mpsc::Receiver<HookEvent>,
        url: String,
        research_id: String,
        run_id: String,
        token: Option<String>,
        run_dir: Option<std::path::PathBuf>,
    ) -> Self {
        Self {
            rx,
            client: Client::builder()
                .timeout(Duration::from_secs(TIMEOUT_SECS))
                .build()
                .expect("reqwest client"),
            url,
            research_id,
            run_id,
            token,
            run_dir,
            stage_order: default_stage_order(),
            stage_assets: default_stage_assets(),
            current_stage: None,
        }
    }

    pub fn from_env(rx: mpsc::Receiver<HookEvent>) -> Option<Self> {
        let url = std::env::var("RDC_URL")
            .ok()
            .unwrap_or_else(|| DEFAULT_RDC_URL.to_string());
        let research_id = std::env::var("RDC_RESEARCH_ID").ok()?;
        let run_id = std::env::var("RDC_SURVEY_RUN_ID").ok()?;
        let token = std::env::var("RDC_TOKEN").ok().filter(|t| !t.is_empty());
        let run_dir = std::env::var("RCM_RUN_DIR")
            .ok()
            .map(std::path::PathBuf::from);
        Some(Self::new(rx, url, research_id, run_id, token, run_dir))
    }

    pub async fn run(mut self) {
        info!(target: "rdc_reporter", "started");
        for event in self.rx.iter() {
            if let Err(e) = self.handle_event(&event).await {
                warn!(target: "rdc_reporter", error = %e, "event failed");
            }
        }
        info!(target: "rdc_reporter", "stopped");
    }

    async fn handle_event(&mut self, event: &HookEvent) -> Result<(), String> {
        match &event.kind {
            HookKind::Component(ComponentEvent::Start(meta)) => {
                let stage = normalize_stage(&meta.name);
                self.current_stage = Some(stage.to_string());
                let progress = stage_progress(stage, &self.stage_order);
                self.update_run(stage, Some(progress)).await?;
                self.create_event("component_start", stage, None).await?;
            }
            HookKind::Component(ComponentEvent::Done(meta)) => {
                let stage = normalize_stage(&meta.name);
                let progress = stage_progress(stage, &self.stage_order);
                self.update_run(stage, Some(progress)).await?;
                self.create_event("component_done", stage, None).await?;
                if let Some(path) = self.stage_assets.get(stage) {
                    self.upload_asset(stage, path).await.ok();
                }
                self.current_stage = None;
            }
            HookKind::Component(ComponentEvent::Skipped(meta)) => {
                let stage = normalize_stage(&meta.name);
                self.create_event("component_done", stage, Some("skipped"))
                    .await?;
            }
            HookKind::Machine(MachineEvent::Done) => {
                self.update_run("completed", Some(100)).await?;
                self.create_event("component_done", "completed", Some("pipeline finished"))
                    .await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn update_run(&self, current_phase: &str, progress: Option<i32>) -> Result<(), String> {
        let research_id_enc = urlencode(&self.research_id);
        let run_id_enc = urlencode(&self.run_id);
        let endpoint = format!(
            "{}/api/v1/research/{}/survey-runs/{}",
            self.url, research_id_enc, run_id_enc
        );

        let mut payload = serde_json::json!({
            "current_phase": current_phase,
        });
        if let Some(p) = progress {
            payload["progress"] = serde_json::json!(p);
        }

        let mut request = self.client.patch(&endpoint);
        if let Some(t) = self.token.as_ref() {
            request = request.bearer_auth(t);
        }
        let response = request
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("update_run request failed: {e}"))?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("update_run failed: HTTP {status} — {text}"));
        }
        info!(target: "rdc_reporter", phase = current_phase, progress = ?progress, "updated run");
        Ok(())
    }

    async fn create_event(
        &self,
        event_type: &str,
        stage: &str,
        message: Option<&str>,
    ) -> Result<(), String> {
        let research_id_enc = urlencode(&self.research_id);
        let run_id_enc = urlencode(&self.run_id);
        let endpoint = format!(
            "{}/api/v1/research/{}/survey-runs/{}/events",
            self.url, research_id_enc, run_id_enc
        );

        let mut payload = serde_json::json!({
            "type": event_type,
            "stage": stage,
            "ts": Utc::now().to_rfc3339(),
        });
        if let Some(m) = message {
            payload["message"] = serde_json::json!(m);
        }

        let mut request = self.client.post(&endpoint);
        if let Some(t) = self.token.as_ref() {
            request = request.bearer_auth(t);
        }
        let response = request
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("create_event request failed: {e}"))?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("create_event failed: HTTP {status} — {text}"));
        }
        Ok(())
    }

    async fn upload_asset(&self, stage: &str, relative_path: &str) -> Result<(), String> {
        let run_dir = self.run_dir.as_ref().ok_or("RCM_RUN_DIR not set")?;
        let path = run_dir.join(relative_path);
        if !path.exists() {
            return Err(format!("asset path not found: {path:?}"));
        }
        // Only upload markdown assets for now; images are not stored as content_md.
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            info!(target: "rdc_reporter", path = ?path, "skipping non-markdown asset");
            return Ok(());
        }
        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| format!("failed to read asset {path:?}: {e}"))?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(relative_path)
            .to_string();

        let research_id_enc = urlencode(&self.research_id);
        let run_id_enc = urlencode(&self.run_id);
        let endpoint = format!(
            "{}/api/v1/research/{}/survey-runs/{}/assets",
            self.url, research_id_enc, run_id_enc
        );
        let payload = serde_json::json!({
            "stage": stage,
            "name": name,
            "content_md": content,
        });

        let mut request = self.client.post(&endpoint);
        if let Some(t) = self.token.as_ref() {
            request = request.bearer_auth(t);
        }
        let response = request
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("upload_asset request failed: {e}"))?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("upload_asset failed: HTTP {status} — {text}"));
        }
        info!(target: "rdc_reporter", stage, path = ?path, "uploaded asset");
        Ok(())
    }
}

/// Percent-encode a path segment for use in URLs. Keeps unreserved characters
/// and encodes everything else.
fn urlencode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            _ => result.push_str(&format!("%{:02X}", byte)),
        }
    }
    result
}

fn normalize_stage(name: &str) -> &str {
    // Map inner components spawned by map nodes to their parent stage.
    match name {
        "paper_card" => "paper_cards",
        "section_expander" => "section_expander",
        "section_translator" => "section_translator",
        _ => name,
    }
}

fn default_stage_order() -> Vec<&'static str> {
    vec![
        "anchor",
        "query_plan",
        "discovery",
        "expansion",
        "rank_pool",
        "card_plan",
        "paper_cards",
        "research_map",
        "judge_panel",
        "image_planner",
        "survey_outline",
        "section_expander",
        "survey_assembler",
        "zh_frame",
        "section_translator",
        "zh_assemble",
    ]
}

fn default_stage_assets() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("anchor", "00_survey_spec.md");
    map.insert("query_plan", "01_query_plan.md");
    map.insert("discovery", "02_candidate_pool.md");
    map.insert("expansion", "03_expansion.md");
    map.insert("rank_pool", "04_ranked_pool.md");
    map.insert("research_map", "05_research_map.md");
    map.insert("judge_panel", "06_judge_panel.md");
    map.insert("survey_outline", "00_outline.md");
    map.insert("survey_assembler", "08_survey.md");
    map.insert("zh_frame", "00_zh_frame.md");
    map.insert("zh_assemble", "08_survey.zh.md");
    map
}

fn stage_progress(stage: &str, order: &[&str]) -> i32 {
    if stage == "completed" {
        return 100;
    }
    if let Some(idx) = order.iter().position(|&s| s == stage) {
        (((idx + 1) as f64 / order.len() as f64) * 100.0).round() as i32
    } else {
        0
    }
}
