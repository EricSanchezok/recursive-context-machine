use accelerator::tools::{IMAGE_GEN_DIAGNOSTIC_ENV, ImageGenTool};
use anyhow::{Context, Result, anyhow};
use machine::{Environment, Tool};

use crate::args::ImageCanaryArgs;

const CANARY_PROMPT: &str = "A simple blue circle centered on a plain white background. No text, logos, people, brands, or identifying information.";
const IMAGE_ENVIRONMENT_KEYS: [&str; 3] = [
    "IMAGE_GEN_API_KEY",
    "IMAGE_GEN_API_URL",
    "IMAGE_GEN_TRUSTED_HOSTS",
];

pub async fn run(args: ImageCanaryArgs) -> Result<()> {
    let parent = args
        .output
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."))
        .canonicalize()
        .with_context(|| format!("resolving canary output parent {}", args.output.display()))?;
    let file_name = args
        .output
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("canary output must have a UTF-8 file name"))?;
    let mut environment = Environment::empty(&parent);
    environment.root = Some(parent.clone());
    for key in IMAGE_ENVIRONMENT_KEYS {
        if let Ok(value) = std::env::var(key) {
            environment.vars.insert(key.to_owned(), value);
        }
    }
    environment
        .vars
        .insert(IMAGE_GEN_DIAGNOSTIC_ENV.to_owned(), "true".to_owned());

    let result = ImageGenTool
        .execute(
            serde_json::json!({
                "prompt": CANARY_PROMPT,
                "filePath": file_name,
                "size": "1024x1024",
            }),
            &environment,
        )
        .await
        .map_err(|error| anyhow!(error))?;
    let output_path = parent.join(file_name);
    let size = tokio::fs::metadata(&output_path)
        .await
        .context("reading verified canary output")?
        .len();
    println!(
        "{}",
        serde_json::json!({
            "schema_version": 1,
            "status": "ok",
            "output_path": output_path,
            "size": size,
            "result": result.content,
        })
    );
    Ok(())
}
