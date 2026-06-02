use machine::{Action, Context, Fragment, Purpose, Role};

pub fn ensure_purpose(ctx: &Context, purpose: &Purpose) -> Option<Action> {
    if purpose.is_empty() {
        return None;
    }
    if ctx
        .fragments()
        .iter()
        .any(|f| f.role == Role::System && f.tag == "purpose")
    {
        return None;
    }
    let text = format!("## Purpose\n{}", purpose.text);
    Some(Action::Append(Fragment::system(text).with_tag("purpose")))
}
