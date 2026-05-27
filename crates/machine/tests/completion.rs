use machine::Fragment;
use machine::completion::encode;
use rig::completion::{AssistantContent, Message};
use serde_json::json;

fn tool_call_fragment() -> Fragment {
    Fragment::tool_call("call_123", "shell", json!({"command": "ls"}))
}

#[test]
fn encode_user_text_maps_to_user_message() {
    let frag = Fragment::user("hello");
    let msg = encode(&frag).expect("user encodes");
    assert!(matches!(msg, Message::User { .. }));
}

#[test]
fn encode_assistant_text_maps_to_assistant_message() {
    let frag = Fragment::assistant("ok");
    let msg = encode(&frag).expect("assistant encodes");
    assert!(matches!(msg, Message::Assistant { .. }));
}

#[test]
fn encode_tool_result_maps_to_user_tool_result() {
    let frag = Fragment::tool_result("call_123", "stdout content", None);
    let msg = encode(&frag).expect("tool result encodes");
    // rig wraps tool results as User messages carrying ToolResult content.
    assert!(matches!(msg, Message::User { .. }));
}

#[test]
fn encode_tool_call_always_attaches_reasoning_placeholder() {
    let frag = tool_call_fragment();
    let msg = encode(&frag).expect("tool call encodes");

    let Message::Assistant { content, .. } = msg else {
        panic!("expected Assistant message for tool call");
    };
    let has_reasoning = content
        .iter()
        .any(|item| matches!(item, AssistantContent::Reasoning(_)));
    assert!(
        has_reasoning,
        "tool call must always include reasoning placeholder for thinking-mode providers"
    );
    let has_tool_call = content
        .iter()
        .any(|item| matches!(item, AssistantContent::ToolCall(_)));
    assert!(has_tool_call, "tool_call content still present");
}

#[test]
fn encode_assistant_text_has_no_reasoning() {
    let frag = Fragment::assistant("plain text response");
    let msg = encode(&frag).expect("encodes");

    let Message::Assistant { content, .. } = msg else {
        panic!("expected Assistant message");
    };
    let has_reasoning = content
        .iter()
        .any(|item| matches!(item, AssistantContent::Reasoning(_)));
    assert!(
        !has_reasoning,
        "plain assistant text must not have reasoning"
    );
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
    let msg = encode(&frag).expect("system hitch encodes");
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
    let msg = encode(&frag).expect("assistant hitch encodes");
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
    let msg = encode(&frag);
    assert!(
        msg.is_none(),
        "Tool hitch without call_id cannot be encoded"
    );
}

#[test]
fn encode_system_text_unchanged_alongside_hitch() {
    let frag = Fragment::system("you are a helper");
    let msg = encode(&frag).expect("system encodes");
    assert!(matches!(msg, Message::System { .. }));
    if let Message::System { content } = &msg {
        assert_eq!(content, "you are a helper");
    }
}
