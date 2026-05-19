use std::collections::HashSet;
use std::path::{Path, PathBuf};

use machine::{
    Action, Context, Environment, Fragment, Phase, PhaseOutcome, Purpose, Resources, Role,
};

/// Discover and inject project-level instruction files into the context.
///
/// Searches upward from the working directory for `AGENTS.md`, `CLAUDE.md`,
/// and `CONTEXT.md`, then checks global locations (`~/.synergy/config/AGENTS.md`,
/// `~/.claude/CLAUDE.md`). All found files are merged into a single `System`
/// fragment tagged `"instruction"`, placed after the system prompt and before
/// the user purpose.
///
/// Runs once per session — the `"instruction"` tag acts as a guard.
pub struct Instructions;

impl Phase for Instructions {
    fn clone_box(&self) -> Box<dyn Phase> {
        Box::new(Self)
    }

    fn name(&self) -> &str {
        "instructions"
    }

    fn decide(
        &self,
        _purpose: &Purpose,
        ctx: &Context,
        _env: &Environment,
        _resources: &Resources,
    ) -> PhaseOutcome {
        if ctx
            .fragments()
            .iter()
            .any(|f| f.role == Role::System && f.tag == "instruction")
        {
            return PhaseOutcome::Done;
        }

        let files = find_instruction_files();
        if files.is_empty() {
            return PhaseOutcome::Done;
        }

        let parts: Vec<String> = files
            .iter()
            .filter(|(_, content)| !content.trim().is_empty())
            .map(|(path, content)| {
                let name = path
                    .file_name()
                    .unwrap_or(path.as_os_str())
                    .to_string_lossy();
                format!(
                    "=== {name} (from {}) ===\n{}",
                    path.display(),
                    content.trim()
                )
            })
            .collect();

        if parts.is_empty() {
            return PhaseOutcome::Done;
        }

        PhaseOutcome::Action(Action::Append(
            Fragment::system(parts.join("\n\n")).with_tag("instruction"),
        ))
    }
}

const FILE_NAMES: [&str; 3] = ["AGENTS.md", "CLAUDE.md", "CONTEXT.md"];

fn global_paths() -> Vec<PathBuf> {
    let Some(home) = std::env::var("HOME").ok() else {
        return Vec::new();
    };
    vec![
        Path::new(&home).join(".synergy/config/AGENTS.md"),
        Path::new(&home).join(".claude/CLAUDE.md"),
    ]
}

/// Collect all instruction files from the project tree and global locations.
fn find_instruction_files() -> Vec<(PathBuf, String)> {
    let mut seen = HashSet::new();
    let mut results = Vec::new();

    if let Ok(cwd) = std::env::current_dir() {
        let mut dir = Some(cwd);
        while let Some(d) = dir {
            if !seen.insert(d.clone()) {
                break;
            }
            for name in &FILE_NAMES {
                let path = d.join(name);
                if path.is_file()
                    && let Ok(content) = std::fs::read_to_string(&path)
                {
                    results.push((path, content));
                }
            }
            dir = d.parent().map(|p| p.to_path_buf());
        }
    }

    for path in global_paths() {
        if path.is_file()
            && !results.iter().any(|(p, _)| *p == path)
            && let Ok(content) = std::fs::read_to_string(&path)
        {
            results.push((path, content));
        }
    }

    results
}
