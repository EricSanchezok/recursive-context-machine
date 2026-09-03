use machine::edit::{ContentSpec, EditOp};
use machine::{Action, Context, Resources, Role};

use super::super::Step;

const AGENT_TAG: &str = "agent";
const AGENT_ANCHOR: &str = "@agent";

pub(crate) fn prepare(ctx: &Context, resources: &Resources, prompt_key: &str) -> Step {
    let desired = resources
        .prompts
        .get(prompt_key)
        .cloned()
        .unwrap_or_default();

    // The agent cell is a named slot: one idempotent Set replaces the whole
    // replace-or-advance-or-swap dance the old verbs required.
    let unchanged = ctx
        .find_anchor(AGENT_ANCHOR)
        .and_then(|id| ctx.get(id))
        .is_some_and(|cell| cell.as_text() == Some(&desired));

    if unchanged {
        return Step::Ready;
    }

    Step::Emit(Action::Edit {
        ops: vec![EditOp::Set {
            anchor: AGENT_ANCHOR.into(),
            content: ContentSpec::Literal {
                text: desired,
                role: Role::System,
                tag: Some(AGENT_TAG.into()),
            },
        }],
        because: None,
    })
}
