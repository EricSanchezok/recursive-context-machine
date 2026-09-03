mod captain;
pub mod moves;
pub mod retry;
mod scripted;

pub use captain::Captain;
pub use scripted::{SEED_BASIC_SOURCE, ScriptedPolicy};

use machine::Action;

use crate::catalog::Catalog;

pub(crate) enum Step {
    Emit(Action),
    Ready,
}

pub fn register(catalog: &mut Catalog) {
    catalog
        .register_policy("captain", || Box::new(Captain::new()))
        .expect("built-in policy names must be unique");
    // seed-basic: captain 确定性脚手架的结构等价移植 (E1 表达力基准)。
    // 编译只发生一次 (OnceLock); 失败即 panic — load-time fail fast
    // (CLI 以 "RCM runtime panicked" 呈现)。
    static SEED: std::sync::OnceLock<ScriptedPolicy> = std::sync::OnceLock::new();
    catalog
        .register_policy("seed-basic", || {
            Box::new(
                SEED.get_or_init(|| {
                    ScriptedPolicy::compile("seed-basic", SEED_BASIC_SOURCE, "captain")
                        .expect("seed-basic policy must compile")
                })
                .clone(),
            )
        })
        .expect("built-in policy names must be unique");
}
