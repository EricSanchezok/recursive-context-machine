//! Tests for `Resources::active_model` and `Resources::use_model` after the
//! API hardening: they must return `Option` / `Result` rather than panic.

#[allow(dead_code)]
mod common;

use common::test_model;
use machine::{Model, ModelNotRegistered, Resources};

#[test]
fn active_model_is_none_when_empty() {
    let resources = Resources::new();
    assert!(resources.active_model().is_none());
}

#[test]
fn first_with_model_becomes_active() {
    let resources = Resources::new().with_model(test_model());
    let active = resources.active_model().expect("should have active");
    assert_eq!(active.name, "test");
}

#[test]
fn use_model_switches_active() {
    let mut resources = Resources::new()
        .with_model(test_model())
        .with_model(Model {
            name: "other".into(),
            ..Default::default()
        });
    assert_eq!(resources.active_model().unwrap().name, "test");

    let previous = resources.use_model("other").expect("registered");
    assert_eq!(previous, "test");
    assert_eq!(resources.active_model().unwrap().name, "other");
}

#[test]
fn use_model_unknown_returns_error_not_panic() {
    let mut resources = Resources::new().with_model(test_model());
    let result = resources.use_model("nope");
    match result {
        Err(ModelNotRegistered(name)) => assert_eq!(name, "nope"),
        Ok(_) => panic!("expected error for unknown model"),
    }
    // Active model unchanged after the failed switch.
    assert_eq!(resources.active_model().unwrap().name, "test");
}

#[test]
fn model_not_registered_error_includes_name() {
    let error = ModelNotRegistered("missing-x".into());
    assert!(error.to_string().contains("missing-x"));
}
