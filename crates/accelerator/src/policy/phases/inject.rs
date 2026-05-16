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
