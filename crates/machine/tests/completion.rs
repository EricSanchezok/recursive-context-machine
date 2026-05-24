use machine::Fragment;
use machine::completion::encode;
use rig::completion::{AssistantContent, Message};
use serde_json::json;

fn tool_call_fragment() -> Fragment {
    Fragment::tool_call("call_123", "shell", json!({"command": "ls"}))
}

#[test]
fn encode_system_text_maps_to_system_message() {
    let frag = Fragment::system("you are a helper");
    let msg = encode(&frag, false).expect("system encodes");
    assert!(matches!(msg, Message::System { .. }));
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
