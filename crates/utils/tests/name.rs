use std::str::FromStr;

use utils::Name;

#[test]
fn accepts_user_facing_labels() {
    assert_eq!(
        Name::new("Research Agent").unwrap().as_str(),
        "Research Agent"
    );
    assert_eq!(Name::new("研究节点").unwrap().as_str(), "研究节点");
    assert_eq!(Name::new("agent_1").unwrap().as_str(), "agent_1");
}

#[test]
fn rejects_empty_names() {
    assert!(Name::new("").is_err());
    assert!(Name::new("   ").is_err());
}

#[test]
fn rejects_control_characters() {
    assert!(Name::new("agent\nname").is_err());
    assert!(Name::new("agent\tname").is_err());
}

#[test]
fn deserialization_rejects_invalid_names() {
    assert!(serde_json::from_str::<Name>("\"\"").is_err());
    assert!(serde_json::from_str::<Name>("\"agent\\nname\"").is_err());
}

#[test]
fn parses_from_string() {
    let name = Name::from_str("local").unwrap();
    assert_eq!(name.as_str(), "local");
}
