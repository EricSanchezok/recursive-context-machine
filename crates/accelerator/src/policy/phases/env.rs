use chrono::Local;
use machine::{
    Action, Context, Environment, Fragment, Phase, PhaseOutcome, Purpose, Resources, Role,
};

/// Inject or replace a `System` fragment tagged `"env"` with current working
/// directory, platform, and timestamp. Runs before every LLM call.
pub struct Env;

impl Phase for Env {
    fn clone_box(&self) -> Box<dyn Phase> {
        Box::new(Self)
    }

    fn name(&self) -> &str {
        "env"
    }

    fn decide(
        &self,
        _purpose: &Purpose,
        ctx: &Context,
        env: &Environment,
        _resources: &Resources,
    ) -> PhaseOutcome {
        let now = Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
        let text = format!(
            "cwd: {}\nplatform: {}\ntime: {}",
            env.cwd.display(),
            env.platform,
            now,
        );

        if let Some(existing) = ctx
            .fragments()
            .iter()
            .find(|f| f.role == Role::System && f.tag == "env")
        {
            if existing.as_text() == Some(&text) {
                return PhaseOutcome::Done;
            }
            return PhaseOutcome::Action(Action::Replace {
                id: existing.id(),
                fragment: Fragment::system(text).with_tag("env"),
            });
        }

        PhaseOutcome::Action(Action::Append(Fragment::system(text).with_tag("env")))
    }
}
