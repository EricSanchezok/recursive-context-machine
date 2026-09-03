use chrono::Local;
use machine::edit::{ContentSpec, EditOp};
use machine::{Action, Context, Environment, Role};

use super::super::Step;

const ENV_TAG: &str = "env";
const ENV_ANCHOR: &str = "@env";

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

    // Idempotent named-slot refresh: the old find-then-Replace-or-Append
    // dance is one Set.
    if let Some(existing_id) = ctx.find_anchor(ENV_ANCHOR)
        && ctx
            .get(existing_id)
            .is_some_and(|cell| cell.as_text() == Some(&text))
    {
        return Step::Ready;
    }

    Step::Emit(Action::Edit {
        ops: vec![EditOp::Set {
            anchor: ENV_ANCHOR.into(),
            content: ContentSpec::Literal {
                text,
                role: Role::System,
                tag: Some(ENV_TAG.into()),
            },
        }],
        because: None,
    })
}
