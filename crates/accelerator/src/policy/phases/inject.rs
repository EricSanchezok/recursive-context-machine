use chrono::Local;
use machine::{
    Action, Context, Environment, Fragment, Phase, PhaseOutcome, Purpose, Resources, Role,
};

/// Append the user's purpose as a [`Role::User`] fragment with tag `"purpose"`.
///
/// - Empty purpose → **Done**
/// - Already injected → **Done**
/// - Otherwise → **Append**, then Done on next call
pub struct InjectPurpose;

impl Phase for InjectPurpose {
    fn clone_box(&self) -> Box<dyn Phase> {
        Box::new(Self)
    }

    fn name(&self) -> &str {
        "inject_purpose"
    }

    fn decide(
        &self,
        purpose: &Purpose,
        ctx: &Context,
        _env: &Environment,
        _resources: &Resources,
    ) -> PhaseOutcome {
        if purpose.is_empty() {
            return PhaseOutcome::Done;
        }

        if ctx
            .fragments()
            .iter()
            .any(|f| f.role == Role::User && f.tag == "purpose")
        {
            return PhaseOutcome::Done;
        }

        PhaseOutcome::Action(Action::Append(
            Fragment::user(&purpose.text).with_tag("purpose"),
        ))
    }
}

/// Inject environment snapshot as a [`Role::System`] fragment with tag `"env"`.
///
/// Captures only what the LLM needs for correct tool use:
/// - `cwd` — base for all relative paths
/// - `platform` — affects path separators and command syntax
/// - `time` — RFC 3339 local time with explicit offset, zero ambiguity
///
/// Time format: `2026-05-17T08:31:30+08:00`
/// - ISO 8601 / RFC 3339 — LLM's most familiar format
/// - The `+08:00` offset makes it unambiguously local time (UTC would be `Z`)
/// - No locale-dependent strings like "Wed May 17"
pub struct InjectEnv;

impl Phase for InjectEnv {
    fn clone_box(&self) -> Box<dyn Phase> {
        Box::new(Self)
    }

    fn name(&self) -> &str {
        "inject_env"
    }

    fn decide(
        &self,
        _purpose: &Purpose,
        ctx: &Context,
        env: &Environment,
        _resources: &Resources,
    ) -> PhaseOutcome {
        if ctx
            .fragments()
            .iter()
            .any(|f| f.role == Role::System && f.tag == "env")
        {
            return PhaseOutcome::Done;
        }

        let now = Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);

        let text = format!(
            "cwd: {}\nplatform: {}\ntime: {}",
            env.cwd.display(),
            std::env::consts::OS,
            now,
        );

        PhaseOutcome::Action(Action::Append(Fragment::system(text).with_tag("env")))
    }
}
