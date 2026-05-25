mod captain;
pub mod retry;

pub use captain::Captain;

use crate::catalog::Catalog;

/// Register all built-in policies in the catalog.
pub fn register(catalog: &mut Catalog) {
    catalog
        .policies
        .insert("captain".into(), || Box::new(Captain::new()));
}
