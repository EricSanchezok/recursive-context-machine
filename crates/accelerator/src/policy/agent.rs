use machine::{Action, Context, Fragment, Resources, Role};

pub fn ensure_agent_prompt(
    ctx: &Context,
    resources: &Resources,
    prompt_key: &str,
) -> Option<Action> {
    let desired = resources
        .prompts
        .get(prompt_key)
        .cloned()
        .unwrap_or_default();

    if let Some(existing) = ctx
        .fragments()
        .iter()
        .find(|f| f.role == Role::System && f.tag == "agent")
    {
        if existing.as_text() == Some(&desired) {
            return None;
        }
        return Some(Action::Replace {
            id: existing.id(),
            fragment: Fragment::system(desired).with_tag("agent"),
        });
    }

    Some(Action::Append(Fragment::system(desired).with_tag("agent")))
}
