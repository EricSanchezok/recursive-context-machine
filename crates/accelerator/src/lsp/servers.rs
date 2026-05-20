//! LSP server definitions and root detection.

use std::path::{Path, PathBuf};

use machine::Environment;

#[derive(Debug, Clone, Copy)]
pub struct ServerSpec {
    pub id: &'static str,
    pub language_id: &'static str,
    pub extensions: &'static [&'static str],
    pub root_markers: &'static [&'static str],
    pub command: &'static str,
    pub args: &'static [&'static str],
}

pub const RUST_ANALYZER: ServerSpec = ServerSpec {
    id: "rust-analyzer",
    language_id: "rust",
    extensions: &["rs"],
    root_markers: &["Cargo.toml"],
    command: "rust-analyzer",
    args: &[],
};

pub fn server_for_file(path: &Path) -> Option<ServerSpec> {
    let extension = path.extension()?.to_str()?;
    [RUST_ANALYZER]
        .into_iter()
        .find(|server| server.extensions.contains(&extension))
}

pub fn find_root(file: &Path, server: ServerSpec, env: &Environment) -> Option<PathBuf> {
    let boundary = env.root.as_ref().unwrap_or(&env.cwd);
    let mut current = file.parent()?;

    loop {
        if server
            .root_markers
            .iter()
            .any(|marker| current.join(marker).exists())
        {
            return Some(current.to_path_buf());
        }

        if current == boundary {
            return None;
        }

        current = current.parent()?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_server_matches_rs() {
        assert_eq!(
            server_for_file(Path::new("src/lib.rs")).unwrap().id,
            "rust-analyzer"
        );
        assert!(server_for_file(Path::new("src/lib.ts")).is_none());
    }
}
