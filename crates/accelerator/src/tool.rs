use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::register::Register;

/// A tool — a machine part that can be invoked during execution.
///
/// Tools are held in the Register, not on the tape. They can be
/// local functions, external services, or child Rica instances.
pub struct Tool {
    pub name: String,
    pub description: String,
    pub schema: Value,
    pub run: Arc<
        dyn Fn(Value, Register) -> Pin<Box<dyn Future<Output = Result<Value, String>> + Send>>
            + Send
            + Sync,
    >,
}

impl std::fmt::Debug for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tool")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("schema", &self.schema)
            .finish_non_exhaustive()
    }
}

impl Clone for Tool {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            description: self.description.clone(),
            schema: self.schema.clone(),
            run: self.run.clone(),
        }
    }
}

impl Tool {
    /// Create a tool from an async function.
    pub fn from_fn<F, Fut>(
        name: impl Into<String>,
        description: impl Into<String>,
        schema: Value,
        f: F,
    ) -> Self
    where
        F: Fn(Value, Register) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Value, String>> + Send + 'static,
    {
        Self {
            name: name.into(),
            description: description.into(),
            schema,
            run: Arc::new(move |args, reg| Box::pin(f(args, reg))),
        }
    }
}
