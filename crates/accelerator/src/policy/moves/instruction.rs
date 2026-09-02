use std::collections::HashSet;
use std::path::{Path, PathBuf};

use machine::{Action, Context, Fragment, Role};

use super::super::Step;

const FILE_NAMES: [&str; 3] = ["AGENTS.md", "CLAUDE.md", "CONTEXT.md"];
const INSTRUCTION_TAG: &str = "instruction";

pub(crate) fn load(ctx: &Context) -> Step {
    if ctx
        .fragments()
        .iter()
        .any(|fragment| fragment.role == Role::System && fragment.tag == INSTRUCTION_TAG)
    {
        return Step::Ready;
    }

    let files = find_instruction_files();
    if files.is_empty() {
        return Step::Ready;
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
        return Step::Ready;
    }

    Step::Emit(Action::Append(
        Fragment::system(parts.join("\n\n")).with_tag(INSTRUCTION_TAG),
    ))
}

fn find_instruction_files() -> Vec<(PathBuf, String)> {
    let mut seen = HashSet::new();
    let mut results = Vec::new();

    if let Ok(cwd) = std::env::current_dir() {
        let mut directory = Some(cwd);
        while let Some(current) = directory {
            if !seen.insert(current.clone()) {
                break;
            }
            let mut found_here = false;
            for name in &FILE_NAMES {
                let path = current.join(name);
                if path.is_file()
                    && let Ok(content) = std::fs::read_to_string(&path)
                {
                    results.push((path, content));
                    found_here = true;
                }
            }
            // Stop at the nearest directory that has instruction files: a local
            // AGENTS.md is meant to override ancestors, not stack on top of them.
            // This lets an example carry its own instructions without inheriting
            // the host repo's development guide.
            if found_here {
                break;
            }
            directory = current.parent().map(|parent| parent.to_path_buf());
        }
    }

    for path in global_paths() {
        if path.is_file()
            && !results.iter().any(|(existing, _)| *existing == path)
            && let Ok(content) = std::fs::read_to_string(&path)
        {
            results.push((path, content));
        }
    }

    results
}

fn global_paths() -> Vec<PathBuf> {
    let Some(home) = std::env::var("HOME").ok() else {
        return Vec::new();
    };
    vec![
        Path::new(&home).join(".synergy/config/AGENTS.md"),
        Path::new(&home).join(".claude/CLAUDE.md"),
    ]
}
