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
    /// Reactor-side execution feedback — LLM failure, tool failure, or any
    /// other non-success signal produced by ω. Distinct from `System` so
    /// downstream code (encode, policy, UI) can treat it as feedback rather
    /// than a system instruction.
    Hitch,
}

/// Source of multimedia data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataSource {
    Url(String),
    Base64(String),
    /// Raw binary data.
    Raw(Vec<u8>),
    /// Plain string.
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

/// Tool call — requests tool execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

/// Outcome of a tool execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolResult {
    /// ID of the originating tool call.
    pub call_id: String,
    /// Output content.
    pub content: String,
    /// Summary label for logging UIs.
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

/// A symbol on the context tape.
///
/// `id` is assigned by [`Context`](crate::Context) on storage and should not be
/// set directly — mutating the id would break the context's internal invariants.
/// The `role`, `tag`, and `content` fields are public but effectively read-only
/// after construction; the only way to modify a fragment in a context is through
/// [`Action`](crate::Action) variants applied by [`Machine`](crate::Machine).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fragment {
    pub(crate) id: u64,
    pub role: Role,
    pub tag: String,
    pub content: Content,
}

impl Fragment {
    /// The context-assigned identifier. `0` means unassigned.
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn system(text: impl Into<String>) -> Self {
        Self {
            id: 0,
            role: Role::System,
            tag: "system".into(),
            content: Content::Text(Text { text: text.into() }),
        }
    }

    pub fn user(text: impl Into<String>) -> Self {
        Self {
            id: 0,
            role: Role::User,
            tag: "user".into(),
            content: Content::Text(Text { text: text.into() }),
        }
    }

    pub fn assistant(text: impl Into<String>) -> Self {
        Self {
            id: 0,
            role: Role::Assistant,
            tag: "assistant".into(),
            content: Content::Text(Text { text: text.into() }),
        }
    }

    pub fn tool_result(
        call_id: impl Into<String>,
        text: impl Into<String>,
        title: Option<String>,
    ) -> Self {
        Self {
            id: 0,
            role: Role::Tool,
            tag: "tool_result".into(),
            content: Content::ToolResult(ToolResult {
                call_id: call_id.into(),
                content: text.into(),
                title,
            }),
        }
    }

    /// Creates a [`Content::Hitch`] fragment. `retryable=false`, `code=None`.
    ///
    /// For richer construction with classification, use [`Fragment::hitch_with`].
    pub fn hitch(message: impl Into<String>) -> Self {
        Self::hitch_with(message, false, None)
    }

    /// Construct a hitch with explicit retryable / code classification.
    ///
    /// `retryable` should be true for transient failures (network blip, tool
    /// timeout, rate limit) and false for permanent ones (tool missing, bad
    /// arguments, auth error). The Policy uses this to decide whether to
    /// retry.
    pub fn hitch_with(
        message: impl Into<String>,
        retryable: bool,
        code: Option<u16>,
    ) -> Self {
        Self {
            id: 0,
            role: Role::Hitch,
            tag: "hitch".into(),
            content: Content::Hitch {
                message: message.into(),
                retryable,
                code,
            },
        }
    }

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
            Content::Text(text) => Some(&text.text),
            _ => None,
        }
    }
}
