use std::str::FromStr;

use utils::Name;

#[test]
fn accepts_clear_machine_names() {
    assert_eq!(Name::new("agent_1").unwrap().as_str(), "agent_1");
    assert_eq!(
        Name::new("context-append").unwrap().as_str(),
        "context-append"
    );
}

#[test]
fn rejects_empty_and_digit_prefixed_names() {
    assert!(Name::new("").is_err());
    assert!(Name::new("1agent").is_err());
}

#[test]
fn rejects_names_with_spaces_or_punctuation() {
    assert!(Name::new("my agent").is_err());
    assert!(Name::new("agent.one").is_err());
    assert!(Name::new("agent/one").is_err());
}

#[test]
fn parses_from_string() {
    let name = Name::from_str("local").unwrap();
    assert_eq!(name.as_str(), "local");
}
