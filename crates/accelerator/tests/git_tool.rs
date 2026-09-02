use accelerator::tools::{check_git_safety, tokenize_git};

fn tokens(input: &str) -> Vec<String> {
    tokenize_git(input).expect("tokenize")
}

fn assert_denied(command: &str, expected_fragment: &str) {
    let result = check_git_safety(&tokens(command));
    let reason = result.expect_err(&format!("expected '{command}' to be denied"));
    assert!(
        reason.contains(expected_fragment),
        "denial reason for '{command}' was '{reason}', expected to contain '{expected_fragment}'"
    );
}

fn assert_allowed(command: &str) {
    let result = check_git_safety(&tokens(command));
    assert!(
        result.is_ok(),
        "expected '{command}' to be allowed, got denial: {:?}",
        result.err()
    );
}

#[test]
fn tokenize_handles_double_quoted_args() {
    let result = tokenize_git("commit -m \"fix: handle empty\"").unwrap();
    assert_eq!(result, vec!["commit", "-m", "fix: handle empty"]);
}

#[test]
fn tokenize_handles_single_quoted_args() {
    let result = tokenize_git("commit -m 'a b c'").unwrap();
    assert_eq!(result, vec!["commit", "-m", "a b c"]);
}

#[test]
fn tokenize_rejects_unbalanced_quotes() {
    assert!(tokenize_git("commit -m \"unclosed").is_err());
    assert!(tokenize_git("commit -m 'unclosed").is_err());
}

#[test]
fn push_force_is_denied() {
    assert_denied("push --force origin feature/foo", "force push");
    assert_denied("push -f origin feature/foo", "force push");
    assert_denied("push --force-with-lease origin feature/foo", "force push");
}

#[test]
fn push_to_main_is_denied() {
    assert_denied("push origin main", "protected branch 'main'");
    assert_denied("push origin master", "protected branch 'master'");
    assert_denied("push upstream HEAD:main", "protected branch 'main'");
    assert_denied("push origin +feature/foo:main", "protected branch 'main'");
}

#[test]
fn push_delete_main_is_denied() {
    assert_denied("push origin :main", "deleting remote branch 'main'");
}

#[test]
fn push_to_feature_branch_is_allowed() {
    assert_allowed("push origin feature/foo");
    assert_allowed("push -u origin feature/maintainer-pipeline");
    assert_allowed("push origin HEAD:feature/foo");
}

#[test]
fn reset_hard_is_denied() {
    assert_denied("reset --hard", "reset --hard");
    assert_denied("reset --hard HEAD~1", "reset --hard");
}

#[test]
fn reset_soft_and_mixed_are_allowed() {
    assert_allowed("reset --soft HEAD~1");
    assert_allowed("reset HEAD~1");
    assert_allowed("reset --mixed HEAD");
}

#[test]
fn checkout_or_switch_main_is_denied() {
    assert_denied("checkout main", "protected default branch");
    assert_denied("switch main", "protected default branch");
    assert_denied("checkout master", "protected default branch");
    assert_denied("switch master", "protected default branch");
}

#[test]
fn checkout_force_is_denied() {
    assert_denied("checkout -f feature/foo", "force checkout");
    assert_denied("checkout --force feature/foo", "force checkout");
    assert_denied("switch --discard-changes feature/foo", "force switch");
}

#[test]
fn checkout_file_restore_is_denied() {
    assert_denied("checkout -- src/foo.rs", "checkout -- <path>");
    assert_denied("checkout .", "checkout . discards");
}

#[test]
fn checkout_create_branch_is_allowed() {
    assert_allowed("checkout -b feature/foo");
    assert_allowed("switch -c feature/foo");
    assert_allowed("checkout feature/foo");
}

#[test]
fn restore_worktree_is_denied() {
    assert_denied("restore src/foo.rs", "discards changes");
    assert_denied("restore --worktree src/foo.rs", "discards changes");
    assert_denied("restore --staged --worktree src/foo.rs", "discards changes");
}

#[test]
fn restore_staged_only_is_allowed() {
    assert_allowed("restore --staged src/foo.rs");
}

#[test]
fn branch_deletion_is_denied() {
    assert_denied("branch -D feature/foo", "branch deletion");
    assert_denied("branch --delete feature/foo", "branch deletion");
    assert_denied("branch -d feature/foo", "branch deletion");
}

#[test]
fn branch_create_and_list_are_allowed() {
    assert_allowed("branch feature/foo");
    assert_allowed("branch -a");
    assert_allowed("branch --list");
}

#[test]
fn stash_drop_and_clear_are_denied() {
    assert_denied("stash drop", "stash drop");
    assert_denied("stash clear", "stash clear");
}

#[test]
fn stash_push_pop_apply_are_allowed() {
    assert_allowed("stash");
    assert_allowed("stash push -m wip");
    assert_allowed("stash pop");
    assert_allowed("stash apply");
}

#[test]
fn clean_force_is_denied() {
    assert_denied("clean -f", "deletes untracked");
    assert_denied("clean -fd", "deletes untracked");
    assert_denied("clean -fdx", "deletes untracked");
    assert_denied("clean --force", "deletes untracked");
}

#[test]
fn clean_dry_run_is_allowed() {
    assert_allowed("clean -n");
    assert_allowed("clean --dry-run");
}

#[test]
fn no_verify_is_denied_on_commit_rebase_merge() {
    assert_denied("commit --no-verify -m fix", "skips required hooks");
    assert_denied("commit --no-gpg-sign -m fix", "skips required hooks");
    assert_denied("rebase --no-verify main", "skips required hooks");
    assert_denied("merge --no-verify feature/foo", "skips required hooks");
}

#[test]
fn config_changes_are_denied() {
    assert_denied("config user.email foo@example.com", "git config changes");
    assert_denied("config --global user.name X", "git config changes");
}

#[test]
fn common_safe_commands_are_allowed() {
    assert_allowed("status");
    assert_allowed("log -n 10");
    assert_allowed("diff");
    assert_allowed("diff --staged");
    assert_allowed("show HEAD");
    assert_allowed("add -A");
    assert_allowed("add src/foo.rs");
    assert_allowed("commit -m \"fix: bug\"");
    assert_allowed("fetch origin");
    assert_allowed("pull --rebase");
    assert_allowed("rev-parse HEAD");
}
