//! Minimal LSP JSON-RPC transport.

use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};

pub async fn read_message<R>(reader: &mut BufReader<R>) -> Result<Value, String>
where
    R: AsyncRead + Unpin,
{
    let mut content_length = None;

    loop {
        let mut line = String::new();
        let bytes = reader
            .read_line(&mut line)
            .await
            .map_err(|error| format!("failed to read LSP header: {error}"))?;

        if bytes == 0 {
            return Err("LSP server closed stdout".to_string());
        }

        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            break;
        }

        let Some((name, value)) = trimmed.split_once(':') else {
            continue;
        };

        if name.eq_ignore_ascii_case("Content-Length") {
            content_length = Some(
                value
                    .trim()
                    .parse::<usize>()
                    .map_err(|error| format!("invalid Content-Length header: {error}"))?,
            );
        }
    }

    let length = content_length.ok_or("missing Content-Length header")?;
    let mut body = vec![0u8; length];
    reader
        .read_exact(&mut body)
        .await
        .map_err(|error| format!("failed to read LSP body: {error}"))?;

    serde_json::from_slice(&body).map_err(|error| format!("invalid LSP JSON body: {error}"))
}

pub async fn write_message<W>(writer: &mut W, value: &Value) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let body =
        serde_json::to_vec(value).map_err(|error| format!("failed to encode LSP JSON: {error}"))?;
    let header = format!("Content-Length: {}\r\n\r\n", body.len());

    writer
        .write_all(header.as_bytes())
        .await
        .map_err(|error| format!("failed to write LSP header: {error}"))?;
    writer
        .write_all(&body)
        .await
        .map_err(|error| format!("failed to write LSP body: {error}"))?;
    writer
        .flush()
        .await
        .map_err(|error| format!("failed to flush LSP message: {error}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn reads_content_length_message() {
        let body = r#"{"jsonrpc":"2.0","id":1,"result":null}"#;
        let input = format!("Content-Length: {}\r\n\r\n{}", body.len(), body);
        let mut reader = BufReader::new(input.as_bytes());
        let value = read_message(&mut reader).await.unwrap();
        assert_eq!(value["id"], 1);
    }

    #[tokio::test]
    async fn writes_content_length_message() {
        let value = json!({"jsonrpc":"2.0","id":7,"method":"test"});
        let mut output = Vec::new();
        write_message(&mut output, &value).await.unwrap();
        let output = String::from_utf8(output).unwrap();
        assert!(output.starts_with("Content-Length: "));
        assert!(output.contains("\r\n\r\n"));
        assert!(output.contains("\"method\":\"test\""));
    }
}
