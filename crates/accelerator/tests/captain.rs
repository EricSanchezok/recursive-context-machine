//! Behavioural tests for Captain's hitch retry logic.
//!
//! Captain must:
//! - retry retryable hitches up to MAX_HITCH_RETRIES times
//! - give up immediately on a non-retryable hitch
//! - reset the retry counter on any non-hitch turn
//! - reset state on clone

use accelerator::policy::Captain;
use machine::{Action, Context, Environment, Fragment, Inbox, Policy, Purpose, Resources};

fn empty_purpose() -> Purpose {
    Purpose::new(String::new())
}

fn empty_env() -> Environment {
    Environment::new(".")
}

fn empty_resources() -> Resources {
    Resources::new()
}

#[tokio::test]
async fn first_decide_halts() {
    let captain = Captain::new();
    let action = captain
        .decide(
            &empty_purpose(),
            &Context::new(),
            &empty_env(),
            &empty_resources(),
            &Inbox::new(),
        )
        .await;
    assert_eq!(action, Action::Halt);
}

#[tokio::test]
async fn retryable_hitch_triggers_three_halts_then_done() {
    let captain = Captain::new();
    let purpose = empty_purpose();
    let env = empty_env();
    let res = empty_resources();
    let inbox = Inbox::new();

    // First decide arms `started`.
    let _ = captain
        .decide(&purpose, &Context::new(), &env, &res, &inbox)
        .await;

    let mut ctx = Context::new();
    ctx.append(Fragment::hitch_with("net blip", true, None));

    let mut halts = 0;
    let mut last = Action::Done;
    for _ in 0..5 {
        last = captain.decide(&purpose, &ctx, &env, &res, &inbox).await;
        if last == Action::Halt {
            halts += 1;
        } else {
            break;
        }
    }
    assert_eq!(halts, 3, "expected MAX_HITCH_RETRIES=3 retries");
    assert_eq!(last, Action::Done);
}

#[tokio::test]
async fn non_retryable_hitch_is_immediate_done() {
    let captain = Captain::new();
    let purpose = empty_purpose();
    let env = empty_env();
    let res = empty_resources();
    let inbox = Inbox::new();

    let _ = captain
        .decide(&purpose, &Context::new(), &env, &res, &inbox)
        .await;

    let mut ctx = Context::new();
    ctx.append(Fragment::hitch_with("bad args", false, None));

    let action = captain.decide(&purpose, &ctx, &env, &res, &inbox).await;
    assert_eq!(action, Action::Done);
}

#[tokio::test]
async fn successful_tool_turn_resets_retry_counter() {
    let captain = Captain::new();
    let purpose = empty_purpose();
    let env = empty_env();
    let res = empty_resources();
    let inbox = Inbox::new();

    let _ = captain
        .decide(&purpose, &Context::new(), &env, &res, &inbox)
        .await;

    // Burn 2 retries.
    let mut ctx = Context::new();
    ctx.append(Fragment::hitch_with("blip", true, None));
    let _ = captain.decide(&purpose, &ctx, &env, &res, &inbox).await;
    let _ = captain.decide(&purpose, &ctx, &env, &res, &inbox).await;

    // Successful tool result → reset.
    let mut ctx = Context::new();
    ctx.append(Fragment::tool_result("call_1", "ok", None));
    let action_after_tool = captain.decide(&purpose, &ctx, &env, &res, &inbox).await;
    assert_eq!(action_after_tool, Action::Halt);

    // Three more retries should be granted from the reset state.
    let mut ctx = Context::new();
    ctx.append(Fragment::hitch_with("blip", true, None));
    let mut halts = 0;
    for _ in 0..5 {
        let action = captain.decide(&purpose, &ctx, &env, &res, &inbox).await;
        if action == Action::Halt {
            halts += 1;
        } else {
            break;
        }
    }
    assert_eq!(halts, 3);
}

#[tokio::test]
async fn clone_resets_retry_state() {
    let captain = Captain::new();
    let purpose = empty_purpose();
    let env = empty_env();
    let res = empty_resources();
    let inbox = Inbox::new();

    let _ = captain
        .decide(&purpose, &Context::new(), &env, &res, &inbox)
        .await;
    let mut ctx = Context::new();
    ctx.append(Fragment::hitch_with("blip", true, None));
    let _ = captain.decide(&purpose, &ctx, &env, &res, &inbox).await;

    let cloned = Policy::clone_box(&captain);

    // Cloned Captain treats first decide as fresh: started reset → Halt.
    let action = cloned
        .decide(&purpose, &Context::new(), &env, &res, &inbox)
        .await;
    assert_eq!(action, Action::Halt);

    // Cloned counter is also reset: 3 retries available.
    let mut ctx = Context::new();
    ctx.append(Fragment::hitch_with("blip", true, None));
    let mut halts = 0;
    for _ in 0..5 {
        let action = cloned.decide(&purpose, &ctx, &env, &res, &inbox).await;
        if action == Action::Halt {
            halts += 1;
        } else {
            break;
        }
    }
    assert_eq!(halts, 3);
}
