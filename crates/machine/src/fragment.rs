use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Fragment role (system, user, assistant, or tool).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// System instruction.
    System,
    /// Human input.
    User,
    /// LLM output (text or tool call).
    Assistant,
    /// Tool execution result.
    Tool,
}

/// Source of multimedia data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataSource {
    Url(String),
    Base64(String),
    Raw(Vec<u8>),
    String(String),
}

/// Plain text content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Text {
    pub text: String,
}

/// Image content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub source: DataSource,
    pub media_type: Option<String>,
}

/// Audio content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Audio {
    pub source: DataSource,
    pub media_type: Option<String>,
}

/// Video content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Video {
    pub source: DataSource,
    pub media_type: Option<String>,
}

/// Document content — PDF, TXT, code files, JSON, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub source: DataSource,
    pub media_type: Option<String>,
}

/// Tool call — produced by the assistant, requesting tool execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

/// The outcome of a tool execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolResult {
    /// Unique ID matching the tool call that produced this result.
    pub call_id: String,

    /// Textual content returned to the model.
    pub content: String,

    /// Optional short title for display/logging.
    pub title: Option<String>,
}

/// Content of a fragment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Content {
    Text(Text),
    Image(Image),
    Audio(Audio),
    Video(Video),
    Document(Document),
    ToolCall(ToolCall),
    ToolResult(ToolResult),
    /// Execution error that Policy may intercept for retry decisions.
    Hitch {
        message: String,
        retryable: bool,
        code: Option<u16>,
    },
}

/// A single symbol on the context tape.
///
/// The `id` field is assigned by [`Context`](crate::Context) when the fragment is stored.
/// A value of `0` means "not yet assigned".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fragment {
    pub id: u64,
    pub role: Role,
    pub tag: String,
    pub content: Content,
}

impl Fragment {
    /// System fragment.
    pub fn system(text: impl Into<String>) -> Self {
        Self {
            id: 0,
            role: Role::System,
            tag: "system".into(),
            content: Content::Text(Text { text: text.into() }),
        }
    }

    /// User fragment — defaults to text.
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            id: 0,
            role: Role::User,
            tag: "user".into(),
            content: Content::Text(Text { text: text.into() }),
        }
    }

    /// Assistant fragment — defaults to text.
    pub fn assistant(text: impl Into<String>) -> Self {
        Self {
            id: 0,
            role: Role::Assistant,
            tag: "assistant".into(),
            content: Content::Text(Text { text: text.into() }),
        }
    }

    /// Tool result fragment.
    pub fn tool_result(call_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: 0,
            role: Role::Tool,
            tag: "tool_result".into(),
            content: Content::ToolResult(ToolResult {
                call_id: call_id.into(),
                content: text.into(),
                title: None,
            }),
        }
    }

    /// Creates a [`Content::Hitch`] fragment.
    pub fn hitch(message: impl Into<String>) -> Self {
        Self {
            id: 0,
            role: Role::System,
            tag: "hitch".into(),
            content: Content::Hitch {
                message: message.into(),
                retryable: false,
                code: None,
            },
        }
    }

    /// Tool call fragment — produced by the assistant.
    pub fn tool_call(id: impl Into<String>, name: impl Into<String>, arguments: Value) -> Self {
        Self {
            id: 0,
            role: Role::Assistant,
            tag: "tool_call".into(),
            content: Content::ToolCall(ToolCall {
                id: id.into(),
                name: name.into(),
                arguments,
            }),
        }
    }

    /// Assign a custom tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = tag.into();
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
