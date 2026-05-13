use machine::Fragment;

#[test]
fn system_has_correct_role_and_tag() {
    let f = Fragment::system("hello");
    assert_eq!(f.role, machine::Role::System);
    assert_eq!(f.tag, "system");
    assert_eq!(f.id, 0);
}

#[test]
fn user_has_correct_role_and_tag() {
    let f = Fragment::user("hello");
    assert_eq!(f.role, machine::Role::User);
    assert_eq!(f.tag, "user");
}

#[test]
fn assistant_has_correct_role_and_tag() {
    let f = Fragment::assistant("hello");
    assert_eq!(f.role, machine::Role::Assistant);
    assert_eq!(f.tag, "assistant");
}

#[test]
fn tool_result_has_correct_role_and_tag() {
    let f = Fragment::tool_result("call_1", "done");
    assert_eq!(f.role, machine::Role::User);
    assert_eq!(f.tag, "tool_result");
}

#[test]
fn with_tag_overrides_default() {
    let f = Fragment::system("hello").with_tag("custom");
    assert_eq!(f.tag, "custom");
    assert_eq!(f.role, machine::Role::System);
}

#[test]
fn as_text_returns_text_content() {
    let f = Fragment::user("hello world");
    assert_eq!(f.as_text(), Some("hello world"));
}

#[test]
fn as_text_returns_none_for_non_text() {
    let f = Fragment::tool_result("id", "result");
    assert_eq!(f.as_text(), None);
}

#[test]
fn id_defaults_to_zero() {
    let f = Fragment::system("test");
    assert_eq!(f.id, 0);
}

#[test]
fn role_is_immutable_via_constructor() {
    // System fragment cannot become User via with_tag
    let f = Fragment::system("s").with_tag("user");
    assert_eq!(f.role, machine::Role::System);
    assert_eq!(f.tag, "user");
}
