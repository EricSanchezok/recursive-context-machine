use cli::rcm::{self, AcceleratorBodyDef};

/// The `map` block parses into `GraphDef.maps` with its inner alias, scatter,
/// and gather captured.
#[test]
fn map_block_parses_into_graph_maps() {
    let src = r#"
name = "t"

use "./inner.rcm" as Inner

graph {
    map expand {
        accelerator = Inner
        scatter = json
        gather  = digest
    }

    expand.done -> output.done
    expand.context -> output.context
}
"#;

    let file = rcm::parse(src).expect("map .rcm should parse");
    let AcceleratorBodyDef::Graph(graph) = file.body else {
        panic!("expected a graph body");
    };
    assert_eq!(graph.maps.len(), 1, "one map node expected");
    let map = &graph.maps[0];
    assert_eq!(map.id, "expand");
    assert_eq!(map.inner_alias, "Inner");
    assert_eq!(map.scatter, "json");
    assert_eq!(map.gather, "digest");
}

/// A map missing a required field is a parse error (not a silent default).
#[test]
fn map_requires_accelerator_scatter_and_gather() {
    let src = r#"
name = "t"
use "./inner.rcm" as Inner
graph {
    map expand {
        scatter = json
        gather  = digest
    }
    expand.done -> output.done
}
"#;
    let err = rcm::parse(src).expect_err("missing accelerator should fail");
    assert!(err.contains("accelerator"), "unexpected error: {err}");
}

/// `scatter = file "<name>"` captures the kind and the filename.
#[test]
fn map_scatter_file_parses() {
    let src = r#"
name = "t"

use "./inner.rcm" as Inner

graph {
    map expand {
        accelerator = Inner
        scatter = file "00_items.json"
        gather  = digest
    }

    expand.done -> output.done
    expand.context -> output.context
}
"#;

    let file = rcm::parse(src).expect("file-scatter map should parse");
    let AcceleratorBodyDef::Graph(graph) = file.body else {
        panic!("expected a graph body");
    };
    let map = &graph.maps[0];
    assert_eq!(map.scatter, "file");
    assert_eq!(map.scatter_file.as_deref(), Some("00_items.json"));
    assert_eq!(map.gather, "digest");
}
