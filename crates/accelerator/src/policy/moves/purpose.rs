use machine::{Action, Context, Fragment, Purpose, Role};

use super::super::Step;

const PURPOSE_TAG: &str = "purpose";

pub(crate) fn append(ctx: &Context, purpose: &Purpose) -> Step {
    if purpose.is_empty() {
        return Step::Ready;
    }

    if ctx.fragments().last().is_some_and(|fragment| {
        fragment.role == Role::User
            && fragment.tag == PURPOSE_TAG
            && fragment.as_text() == Some(&purpose.text)
    }) {
        return Step::Ready;
    }

    Step::Emit(Action::Append(
        Fragment::user(purpose.text.clone()).with_tag(PURPOSE_TAG),
    ))
}
