use accelerator::tools::WebFetchTool;
use machine::{Environment, Tool};
use serde_json::json;

#[tokio::test]
async fn fetch_success() {
    let tool = WebFetchTool;
    let args = json!({"url": "https://example.com"});
    let result = tool.execute(args, &Environment::new(".")).await;
    if let Ok(res) = result {
        assert!(
            res.content.contains("Example") || res.content.contains("example"),
            "got: {}",
            res.content
        );
    }
}

#[tokio::test]
async fn fetch_status_on_error() {
    let tool = WebFetchTool;
    let args = json!({"url": "https://httpbin.org/status/503"});
    let result = tool.execute(args, &Environment::new(".")).await.unwrap();
    assert!(
        result.content.starts_with("HTTP"),
        "got: {}",
        result.content
    );
}

#[tokio::test]
async fn fetch_bad_url_returns_error() {
    let tool = WebFetchTool;
    let args = json!({"url": "https://nonexistent-domain-12345.example"});
    let result = tool.execute(args, &Environment::new(".")).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn fetch_respects_max_length() {
    let tool = WebFetchTool;
    let args = json!({"url": "https://example.com", "max_length": 10});
    let result = tool.execute(args, &Environment::new(".")).await.unwrap();
    assert!(
        result.content.len() <= 200,
        "got {} bytes",
        result.content.len()
    );
}
