use machine::{Action, Context, Environment, Fragment, Phase, PhaseOutcome, Resources, Role};

/// Inject the user's purpose into the context.
///
/// Appends a `User` fragment tagged `"purpose"` when the purpose is non-empty.
/// Runs once — the `"purpose"` tag acts as a guard.
pub struct Purpose;

impl Phase for Purpose {
    fn clone_box(&self) -> Box<dyn Phase> {
        Box::new(Self)
    }

    fn name(&self) -> &str {
        "purpose"
    }

    fn decide(
        &self,
        purpose: &machine::Purpose,
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
