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

pub fn register(catalog: &mut Catalog) {
    catalog
        .register_policy("captain", || Box::new(Captain::new()))
        .expect("built-in policy names must be unique");
}
