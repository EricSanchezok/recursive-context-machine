use machine::edit::{ContentSpec, EditOp, Position};
use machine::{Action, Context, Purpose, Role};

use super::super::Step;

const PURPOSE_TAG: &str = "purpose";

pub(crate) fn append(ctx: &Context, purpose: &Purpose) -> Step {
    if purpose.is_empty() {
        return Step::Ready;
    }

    // The purpose rides at the tail (per-run steering arrives late); the old
    // dedup-by-last check collapses to a tail scan.
    if ctx
        .fragments()
        .last()
        .is_some_and(|fragment| fragment.as_text() == Some(&purpose.text))
    {
        return Step::Ready;
    }

    Step::Emit(Action::Edit {
        ops: vec![EditOp::Insert {
            position: Position::End,
            content: ContentSpec::Literal {
                text: purpose.text.clone(),
                role: Role::User,
                tag: Some(PURPOSE_TAG.into()),
            },
            anchor: None,
        }],
        because: None,
    })
}
