use chrono::Local;
use machine::{Action, Context, Environment, Fragment, Role};

use super::super::Step;

const ENV_TAG: &str = "env";

pub(crate) fn refresh(ctx: &Context, env: &Environment) -> Step {
    let now = Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
    let mut text = format!(
        "cwd: {}\nplatform: {}\ntime: {}",
        env.cwd.display(),
        env.platform,
        now,
    );
    if let Some(ref run_dir) = env.run_dir {
        text.push_str(&format!("\nrun_dir: {}", run_dir.display()));
    }
    let fragment = Fragment::system(text.clone()).with_tag(ENV_TAG);

    if let Some(existing) = ctx
        .fragments()
        .iter()
        .find(|fragment| fragment.role == Role::System && fragment.tag == ENV_TAG)
    {
        if existing.as_text() == Some(&text) {
            return Step::Ready;
        }
        return Step::Emit(Action::Replace {
            id: existing.id(),
            fragment,
        });
    }

    Step::Emit(Action::Append(fragment))
}
