use std::collections::HashSet;
use std::path::{Path, PathBuf};

use machine::{Action, Context, Fragment, Role};

const FILE_NAMES: [&str; 3] = ["AGENTS.md", "CLAUDE.md", "CONTEXT.md"];

pub fn ensure_instructions(ctx: &Context) -> Option<Action> {
    if ctx
        .fragments()
        .iter()
        .any(|f| f.role == Role::System && f.tag == "instruction")
    {
        return None;
    }

    let files = find_instruction_files();
    if files.is_empty() {
        return None;
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
        return None;
    }

    Some(Action::Append(
        Fragment::system(parts.join("\n\n")).with_tag("instruction"),
    ))
}

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

fn global_paths() -> Vec<PathBuf> {
    let Some(home) = std::env::var("HOME").ok() else {
        return Vec::new();
    };
    vec![
        Path::new(&home).join(".synergy/config/AGENTS.md"),
        Path::new(&home).join(".claude/CLAUDE.md"),
    ]
}
