mod captain;
pub mod moves;
pub mod retry;

pub use captain::Captain;

use machine::Action;

use crate::catalog::Catalog;

pub(crate) enum Step {
    Emit(Action),
    Ready,
}

/// Register all built-in policies in the catalog.
pub fn register(catalog: &mut Catalog) {
    catalog
        .policies
        .insert("captain".into(), || Box::new(Captain::new()));
}
