use machine::Environment;

/// Default working environment — current directory.
pub fn default_env() -> Environment {
    Environment::new(".")
}
