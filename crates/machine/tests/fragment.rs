use machine::{Content, Fragment, Role};

#[test]
fn fragment_constructor_roles() {
    assert_eq!(Fragment::system("x").role, Role::System);
    assert_eq!(Fragment::user("x").role, Role::User);
    assert_eq!(Fragment::assistant("x").role, Role::Assistant);
    assert_eq!(Fragment::tool_result("id", "ok", None).role, Role::Tool);
}

#[test]
fn as_text_roundtrips() {
    assert_eq!(Fragment::user("hello").as_text(), Some("hello"));
}

#[test]
fn as_text_none_for_nontinuitive_types() {
    assert_eq!(Fragment::tool_result("id", "ok", None).as_text(), None);
}

#[test]
fn hitch_content() {
    let f = Fragment::hitch("broken");
    assert!(matches!(f.content, Content::Hitch { ref message, .. } if message == "broken"));
    assert_eq!(f.role, Role::Hitch);
    if let Content::Hitch {
        retryable, code, ..
    } = f.content
    {
        assert!(!retryable);
        assert!(code.is_none());
    } else {
        panic!("expected Content::Hitch");
    }
}

#[test]
fn hitch_with_classification() {
    let f = Fragment::hitch_with("timeout", true, Some(504));
    assert_eq!(f.role, Role::Hitch);
    if let Content::Hitch {
        message,
        retryable,
        code,
    } = f.content
    {
        assert_eq!(message, "timeout");
        assert!(retryable);
        assert_eq!(code, Some(504));
    } else {
        panic!("expected Content::Hitch");
    }
}

#[test]
fn tag_override() {
    let f = Fragment::system("x").with_tag("custom");
    assert_eq!(f.tag, "custom");
    assert_eq!(f.role, Role::System); // role preserved
}

#[test]
fn tool_call_preserves_data() {
    let f = Fragment::tool_call("tc1", "add", serde_json::json!({"a": 1}));
    assert!(matches!(f.content, Content::ToolCall(ref tc) if tc.id == "tc1" && tc.name == "add"));
}
