use accelerator::tools::ImageGenTool;
use machine::{Environment, Tool};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

async fn image_gateway() -> (String, JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("test gateway should bind");
    let address = listener
        .local_addr()
        .expect("test gateway should expose its address");
    let handle = tokio::spawn(async move {
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
            let Some(header_end) = request.windows(4).position(|part| part == b"\r\n\r\n") else {
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

        let response_body = r#"{"data":[{"b64_json":"cG5n"}]}"#;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            response_body.len(),
            response_body
        );
        socket
            .write_all(response.as_bytes())
            .await
            .expect("test gateway should return a response");
        String::from_utf8(request).expect("image request should be UTF-8")
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
        "IMAGE_GEN_API_KEY is not set in the environment"
    );
}

#[tokio::test]
async fn image_generation_uses_gateway_contract_and_writes_png() {
    let (gateway_url, received_request) = image_gateway().await;
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

    let request = received_request
        .await
        .expect("test gateway should finish cleanly");
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
        b"png"
    );
    tokio::fs::remove_dir_all(root)
        .await
        .expect("test output directory should be removed");
}
