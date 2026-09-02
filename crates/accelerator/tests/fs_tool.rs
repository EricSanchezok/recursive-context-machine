use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use accelerator::tools::FsTool;
use machine::{Environment, Tool};
use serde_json::json;

fn unique_test_root(label: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after the Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("rcm-{label}-{}-{nonce}", std::process::id()))
}

async fn assert_short_file_edit_returns_no_match(content: &str, old_string: &str) {
    let root = unique_test_root("short-edit");
    fs::create_dir_all(&root).expect("test root must be created");
    let path = root.join("artifact.md");
    fs::write(&path, content).expect("fixture must be written");
    let environment = Environment::empty(&root);
    let tool = FsTool;

    tool.execute(json!({"action": "read", "filePath": path}), &environment)
        .await
        .expect("fixture must be readable");
    let result = tool
        .execute(
            json!({
                "action": "edit",
                "filePath": path,
                "oldString": old_string,
                "newString": "replacement"
            }),
            &environment,
        )
        .await;

    assert_eq!(
        result.expect_err("short files must not match"),
        "No match found."
    );
    assert_eq!(
        fs::read_to_string(&path).expect("fixture must remain readable"),
        content
    );
    fs::remove_dir_all(root).expect("test root must be removed");
}

#[tokio::test]
async fn editing_an_empty_file_does_not_panic() {
    assert_short_file_edit_returns_no_match("", "missing").await;
}

#[tokio::test]
async fn editing_a_file_shorter_than_the_search_block_does_not_panic() {
    assert_short_file_edit_returns_no_match("one line\n", "one line\nsecond line").await;
}
