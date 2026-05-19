use accelerator::mcp::McpServerConfig;

#[test]
fn parse_stdio_simple_command() {
    let cfg = McpServerConfig::parse("gh=npx -y @modelcontextprotocol/server-github").unwrap();
    assert_eq!(cfg.label, "gh");
    assert_eq!(cfg.command.as_deref(), Some("npx"));
    assert_eq!(cfg.args, vec!["-y", "@modelcontextprotocol/server-github"]);
    assert!(cfg.url.is_none());
    assert!(cfg.headers.is_empty());
}

#[test]
fn parse_stdio_no_args() {
    let cfg = McpServerConfig::parse("myapp=./my-server").unwrap();
    assert_eq!(cfg.label, "myapp");
    assert_eq!(cfg.command.as_deref(), Some("./my-server"));
    assert!(cfg.args.is_empty());
    assert!(cfg.url.is_none());
}

#[test]
fn parse_http_bare_url() {
    let cfg = McpServerConfig::parse("search=https://api.anysearch.com/mcp").unwrap();
    assert_eq!(cfg.label, "search");
    assert!(cfg.command.is_none());
    assert_eq!(cfg.url.as_deref(), Some("https://api.anysearch.com/mcp"));
    assert!(cfg.headers.is_empty());
}

#[test]
fn parse_http_with_headers() {
    let cfg = McpServerConfig::parse(
        "search=https://api.anysearch.com/mcp|Authorization:Bearer tok123|X-Custom:val",
    )
    .unwrap();
    assert_eq!(cfg.label, "search");
    assert_eq!(cfg.url.as_deref(), Some("https://api.anysearch.com/mcp"));
    assert_eq!(
        cfg.headers,
        vec![
            ("Authorization".into(), "Bearer tok123".into()),
            ("X-Custom".into(), "val".into()),
        ]
    );
}

#[test]
fn parse_http_with_colon_in_value() {
    let cfg = McpServerConfig::parse(
        "search=https://api.anysearch.com/mcp|Authorization:Bearer tok:123:456",
    )
    .unwrap();
    assert_eq!(
        cfg.headers[0],
        ("Authorization".into(), "Bearer tok:123:456".into())
    );
}

#[test]
fn parse_missing_label() {
    assert!(McpServerConfig::parse("=npx foo").is_err());
}

#[test]
fn parse_missing_value() {
    assert!(McpServerConfig::parse("myapp=").is_err());
}

#[test]
fn parse_no_equals() {
    assert!(McpServerConfig::parse("justatext").is_err());
}
