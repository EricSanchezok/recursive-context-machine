use accelerator::tools::WebFetchTool;
use machine::{Environment, Tool};
use serde_json::json;

#[tokio::test]
async fn fetch_bad_url_returns_error() {
    let tool = WebFetchTool;
    let args = json!({"url": "https://nonexistent-domain-12345.example"});
    let result = tool.execute(args, &Environment::new(".")).await;
    assert!(result.is_err());
}
