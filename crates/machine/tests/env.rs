//! Tests for `Environment::new` (host snapshot) vs `Environment::empty`
//! (sandbox). The contract: `new` must give the agent an honest view of the
//! host; `empty` must give a deliberately blank slate.

use machine::Environment;
use std::sync::Mutex;

/// Env-mutating tests in this file race against any other test that touches
/// the same vars; serialize them through a module-level mutex.
static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn new_inherits_host_path() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let key = "RCM_ENV_TEST_NEW_INHERITS";
    unsafe {
        std::env::set_var(key, "yes");
    }
    let env = Environment::new(".");
    let inherited = env.vars.get(key).map(|s| s.as_str());
    unsafe {
        std::env::remove_var(key);
    }
    assert_eq!(inherited, Some("yes"));
}

#[test]
fn new_inherits_path_for_subprocess_use() {
    // PATH is the canonical example; without it shell tools can't find any
    // binary. This is the bug shell tool was hitting before the fix.
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let host_path = std::env::var("PATH").ok();
    let env = Environment::new(".");
    if host_path.is_some() {
        assert!(
            env.vars.contains_key("PATH"),
            "Environment::new must inherit PATH from the host"
        );
    }
}

#[test]
fn empty_keeps_vars_empty() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let env = Environment::empty(".");
    assert!(
        env.vars.is_empty(),
        "Environment::empty must not inherit vars"
    );
}

#[test]
fn empty_does_not_leak_when_host_has_vars() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let key = "RCM_ENV_TEST_EMPTY_NO_LEAK";
    unsafe {
        std::env::set_var(key, "leaked");
    }
    let env = Environment::empty(".");
    unsafe {
        std::env::remove_var(key);
    }
    assert!(!env.vars.contains_key(key));
}

#[test]
fn named_inherits_too() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let key = "RCM_ENV_TEST_NAMED_INHERITS";
    unsafe {
        std::env::set_var(key, "yes");
    }
    let env = Environment::named("scope", ".");
    let inherited = env.vars.get(key).map(|s| s.as_str());
    unsafe {
        std::env::remove_var(key);
    }
    assert_eq!(env.name.as_str(), "scope");
    assert_eq!(inherited, Some("yes"));
}

#[test]
fn empty_named_keeps_vars_empty() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let env = Environment::empty_named("sandbox", ".");
    assert_eq!(env.name.as_str(), "sandbox");
    assert!(env.vars.is_empty());
}
