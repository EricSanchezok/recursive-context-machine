use machine::{Action, Content, Context, Inbox, Role};
use tracing::{trace, warn};

use crate::policy::retry::{HTTP_FORBIDDEN, HTTP_UNAUTHORIZED, Retry};

pub enum ReactDecision {
    Action(Action),
    Respond,
}

pub async fn decide(ctx: &Context, inbox: &Inbox, retry: &Retry) -> ReactDecision {
    if inbox.peek().is_some() {
        return ReactDecision::Action(Action::Take);
    }

    let last_fragment = ctx.fragments().last();

    if let Some(fragment) = last_fragment
        && let Content::Hitch { code, .. } = &fragment.content
    {
        if let Some(status_code) = code
            && (*status_code == HTTP_UNAUTHORIZED || *status_code == HTTP_FORBIDDEN)
        {
            warn!(code = *status_code, "decide: permanent hitch, done");
            return ReactDecision::Action(Action::Done);
        }

        if retry.backoff().await {
            let attempts = retry.count();
            trace!(attempts, "decide: hitched, retrying");
            return ReactDecision::Respond;
        }

        warn!("decide: retry budget exhausted, done");
        return ReactDecision::Action(Action::Done);
    }

    retry.reset();

    if last_fragment.is_some_and(|fragment| fragment.role == Role::Tool) {
        trace!("decide: last is Tool, halting");
        return ReactDecision::Respond;
    }

    trace!("decide: done");
    ReactDecision::Action(Action::Done)
}
