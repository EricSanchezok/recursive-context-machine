use machine::{
    Action, Context, Environment, Fragment, Phase, PhaseOutcome, Purpose, Resources, Role,
};

/// Ensure the `tag == "agent"` system prompt is present and up to date.
///
/// - Exists with correct content → **Done**
/// - Exists with wrong content → **Replace**, then Done on next call
/// - Missing → **Append**, then Done on next call
pub struct BootstrapAgent {
    prompt_name: String,
}

impl BootstrapAgent {
    pub fn new(prompt_name: impl Into<String>) -> Self {
        Self {
            prompt_name: prompt_name.into(),
        }
    }
}

impl Phase for BootstrapAgent {
    fn clone_box(&self) -> Box<dyn Phase> {
        Box::new(Self {
            prompt_name: self.prompt_name.clone(),
        })
    }

    fn name(&self) -> &str {
        "bootstrap_agent"
    }

    fn decide(
        &self,
        _purpose: &Purpose,
        ctx: &Context,
        _env: &Environment,
        resources: &Resources,
    ) -> PhaseOutcome {
        let desired = resources
            .prompts
            .get(&self.prompt_name)
            .cloned()
            .unwrap_or_default();

        if let Some(existing) = ctx
            .fragments()
            .iter()
            .find(|f| f.role == Role::System && f.tag == "agent")
        {
            if existing.as_text() == Some(&desired) {
                return PhaseOutcome::Done;
            }
            return PhaseOutcome::Action(Action::Replace {
                id: existing.id(),
                fragment: Fragment::system(desired).with_tag("agent"),
            });
        }

        PhaseOutcome::Action(Action::Append(Fragment::system(desired).with_tag("agent")))
    }
}
