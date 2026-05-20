use std::collections::HashSet;

use utils::{AcceleratorId, FluxId, GraphId};

#[test]
fn generated_ids_use_type_prefixes() {
    assert!(GraphId::new().as_str().starts_with("graph_"));
    assert!(AcceleratorId::new().as_str().starts_with("accel_"));
    assert!(FluxId::new().as_str().starts_with("flux_"));
}

#[test]
fn generated_ids_are_unique_within_process() {
    let mut seen = HashSet::new();
    for _ in 0..1000 {
        let id = AcceleratorId::new();
        assert!(seen.insert(id.as_str().to_string()));
    }
}

#[test]
fn from_raw_preserves_valid_existing_id() {
    let id = GraphId::from_raw("graph_existing".to_string()).unwrap();
    assert_eq!(id.as_str(), "graph_existing");
}

#[test]
fn from_raw_rejects_wrong_prefix() {
    assert!(GraphId::from_raw("accel_wrong".to_string()).is_err());
}

#[test]
fn from_raw_rejects_empty_suffix() {
    assert!(GraphId::from_raw("graph_".to_string()).is_err());
}

#[test]
fn deserialization_rejects_wrong_prefix() {
    let parsed = serde_json::from_str::<GraphId>("\"accel_wrong\"");
    assert!(parsed.is_err());
}
