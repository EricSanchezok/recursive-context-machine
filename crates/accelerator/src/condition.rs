use machine::{Content, Environment, Fragment, Resources, Role};
use serde::{Deserialize, Serialize};
use utils::{ConditionId, Name};

use crate::state::State;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConditionBranch {
    True,
    False,
}

#[derive(Clone)]
pub struct Condition {
    id: ConditionId,
    pub name: Name,
    pub predicate: Predicate,
}

impl Condition {
    pub fn new(name: impl Into<String>, predicate: Predicate) -> Self {
        Self {
            id: ConditionId::new(),
            name: Name::new(name).expect("condition name must be valid"),
            predicate,
        }
    }

    pub fn id(&self) -> &ConditionId {
        &self.id
    }

    pub fn route(&self, state: &State) -> ConditionBranch {
        if self.predicate.evaluate(state) {
            ConditionBranch::True
        } else {
            ConditionBranch::False
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Predicate {
    Purpose(PurposePredicate),
    Context(ContextPredicate),
    Environment(EnvironmentPredicate),
    Resources(ResourcesPredicate),
    All(Vec<Predicate>),
    Any(Vec<Predicate>),
    Not(Box<Predicate>),
}

impl Predicate {
    pub fn evaluate(&self, state: &State) -> bool {
        match self {
            Self::Purpose(predicate) => predicate.evaluate(&state.purpose),
            Self::Context(predicate) => predicate.evaluate(&state.ctx),
            Self::Environment(predicate) => predicate.evaluate(&state.env),
            Self::Resources(predicate) => predicate.evaluate(&state.res),
            Self::All(predicates) => predicates.iter().all(|predicate| predicate.evaluate(state)),
            Self::Any(predicates) => predicates.iter().any(|predicate| predicate.evaluate(state)),
            Self::Not(predicate) => !predicate.evaluate(state),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PurposePredicate {
    Contains(String),
    Equals(String),
    StartsWith(String),
    EndsWith(String),
    IsEmpty,
}

impl PurposePredicate {
    fn evaluate(&self, purpose: &str) -> bool {
        match self {
            Self::Contains(value) => purpose.contains(value),
            Self::Equals(value) => purpose == value,
            Self::StartsWith(value) => purpose.starts_with(value),
            Self::EndsWith(value) => purpose.ends_with(value),
            Self::IsEmpty => purpose.is_empty(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ContextPredicate {
    HasTag(String),
    HasRole(Role),
    Contains(String),
    IsEmpty,
}

impl ContextPredicate {
    fn evaluate(&self, context: &machine::Context) -> bool {
        match self {
            Self::HasTag(tag) => context
                .fragments()
                .iter()
                .any(|fragment| fragment.tag == *tag),
            Self::HasRole(role) => context
                .fragments()
                .iter()
                .any(|fragment| fragment.role == *role),
            Self::Contains(value) => context
                .fragments()
                .iter()
                .filter_map(fragment_text)
                .any(|text| text.contains(value)),
            Self::IsEmpty => context.is_empty(),
        }
    }
}

fn fragment_text(fragment: &Fragment) -> Option<&str> {
    match &fragment.content {
        Content::Text(text) => Some(text.text.as_str()),
        Content::ToolResult(result) => Some(result.content.as_str()),
        Content::Hitch { message, .. } => Some(message.as_str()),
        _ => None,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnvironmentPredicate {
    VarExists(String),
    VarEquals(String, String),
    CwdContains(String),
    PlatformIs(String),
}

impl EnvironmentPredicate {
    fn evaluate(&self, env: &Environment) -> bool {
        match self {
            Self::VarExists(key) => env.vars.contains_key(key),
            Self::VarEquals(key, value) => env.vars.get(key) == Some(value),
            Self::CwdContains(value) => env.cwd.to_string_lossy().contains(value),
            Self::PlatformIs(value) => env.platform == *value,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ResourcesPredicate {
    HasModel(String),
    ActiveModelIs(String),
    HasTool(String),
    ToolEnabled(String),
    HasPrompt(String),
}

impl ResourcesPredicate {
    fn evaluate(&self, resources: &Resources) -> bool {
        match self {
            Self::HasModel(name) => resources.models.contains_key(name),
            Self::ActiveModelIs(name) => resources.active_model == *name,
            Self::HasTool(name) => resources.tool_definitions.contains_key(name),
            Self::ToolEnabled(name) => resources.active_tools.contains(name),
            Self::HasPrompt(name) => resources.prompts.contains_key(name),
        }
    }
}
