use std::time::Duration;

use cli::cmd::report::{self, RunMeasures};
use machine::{Context, Fragment};

fn context_with_answer(answer: &str) -> Context {
    let mut context = Context::new();
    context.append(Fragment::user("task"));
    context.append(Fragment::assistant(answer));
    context
}

fn read_report_json(dir: &std::path::Path) -> serde_json::Value {
    let raw = std::fs::read_to_string(dir.join("report.json")).expect("report.json written");
    serde_json::from_str(&raw).expect("report.json is valid JSON")
}

#[test]
fn report_carries_label_answer_cost_and_schema() {
    let dir = tempfile::tempdir().unwrap();
    let measures = RunMeasures {
        completions: 3,
        tool_calls: 7,
        input_tokens: 12_000,
        output_tokens: 800,
    };
    let path = report::write(
        dir.path(),
        "tb2-iter04-task17",
        "solve the task",
        &context_with_answer("final answer: 42"),
        measures,
        Duration::from_millis(1_500),
    );
    assert_eq!(path, Some(dir.path().join("report.json")));

    let report = read_report_json(dir.path());
    assert_eq!(report["schema"], "rcm.run.report/v1");
    assert_eq!(report["label"], "tb2-iter04-task17");
    assert_eq!(report["purpose"], "solve the task");
    assert_eq!(report["answer"], "final answer: 42");
    assert_eq!(report["steps"]["completions"], 3);
    assert_eq!(report["steps"]["tool_calls"], 7);
    assert_eq!(report["cost"]["input_tokens"], 12_000);
    assert_eq!(report["cost"]["output_tokens"], 800);
    assert_eq!(report["cost"]["total_tokens"], 12_800);
    assert_eq!(report["cost"]["wall_ms"], 1_500);
}

#[test]
fn report_scans_artifacts_from_the_run_directory() {
    let dir = tempfile::tempdir().unwrap();

    // Artifacts absent: pointers must be empty, not phantom.
    report::write(
        dir.path(),
        "scan",
        "p",
        &context_with_answer("a"),
        RunMeasures::default(),
        Duration::from_millis(1),
    );
    let bare = read_report_json(dir.path());
    assert_eq!(
        bare["artifacts"]["trajectory_dirs"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert!(bare["artifacts"]["registry"].is_null());
    assert!(bare["artifacts"]["ledger"].is_null());

    // Create the artifacts an optimizer would look for.
    let trajectory_dir = dir.path().join("trajectory").join("planner-1");
    std::fs::create_dir_all(&trajectory_dir).unwrap();
    std::fs::create_dir_all(dir.path().join("resources")).unwrap();
    std::fs::write(dir.path().join("resources").join("registry.json"), "{}").unwrap();
    std::fs::write(dir.path().join("ledger.json"), "{}").unwrap();

    report::write(
        dir.path(),
        "scan",
        "p",
        &context_with_answer("a"),
        RunMeasures::default(),
        Duration::from_millis(1),
    );
    let full = read_report_json(dir.path());
    let trajectory_dirs: Vec<&str> = full["artifacts"]["trajectory_dirs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry.as_str().unwrap())
        .collect();
    assert_eq!(trajectory_dirs, vec!["trajectory/planner-1"]);
    assert_eq!(full["artifacts"]["registry"], "resources/registry.json");
    assert_eq!(full["artifacts"]["ledger"], "ledger.json");
}

#[test]
fn long_purpose_and_answer_are_truncated_with_marker() {
    let dir = tempfile::tempdir().unwrap();
    let long_purpose = "x".repeat(3_000);
    let long_answer = "y".repeat(9_000);

    report::write(
        dir.path(),
        "trunc",
        &long_purpose,
        &context_with_answer(&long_answer),
        RunMeasures::default(),
        Duration::from_millis(1),
    );
    let report = read_report_json(dir.path());
    let purpose = report["purpose"].as_str().unwrap();
    assert!(purpose.contains("[truncated for report preview]"));
    assert!(purpose.len() < long_purpose.len() + 64);
    let answer = report["answer"].as_str().unwrap();
    assert!(answer.contains("[truncated for report preview]"));
    assert!(answer.len() < long_answer.len() + 64);
}

#[test]
fn report_without_run_dir_is_skipped_by_caller_not_by_writer() {
    // The writer always targets a directory; the run path only calls it when
    // run_dir exists. A missing directory must surface as a failed write the
    // caller already guards, verified here by writing into an existing empty dir.
    let dir = tempfile::tempdir().unwrap();
    let path = report::write(
        dir.path(),
        "noop",
        "p",
        &Context::new(),
        RunMeasures::default(),
        Duration::from_millis(0),
    );
    assert!(path.is_some());
    let report = read_report_json(dir.path());
    // No assistant fragment: answer is empty, never a panic.
    assert_eq!(report["answer"], "");
}
