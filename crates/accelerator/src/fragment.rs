use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Fragment role — maps to rig's Message role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Role {
    System,
    User,
    Assistant,
}

/// A tool call declaration produced by the assistant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallDef {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

/// Fragment content — mirrors rig's UserContent + AssistantContent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Content {
    Text(String),
    Image {
        data: String,
        mime: String,
    },
    Audio {
        data: String,
        mime: String,
    },
    Video {
        data: String,
        mime: String,
    },
    Document {
        data: String,
        mime: String,
        filename: String,
    },
    ToolResult {
        call_id: String,
        text: String,
    },
    ToolCalls(Vec<ToolCallDef>),
    Reasoning(String),
}

/// A single symbol on the Turing machine tape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fragment {
    pub role: Role,
    pub content: Content,
    pub meta: HashMap<String, Value>,
}

impl Fragment {
    pub fn system(text: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: Content::Text(text.into()),
            meta: HashMap::new(),
        }
    }

    pub fn user_text(text: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: Content::Text(text.into()),
            meta: HashMap::new(),
        }
    }

    pub fn assistant_text(text: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: Content::Text(text.into()),
            meta: HashMap::new(),
        }
    }

    pub fn assistant_tool_calls(calls: Vec<ToolCallDef>) -> Self {
        Self {
            role: Role::Assistant,
            content: Content::ToolCalls(calls),
            meta: HashMap::new(),
        }
    }

    pub fn tool_result(call_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: Content::ToolResult {
                call_id: call_id.into(),
                text: text.into(),
            },
            meta: HashMap::new(),
        }
    }

    pub fn with_meta(mut self, key: impl Into<String>, value: Value) -> Self {
        self.meta.insert(key.into(), value);
        self
    }

    pub fn as_text(&self) -> Option<&str> {
        match &self.content {
            Content::Text(t) => Some(t),
            _ => None,
        }
    }
}
