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

/// Inject an environment snapshot as a [`Role::System`] fragment with tag `"env"`.
///
/// Delegates formatting to [`Environment::snapshot`], which produces:
/// ```text
/// cwd: /path/to/dir
/// platform: macos
/// time: 2026-05-17T13:17:12+08:00
/// ```
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

        PhaseOutcome::Action(Action::Append(
            Fragment::system(env.snapshot()).with_tag("env"),
        ))
    }
}
