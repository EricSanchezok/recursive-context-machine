use cli::rcm;
use cli::rcm::{
    AcceleratorBodyDef, AcceleratorSourceDef, EndpointDef, McpTransportDef, McpValueDef,
    PortOwnerDef, PromptSourceDef,
};

#[test]
fn parse_primitive_accelerator_file() {
    let source = r#"
        name = "reviewer"
        model gpt {
            protocol = "openai"
            credentials = { env = "MY_KEY" }
            limit = { context = "100000", output = "4096" }
            modalities = { input = ["text"], output = ["text"] }
        }
        accelerator {
            purpose = "review code"
            models = ["gpt"]
            prompts = {
                captain = file "./captain.txt"
                review = "Review carefully"
            }
            tools = ["fs", "shell"]
            mcps = ["docs"]
        }
    "#;

    let file = rcm::parse(source).unwrap();

    assert_eq!(file.name, "reviewer");
    assert_eq!(file.models.len(), 1);
    match file.body {
        AcceleratorBodyDef::Primitive(primitive) => {
            assert_eq!(primitive.purpose.as_deref(), Some("review code"));
            assert_eq!(primitive.models, vec!["gpt"]);
            assert_eq!(primitive.tools, Some(vec!["fs".into(), "shell".into()]));
            assert_eq!(primitive.mcps, Some(vec!["docs".into()]));
            let prompts = primitive.prompts.as_ref().unwrap();
            assert_eq!(
                prompts.get("captain"),
                Some(&PromptSourceDef::File("./captain.txt".into()))
            );
            assert_eq!(
                prompts.get("review"),
                Some(&PromptSourceDef::Inline("Review carefully".into()))
            );
        }
        AcceleratorBodyDef::Graph(_) => panic!("expected primitive accelerator"),
    }
}

#[test]
fn parse_mcp_transports() {
    let source = r#"
        name = "mcp demo"
        mcp docs {
            transport = stdio
            command = "npx"
            args = ["-y", "@example/server"]
            env = { API_KEY = env "DOCS_API_KEY" }
            cwd = "."
        }
        mcp remote {
            transport = http
            url = "https://example.com/mcp"
            headers = { Authorization = env "DOCS_TOKEN" X_Project = "RCM" }
        }
        mcp legacy {
            transport = sse
            url = "https://example.com/sse"
        }
        accelerator {
            models = ["gpt"]
            mcps = ["docs", "remote"]
        }
    "#;

    let file = rcm::parse(source).unwrap();

    assert_eq!(file.mcps.len(), 3);
    match &file.mcps[0].transport {
        McpTransportDef::Stdio {
            command,
            args,
            env,
            cwd,
        } => {
            assert_eq!(command, "npx");
            assert_eq!(args, &vec!["-y".to_string(), "@example/server".to_string()]);
            assert_eq!(
                env.get("API_KEY"),
                Some(&McpValueDef::Env("DOCS_API_KEY".into()))
            );
            assert_eq!(cwd.as_deref(), Some("."));
        }
        _ => panic!("expected stdio transport"),
    }
    match &file.mcps[1].transport {
        McpTransportDef::Http { url, headers } => {
            assert_eq!(url, "https://example.com/mcp");
            assert_eq!(
                headers.get("Authorization"),
                Some(&McpValueDef::Env("DOCS_TOKEN".into()))
            );
            assert_eq!(
                headers.get("X_Project"),
                Some(&McpValueDef::Literal("RCM".into()))
            );
        }
        _ => panic!("expected http transport"),
    }
    assert!(matches!(
        file.mcps[2].transport,
        McpTransportDef::Sse { .. }
    ));
}

#[test]
fn parse_graph_with_imported_accelerator_and_boundary_wires() {
    let source = r#"
        name = "pipeline"
        use "./reviewer.rcm" as Reviewer
        graph {
            accelerator review = Reviewer
            input.purpose -> review.purpose
            review.done -> output.done
            review.context -> output.context
        }
    "#;

    let file = rcm::parse(source).unwrap();

    assert_eq!(file.uses[0].alias, "Reviewer");
    let graph = match file.body {
        AcceleratorBodyDef::Graph(graph) => graph,
        AcceleratorBodyDef::Primitive(_) => panic!("expected graph accelerator"),
    };
    assert_eq!(graph.accelerators.len(), 1);
    assert_eq!(graph.wires.len(), 3);
    assert_eq!(graph.wires[0].from.owner, PortOwnerDef::Input);
    assert_eq!(
        graph.wires[0].to.endpoint,
        EndpointDef::State("purpose".into())
    );
}

#[test]
fn parse_graph_with_flux_and_condition_ports() {
    let source = r#"
        name = "pipeline"
        graph {
            accelerator source {
                purpose = "source"
                models = ["gpt"]
            }
            flux joined {
                channel = context
                mode = append
                arity = 2
            }
            condition approved {
                any {
                    purpose contains "LGTM"
                    context contains "approved"
                }
            }
            source.context -> joined.slot(0)
            joined.out -> source.context
            source.done -> approved.trigger
            approved.true -> output.done
        }
    "#;

    let file = rcm::parse(source).unwrap();
    let graph = match file.body {
        AcceleratorBodyDef::Graph(graph) => graph,
        AcceleratorBodyDef::Primitive(_) => panic!("expected graph accelerator"),
    };

    assert_eq!(graph.fluxes[0].arity, 2);
    assert_eq!(graph.conditions.len(), 1);
    assert_eq!(graph.wires[0].to.endpoint, EndpointDef::FluxSlot(0));
    assert_eq!(graph.wires[2].to.endpoint, EndpointDef::Trigger);
    assert_eq!(graph.wires[3].from.endpoint, EndpointDef::ConditionTrue);
}

#[test]
fn parse_inline_graph_accelerator() {
    let source = r#"
        name = "pipeline"
        graph {
            accelerator fetch {
                purpose = "fetch diff"
                models = ["gpt"]
                tools = ["shell"]
            }
        }
    "#;

    let file = rcm::parse(source).unwrap();
    let graph = match file.body {
        AcceleratorBodyDef::Graph(graph) => graph,
        AcceleratorBodyDef::Primitive(_) => panic!("expected graph accelerator"),
    };

    match &graph.accelerators[0].source {
        AcceleratorSourceDef::Inline(primitive) => {
            assert_eq!(primitive.purpose.as_deref(), Some("fetch diff"));
        }
        AcceleratorSourceDef::Import { .. } => panic!("expected inline accelerator"),
    }
}

#[test]
fn parse_rejects_unclosed_string() {
    let result = rcm::parse(r#"name = "test" accelerator { purpose = "unclosed"#);
    assert!(result.is_err());
}

#[test]
fn parse_model_with_headers() {
    let source = r#"
        name = "kimi-coder"
        model kimi {
            protocol = "openai"
            endpoint = "https://api.kimi.com/coding"
            credentials = { env = "KIMI_KEY" }
            headers = { User-Agent = "KimiCLI/1.5" X-Custom = "value" }
            limit = { context = "262144", output = "32768" }
            modalities = { input = ["text"], output = ["text"] }
        }
        accelerator {
            models = ["kimi"]
        }
    "#;

    let file = rcm::parse(source).unwrap();
    assert_eq!(file.models.len(), 1);
    let model = &file.models[0];
    assert_eq!(model.id, "kimi");
    assert_eq!(model.protocol, "openai");
    assert_eq!(
        model.endpoint.as_deref(),
        Some("https://api.kimi.com/coding")
    );
    assert_eq!(
        model.headers.get("User-Agent"),
        Some(&"KimiCLI/1.5".to_string())
    );
    assert_eq!(model.headers.get("X-Custom"), Some(&"value".to_string()));
}

#[test]
fn parse_model_without_headers() {
    let source = r#"
        name = "basic"
        model gpt {
            protocol = "openai"
            credentials = { env = "OPENAI_KEY" }
            limit = { context = "100000", output = "4096" }
            modalities = { input = ["text"], output = ["text"] }
        }
        accelerator {
            models = ["gpt"]
        }
    "#;

    let file = rcm::parse(source).unwrap();
    assert!(file.models[0].headers.is_empty());
}

#[test]
fn parse_model_with_thinking_true() {
    let source = r#"
        name = "kimi-thinking"
        model kimi {
            protocol = "openai"
            credentials = { env = "KIMI_KEY" }
            limit = { context = "262144", output = "32768" }
            modalities = { input = ["text"], output = ["text"] }
            thinking = "true"
        }
        accelerator {
            models = ["kimi"]
        }
    "#;

    let file = rcm::parse(source).unwrap();
    assert!(
        file.models[0].thinking,
        "thinking = \"true\" must be parsed as bool true"
    );
}

#[test]
fn parse_model_without_thinking_defaults_to_false() {
    let source = r#"
        name = "basic"
        model gpt {
            protocol = "openai"
            credentials = { env = "OPENAI_KEY" }
            limit = { context = "100000", output = "4096" }
            modalities = { input = ["text"], output = ["text"] }
        }
        accelerator {
            models = ["gpt"]
        }
    "#;

    let file = rcm::parse(source).unwrap();
    assert!(
        !file.models[0].thinking,
        "models without `thinking =` default to false (no reasoning_content pollution)"
    );
}

#[test]
fn parse_model_thinking_invalid_value_errors() {
    let source = r#"
        name = "bad"
        model x {
            protocol = "openai"
            credentials = { env = "K" }
            limit = { context = "1", output = "1" }
            modalities = { input = ["text"], output = ["text"] }
            thinking = "maybe"
        }
        accelerator { models = ["x"] }
    "#;

    let error = rcm::parse(source).unwrap_err();
    assert!(
        error.contains("thinking"),
        "error must mention 'thinking', got: {}",
        error
    );
}
