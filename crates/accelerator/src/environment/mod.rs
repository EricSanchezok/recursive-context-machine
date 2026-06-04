use std::path::PathBuf;

use machine::Environment;

use crate::catalog::Catalog;

pub fn register(catalog: &mut Catalog) {
    catalog
        .register_environment("local", local)
        .expect("built-in environment names must be unique");
}

fn local() -> Environment {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut environment = Environment::named("local", cwd);
    environment.root = Some(environment.cwd.clone());
    environment
}
