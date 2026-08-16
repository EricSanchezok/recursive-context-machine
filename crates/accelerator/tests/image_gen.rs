use accelerator::tools::ImageGenTool;
use machine::{Environment, Tool};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

const PNG_BASE64: &str =
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=";

async fn image_gateway(responses: Vec<(u16, &'static str)>) -> (String, JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("test gateway should bind");
    let address = listener
        .local_addr()
        .expect("test gateway should expose its address");
    let handle = tokio::spawn(async move {
        let mut requests = Vec::new();
        for (status, response_body) in responses {
            let (mut socket, _) = listener
                .accept()
                .await
                .expect("test gateway should accept a request");
            let mut request = Vec::new();
            let mut buffer = [0_u8; 4096];
            let (header_end, content_length) = loop {
                let count = socket
                    .read(&mut buffer)
                    .await
                    .expect("test gateway should read a request");
                assert!(count > 0, "image request ended before its body arrived");
                request.extend_from_slice(&buffer[..count]);
                let Some(header_end) = request.windows(4).position(|part| part == b"\r\n\r\n")
                else {
                    continue;
                };
                let headers = String::from_utf8_lossy(&request[..header_end]);
                let content_length = headers
                    .lines()
                    .find_map(|line| {
                        line.to_ascii_lowercase()
                            .strip_prefix("content-length:")
                            .map(str::trim)
                            .and_then(|value| value.parse::<usize>().ok())
                    })
                    .expect("image request should declare content length");
                break (header_end, content_length);
            };
            let expected_length = header_end + 4 + content_length;
            while request.len() < expected_length {
                let count = socket
                    .read(&mut buffer)
                    .await
                    .expect("test gateway should finish reading the request");
                assert!(count > 0, "image request body ended early");
                request.extend_from_slice(&buffer[..count]);
            }

            let reason = if status == 200 { "OK" } else { "Error" };
            let response = format!(
                "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            socket
                .write_all(response.as_bytes())
                .await
                .expect("test gateway should return a response");
            requests.push(String::from_utf8(request).expect("image request should be UTF-8"));
        }
        requests
    });
    (format!("http://{address}/v1/images/generations"), handle)
}

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
        "image_gen_error code=image_configuration_missing retryable=false provider_code=api_key"
    );
}

#[tokio::test]
async fn image_generation_uses_gateway_contract_and_writes_png() {
    let success_body = format!(r#"{{"data":[{{"b64_json":"{PNG_BASE64}"}}]}}"#);
    let (gateway_url, received_requests) =
        image_gateway(vec![(200, Box::leak(success_body.into_boxed_str()))]).await;
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after the Unix epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "rcm_image_gateway_{}_{}",
        std::process::id(),
        unique
    ));
    tokio::fs::create_dir_all(&root)
        .await
        .expect("test output directory should be created");
    let mut environment = Environment::empty(&root);
    environment
        .vars
        .insert("IMAGE_GEN_API_KEY".into(), "gateway-secret".into());
    environment
        .vars
        .insert("IMAGE_GEN_API_URL".into(), gateway_url);

    ImageGenTool
        .execute(
            json!({
                "prompt": "A red fox in an autumn forest",
                "filePath": "generated.png",
                "size": "1024x1024"
            }),
            &environment,
        )
        .await
        .expect("gateway image generation should succeed");

    let requests = received_requests
        .await
        .expect("test gateway should finish cleanly");
    let request = &requests[0];
    let (_, body) = request
        .split_once("\r\n\r\n")
        .expect("request should contain an HTTP body");
    let payload: serde_json::Value =
        serde_json::from_str(body).expect("image request body should be JSON");
    assert!(
        request
            .lines()
            .any(|line| line.eq_ignore_ascii_case("authorization: bearer gateway-secret"))
    );
    assert_eq!(
        payload,
        json!({
            "model": "gpt-image-2",
            "prompt": "A red fox in an autumn forest",
            "size": "1024x1024",
            "n": 1
        })
    );
    assert_eq!(
        tokio::fs::read(root.join("generated.png"))
            .await
            .expect("generated image should be readable"),
        base64::Engine::decode(&base64::engine::general_purpose::STANDARD, PNG_BASE64,)
            .expect("fixture should be valid base64")
    );
    tokio::fs::remove_dir_all(root)
        .await
        .expect("test output directory should be removed");
}

#[tokio::test]
async fn image_generation_does_not_retry_http_400() {
    let (gateway_url, received_requests) = image_gateway(vec![(
        400,
        r#"{"error":{"code":"unsupported_parameter","message":"sensitive detail"}}"#,
    )])
    .await;
    let mut environment = Environment::empty(".");
    environment
        .vars
        .insert("IMAGE_GEN_API_KEY".into(), "gateway-secret".into());
    environment
        .vars
        .insert("IMAGE_GEN_API_URL".into(), gateway_url);

    let error = ImageGenTool
        .execute(
            json!({"prompt": "fixed canary", "filePath": "unused.png"}),
            &environment,
        )
        .await
        .expect_err("HTTP 400 should fail");

    assert!(error.contains("code=image_request_rejected"));
    assert!(error.contains("http_status=400"));
    assert!(error.contains("retryable=false"));
    assert!(error.contains("provider_code=unsupported_parameter"));
    assert!(!error.contains("sensitive detail"));
    assert_eq!(received_requests.await.unwrap().len(), 1);
}

#[tokio::test]
async fn image_generation_retries_http_429_once() {
    let success_body = format!(r#"{{"data":[{{"b64_json":"{PNG_BASE64}"}}]}}"#);
    let (gateway_url, received_requests) = image_gateway(vec![
        (429, r#"{"error":{"code":"rate_limit_exceeded"}}"#),
        (200, Box::leak(success_body.into_boxed_str())),
    ])
    .await;
    let root = std::env::temp_dir().join(format!(
        "rcm_image_retry_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    tokio::fs::create_dir_all(&root).await.unwrap();
    let mut environment = Environment::empty(&root);
    environment
        .vars
        .insert("IMAGE_GEN_API_KEY".into(), "gateway-secret".into());
    environment
        .vars
        .insert("IMAGE_GEN_API_URL".into(), gateway_url);

    ImageGenTool
        .execute(
            json!({"prompt": "fixed canary", "filePath": "retry.png"}),
            &environment,
        )
        .await
        .expect("the one bounded retry should succeed");

    assert_eq!(received_requests.await.unwrap().len(), 2);
    tokio::fs::remove_dir_all(root).await.unwrap();
}

#[tokio::test]
async fn image_generation_retries_http_5xx_only_once() {
    let (gateway_url, received_requests) = image_gateway(vec![
        (503, r#"{"error":{"code":"upstream_unavailable"}}"#),
        (503, r#"{"error":{"code":"upstream_unavailable"}}"#),
    ])
    .await;
    let mut environment = Environment::empty(".");
    environment
        .vars
        .insert("IMAGE_GEN_API_KEY".into(), "gateway-secret".into());
    environment
        .vars
        .insert("IMAGE_GEN_API_URL".into(), gateway_url);

    let error = ImageGenTool
        .execute(
            json!({"prompt": "fixed canary", "filePath": "unused.png"}),
            &environment,
        )
        .await
        .expect_err("two HTTP 503 responses should fail");

    assert!(error.contains("code=image_provider_unavailable"));
    assert!(error.contains("http_status=503"));
    assert!(error.contains("retryable=true"));
    assert_eq!(received_requests.await.unwrap().len(), 2);
}
