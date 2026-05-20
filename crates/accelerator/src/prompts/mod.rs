use crate::catalog::Catalog;

/// Register all built-in prompts in the catalog.
pub fn register(catalog: &mut Catalog) {
    catalog
        .prompts
        .insert("captain".into(), include_str!("captain.txt").into());
}
