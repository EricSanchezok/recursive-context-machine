use std::path::PathBuf;

use machine::Environment;

use crate::catalog::Catalog;

/// Register built-in environment presets in the catalog.
pub fn register(catalog: &mut Catalog) {
    catalog.environments.insert("local".into(), local());
}

fn local() -> Environment {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut env = Environment::named("local", cwd);
    env.root = Some(env.cwd.clone());
    env
}
