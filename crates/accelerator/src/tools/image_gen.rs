//! Image generation tool — text-to-image via the OpenAI images API.
//!
//! Generates an image from a prompt and writes it to disk, returning the
//! saved path. Used by pipelines that need a generated figure (e.g. a survey's
//! opening "global picture") as a real artifact rather than inline data.

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use base64::Engine;
use machine::{DEFAULT_TOOL_TIMEOUT_SECS, Environment, Tool, ToolResult};
use serde_json::Value;
use tracing::info;

use super::{relative_path, resolve_path};

const API_URL: &str = "https://apicz.boyuerichdata.com/v1/images/generations";
const DEFAULT_MODEL: &str = "gpt-image-2";
const API_KEY_ENV: &str = "IMAGE_GEN_API_KEY";

pub struct ImageGenTool;

impl Tool for ImageGenTool {
    fn name(&self) -> &str {
        "image_gen"
    }

    fn description(&self) -> &str {
        "Generate an image from a text prompt and save it to a file. Provide a vivid, \
         self-contained prompt and an output filePath (e.g. run_dir/08_global_picture.png). \
         Returns the saved path."
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "prompt": {
                    "type": "string",
                    "description": "Full text-to-image prompt. Be concrete and self-contained; the image model sees only this text."
                },
                "filePath": {
                    "type": "string",
                    "description": "Where to write the PNG, relative to the working directory (e.g. runs/<ts>/08_global_picture.png). Use the path as given; do not add extra directory prefixes."
                },
                "size": {
                    "type": "string",
                    "default": "1536x1024",
                    "description": "Image size, e.g. 1024x1024, 1536x1024, or 1024x1536. Default 1536x1024 (landscape, good for a banner figure)."
                }
            },
            "required": ["prompt", "filePath"]
        })
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(DEFAULT_TOOL_TIMEOUT_SECS)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let prompt = args["prompt"]
                .as_str()
                .ok_or("missing required parameter 'prompt'")?;
            let file_path = args["filePath"]
                .as_str()
                .ok_or("missing required parameter 'filePath'")?;
            let size = args["size"].as_str().unwrap_or("1536x1024");

            let api_key = env
                .vars
                .get(API_KEY_ENV)
                .filter(|key| !key.is_empty())
                .ok_or_else(|| format!("{API_KEY_ENV} is not set in the environment"))?;

            let output_path = resolve_path(file_path, env)?;

            info!(target: "image_gen", model = DEFAULT_MODEL, size, path = ?output_path, "generating image");

            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(DEFAULT_TOOL_TIMEOUT_SECS))
                .build()
                .map_err(|error| format!("failed to create HTTP client: {error}"))?;

            let response = client
                .post(API_URL)
                .bearer_auth(api_key)
                .json(&serde_json::json!({
                    "model": DEFAULT_MODEL,
                    "prompt": prompt,
                    "size": size,
                    "n": 1,
                }))
                .send()
                .await
                .map_err(|error| format!("image generation request failed: {error}"))?;

            let status = response.status();
            let body: Value = response
                .json()
                .await
                .map_err(|error| format!("failed to parse image response: {error}"))?;

            if !status.is_success() {
                let message = body["error"]["message"].as_str().unwrap_or("unknown error");
                return Err(format!(
                    "image generation failed: HTTP {status} — {message}"
                ));
            }

            let b64 = body["data"][0]["b64_json"]
                .as_str()
                .ok_or("image response did not contain b64_json data")?;
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(b64)
                .map_err(|error| format!("failed to decode image data: {error}"))?;

            if let Some(parent) = output_path.parent() {
                tokio::fs::create_dir_all(parent)
                    .await
                    .map_err(|error| format!("failed to create output directory: {error}"))?;
            }
            tokio::fs::write(&output_path, &bytes)
                .await
                .map_err(|error| format!("failed to write image: {error}"))?;

            let relative = relative_path(&output_path, &env.cwd);
            let size_str = format!("{:.1} KB", bytes.len() as f64 / 1024.0);
            info!(target: "image_gen", path = %relative, size = %size_str, "image written");

            Ok(ToolResult {
                call_id: String::new(),
                content: format!("Wrote image to {relative} ({size_str})"),
                title: Some(format!("image {relative}")),
            })
        })
    }
}
