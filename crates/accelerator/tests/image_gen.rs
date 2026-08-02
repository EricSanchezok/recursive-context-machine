use accelerator::tools::ImageGenTool;
use machine::{Environment, Tool};
use serde_json::json;

#[test]
fn image_generation_allows_thirty_minute_requests() {
    assert_eq!(ImageGenTool.timeout().as_secs(), 1_800);
}

#[tokio::test]
async fn image_generation_requires_dedicated_api_key() {
    let tool = ImageGenTool;
    let environment = Environment::empty(".");
    let result = tool
        .execute(
            json!({
                "prompt": "A compact research landscape",
                "filePath": "landscape.png"
            }),
            &environment,
        )
        .await;

    assert_eq!(
        result.unwrap_err(),
        "IMAGE_GEN_API_KEY is not set in the environment"
    );
}
