use std::path::PathBuf;

/// Test that RunState with run_dir properly sets environment vars.
#[test]
fn run_dir_sets_rcm_run_dir_env_var() {
    let run_dir = PathBuf::from("/tmp/my-run");

    let mut state = machine::RunState {
        run_dir: Some(run_dir.clone()),
        ..machine::RunState::default()
    };
    state.environment.run_dir = Some(PathBuf::from("/tmp/my-run"));
    state
        .environment
        .vars
        .insert("RCM_RUN_DIR".to_string(), "/tmp/my-run".to_string());

    assert_eq!(state.run_dir, Some(PathBuf::from("/tmp/my-run")));
    assert_eq!(
        state.environment.run_dir,
        Some(PathBuf::from("/tmp/my-run"))
    );
    assert_eq!(
        state.environment.vars.get("RCM_RUN_DIR"),
        Some(&"/tmp/my-run".to_string())
    );
}

/// Test that RunState defaults have run_dir as None.
#[test]
fn run_dir_default_state_is_none() {
    let state = machine::RunState::default();
    assert!(state.run_dir.is_none(), "default run_dir should be None");
    assert!(
        state.environment.run_dir.is_none(),
        "default env.run_dir should be None"
    );
    assert!(
        !state.environment.vars.contains_key("RCM_RUN_DIR"),
        "default env should not contain RCM_RUN_DIR"
    );
}

/// Test that run_dir handles unicode paths.
#[test]
fn run_dir_handles_unicode_path() {
    let run_dir = PathBuf::from("/tmp/测试-运行");

    let mut state = machine::RunState {
        run_dir: Some(run_dir),
        ..machine::RunState::default()
    };
    state.environment.run_dir = Some(PathBuf::from("/tmp/测试-运行"));
    state
        .environment
        .vars
        .insert("RCM_RUN_DIR".to_string(), "/tmp/测试-运行".to_string());

    assert_eq!(state.run_dir, Some(PathBuf::from("/tmp/测试-运行")));
    assert_eq!(
        state.environment.vars.get("RCM_RUN_DIR"),
        Some(&"/tmp/测试-运行".to_string())
    );
}

/// Test that run_dir with relative path works.
#[test]
fn run_dir_accepts_relative_path() {
    let run_dir = PathBuf::from("relative/path/run");

    let mut state = machine::RunState {
        run_dir: Some(run_dir),
        ..machine::RunState::default()
    };
    state.environment.run_dir = Some(PathBuf::from("relative/path/run"));
    state
        .environment
        .vars
        .insert("RCM_RUN_DIR".to_string(), "relative/path/run".to_string());

    assert_eq!(state.run_dir, Some(PathBuf::from("relative/path/run")));
    assert_eq!(
        state.environment.vars.get("RCM_RUN_DIR"),
        Some(&"relative/path/run".to_string())
    );
}
