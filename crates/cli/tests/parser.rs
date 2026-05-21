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
