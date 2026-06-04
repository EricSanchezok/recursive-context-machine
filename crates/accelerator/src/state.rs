use std::collections::HashMap;

use machine::{Context, Environment, Resources, Usage};

#[derive(Clone)]
pub struct State {
    pub purpose: String,
    pub fold_payload: String,
    pub ctx: Context,
    pub env: Environment,
    pub res: Resources,
    pub usages: Vec<Usage>,
    pub counts: HashMap<String, u64>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            purpose: String::new(),
            fold_payload: String::new(),
            ctx: Context::new(),
            env: Environment::new("."),
            res: Resources::new(),
            usages: Vec::new(),
            counts: HashMap::new(),
        }
    }
}
