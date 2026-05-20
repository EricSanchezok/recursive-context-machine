use cli::rcm;

#[test]
fn parse_accelerator_block() {
    let source = r#"
        name = "test"
        accelerator research {
            purpose = "study quantum computing"
            model = "gpt"
            tools = ["websearch", "fs"]
        }
    "#;
    let file = rcm::parse(source).unwrap();
    assert_eq!(file.name, "test");
    assert_eq!(file.accelerators.len(), 1);
    assert_eq!(file.accelerators[0].id, "research");
    assert_eq!(
        file.accelerators[0].purpose.as_deref(),
        Some("study quantum computing")
    );
    assert_eq!(file.accelerators[0].tools, vec!["websearch", "fs"]);
}

#[test]
fn parse_model_with_credentials_and_limit() {
    let source = r#"
        name = "test"
        model gpt {
            protocol = "openai"
            endpoint = "https://api.example.com"
            credentials = { env = "MY_KEY" }
            limit = { context = "100000", output = "4096" }
            modalities = { input = ["text"], output = ["text"] }
        }
    "#;
    let file = rcm::parse(source).unwrap();
    let m = &file.models[0];
    assert_eq!(m.id, "gpt");
    assert_eq!(m.protocol, "openai");
    assert_eq!(m.credentials_env.as_deref(), Some("MY_KEY"));
    assert_eq!(m.limit_context, Some(100000));
    assert_eq!(m.limit_output, 4096);
    assert_eq!(m.modalities_input, vec!["text"]);
}

#[test]
fn parse_model_with_inline_key() {
    let source = r#"
        name = "test"
        model custom {
            protocol = "openai"
            credentials = { key = "sk-abc" }
            limit = { context = "1000", output = "500" }
        }
    "#;
    let file = rcm::parse(source).unwrap();
    let m = &file.models[0];
    assert_eq!(m.credentials_key.as_deref(), Some("sk-abc"));
}

#[test]
fn parse_condition_with_predicate() {
    let source = r#"
        name = "test"
        condition check {
            name = "Quality Check"
            all {
                purpose contains "done"
                context has_tag "results"
            }
        }
    "#;
    let file = rcm::parse(source).unwrap();
    assert_eq!(file.conditions.len(), 1);
}

#[test]
fn parse_wires() {
    let source = r#"
        name = "test"
        accelerator a {}
        accelerator b {}
        a.pulse -> b.pulse
    "#;
    let file = rcm::parse(source).unwrap();
    assert_eq!(file.wires.len(), 1);
}

#[test]
fn parse_rejects_unclosed_string() {
    let result = rcm::parse(r#"name = "test" accelerator x { purpose = "unclosed"#);
    assert!(result.is_err());
}

#[test]
fn parse_skips_comments() {
    let source = "// comment\nname = \"test\"\naccelerator a {}\n";
    let file = rcm::parse(source).unwrap();
    assert_eq!(file.accelerators.len(), 1);
}
