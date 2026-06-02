use std::path::Path;

#[test]
fn compile_autoresearch_survey_rcm_files() {
    let cli_manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = cli_manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("cli crate should be inside workspace/crates/cli");

    let example_files = [
        "examples/autoresearch-survey/rcm/anchor.rcm",
        "examples/autoresearch-survey/rcm/query_plan.rcm",
        "examples/autoresearch-survey/rcm/discovery.rcm",
        "examples/autoresearch-survey/rcm/expansion.rcm",
        "examples/autoresearch-survey/rcm/rank_pool.rcm",
        "examples/autoresearch-survey/rcm/research_map.rcm",
        "examples/autoresearch-survey/rcm/judge_panel.rcm",
        "examples/autoresearch-survey/rcm/survey_brief.rcm",
        "examples/autoresearch-survey/rcm/autoresearch_survey.rcm",
    ];

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    for example_file in example_files {
        let path = workspace_root.join(example_file);
        runtime
            .block_on(cli::rcm::compile::compile_file(&path))
            .unwrap_or_else(|error| panic!("failed to compile {}: {}", path.display(), error));
    }
}
