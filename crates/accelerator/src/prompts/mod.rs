use crate::catalog::Catalog;

pub fn register(catalog: &mut Catalog) {
    catalog
        .register_prompt("captain", include_str!("captain.txt"))
        .expect("built-in prompt names must be unique");
}
