use accelerator::tools::shell::{OUTPUT_CAP_BYTES, build_result, collect_output};
use std::process::Stdio;
use tokio::process::Command;

#[test]
fn build_result_truncates_oversized_output() {
    let big = "x".repeat(OUTPUT_CAP_BYTES + 1024);
    let result = build_result("echo".into(), big.into_bytes(), Vec::new(), Some(0));
    let output = result.unwrap();
    assert!(
        output.content.len() <= OUTPUT_CAP_BYTES + 64,
        "output should be capped to OUTPUT_CAP_BYTES + suffix"
    );
    assert!(
        output.content.contains("truncated"),
        "truncation marker should be appended"
    );
}

#[test]
fn build_result_passes_small_output() {
    let result = build_result("echo".into(), b"hello".to_vec(), Vec::new(), Some(0));
    let output = result.unwrap();
    assert_eq!(output.content, "hello");
    assert!(!output.content.contains("truncated"));
}

#[test]
fn build_result_includes_stderr() {
    let result = build_result(
        "sh".into(),
        Vec::new(),
        b"warning: something".to_vec(),
        Some(1),
    );
    let output = result.unwrap();
    assert!(output.content.contains("warning"));
}

#[tokio::test]
async fn collect_output_caps_at_output_cap_bytes() {
    // Use head -c to generate output larger than OUTPUT_CAP_BYTES.
    // This avoids passing a huge argument on the command line (ARG_MAX).
    let generate = format!(
        "head -c {} < /dev/zero | tr '\\0' x",
        OUTPUT_CAP_BYTES + OUTPUT_CAP_BYTES / 2
    );
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(&generate)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    let (stdout, stderr, _exit_code) = collect_output(&mut child).await;

    assert!(
        stdout.len() <= OUTPUT_CAP_BYTES,
        "collect_output should cap stdout at OUTPUT_CAP_BYTES (got {})",
        stdout.len()
    );
    let _ = stderr;
}

#[tokio::test]
async fn collect_output_passes_small_through() {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg("echo hello world")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    let (stdout, stderr, _exit_code) = collect_output(&mut child).await;

    let text = String::from_utf8_lossy(&stdout);
    assert!(text.contains("hello world"));
    let _ = stderr;
}
