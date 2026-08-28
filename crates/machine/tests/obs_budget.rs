use machine::obs::{self, Obs};
use machine::{
    CompletionId, CompletionRecord, Fragment, Limit, Model, Resources, Role, RunState, Telemetry,
    TokenUsage,
};

fn run_state_with_model(context_limit: u64) -> RunState {
    let mut resources = Resources::named("test");
    resources = resources.with_model(Model {
        name: "flash".into(),
        limit: Some(Limit {
            context: context_limit,
            input: None,
            output: 4096,
        }),
        ..Default::default()
    });
    let active = resources.use_model("flash");
    assert!(active.is_ok());

    RunState {
        resources,
        ..RunState::default()
    }
}

fn record_with_input(step: u64, input_tokens: u64) -> CompletionRecord {
    CompletionRecord {
        id: CompletionId(step),
        step,
        model: Some("flash".into()),
        tokens: TokenUsage {
            input_tokens,
            output_tokens: 10,
            total_tokens: input_tokens + 10,
            cached_input_tokens: 0,
            cache_creation_input_tokens: 0,
        },
        output_fragment_ids: Vec::new(),
    }
}

#[test]
fn budget_is_zero_without_active_model() {
    let run = RunState::default();
    let observation = obs::measure(&run);
    assert_eq!(observation.budget.context_limit, 0);
    assert_eq!(observation.budget.soft_threshold, 0);
    assert_eq!(observation.budget.headroom, 0);
    assert_eq!(observation.budget.last_actual_input, None);
}

#[test]
fn budget_derives_limit_soft_threshold_and_headroom() {
    let mut run = run_state_with_model(1_000_000);
    run.context.append(Fragment::user("hello world"));

    let observation = obs::measure(&run);
    assert_eq!(observation.budget.context_limit, 1_000_000);
    assert_eq!(observation.budget.soft_threshold, 850_000);
    // 11 chars / 4 = 2 tokens of text + 4 framing tokens for one fragment.
    assert_eq!(observation.budget.estimated_input, 2 + 4);
    assert_eq!(
        observation.budget.headroom,
        1_000_000u64.saturating_sub(2 + 4)
    );
}

#[test]
fn estimated_input_grows_monotonically_with_fragments() {
    let mut run = run_state_with_model(100_000);
    let empty = obs::measure(&run).budget.estimated_input;

    run.context.append(Fragment::user("first"));
    let one = obs::measure(&run).budget.estimated_input;
    assert!(one >= empty);

    run.context.append(Fragment::assistant("second message"));
    let two = obs::measure(&run).budget.estimated_input;
    assert!(two > one);
}

#[test]
fn estimated_input_counts_active_tool_manifest_only() {
    let mut run = run_state_with_model(100_000);
    run.context.append(Fragment::user("task"));

    let before = obs::measure(&run).budget.estimated_input;

    let definition = machine::ToolDefinition {
        name: "lookup".into(),
        description: "look things up in the index".into(),
        parameters: serde_json::json!({"type": "object"}),
    };
    let mut resources = run.resources.clone();
    resources = resources.with_tool_definition(definition.clone());
    let inactive = {
        let mut state = run.clone();
        state.resources = resources.clone();
        obs::measure(&state).budget.estimated_input
    };
    assert_eq!(inactive, before, "inactive tools must not cost tokens");

    let enabled = resources.enable("lookup");
    assert!(enabled.is_ok());
    let mut state = run.clone();
    state.resources = resources;
    let after = obs::measure(&state).budget.estimated_input;
    assert!(after > before, "active tool manifest must add tokens");
}

#[test]
fn media_fragments_carry_fixed_token_estimate() {
    let mut run = run_state_with_model(100_000);
    run.context.append(Fragment::user("text only"));
    let text_only = obs::measure(&run).budget.estimated_input;

    let image = Fragment::image(
        machine::DataSource::Base64("aGVsbG8=".into()),
        Some("image/png".into()),
    );
    run.context.append(image);
    let with_image = obs::measure(&run).budget.estimated_input;

    // 500 for the image + 4 framing tokens for the extra fragment.
    assert!(with_image > text_only);
    assert!(with_image - text_only >= 500);
}

#[test]
fn calibration_prefers_newest_nonzero_measurement() {
    let mut run = run_state_with_model(100_000);
    let mut telemetry = Telemetry::default();
    telemetry.completions.push(record_with_input(1, 1_200));
    telemetry.completions.push(record_with_input(2, 0));
    telemetry.completions.push(record_with_input(3, 2_400));
    run.telemetry = telemetry;

    let observation = obs::measure(&run);
    assert_eq!(observation.budget.last_actual_input, Some(2_400));
}
#[test]
fn headroom_saturates_at_zero_when_estimate_exceeds_limit() {
    let mut run = run_state_with_model(10);
    let big_text = "x".repeat(400);
    run.context.append(Fragment::user(big_text));

    let observation = obs::measure(&run);
    assert_eq!(observation.budget.headroom, 0);
    assert!(observation.budget.estimated_input > observation.budget.context_limit);
}

#[test]
fn obs_default_enrichment_slots_are_empty() {
    let run = run_state_with_model(1_000);
    let observation: Obs = obs::measure(&run);
    assert_eq!(observation.ledger_digest, None);
    assert!(!observation.overlay_status.declared);
    assert_eq!(observation.overlay_status.system_prefix_count, 0);
    assert_eq!(observation.overlay_status.tail_count, 0);
}

#[test]
fn hitch_and_tool_result_text_counted_in_estimate() {
    let mut run = run_state_with_model(100_000);
    run.context.append(Fragment::user("task"));
    let baseline = obs::measure(&run).budget.estimated_input;

    let hitch = Fragment::hitch(
        "transport failed after retries",
        None,
        Role::Assistant,
        None::<&str>,
    );
    run.context.append(hitch);

    let tool_result = Fragment::tool_result(
        "call-1",
        "the quick brown fox jumps over the lazy dog",
        None,
    );
    run.context.append(tool_result);

    let after = obs::measure(&run).budget.estimated_input;
    assert!(after > baseline);
}
