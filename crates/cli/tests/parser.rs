use cli::rcm;
use cli::rcm::{AcceleratorBodyDef, AcceleratorSourceDef, EndpointDef, PortOwnerDef};

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
            model = "gpt"
            tools = ["fs", "shell"]
        }
    "#;

    let file = rcm::parse(source).unwrap();

    assert_eq!(file.name, "reviewer");
    assert_eq!(file.models.len(), 1);
    match file.body {
        AcceleratorBodyDef::Primitive(primitive) => {
            assert_eq!(primitive.purpose.as_deref(), Some("review code"));
            assert_eq!(primitive.tools, vec!["fs", "shell"]);
        }
        AcceleratorBodyDef::Graph(_) => panic!("expected primitive accelerator"),
    }
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
                model = "gpt"
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
                model = "gpt"
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
