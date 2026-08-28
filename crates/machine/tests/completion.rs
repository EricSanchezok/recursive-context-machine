use machine::completion::{build_request, decode, encode, encode_context};
use machine::{Content, Fragment, Limit, Model, Protocol, Role};
use rig::completion::message::{Reasoning, Text as RigText, ToolCall as RigToolCall, ToolFunction};
use rig::completion::{AssistantContent, Message};
use serde_json::json;

fn tool_call_fragment() -> Fragment {
    Fragment::tool_call("call_123", "shell", json!({"command": "ls"}))
}

#[test]
fn encode_user_text_maps_to_user_message() {
    let frag = Fragment::user("hello");
    let msg = encode(&frag, false).expect("user encodes");
    assert!(matches!(msg, Message::User { .. }));
}

#[test]
fn encode_assistant_text_maps_to_assistant_message() {
    let frag = Fragment::assistant("ok");
    let msg = encode(&frag, false).expect("assistant encodes");
    assert!(matches!(msg, Message::Assistant { .. }));
}

#[test]
fn encode_tool_result_maps_to_user_tool_result() {
    let frag = Fragment::tool_result("call_123", "stdout content", None);
    let msg = encode(&frag, false).expect("tool result encodes");
    // rig wraps tool results as User messages carrying ToolResult content.
    assert!(matches!(msg, Message::User { .. }));
}

#[test]
fn encode_context_drops_assistant_transport_hitches_for_exact_retry() {
    let fragments = vec![
        Fragment::system("system"),
        Fragment::user("request"),
        Fragment::hitch(
            "HTTP 504 Gateway Timeout",
            Some(504),
            Role::Assistant,
            None::<&str>,
        ),
    ];

    let messages = encode_context(&fragments, false);

    assert_eq!(messages.len(), 2);
    assert!(matches!(&messages[0], Message::System { .. }));
    assert!(matches!(&messages[1], Message::User { .. }));
}

#[test]
fn encode_context_reconstructs_parallel_tool_call_turn() {
    let reasoning = "Search both aspects before drafting";
    let fragments = vec![
        Fragment::system("system"),
        Fragment::user("request"),
        Fragment::tool_call("call_1", "search", json!({"query": "first"}))
            .with_reasoning(reasoning),
        Fragment::tool_result("call_1", "first result", None),
        Fragment::tool_call("call_2", "search", json!({"query": "second"}))
            .with_reasoning(reasoning),
        Fragment::tool_result("call_2", "second result", None),
    ];

    let messages = encode_context(&fragments, false);

    assert_eq!(messages.len(), 5);
    let Message::Assistant { content, .. } = &messages[2] else {
        panic!("parallel calls must be reconstructed as one assistant message");
    };
    assert_eq!(
        content
            .iter()
            .filter(|item| matches!(item, AssistantContent::ToolCall(_)))
            .count(),
        2,
    );
    assert_eq!(
        content
            .iter()
            .filter(|item| matches!(item, AssistantContent::Reasoning(_)))
            .count(),
        1,
    );
    assert!(matches!(&messages[3], Message::User { .. }));
    assert!(matches!(&messages[4], Message::User { .. }));
}

#[test]
fn encode_context_reconstructs_text_and_tool_call_as_one_assistant_turn() {
    let reasoning = "Read the canary input before writing the output";
    let response = [
        assistant_reasoning(reasoning),
        AssistantContent::Text(RigText {
            text: "I will read the fixed input first.".into(),
        }),
        assistant_tool_call("call_1", "fs"),
    ];
    let mut fragments = vec![Fragment::system("system"), Fragment::user("request")];
    fragments.extend(decode(response.iter()));
    fragments.push(Fragment::tool_result("call_1", "CANARY INPUT", None));

    let messages = encode_context(&fragments, true);

    assert_eq!(messages.len(), 4);
    let Message::Assistant { content, .. } = &messages[2] else {
        panic!("mixed response must be reconstructed as one assistant message");
    };
    assert_eq!(
        content
            .iter()
            .filter(|item| matches!(item, AssistantContent::Text(_)))
            .count(),
        1,
    );
    assert_eq!(
        content
            .iter()
            .filter(|item| matches!(item, AssistantContent::ToolCall(_)))
            .count(),
        1,
    );
    assert!(content.iter().any(|item| {
        matches!(item, AssistantContent::Reasoning(value) if value.display_text() == reasoning)
    }));
    assert!(matches!(&messages[3], Message::User { .. }));
}

#[test]
fn encode_context_reconstructs_mixed_turn_with_failed_tool_result() {
    let reasoning = "Read the input before writing the output";
    let fragments = vec![
        Fragment::system("system"),
        Fragment::user("request"),
        Fragment::assistant("I will read the input first."),
        Fragment::tool_call("call_1", "fs", json!({"path": "input.txt"})).with_reasoning(reasoning),
        Fragment::hitch(
            "tool request timed out",
            Some(504),
            Role::Tool,
            Some("call_1"),
        ),
    ];

    let messages = encode_context(&fragments, true);

    assert_eq!(messages.len(), 4);
    let Message::Assistant { content, .. } = &messages[2] else {
        panic!("failed mixed turn must remain one assistant message");
    };
    assert_eq!(
        content
            .iter()
            .filter(|item| matches!(item, AssistantContent::Text(_)))
            .count(),
        1,
    );
    assert_eq!(
        content
            .iter()
            .filter(|item| matches!(item, AssistantContent::ToolCall(_)))
            .count(),
        1,
    );
    assert!(content.iter().any(|item| {
        matches!(item, AssistantContent::Reasoning(value) if value.display_text() == reasoning)
    }));
    assert!(matches!(&messages[3], Message::User { .. }));
}

#[test]
fn encode_context_reconstructs_parallel_turn_with_failed_tool_result() {
    let reasoning = "Search both sources before drafting";
    let fragments = vec![
        Fragment::system("system"),
        Fragment::user("request"),
        Fragment::tool_call("call_1", "search", json!({"query": "first"}))
            .with_reasoning(reasoning),
        Fragment::tool_result("call_1", "first result", None),
        Fragment::tool_call("call_2", "search", json!({"query": "second"}))
            .with_reasoning(reasoning),
        Fragment::hitch(
            "upstream search unavailable",
            Some(503),
            Role::Tool,
            Some("call_2"),
        ),
    ];

    let messages = encode_context(&fragments, true);

    assert_eq!(messages.len(), 5);
    let Message::Assistant { content, .. } = &messages[2] else {
        panic!("failed parallel turn must remain one assistant message");
    };
    assert_eq!(
        content
            .iter()
            .filter(|item| matches!(item, AssistantContent::ToolCall(_)))
            .count(),
        2,
    );
    assert!(content.iter().any(|item| {
        matches!(item, AssistantContent::Reasoning(value) if value.display_text() == reasoning)
    }));
    assert!(matches!(&messages[3], Message::User { .. }));
    assert!(matches!(&messages[4], Message::User { .. }));
}

#[test]
fn encode_tool_call_without_thinking_omits_reasoning() {
    let frag = tool_call_fragment();
    let msg = encode(&frag, false).expect("tool call encodes");

    let Message::Assistant { content, .. } = msg else {
        panic!("expected Assistant message for tool call");
    };
    let has_reasoning = content
        .iter()
        .any(|item| matches!(item, AssistantContent::Reasoning(_)));
    assert!(
        !has_reasoning,
        "thinking=false must not attach reasoning placeholder — would pollute providers like DeepSeek/OpenAI"
    );
    let has_tool_call = content
        .iter()
        .any(|item| matches!(item, AssistantContent::ToolCall(_)));
    assert!(has_tool_call, "tool_call content still present");
}

#[test]
fn encode_tool_call_with_thinking_attaches_reasoning_placeholder() {
    let frag = tool_call_fragment();
    let msg = encode(&frag, true).expect("tool call encodes");

    let Message::Assistant { content, .. } = msg else {
        panic!("expected Assistant message for tool call");
    };
    let reasoning_count = content
        .iter()
        .filter(|item| matches!(item, AssistantContent::Reasoning(_)))
        .count();
    assert_eq!(
        reasoning_count, 1,
        "thinking=true must attach exactly one reasoning placeholder for providers like Kimi"
    );
    let tool_call_count = content
        .iter()
        .filter(|item| matches!(item, AssistantContent::ToolCall(_)))
        .count();
    assert_eq!(
        tool_call_count, 1,
        "tool_call content preserved alongside reasoning"
    );
}

/// DeepSeek thinking mode rejects requests whose assistant tool-call turn
/// carries a placeholder reasoning. When the fragment was decoded from a real
/// LLM response, `encode` must replay the model's original reasoning text
/// verbatim — not the `.` stub. Regression for the 400 loop observed when
/// running grpc_demo against `deepseek-v4-flash`.
#[test]
fn encode_tool_call_emits_stored_reasoning_not_placeholder() {
    let frag = Fragment::tool_call("call_x", "shell", json!({"command": "ls"}))
        .with_reasoning("the user asked for a directory listing");

    for thinking in [true, false] {
        let msg = encode(&frag, thinking).expect("encodes");
        let Message::Assistant { content, .. } = msg else {
            panic!("expected Assistant message");
        };
        let reasoning_texts: Vec<String> = content
            .iter()
            .filter_map(|item| match item {
                AssistantContent::Reasoning(r) => Some(r.display_text()),
                _ => None,
            })
            .collect();
        assert_eq!(
            reasoning_texts.len(),
            1,
            "thinking={thinking}: exactly one reasoning block emitted",
        );
        assert_eq!(
            reasoning_texts[0], "the user asked for a directory listing",
            "thinking={thinking}: reasoning text must come from fragment, not '.'",
        );
    }
}

/// Symmetric: text-only assistant fragments never carry reasoning, even when
/// `thinking=true`. The placeholder is tool-call-only.
#[test]
fn encode_assistant_text_with_thinking_does_not_synthesize_reasoning() {
    let frag = Fragment::assistant("here is your answer");
    let msg = encode(&frag, true).expect("encodes");
    let Message::Assistant { content, .. } = msg else {
        panic!("expected Assistant message");
    };
    assert!(
        !content
            .iter()
            .any(|item| matches!(item, AssistantContent::Reasoning(_))),
        "text assistant turns must not gain reasoning content",
    );
}

#[test]
fn encode_assistant_non_tool_call_ignores_thinking_flag() {
    let frag = Fragment::assistant("plain text response");
    let msg_off = encode(&frag, false).expect("encodes");
    let msg_on = encode(&frag, true).expect("encodes");

    // Plain assistant text never gets a Reasoning attached — that path is
    // tool-call-only.
    for (label, msg) in [("thinking=false", msg_off), ("thinking=true", msg_on)] {
        let Message::Assistant { content, .. } = msg else {
            panic!("expected Assistant message ({label})");
        };
        let has_reasoning = content
            .iter()
            .any(|item| matches!(item, AssistantContent::Reasoning(_)));
        assert!(
            !has_reasoning,
            "{label}: plain assistant text must not gain reasoning"
        );
    }
}

// ── Hitch encoding (issue #43 subproblem 3) ──

#[test]
fn encode_system_hitch_maps_to_system_message() {
    let frag = Fragment::hitch(
        "model 'xyz' not registered",
        None,
        machine::Role::System,
        None::<&str>,
    );
    let msg = encode(&frag, false).expect("system hitch encodes");
    assert!(matches!(msg, Message::System { .. }));
    if let Message::System { content } = &msg {
        assert!(content.contains("not registered"));
    }
}

#[test]
fn encode_assistant_hitch_maps_to_assistant_message() {
    let frag = Fragment::hitch(
        "HttpError: 502 Bad Gateway",
        None,
        machine::Role::Assistant,
        None::<&str>,
    );
    let msg = encode(&frag, false).expect("assistant hitch encodes");
    assert!(matches!(msg, Message::Assistant { .. }));
    if let Message::Assistant { content, .. } = &msg {
        let texts: Vec<_> = content
            .iter()
            .filter_map(|c| {
                if let AssistantContent::Text(t) = c {
                    Some(t.text.as_str())
                } else {
                    None
                }
            })
            .collect();
        assert!(texts.iter().any(|t| t.contains("502")));
    }
}

#[test]
fn encode_tool_hitch_without_call_id_returns_none() {
    let frag = Fragment::hitch(
        "tool 'unknown' not found",
        None,
        machine::Role::Tool,
        None::<&str>,
    );
    let msg = encode(&frag, false);
    assert!(
        msg.is_none(),
        "Tool hitch without call_id cannot be encoded"
    );
}

#[test]
fn encode_system_text_unchanged_alongside_hitch() {
    let frag = Fragment::system("you are a helper");
    let msg = encode(&frag, false).expect("system encodes");
    assert!(matches!(msg, Message::System { .. }));
    if let Message::System { content } = &msg {
        assert_eq!(content, "you are a helper");
    }
}

// ── build_request: chat_history shape ──

fn dummy_model() -> Model {
    Model {
        name: "test".into(),
        protocol: Protocol::OpenAI,
        ..Default::default()
    }
}

/// The whole point of building `CompletionRequest` directly: the
/// `chat_history` we send to the provider matches input order byte-for-byte.
/// Guards against the #43 / #86 regressions where a builder-managed
/// "initial prompt" rotated messages.
#[test]
fn build_request_preserves_message_order() {
    let msgs = vec![
        Message::system("you are a helper"),
        Message::user("search for papers"),
        Message::assistant("let me check"),
    ];

    let req = build_request(&msgs, &[], &dummy_model()).expect("non-empty builds");
    let history: Vec<Message> = req.chat_history.into_iter().collect();

    assert_eq!(history.len(), 3, "no message dropped or duplicated");
    assert!(
        matches!(&history[0], Message::System { content } if content == "you are a helper"),
        "system message stays at the head"
    );
    assert!(matches!(&history[1], Message::User { .. }));
    assert!(matches!(&history[2], Message::Assistant { .. }));
}

#[test]
fn build_request_merges_adjacent_system_messages_without_reordering_turns() {
    let messages = vec![
        Message::system("agent instructions"),
        Message::system("cwd: .\nplatform: linux"),
        Message::user("search for papers"),
        Message::system("later instruction"),
    ];

    let request = build_request(&messages, &[], &dummy_model()).expect("non-empty builds");
    let history: Vec<Message> = request.chat_history.into_iter().collect();

    assert_eq!(history.len(), 3);
    assert!(matches!(
        &history[0],
        Message::System { content }
            if content == "agent instructions\n\ncwd: .\nplatform: linux"
    ));
    assert!(matches!(&history[1], Message::User { .. }));
    assert!(matches!(
        &history[2],
        Message::System { content } if content == "later instruction"
    ));
}

#[test]
fn build_request_empty_messages_returns_hitch() {
    let result = build_request(&[], &[], &dummy_model());
    let Err(hitch) = result else {
        panic!("expected Err(hitch) for empty messages");
    };
    assert!(matches!(hitch.role, Role::System));
    assert!(
        matches!(&hitch.content, Content::Hitch { message, .. } if message.contains("empty context"))
    );
}

#[test]
fn build_request_single_message_chat_history_has_one() {
    let msgs = vec![Message::user("hello")];
    let req = build_request(&msgs, &[], &dummy_model()).expect("single builds");
    let history: Vec<Message> = req.chat_history.into_iter().collect();
    assert_eq!(history.len(), 1);
    assert!(matches!(&history[0], Message::User { .. }));
}

#[test]
fn build_request_passes_model_temperature_and_max_tokens() {
    let mut model = dummy_model();
    model.temperature = Some(0.42);
    model.limit = Some(Limit {
        context: 100_000,
        input: None,
        output: 4096,
    });

    let msgs = vec![Message::user("hi")];
    let req = build_request(&msgs, &[], &model).expect("builds");

    assert_eq!(req.temperature, Some(0.42));
    assert_eq!(req.max_tokens, Some(4096));
}

// ── decode: reasoning preservation ──

fn assistant_reasoning(text: &str) -> AssistantContent {
    AssistantContent::Reasoning(Reasoning::new(text))
}

fn assistant_tool_call(id: &str, name: &str) -> AssistantContent {
    AssistantContent::ToolCall(RigToolCall {
        id: id.into(),
        call_id: None,
        function: ToolFunction {
            name: name.into(),
            arguments: json!({}),
        },
        signature: None,
        additional_params: None,
    })
}

/// Regression for the deepseek-v4-flash 400 loop: when the model emits
/// reasoning followed by a tool_call in the same turn, decode must store
/// the reasoning on the ToolCall fragment so the next request can echo it.
#[test]
fn decode_attaches_reasoning_to_following_tool_call() {
    let response = [
        assistant_reasoning("I should list the files"),
        assistant_tool_call("call_1", "shell"),
    ];

    let fragments = decode(response.iter());

    assert_eq!(fragments.len(), 1, "reasoning is folded into the tool call");
    let Content::ToolCall(tc) = &fragments[0].content else {
        panic!("expected ToolCall content");
    };
    assert_eq!(tc.reasoning.as_deref(), Some("I should list the files"));
}

#[test]
fn decode_concatenates_multi_block_reasoning_before_tool_call() {
    let response = [
        assistant_reasoning("first thought"),
        assistant_reasoning("refinement"),
        assistant_tool_call("call_1", "shell"),
    ];

    let fragments = decode(response.iter());

    let Content::ToolCall(tc) = &fragments[0].content else {
        panic!("expected ToolCall");
    };
    assert_eq!(tc.reasoning.as_deref(), Some("first thought\nrefinement"));
}

#[test]
fn decode_discards_reasoning_before_text_turn() {
    let response = [
        assistant_reasoning("musing aloud"),
        AssistantContent::Text(RigText {
            text: "here is the answer".into(),
        }),
    ];

    let fragments = decode(response.iter());

    assert_eq!(fragments.len(), 1);
    assert!(matches!(&fragments[0].content, Content::Text(t) if t.text == "here is the answer"));
}

#[test]
fn decode_preserves_reasoning_across_text_before_tool_call() {
    let response = [
        assistant_reasoning("I should explain before writing"),
        AssistantContent::Text(RigText {
            text: "The schema is clear.".into(),
        }),
        assistant_tool_call("call_1", "fs"),
    ];

    let mut fragments = decode(response.iter());

    assert_eq!(fragments.len(), 2);
    assert!(matches!(&fragments[0].content, Content::Text(t) if t.text == "The schema is clear."));
    let Content::ToolCall(tool_call) = &fragments[1].content else {
        panic!("expected ToolCall after visible assistant text");
    };
    assert_eq!(
        tool_call.reasoning.as_deref(),
        Some("I should explain before writing")
    );

    fragments.push(Fragment::tool_result("call_1", "written", None));
    let messages = encode_context(&fragments, true);
    assert_eq!(messages.len(), 2);
    let Message::Assistant { content, .. } = &messages[0] else {
        panic!("expected reconstructed mixed assistant message");
    };
    assert!(content.iter().any(|item| {
        matches!(item, AssistantContent::Text(text) if text.text == "The schema is clear.")
    }));
    assert!(
        content
            .iter()
            .any(|item| matches!(item, AssistantContent::ToolCall(_)))
    );
    assert!(content.iter().any(|item| {
        matches!(item, AssistantContent::Reasoning(reasoning)
            if reasoning.display_text() == "I should explain before writing")
    }));
}

#[test]
fn decode_tool_call_without_reasoning_has_none() {
    let response = [assistant_tool_call("call_1", "shell")];

    let fragments = decode(response.iter());

    let Content::ToolCall(tc) = &fragments[0].content else {
        panic!("expected ToolCall");
    };
    assert!(tc.reasoning.is_none(), "no reasoning emitted, none stored");
}

/// When the LLM emits parallel tool_calls in one turn (one reasoning
/// block, multiple tool_calls), every fragment must carry the same
/// reasoning. Otherwise the second fragment re-encodes with a placeholder
/// and providers like DeepSeek reject the request with HTTP 400
/// "reasoning_content must be passed back". Regression for paper_digest
/// demo at 2026-05-27T14:10:36.
#[test]
fn decode_parallel_tool_calls_share_one_reasoning() {
    let response = [
        assistant_reasoning("I will run two searches in parallel"),
        assistant_tool_call("call_1", "arxiv_search"),
        assistant_tool_call("call_2", "arxiv_search"),
    ];

    let fragments = decode(response.iter());

    assert_eq!(fragments.len(), 2);
    for (idx, frag) in fragments.iter().enumerate() {
        let Content::ToolCall(tc) = &frag.content else {
            panic!("fragment {idx} is not a ToolCall");
        };
        assert_eq!(
            tc.reasoning.as_deref(),
            Some("I will run two searches in parallel"),
            "fragment {idx}: both tool_calls in the same turn must carry the same reasoning",
        );
    }
}
