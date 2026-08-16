//! Image generation tool — text-to-image via the OpenAI images API.
//!
//! Generates an image from a prompt and writes it to disk, returning the
//! saved path. Used by pipelines that need a generated figure (e.g. a survey's
//! opening "global picture") as a real artifact rather than inline data.

use std::future::Future;
use std::net::{IpAddr, SocketAddr};
use std::pin::Pin;
use std::time::Duration;

use base64::Engine;
use machine::{DEFAULT_TOOL_TIMEOUT_SECS, Environment, Tool, ToolResult};
use reqwest::{Response, StatusCode, redirect::Policy};
use serde_json::Value;
use tracing::info;
use url::Url;

use super::{relative_path, resolve_path};

const API_URL: &str = "https://llm.yuzhu.tech/v1/images/generations";
const DEFAULT_MODEL: &str = "gpt-image-2";
const API_KEY_ENV: &str = "IMAGE_GEN_API_KEY";
const API_URL_ENV: &str = "IMAGE_GEN_API_URL";
const TRUSTED_HOSTS_ENV: &str = "IMAGE_GEN_TRUSTED_HOSTS";
const IMAGE_MAX_BYTES: usize = 20 * 1024 * 1024;
const JSON_MAX_BYTES: usize = (IMAGE_MAX_BYTES * 4 / 3) + (1024 * 1024);
const ERROR_BODY_MAX_BYTES: usize = 64 * 1024;
const IMAGE_OPERATION_BUDGET: Duration = Duration::from_secs(12 * 60);
const RETRY_DELAY: Duration = Duration::from_millis(250);

fn sanitized_provider_code(body: &[u8]) -> Option<String> {
    let payload: Value = serde_json::from_slice(body).ok()?;
    let value = payload["error"]["code"]
        .as_str()
        .or_else(|| payload["error"]["type"].as_str())?;
    let sanitized: String = value
        .chars()
        .filter(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.')
        })
        .take(64)
        .collect();
    (!sanitized.is_empty()).then_some(sanitized)
}

fn tool_error(
    code: &str,
    status: Option<StatusCode>,
    retryable: bool,
    provider_code: Option<&str>,
) -> String {
    let mut fields = vec![format!("code={code}"), format!("retryable={retryable}")];
    if let Some(status) = status {
        fields.push(format!("http_status={}", status.as_u16()));
    }
    if let Some(provider_code) = provider_code {
        fields.push(format!("provider_code={provider_code}"));
    }
    format!("image_gen_error {}", fields.join(" "))
}

fn status_error(status: StatusCode, body: &[u8]) -> String {
    let retryable = status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error();
    let code = if status == StatusCode::TOO_MANY_REQUESTS {
        "image_rate_limited"
    } else if matches!(status, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN) {
        "image_authentication_failed"
    } else if status.is_server_error() {
        "image_provider_unavailable"
    } else {
        "image_request_rejected"
    };
    let provider_code = sanitized_provider_code(body);
    tool_error(code, Some(status), retryable, provider_code.as_deref())
}

async fn response_bytes_bounded(mut response: Response, limit: usize) -> Result<Vec<u8>, String> {
    if response
        .content_length()
        .is_some_and(|content_length| content_length > limit as u64)
    {
        return Err(tool_error(
            "image_response_too_large",
            Some(response.status()),
            false,
            None,
        ));
    }
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| {
        tool_error(
            "image_provider_unavailable",
            Some(response.status()),
            true,
            None,
        )
    })? {
        if body.len().saturating_add(chunk.len()) > limit {
            return Err(tool_error(
                "image_response_too_large",
                Some(response.status()),
                false,
                None,
            ));
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

async fn send_before_deadline(
    request: reqwest::RequestBuilder,
    deadline: tokio::time::Instant,
) -> Result<Response, String> {
    match tokio::time::timeout_at(deadline, request.send()).await {
        Ok(Ok(response)) => Ok(response),
        Ok(Err(_)) => Err(tool_error(
            "image_provider_unavailable",
            None,
            true,
            Some("network_error"),
        )),
        Err(_) => Err(tool_error(
            "image_provider_unavailable",
            None,
            true,
            Some("timeout"),
        )),
    }
}

fn image_mime(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("image/png")
    } else if bytes.starts_with(b"\xff\xd8\xff") {
        Some("image/jpeg")
    } else if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        Some("image/webp")
    } else {
        None
    }
}

fn validate_image(bytes: Vec<u8>, declared_mime: Option<&str>) -> Result<Vec<u8>, String> {
    if bytes.is_empty() || bytes.len() > IMAGE_MAX_BYTES {
        return Err(tool_error("image_response_too_large", None, false, None));
    }
    let detected = image_mime(&bytes).ok_or_else(|| {
        tool_error(
            "image_response_invalid",
            None,
            false,
            Some("invalid_image_signature"),
        )
    })?;
    if let Some(declared) = declared_mime {
        let normalized = declared
            .split(';')
            .next()
            .unwrap_or_default()
            .trim()
            .to_ascii_lowercase();
        if normalized != detected {
            return Err(tool_error(
                "image_response_invalid",
                None,
                false,
                Some("invalid_content_type"),
            ));
        }
    }
    Ok(bytes)
}

fn ip_is_forbidden(address: IpAddr) -> bool {
    match address {
        IpAddr::V4(address) => {
            address.is_private()
                || address.is_loopback()
                || address.is_link_local()
                || address.is_broadcast()
                || address.is_documentation()
                || address.is_unspecified()
                || address.is_multicast()
        }
        IpAddr::V6(address) => {
            address.is_loopback()
                || address.is_unspecified()
                || address.is_unique_local()
                || address.is_unicast_link_local()
                || address.is_multicast()
        }
    }
}

fn trusted_hosts(api_url: &str, configured: Option<&String>) -> Result<Vec<String>, String> {
    let mut hosts = Vec::new();
    if let Some(host) = Url::parse(api_url)
        .ok()
        .and_then(|url| url.host_str().map(str::to_ascii_lowercase))
    {
        hosts.push(host);
    }
    if let Some(configured) = configured {
        hosts.extend(
            configured
                .split(',')
                .map(str::trim)
                .filter(|host| !host.is_empty())
                .map(str::to_ascii_lowercase),
        );
    }
    hosts.sort();
    hosts.dedup();
    if hosts.is_empty() {
        return Err(tool_error("image_configuration_invalid", None, false, None));
    }
    Ok(hosts)
}

fn validated_download_host<'a>(url: &'a Url, trusted: &[String]) -> Result<&'a str, String> {
    if url.scheme() != "https" || url.username() != "" || url.password().is_some() {
        return Err(tool_error(
            "image_url_rejected",
            None,
            false,
            Some("invalid_url"),
        ));
    }
    let host = url
        .host_str()
        .ok_or_else(|| tool_error("image_url_rejected", None, false, Some("missing_host")))?;
    let normalized_host = host.to_ascii_lowercase();
    if !trusted.iter().any(|trusted| trusted == &normalized_host) {
        return Err(tool_error(
            "image_url_rejected",
            None,
            false,
            Some("untrusted_host"),
        ));
    }
    if host
        .trim_start_matches('[')
        .trim_end_matches(']')
        .parse::<IpAddr>()
        .is_ok_and(ip_is_forbidden)
    {
        return Err(tool_error(
            "image_url_rejected",
            None,
            false,
            Some("private_address"),
        ));
    }
    Ok(host)
}

async fn pinned_download_client(url: &Url, trusted: &[String]) -> Result<reqwest::Client, String> {
    let host = validated_download_host(url, trusted)?;
    let port = url.port_or_known_default().unwrap_or(443);
    let addresses: Vec<SocketAddr> = tokio::net::lookup_host((host, port))
        .await
        .map_err(|_| {
            tool_error(
                "image_provider_unavailable",
                None,
                true,
                Some("dns_failure"),
            )
        })?
        .collect();
    if addresses.is_empty()
        || addresses
            .iter()
            .any(|address| ip_is_forbidden(address.ip()))
    {
        return Err(tool_error(
            "image_url_rejected",
            None,
            false,
            Some("private_address"),
        ));
    }
    reqwest::Client::builder()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(30))
        .resolve(host, addresses[0])
        .build()
        .map_err(|_| {
            tool_error(
                "image_configuration_invalid",
                None,
                false,
                Some("http_client"),
            )
        })
}

async fn download_image(
    url: &str,
    trusted: &[String],
    deadline: tokio::time::Instant,
    retry_used: &mut bool,
) -> Result<Vec<u8>, String> {
    let url = Url::parse(url)
        .map_err(|_| tool_error("image_url_rejected", None, false, Some("invalid_url")))?;
    let client = pinned_download_client(&url, trusted).await?;
    loop {
        let response = match send_before_deadline(client.get(url.clone()), deadline).await {
            Ok(response) => response,
            Err(_) if !*retry_used => {
                *retry_used = true;
                tokio::time::sleep(RETRY_DELAY).await;
                continue;
            }
            Err(error) => return Err(error),
        };
        let status = response.status();
        if !status.is_success() {
            let body = response_bytes_bounded(response, ERROR_BODY_MAX_BYTES)
                .await
                .unwrap_or_default();
            let retryable = status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error();
            if retryable && !*retry_used {
                *retry_used = true;
                tokio::time::sleep(RETRY_DELAY).await;
                continue;
            }
            return Err(status_error(status, &body));
        }
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let Some(content_type) = content_type else {
            return Err(tool_error(
                "image_response_invalid",
                None,
                false,
                Some("invalid_content_type"),
            ));
        };
        let bytes = match response_bytes_bounded(response, IMAGE_MAX_BYTES).await {
            Ok(bytes) => bytes,
            Err(error) if error.contains("retryable=true") && !*retry_used => {
                *retry_used = true;
                tokio::time::sleep(RETRY_DELAY).await;
                continue;
            }
            Err(error) => return Err(error),
        };
        return validate_image(bytes, Some(&content_type));
    }
}

pub struct ImageGenTool;

impl Tool for ImageGenTool {
    fn name(&self) -> &str {
        "image_gen"
    }

    fn description(&self) -> &str {
        "Generate an image from a text prompt and save it to a file. Provide a vivid, \
         self-contained prompt and an output filePath (e.g. run_dir/08_global_picture.png). \
         Returns the saved path."
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "prompt": {
                    "type": "string",
                    "description": "Full text-to-image prompt. Be concrete and self-contained; the image model sees only this text."
                },
                "filePath": {
                    "type": "string",
                    "description": "Where to write the PNG, relative to the working directory (e.g. runs/<ts>/08_global_picture.png). Use the path as given; do not add extra directory prefixes."
                },
                "size": {
                    "type": "string",
                    "default": "1536x1024",
                    "description": "Image size, e.g. 1024x1024, 1536x1024, or 1024x1536. Default 1536x1024 (landscape, good for a banner figure)."
                }
            },
            "required": ["prompt", "filePath"]
        })
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(DEFAULT_TOOL_TIMEOUT_SECS)
    }

    fn execute<'a>(
        &'a self,
        args: Value,
        env: &'a Environment,
    ) -> Pin<Box<dyn Future<Output = Result<ToolResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let prompt = args["prompt"]
                .as_str()
                .ok_or("missing required parameter 'prompt'")?;
            let file_path = args["filePath"]
                .as_str()
                .ok_or("missing required parameter 'filePath'")?;
            let size = args["size"].as_str().unwrap_or("1536x1024");

            let api_key = env
                .vars
                .get(API_KEY_ENV)
                .filter(|key| !key.is_empty())
                .ok_or_else(|| {
                    tool_error("image_configuration_missing", None, false, Some("api_key"))
                })?;
            let api_url = env
                .vars
                .get(API_URL_ENV)
                .filter(|url| !url.is_empty())
                .map(String::as_str)
                .unwrap_or(API_URL);

            let output_path = resolve_path(file_path, env)?;

            info!(target: "image_gen", model = DEFAULT_MODEL, size, path = ?output_path, "generating image");

            let client = reqwest::Client::builder()
                .redirect(Policy::none())
                .connect_timeout(Duration::from_secs(30))
                .build()
                .map_err(|_| {
                    tool_error(
                        "image_configuration_invalid",
                        None,
                        false,
                        Some("http_client"),
                    )
                })?;
            let deadline = tokio::time::Instant::now() + IMAGE_OPERATION_BUDGET;
            let mut retry_used = false;
            let response_body = loop {
                let response = match send_before_deadline(
                    client
                        .post(api_url)
                        .bearer_auth(api_key)
                        .json(&serde_json::json!({
                            "model": DEFAULT_MODEL,
                            "prompt": prompt,
                            "size": size,
                            "n": 1,
                        })),
                    deadline,
                )
                .await
                {
                    Ok(response) => response,
                    Err(_) if !retry_used => {
                        retry_used = true;
                        tokio::time::sleep(RETRY_DELAY).await;
                        continue;
                    }
                    Err(error) => return Err(error),
                };
                let status = response.status();
                let body_limit = if status.is_success() {
                    JSON_MAX_BYTES
                } else {
                    ERROR_BODY_MAX_BYTES
                };
                let body = match response_bytes_bounded(response, body_limit).await {
                    Ok(body) => body,
                    Err(error) if error.contains("retryable=true") && !retry_used => {
                        retry_used = true;
                        tokio::time::sleep(RETRY_DELAY).await;
                        continue;
                    }
                    Err(error) => return Err(error),
                };
                if status.is_success() {
                    break body;
                }
                let retryable = status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error();
                if retryable && !retry_used {
                    retry_used = true;
                    tokio::time::sleep(RETRY_DELAY).await;
                    continue;
                }
                return Err(status_error(status, &body));
            };

            let body: Value = serde_json::from_slice(&response_body).map_err(|_| {
                tool_error("image_response_invalid", None, false, Some("invalid_json"))
            })?;
            let data = body["data"].as_array().and_then(|items| items.first());
            let bytes = if let Some(b64) = data.and_then(|item| item["b64_json"].as_str()) {
                let encoded_limit = IMAGE_MAX_BYTES.saturating_mul(4) / 3 + 4;
                if b64.len() > encoded_limit {
                    return Err(tool_error("image_response_too_large", None, false, None));
                }
                let decoded = base64::engine::general_purpose::STANDARD
                    .decode(b64)
                    .map_err(|_| {
                        tool_error(
                            "image_response_invalid",
                            None,
                            false,
                            Some("invalid_base64"),
                        )
                    })?;
                validate_image(decoded, None)?
            } else if let Some(url) = data.and_then(|item| item["url"].as_str()) {
                let trusted = trusted_hosts(api_url, env.vars.get(TRUSTED_HOSTS_ENV))?;
                download_image(url, &trusted, deadline, &mut retry_used).await?
            } else {
                return Err(tool_error(
                    "image_response_invalid",
                    None,
                    false,
                    Some("missing_image_data"),
                ));
            };

            if let Some(parent) = output_path.parent() {
                tokio::fs::create_dir_all(parent)
                    .await
                    .map_err(|error| format!("failed to create output directory: {error}"))?;
            }
            tokio::fs::write(&output_path, &bytes)
                .await
                .map_err(|error| format!("failed to write image: {error}"))?;

            let relative = relative_path(&output_path, &env.cwd);
            let size_str = format!("{:.1} KB", bytes.len() as f64 / 1024.0);
            info!(target: "image_gen", path = %relative, size = %size_str, "image written");

            Ok(ToolResult {
                call_id: String::new(),
                content: format!("Wrote image to {relative} ({size_str})"),
                title: Some(format!("image {relative}")),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PNG: &[u8] = b"\x89PNG\r\n\x1a\nfixture";

    #[test]
    fn trusted_https_download_url_is_accepted() {
        let url = Url::parse("https://images.example.test/result.png").unwrap();
        let trusted = vec!["images.example.test".to_owned()];

        assert_eq!(
            validated_download_host(&url, &trusted).unwrap(),
            "images.example.test"
        );
    }

    #[test]
    fn private_or_loopback_download_url_is_rejected() {
        for value in [
            "https://127.0.0.1/result.png",
            "https://10.0.0.8/result.png",
            "https://[::1]/result.png",
        ] {
            let url = Url::parse(value).unwrap();
            let trusted = vec![url.host_str().unwrap().to_ascii_lowercase()];
            let error = validated_download_host(&url, &trusted).unwrap_err();
            assert!(error.contains("code=image_url_rejected"));
            assert!(error.contains("provider_code=private_address"));
        }
    }

    #[test]
    fn insecure_or_untrusted_download_url_is_rejected() {
        let trusted = vec!["images.example.test".to_owned()];
        let insecure = Url::parse("http://images.example.test/result.png").unwrap();
        let untrusted = Url::parse("https://attacker.example/result.png").unwrap();

        assert!(
            validated_download_host(&insecure, &trusted)
                .unwrap_err()
                .contains("provider_code=invalid_url")
        );
        assert!(
            validated_download_host(&untrusted, &trusted)
                .unwrap_err()
                .contains("provider_code=untrusted_host")
        );
    }

    #[test]
    fn image_content_type_must_match_its_signature() {
        assert_eq!(
            validate_image(PNG.to_vec(), Some("image/png; charset=binary")).unwrap(),
            PNG
        );
        let error = validate_image(PNG.to_vec(), Some("image/jpeg")).unwrap_err();
        assert!(error.contains("code=image_response_invalid"));
        assert!(error.contains("provider_code=invalid_content_type"));
    }

    #[test]
    fn oversized_or_invalid_image_is_rejected() {
        let oversized = vec![0_u8; IMAGE_MAX_BYTES + 1];
        assert!(
            validate_image(oversized, None)
                .unwrap_err()
                .contains("code=image_response_too_large")
        );
        assert!(
            validate_image(b"not an image".to_vec(), None)
                .unwrap_err()
                .contains("provider_code=invalid_image_signature")
        );
    }

    #[tokio::test]
    async fn expired_request_deadline_is_reported_as_retryable_timeout() {
        let client = reqwest::Client::new();
        let deadline = tokio::time::Instant::now() - Duration::from_millis(1);

        let error = send_before_deadline(
            client.get("http://192.0.2.1/never-contact-this-test-address"),
            deadline,
        )
        .await
        .unwrap_err();

        assert!(error.contains("code=image_provider_unavailable"));
        assert!(error.contains("retryable=true"));
        assert!(error.contains("provider_code=timeout"));
    }
}
