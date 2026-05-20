//! File URI conversion helpers.

use std::path::{Path, PathBuf};

use url::Url;

pub fn path_to_uri(path: &Path) -> Result<String, String> {
    Url::from_file_path(path)
        .map(|url| url.to_string())
        .map_err(|_| format!("cannot convert path to file URI: {}", path.display()))
}

pub fn uri_to_path(uri: &str) -> Result<PathBuf, String> {
    Url::parse(uri)
        .map_err(|error| format!("invalid file URI '{uri}': {error}"))?
        .to_file_path()
        .map_err(|_| format!("URI is not a file path: {uri}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_uri_roundtrip() {
        let path = std::env::temp_dir().join("rcm lsp uri test.rs");
        let uri = path_to_uri(&path).unwrap();
        assert_eq!(uri_to_path(&uri).unwrap(), path);
    }
}
