use std::future::Future;
use std::pin::Pin;
use std::sync::LazyLock;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;
use tokio::process::Command;

const MAX_TIMEOUT_SECS: u64 = 180;
const DEFAULT_TIMEOUT_SECS: u64 = 120;
const OUTPUT_CAP_BYTES: usize = 512 * 1024;

static SHELL_DESC: LazyLock<String> = LazyLock::new(|| {
    include_str!("shell.txt")
        .replace("{MAX_TIMEOUT_SECS}", &MAX_TIMEOUT_SECS.to_string())
        .replace("{DEFAULT_TIMEOUT_SECS}", &DEFAULT_TIMEOUT_SECS.to_string())
        .replace("{OUTPUT_CAP_KB}", &(OUTPUT_CAP_BYTES / 1024).to_string())
});

pub struct ShellTool;

impl Tool for ShellTool {
    fn name(&self) -> &str {
        "shell"
    }

    fn description(&self) -> &str {
        &SHELL_DESC
    }

    fn parameters(&self) -> Value {
        let mut props = serde_json::Map::new();
        props.insert(
            "command".into(),
            serde_json::json!({
                "type": "string",
                "description": "Shell command to execute."
            }),
        );
        props.insert(
            "timeout_secs".into(),
            serde_json::json!({
                "type": "integer",
                "description": format!(
                    "Per-call timeout in seconds (default {}, max {}).",
                    DEFAULT_TIMEOUT_SECS, MAX_TIMEOUT_SECS
                )
            }),
        );
        serde_json::json!({
            "type": "object",
            "properties": props,
            "required": ["command"]
        })
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(MAX_TIMEOUT_SECS)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let command = args["command"]
                .as_str()
                .ok_or("missing required parameter 'command'")?
                .to_string();

            let per_call_timeout = args["timeout_secs"]
                .as_u64()
                .unwrap_or(DEFAULT_TIMEOUT_SECS)
                .min(MAX_TIMEOUT_SECS);

            let mut child = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .current_dir(&env.cwd)
                .env_clear()
                .envs(&env.vars)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .map_err(|e| format!("failed to spawn: {e}"))?;

            match tokio::time::timeout(
                Duration::from_secs(per_call_timeout),
                collect_output(&mut child),
            )
            .await
            {
                Ok((stdout, stderr, exit_code)) => build_result(command, stdout, stderr, exit_code),
                Err(_) => {
                    let _ = child.kill().await;
                    let _ = child.wait().await;
                    Ok(ToolResult {
                        call_id: String::new(),
                        content: format!(
                            "[timeout] command killed after {per_call_timeout}s:\n$ {command}"
                        ),
                        title: Some("timeout".to_string()),
                    })
                }
            }
        })
    }
}

async fn collect_output(child: &mut tokio::process::Child) -> (Vec<u8>, Vec<u8>, Option<i32>) {
    use tokio::io::AsyncReadExt;

    let mut stdout = Vec::with_capacity(OUTPUT_CAP_BYTES);
    let mut stderr = Vec::with_capacity(OUTPUT_CAP_BYTES);

    if let Some(out) = child.stdout.take() {
        let mut reader = tokio::io::BufReader::new(out).take(OUTPUT_CAP_BYTES as u64);
        let _ = reader.read_to_end(&mut stdout).await;
    }
    if let Some(err) = child.stderr.take() {
        let mut reader = tokio::io::BufReader::new(err).take(OUTPUT_CAP_BYTES as u64);
        let _ = reader.read_to_end(&mut stderr).await;
    }

    let exit_code = child.wait().await.ok().and_then(|s| s.code());
    (stdout, stderr, exit_code)
}

fn build_result(
    command: String,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    exit_code: Option<i32>,
) -> Result<ToolResult, String> {
    let stdout = String::from_utf8_lossy(&stdout).to_string();
    let stderr = String::from_utf8_lossy(&stderr).to_string();
    let code = exit_code.unwrap_or(-1);

    let mut output = String::with_capacity(stdout.len() + stderr.len() + 64);
    output.push_str(&stdout);
    if !stderr.is_empty() {
        if !output.is_empty() && !output.ends_with('\n') {
            output.push('\n');
        }
        output.push_str(&stderr);
    }

    let truncated = output.len() > OUTPUT_CAP_BYTES;
    if truncated {
        output.truncate(OUTPUT_CAP_BYTES);
        output.push_str(&format!(
            "\n\n[output truncated at {} KB]",
            OUTPUT_CAP_BYTES / 1024
        ));
    }

    let title = if code == 0 {
        let short = if command.len() > 60 {
            format!("{}…", &command[..60])
        } else {
            command.clone()
        };
        format!("✓ {short}")
    } else {
        format!("exit {code}")
    };

    Ok(ToolResult {
        call_id: String::new(),
        content: output,
        title: Some(title),
    })
}
