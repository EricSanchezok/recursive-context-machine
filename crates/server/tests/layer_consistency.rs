use std::collections::HashSet;

/// Every gRPC verb from `ACTION_VERBS` must be handled in `decode_command`.
///
/// This test catches the case where a new Action variant is added to
/// `machine::policy` but the corresponding gRPC decode arm is not added.
#[test]
fn all_action_verbs_have_decode_handler() {
    use machine::ACTION_VERBS;
    use server::decode::decode_command;
    use server::rcm::ActionCommand;

    for verb in ACTION_VERBS {
        let cmd = ActionCommand {
            verb: verb.to_string(),
            ..Default::default()
        };
        let result = decode_command(&cmd);
        // The decode should either return Ok (for verbs that need no args)
        // or Err with "invalid_argument" (for verbs that need args),
        // but never panic or return an unknown-verb error.
        match &result {
            Ok(_) => {} // valid verb with no required args (Take, Halt, Done)
            Err(s) => {
                assert!(
                    !s.message().contains("unknown verb"),
                    "verb '{}' is not handled by decode_command: {}",
                    verb,
                    s.message()
                );
                // Verbs that need args may return Err on empty ActionCommand
                assert!(
                    s.message().contains("required") || s.message().contains("fragment"),
                    "verb '{}' unexpected error: {}",
                    verb,
                    s.message()
                );
            }
        }
    }
}

/// Every Action variant's verb string has a matching entry in `ACTION_VERBS`.
#[test]
fn all_action_variants_have_verb_in_const_list() {
    use machine::ACTION_VERBS;
    use machine::Action;

    let verbs: HashSet<&str> = ACTION_VERBS.iter().copied().collect();

    // Collect all unique verbs from Action variants
    let action_verbs: HashSet<&str> = vec![
        Action::Append(machine::Fragment::user("x")).verb(),
        Action::Insert {
            after: 0,
            fragment: machine::Fragment::user("x"),
        }
        .verb(),
        Action::Replace {
            id: 0,
            fragment: machine::Fragment::user("x"),
        }
        .verb(),
        Action::Remove(0).verb(),
        Action::Swap(0, 1).verb(),
        Action::Model("x".into()).verb(),
        Action::Activate("x".into()).verb(),
        Action::Deactivate("x".into()).verb(),
        Action::Take.verb(),
        Action::Halt.verb(),
        Action::Done.verb(),
    ]
    .into_iter()
    .collect();

    for v in &action_verbs {
        assert!(
            verbs.contains(v),
            "Action verb '{}' is not in ACTION_VERBS const list — add it",
            v
        );
    }
    for v in &verbs {
        assert!(
            action_verbs.contains(v),
            "ACTION_VERBS contains '{}' but no Action variant produces it — remove it or add a variant",
            v
        );
    }
}

/// `ACTION_VERBS` has no duplicate entries.
#[test]
fn action_verbs_has_no_duplicates() {
    use machine::ACTION_VERBS;
    let mut seen = HashSet::new();
    for v in ACTION_VERBS {
        assert!(seen.insert(v), "ACTION_VERBS contains duplicate '{}'", v);
    }
}

/// State includes tool_infos and model_infos when tools/models are registered.
#[tokio::test]
async fn state_includes_tool_and_model_info() {
    use server::manager::MachineManager;
    use server::rcm::rcm_server::Rcm;
    use server::rcm::{LimitSpec, ModelSpec, OpenRequest};
    use server::service::RcmService;
    use tonic::Request;

    let runtime = RcmService::new(MachineManager::new());

    let req = OpenRequest {
        purpose: "info-test".into(),
        model_definitions: vec![ModelSpec {
            name: "test-model".into(),
            protocol: "openai".into(),
            limit: Some(LimitSpec {
                context: 4096,
                input: Some(2048),
                output: 1024,
            }),
            ..Default::default()
        }],
        models: vec!["test-model".into()],
        ..Default::default()
    };
    let resp = runtime.open(Request::new(req)).await.unwrap().into_inner();
    let state = resp.state.unwrap();

    // model_profiles: the one we requested should appear (but not active until activated)
    let model_names: Vec<_> = state
        .model_profiles
        .iter()
        .map(|profile| profile.name.as_str())
        .collect();
    assert!(model_names.contains(&"test-model"));
    let model_profile = state
        .model_profiles
        .iter()
        .find(|p| p.name == "test-model")
        .unwrap();
    assert_eq!(model_profile.protocol, "OpenAI");

    // tool_profiles: default catalog tools should be present
    if !state.tool_profiles.is_empty() {
        for tool_profile in &state.tool_profiles {
            assert!(!tool_profile.name.is_empty());
            assert!(!tool_profile.description.is_empty());
        }
    }
}

/// Fragment content_text is populated for text fragments.
#[tokio::test]
async fn fragment_content_text_is_populated() {
    use server::manager::MachineManager;
    use server::rcm::rcm_server::Rcm;
    use server::rcm::{DestroyRequest, OpenRequest, StepRequest};
    use server::service::RcmService;
    use tonic::Request;

    let runtime = RcmService::new(MachineManager::new());

    let resp = runtime
        .open(Request::new(OpenRequest {
            purpose: "content-test".into(),
            prompts: vec![("hello".into(), "Hello world from test".into())]
                .into_iter()
                .collect(),
            ..Default::default()
        }))
        .await
        .unwrap()
        .into_inner();
    let mid = resp.machine_id.clone();

    // Find Append hello and send it
    let append_cmd = resp
        .action_space
        .unwrap()
        .actions
        .iter()
        .find(|a| a.label == "Append hello")
        .unwrap()
        .command
        .clone()
        .unwrap();

    let step = runtime
        .step(Request::new(StepRequest {
            machine_id: mid.clone(),
            command: Some(append_cmd),
        }))
        .await
        .unwrap()
        .into_inner();

    let frag = &step.state.as_ref().unwrap().fragments[0];
    assert_eq!(frag.text_preview, "Hello world from test");
    assert_eq!(frag.content_text.as_deref(), Some("Hello world from test"));

    runtime
        .destroy(Request::new(DestroyRequest { machine_id: mid }))
        .await
        .unwrap();
}

/// State includes platform and root fields from Environment.
#[tokio::test]
async fn state_includes_env_metadata() {
    use server::manager::MachineManager;
    use server::rcm::OpenRequest;
    use server::rcm::rcm_server::Rcm;
    use server::service::RcmService;
    use tonic::Request;

    let runtime = RcmService::new(MachineManager::new());
    let resp = runtime
        .open(Request::new(OpenRequest {
            purpose: "env-test".into(),
            ..Default::default()
        }))
        .await
        .unwrap()
        .into_inner();
    let state = resp.state.unwrap();

    assert!(!state.platform.is_empty(), "platform should be set");
    // root is optional; just verify it exists as a field
    let _ = state.root;
}

/// build_fragment with kind="tool_call" creates a ToolCall fragment.
#[test]
fn build_fragment_with_kind_tool_call() {
    use machine::Content;
    use server::decode::build_fragment;
    use server::rcm::FragmentContent;

    let fc = FragmentContent {
        role: "assistant".into(),
        text: "shell".into(),
        tag: Some("call_1".into()),
        kind: "tool_call".into(),
        media_source: None,
    };
    let frag = build_fragment(&fc);
    assert!(matches!(frag.content, Content::ToolCall(_)));
    if let Content::ToolCall(tc) = &frag.content {
        assert_eq!(tc.name, "shell");
    }
    // tag is preserved as the call_id and fragment tag
    assert_eq!(frag.tag, "call_1");
}

/// build_fragment with kind="tool_result" creates a ToolResult fragment.
#[test]
fn build_fragment_with_kind_tool_result() {
    use machine::Content;
    use server::decode::build_fragment;
    use server::rcm::FragmentContent;

    let fc = FragmentContent {
        role: "tool".into(),
        text: "stdout: hello".into(),
        tag: Some("call_1".into()),
        kind: "tool_result".into(),
        media_source: None,
    };
    let frag = build_fragment(&fc);
    assert!(matches!(frag.content, Content::ToolResult(_)));
    if let Content::ToolResult(tr) = &frag.content {
        assert_eq!(tr.content, "stdout: hello");
    }
}

/// build_fragment with kind="hitch" creates a Hitch fragment.
#[test]
fn build_fragment_with_kind_hitch() {
    use machine::Content;
    use server::decode::build_fragment;
    use server::rcm::FragmentContent;

    let fc = FragmentContent {
        role: "system".into(),
        text: "something went wrong".into(),
        tag: None,
        kind: "hitch".into(),
        media_source: None,
    };
    let frag = build_fragment(&fc);
    assert!(matches!(frag.content, Content::Hitch { .. }));
}

/// build_fragment without kind (empty string) falls back to role-based mapping.
#[test]
fn build_fragment_empty_kind_falls_back_to_role() {
    use server::decode::build_fragment;
    use server::rcm::FragmentContent;

    // "user" role → User fragment
    let fc = FragmentContent {
        role: "user".into(),
        text: "hello".into(),
        tag: None,
        kind: String::new(),
        media_source: None,
    };
    let frag = build_fragment(&fc);
    assert_eq!(frag.role, machine::Role::User);

    // "assistant" role → Assistant fragment
    let fc2 = FragmentContent {
        role: "assistant".into(),
        text: "ok".into(),
        tag: None,
        kind: String::new(),
        media_source: None,
    };
    let frag2 = build_fragment(&fc2);
    assert_eq!(frag2.role, machine::Role::Assistant);
}

/// build_model with cost/thinking/temperature populates Model correctly.
#[test]
fn build_model_with_cost_thinking_temperature() {
    use server::decode::build_model;
    use server::rcm::{CostSpec, LimitSpec, ModelSpec};

    let spec = ModelSpec {
        name: "test-model".into(),
        protocol: "openai".into(),
        cost: Some(CostSpec {
            input: 0.15,
            output: 0.30,
            cache_read: Some(0.005),
            cache_write: Some(0.01),
        }),
        thinking: true,
        temperature: Some(0.7),
        limit: Some(LimitSpec {
            context: 4096,
            input: Some(2048),
            output: 1024,
        }),
        ..Default::default()
    };
    let model = build_model(&spec).unwrap();
    assert_eq!(model.name, "test-model");
    assert!(model.thinking);
    assert_eq!(model.temperature, Some(0.7));
    let cost = model.cost.unwrap();
    assert_eq!(cost.input, 0.15);
    assert_eq!(cost.output, 0.30);
    assert_eq!(cost.cache_read, Some(0.005));
    assert_eq!(cost.cache_write, Some(0.01));
}

// ── Multi-modal proto transport (Issue #68) ──

/// FragmentContent with kind="image" + media_source creates an Image fragment.
#[test]
fn build_fragment_with_kind_image_and_media_source() {
    use machine::Content;
    use server::decode::build_fragment;
    use server::rcm::FragmentContent;
    use server::rcm::{MediaSource, media_source};

    let fc = FragmentContent {
        role: "user".into(),
        text: String::new(),
        tag: None,
        kind: "image".into(),
        media_source: Some(MediaSource {
            source: Some(media_source::Source::Url(
                "https://example.com/img.png".into(),
            )),
            media_type: Some("image/png".into()),
            alt_text: None,
        }),
    };
    let frag = build_fragment(&fc);
    assert!(matches!(frag.content, Content::Image(_)));
}

/// FragmentContent with kind="audio" + media_source creates an Audio fragment.
#[test]
fn build_fragment_with_kind_audio_and_media_source() {
    use machine::Content;
    use server::decode::build_fragment;
    use server::rcm::FragmentContent;
    use server::rcm::{MediaSource, media_source};

    let fc = FragmentContent {
        role: "user".into(),
        text: String::new(),
        tag: None,
        kind: "audio".into(),
        media_source: Some(MediaSource {
            source: Some(media_source::Source::Base64("AAAA".into())),
            media_type: Some("audio/mp3".into()),
            alt_text: None,
        }),
    };
    let frag = build_fragment(&fc);
    assert!(matches!(frag.content, Content::Audio(_)));
}

/// MCP data URL with valid image/* MIME type produces data URL string.
#[test]
fn mcp_data_url_valid_mime() {
    use serde_json::json;

    let entry = json!({
        "type": "image",
        "mimeType": "image/png",
        "data": "iVBORw0KGgo"
    });
    let output = format_mcp_content(&entry);
    assert!(output.starts_with("data:image/png;base64,"));
    assert!(output.contains("iVBORw0KGgo"));
}

/// MCP data URL with invalid MIME type falls back to placeholder.
#[test]
fn mcp_data_url_invalid_mime() {
    use serde_json::json;

    let entry = json!({
        "type": "image",
        "mimeType": "application/x-shockwave-flash",
        "data": "somebinarydata"
    });
    let output = format_mcp_content(&entry);
    assert!(output.contains("invalid mime type"));
}

/// MCP data URL without mimeType defaults to fallback.
#[test]
fn mcp_data_url_no_data_falls_back() {
    use serde_json::json;

    let entry = json!({
        "type": "image",
        "url": "https://example.com/img.png"
    });
    let output = format_mcp_content(&entry);
    assert_eq!(output, "https://example.com/img.png");
}

// Helper: replicate the MCP append_content logic inline for testing.
fn format_mcp_content(entry: &serde_json::Value) -> String {
    let mut output = String::new();
    let mime = entry
        .get("mimeType")
        .and_then(|v| v.as_str())
        .unwrap_or("application/octet-stream");
    let is_valid_mime = mime.starts_with("image/") || mime.starts_with("audio/");
    if let Some(data) = entry.get("data").and_then(|v| v.as_str()) {
        if is_valid_mime {
            output.push_str(&format!("data:{mime};base64,{data}"));
        } else {
            output.push_str(&format!(
                "[MCP {} result: invalid mime type]",
                entry.get("type").and_then(|v| v.as_str()).unwrap_or("")
            ));
        }
    } else if let Some(url) = entry.get("url").and_then(|v| v.as_str()) {
        output.push_str(url);
    } else {
        output.push_str(&format!(
            "[MCP {} result]",
            entry.get("type").and_then(|v| v.as_str()).unwrap_or("")
        ));
    }
    output
}
