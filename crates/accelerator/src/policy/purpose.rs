use machine::{Action, Context, Fragment, Purpose, Role};

use super::Step;

const PURPOSE_TAG: &str = "purpose";

pub(crate) fn append(ctx: &Context, purpose: &Purpose) -> Step {
    if purpose.is_empty() {
        return Step::Ready;
    }

    let text = format!("## Purpose\n{}", purpose.text);

    if ctx.fragments().last().is_some_and(|fragment| {
        fragment.role == Role::User
            && fragment.tag == PURPOSE_TAG
            && fragment.as_text() == Some(&text)
    }) {
        return Step::Ready;
    }

    Step::Emit(Action::Append(Fragment::user(text).with_tag(PURPOSE_TAG)))
}
