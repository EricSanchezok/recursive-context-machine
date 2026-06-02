use machine::{Action, Context, Fragment, Resources, Role};

use super::Step;

const AGENT_TAG: &str = "agent";

pub(crate) fn prepare(ctx: &Context, resources: &Resources, prompt_key: &str) -> Step {
    let desired = resources
        .prompts
        .get(prompt_key)
        .cloned()
        .unwrap_or_default();
    let fragment = Fragment::system(desired.clone()).with_tag(AGENT_TAG);
    let fragments = ctx.fragments();

    let Some(first) = fragments.first() else {
        return Step::Emit(Action::Append(fragment));
    };

    if is_agent(first) {
        if let Some(extra) = fragments.iter().skip(1).find(|fragment| is_agent(fragment)) {
            return Step::Emit(Action::Remove(extra.id()));
        }
        if first.as_text() != Some(&desired) {
            return Step::Emit(Action::Replace {
                id: first.id(),
                fragment,
            });
        }
        return Step::Ready;
    }

    if let Some(existing) = fragments.iter().skip(1).find(|fragment| is_agent(fragment)) {
        return Step::Emit(Action::Swap(first.id(), existing.id()));
    }

    Step::Emit(Action::Insert {
        after: first.id(),
        fragment,
    })
}

fn is_agent(fragment: &Fragment) -> bool {
    fragment.role == Role::System && fragment.tag == AGENT_TAG
}
