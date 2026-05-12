use crate::fragment::Fragment;
use crate::register::Register;

/// Channel — the communication pipe between two machines.
///
/// Used in static composition (Pipeline, Parallel, Ensemble).
/// `transform` converts upstream output fragments into downstream input fragments.
/// `isolate` builds the downstream Register from the upstream one.
pub struct Channel {
    pub transform: Box<dyn Fn(Vec<Fragment>) -> Vec<Fragment> + Send + Sync>,
    pub isolate: Box<dyn Fn(&Register) -> Register + Send + Sync>,
}

impl Channel {
    /// Pass-through: no transformation, no isolation.
    pub fn passthrough() -> Self {
        Self {
            transform: Box::new(|frags| frags),
            isolate: Box::new(|reg| reg.child()),
        }
    }

    /// Take only the last fragment as the downstream's user intent.
    pub fn last_as_intent() -> Self {
        Self {
            transform: Box::new(|frags| {
                frags
                    .last()
                    .map(|f| vec![Fragment::user_text(f.as_text().unwrap_or(""))])
                    .unwrap_or_default()
            }),
            isolate: Box::new(|reg| reg.child()),
        }
    }

    /// Isolate tools: only pass the named tools to the downstream.
    pub fn with_tools(names: &'static [&'static str]) -> Self {
        let owned: Vec<String> = names.iter().map(|s| s.to_string()).collect();
        Self {
            transform: Box::new(|frags| frags),
            isolate: Box::new(move |reg| {
                let mut child = reg.child();
                child.tools.retain(|t| owned.iter().any(|n| n == &t.name));
                child
            }),
        }
    }
}
