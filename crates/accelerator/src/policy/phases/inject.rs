use chrono::Local;
use machine::{
    Action, Context, Environment, Fragment, Phase, PhaseOutcome, Purpose, Resources, Role,
};

/// Append the user's purpose as a [`Role::User`] fragment with tag `"purpose"`.
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

/// Inject an environment snapshot as a system fragment with tag `"env"`.
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
            env.platform,
            now,
        );

        PhaseOutcome::Action(Action::Append(Fragment::system(text).with_tag("env")))
    }
}
