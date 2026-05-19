use accelerator::mcp::McpServerConfig;

#[test]
fn parse_simple_command() {
    let cfg = McpServerConfig::parse("gh=npx -y @modelcontextprotocol/server-github").unwrap();
    assert_eq!(cfg.label, "gh");
    assert_eq!(cfg.command, "npx");
    assert_eq!(cfg.args, vec!["-y", "@modelcontextprotocol/server-github"]);
}

#[test]
fn parse_no_args() {
    let cfg = McpServerConfig::parse("myapp=./my-server").unwrap();
    assert_eq!(cfg.label, "myapp");
    assert_eq!(cfg.command, "./my-server");
    assert!(cfg.args.is_empty());
}

#[test]
fn parse_missing_label() {
    assert!(McpServerConfig::parse("=npx foo").is_err());
}

#[test]
fn parse_missing_command() {
    assert!(McpServerConfig::parse("myapp=").is_err());
}

#[test]
fn parse_no_equals() {
    assert!(McpServerConfig::parse("justatext").is_err());
}
