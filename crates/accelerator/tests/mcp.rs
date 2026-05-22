use std::collections::HashMap;
use std::path::PathBuf;

use accelerator::mcp::{McpServerConfig, McpTransportConfig};

#[test]
fn stdio_config_keeps_process_parameters() {
    let mut env = HashMap::new();
    env.insert("API_KEY".to_string(), "secret".to_string());
    let config = McpServerConfig {
        label: "docs".into(),
        transport: McpTransportConfig::Stdio {
            command: "npx".into(),
            args: vec!["-y".into(), "@example/server".into()],
            env,
            cwd: Some(PathBuf::from(".")),
        },
    };

    match config.transport {
        McpTransportConfig::Stdio {
            command,
            args,
            env,
            cwd,
        } => {
            assert_eq!(command, "npx");
            assert_eq!(args, vec!["-y", "@example/server"]);
            assert_eq!(env.get("API_KEY").map(String::as_str), Some("secret"));
            assert_eq!(cwd, Some(PathBuf::from(".")));
        }
        _ => panic!("expected stdio config"),
    }
}

#[test]
fn http_config_keeps_endpoint_and_headers() {
    let config = McpServerConfig {
        label: "remote".into(),
        transport: McpTransportConfig::Http {
            url: "https://example.com/mcp".into(),
            headers: vec![
                ("Authorization".into(), "Bearer token".into()),
                ("X-Project".into(), "RCM".into()),
            ],
        },
    };

    match config.transport {
        McpTransportConfig::Http { url, headers } => {
            assert_eq!(url, "https://example.com/mcp");
            assert_eq!(headers.len(), 2);
        }
        _ => panic!("expected http config"),
    }
}

#[test]
fn sse_config_marks_legacy_transport_explicitly() {
    let config = McpServerConfig {
        label: "legacy".into(),
        transport: McpTransportConfig::Sse {
            url: "https://example.com/sse".into(),
            headers: Vec::new(),
        },
    };

    match config.transport {
        McpTransportConfig::Sse { url, headers } => {
            assert_eq!(url, "https://example.com/sse");
            assert!(headers.is_empty());
        }
        _ => panic!("expected sse config"),
    }
}
