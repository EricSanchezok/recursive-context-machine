use machine::Resources;

use crate::catalog::Catalog;

/// Register built-in resource presets in the catalog.
pub fn register(catalog: &mut Catalog) {
    catalog.resources.insert("kit".into(), kit_base());
}

fn kit_base() -> Resources {
    Resources::named("kit")
}
