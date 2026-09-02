use std::future::Future;
use std::pin::Pin;
use std::sync::LazyLock;
use std::time::Duration;

use machine::{Environment, Tool, ToolResult};
use serde_json::Value;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

const MAX_TIMEOUT_SECS: u64 = 120;
const DEFAULT_TIMEOUT_SECS: u64 = 60;
const OUTPUT_CAP_BYTES: usize = 256 * 1024;

const PROTECTED_BRANCHES: &[&str] = &["main", "master"];

static GIT_DESC: LazyLock<String> = LazyLock::new(|| {
    include_str!("git.txt")
        .replace("{MAX_TIMEOUT_SECS}", &MAX_TIMEOUT_SECS.to_string())
        .replace("{DEFAULT_TIMEOUT_SECS}", &DEFAULT_TIMEOUT_SECS.to_string())
});

pub struct GitTool;

impl Tool for GitTool {
    fn name(&self) -> &str {
        "git"
    }

    fn description(&self) -> &str {
        &GIT_DESC
    }

    fn parameters(&self) -> Value {
        let mut props = serde_json::Map::new();
        props.insert(
            "command".into(),
            serde_json::json!({
                "type": "string",
                "description": "Git subcommand and arguments (without the leading 'git'). Example: \"commit -m 'fix: handle empty input'\""
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
            let raw = args["command"]
                .as_str()
                .ok_or("missing required parameter 'command'")?
                .to_string();

            let tokens = match tokenize(&raw) {
                Ok(tokens) => tokens,
                Err(reason) => return Ok(deny_result(&raw, &reason)),
            };

            if let Err(reason) = check_safety(&tokens) {
                return Ok(deny_result(&raw, &reason));
            }

            let per_call_timeout = args["timeout_secs"]
                .as_u64()
                .unwrap_or(DEFAULT_TIMEOUT_SECS)
                .min(MAX_TIMEOUT_SECS);

            let mut child = Command::new("git")
                .args(&tokens)
                .current_dir(&env.cwd)
                .env_clear()
                .envs(&env.vars)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .map_err(|spawn_err| format!("failed to spawn git: {spawn_err}"))?;

            match tokio::time::timeout(
                Duration::from_secs(per_call_timeout),
                collect_output(&mut child),
            )
            .await
            {
                Ok((stdout, stderr, exit_code)) => build_result(&raw, stdout, stderr, exit_code),
                Err(_) => {
                    let _ = child.kill().await;
                    let _ = child.wait().await;
                    Ok(ToolResult {
                        call_id: String::new(),
                        content: format!(
                            "[timeout] git command killed after {per_call_timeout}s:\n$ git {raw}"
                        ),
                        title: Some("timeout".to_string()),
                    })
                }
            }
        })
    }
}

pub fn tokenize(input: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_double = false;
    let mut in_single = false;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' if !in_single => in_double = !in_double,
            '\'' if !in_double => in_single = !in_single,
            ' ' | '\t' | '\n' if !in_double && !in_single => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            '\\' if !in_single => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            other => current.push(other),
        }
    }

    if in_double {
        return Err("unbalanced double quote".into());
    }
    if in_single {
        return Err("unbalanced single quote".into());
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    Ok(tokens)
}

pub fn check_safety(tokens: &[String]) -> Result<(), String> {
    if tokens.is_empty() {
        return Err("empty command".into());
    }

    let subcommand = tokens[0].as_str();
    let rest: Vec<&str> = tokens[1..].iter().map(String::as_str).collect();

    match subcommand {
        "push" => check_push(&rest),
        "reset" => check_reset(&rest),
        "checkout" => check_checkout(&rest),
        "switch" => check_switch(&rest),
        "restore" => check_restore(&rest),
        "branch" => check_branch(&rest),
        "stash" => check_stash(&rest),
        "clean" => check_clean(&rest),
        "commit" | "rebase" | "merge" | "cherry-pick" | "revert" => check_hook_skip(&rest),
        "config" => {
            Err("git config changes are denied; do not modify repo or global config".into())
        }
        _ => Ok(()),
    }
}

fn check_push(args: &[&str]) -> Result<(), String> {
    for arg in args {
        if *arg == "-f" || *arg == "--force" || arg.starts_with("--force-with-lease") {
            return Err("force push is denied".into());
        }
    }

    for arg in args.iter().filter(|arg| !arg.starts_with('-')) {
        if let Some(branch) = arg.strip_prefix(':')
            && PROTECTED_BRANCHES.contains(&branch)
        {
            return Err(format!("deleting remote branch '{branch}' is denied"));
        }
        let target = arg.split(':').next_back().unwrap_or(arg);
        let target = target.trim_start_matches('+');
        if PROTECTED_BRANCHES.contains(&target) {
            return Err(format!("push to protected branch '{target}' is denied"));
        }
    }

    Ok(())
}

fn check_reset(args: &[&str]) -> Result<(), String> {
    if args.contains(&"--hard") {
        return Err("reset --hard discards working tree changes; denied".into());
    }
    Ok(())
}

fn check_checkout(args: &[&str]) -> Result<(), String> {
    if args.contains(&"-f") || args.contains(&"--force") {
        return Err("force checkout discards local changes; denied".into());
    }
    if args.contains(&"--") {
        return Err("checkout -- <path> discards working-tree changes; denied".into());
    }
    if args.iter().any(|arg| PROTECTED_BRANCHES.contains(arg)) {
        return Err("checkout to a protected default branch is denied".into());
    }
    if args.first().map(|arg| *arg == ".").unwrap_or(false) {
        return Err("checkout . discards all working-tree changes; denied".into());
    }
    Ok(())
}

fn check_switch(args: &[&str]) -> Result<(), String> {
    if args.contains(&"-f") || args.contains(&"--force") || args.contains(&"--discard-changes") {
        return Err("force switch discards local changes; denied".into());
    }
    if args.iter().any(|arg| PROTECTED_BRANCHES.contains(arg)) {
        return Err("switch to a protected default branch is denied".into());
    }
    Ok(())
}

fn check_restore(args: &[&str]) -> Result<(), String> {
    let touches_worktree = args.contains(&"--worktree")
        || args.contains(&"-W")
        || !(args.contains(&"--staged") || args.contains(&"-S"));
    if touches_worktree {
        return Err("git restore against the working tree discards changes; denied".into());
    }
    Ok(())
}

fn check_branch(args: &[&str]) -> Result<(), String> {
    if args.contains(&"-D") || args.contains(&"--delete") || args.contains(&"-d") {
        return Err("branch deletion is denied".into());
    }
    Ok(())
}

fn check_stash(args: &[&str]) -> Result<(), String> {
    if let Some(first) = args.first()
        && (*first == "drop" || *first == "clear")
    {
        return Err(format!("stash {first} discards stashed work; denied"));
    }
    Ok(())
}

fn check_clean(args: &[&str]) -> Result<(), String> {
    let dangerous = args.iter().any(|arg| {
        if arg.starts_with("--") {
            *arg == "--force"
        } else if let Some(flags) = arg.strip_prefix('-') {
            flags.contains('f')
        } else {
            false
        }
    });
    if dangerous {
        return Err("git clean -f deletes untracked files; denied".into());
    }
    Ok(())
}

fn check_hook_skip(args: &[&str]) -> Result<(), String> {
    if args.contains(&"--no-verify") || args.contains(&"--no-gpg-sign") {
        return Err("--no-verify / --no-gpg-sign skips required hooks; denied".into());
    }
    Ok(())
}

fn deny_result(raw: &str, reason: &str) -> ToolResult {
    ToolResult {
        call_id: String::new(),
        content: format!("[denied] {reason}\n\nattempted: git {raw}"),
        title: Some(format!("denied: {reason}")),
    }
}

async fn collect_output(child: &mut tokio::process::Child) -> (Vec<u8>, Vec<u8>, Option<i32>) {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    if let Some(out) = child.stdout.take() {
        let mut reader = tokio::io::BufReader::new(out);
        let _ = reader.read_to_end(&mut stdout).await;
    }
    if let Some(err) = child.stderr.take() {
        let mut reader = tokio::io::BufReader::new(err);
        let _ = reader.read_to_end(&mut stderr).await;
    }

    let exit_code = child.wait().await.ok().and_then(|status| status.code());
    (stdout, stderr, exit_code)
}

fn build_result(
    raw: &str,
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
        let short = if raw.len() > 60 {
            format!("{}…", &raw[..60])
        } else {
            raw.to_string()
        };
        format!("✓ git {short}")
    } else {
        format!("exit {code}")
    };

    Ok(ToolResult {
        call_id: String::new(),
        content: output,
        title: Some(title),
    })
}
