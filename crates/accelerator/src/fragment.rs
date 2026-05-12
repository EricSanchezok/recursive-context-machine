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

/// Source of multimedia data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    Url(String),
    Base64(String),
    Raw(Vec<u8>),
    String(String),
}

/// Plain text content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub text: String,
}

/// Image content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub source: DataSource,
    pub media_type: Option<String>,
}

/// Audio content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    pub source: DataSource,
    pub media_type: Option<String>,
}

/// Video content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub source: DataSource,
    pub media_type: Option<String>,
}

/// Document content — PDF, TXT, code files, JSON, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub source: DataSource,
    pub media_type: Option<String>,
}

/// Tool definition — the schema LLM sees to decide tool usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

/// Tool call — produced by the assistant, requesting tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

/// Tool result — returned after executing a tool call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub call_id: String,
    pub content: String,
}

/// Content of a fragment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Content {
    Text(Text),
    Image(Image),
    Audio(Audio),
    Video(Video),
    Document(Document),
    ToolDef(ToolDef),
    ToolCall(ToolCall),
    ToolResult(ToolResult),
}

/// A single symbol on the Turing machine tape.
///
/// The `id` field is assigned by [`Context`] when the fragment is stored.
/// A value of `0` means "not yet assigned".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fragment {
    pub id: u64,
    pub role: Role,
    pub tag: String,
    pub content: Content,
    pub meta: HashMap<String, Value>,
}

impl Fragment {
    /// System fragment.
    pub fn system(text: impl Into<String>) -> Self {
        Self {
            id: 0,
            role: Role::System,
            tag: "system".into(),
            content: Content::Text(Text { text: text.into() }),
            meta: HashMap::new(),
        }
    }

    /// User fragment — defaults to text.
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            id: 0,
            role: Role::User,
            tag: "user".into(),
            content: Content::Text(Text { text: text.into() }),
            meta: HashMap::new(),
        }
    }

    /// Assistant fragment — defaults to text.
    pub fn assistant(text: impl Into<String>) -> Self {
        Self {
            id: 0,
            role: Role::Assistant,
            tag: "assistant".into(),
            content: Content::Text(Text { text: text.into() }),
            meta: HashMap::new(),
        }
    }

    /// Tool result fragment.
    pub fn tool_result(call_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: 0,
            role: Role::User,
            tag: "tool_result".into(),
            content: Content::ToolResult(ToolResult {
                call_id: call_id.into(),
                content: text.into(),
            }),
            meta: HashMap::new(),
        }
    }

    /// Build with a custom tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = tag.into();
        self
    }

    /// Attach metadata.
    pub fn with_meta(mut self, key: impl Into<String>, value: Value) -> Self {
        self.meta.insert(key.into(), value);
        self
    }

    /// Extract text if this is a Text fragment.
    pub fn as_text(&self) -> Option<&str> {
        match &self.content {
            Content::Text(t) => Some(&t.text),
            _ => None,
        }
    }
}
