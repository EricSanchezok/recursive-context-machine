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
    Hitch {
        message: String,
        /// ID of the originating tool call, if this hitch is a tool error.
        call_id: Option<String>,
        /// HTTP status code for retry strategy (Policy decides permanent vs transient).
        code: Option<u16>,
    },
}

/// A symbol on the context tape.
///
/// `id` is assigned by [`Context`](crate::Context) on storage.
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

    /// Creates a [`Content::Hitch`] fragment.
    ///
    /// `code` is an optional HTTP status code from the failed request.
    /// `call_id` is the originating tool call ID when this hitch represents a
    /// tool error; required for `completion::encode` to emit a valid
    /// `role: "tool"` message.
    /// `code` and `role` convey the failure context for Policy. Role should
    /// match the origin: tool errors → [`Role::Tool`], LLM errors →
    /// [`Role::Assistant`], system errors → [`Role::System`].
    pub fn hitch(
        message: impl Into<String>,
        code: Option<u16>,
        role: Role,
        call_id: Option<impl Into<String>>,
    ) -> Self {
        Self {
            id: 0,
            role,
            tag: "hitch".into(),
            content: Content::Hitch {
                message: message.into(),
                call_id: call_id.map(Into::into),
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

    // ── Multi-modal constructors (P2) ──

    pub fn image(source: DataSource, media_type: Option<String>) -> Self {
        Self {
            id: 0,
            role: Role::User,
            tag: "image".into(),
            content: Content::Image(Image { source, media_type }),
        }
    }

    pub fn audio(source: DataSource, media_type: Option<String>) -> Self {
        Self {
            id: 0,
            role: Role::User,
            tag: "audio".into(),
            content: Content::Audio(Audio { source, media_type }),
        }
    }

    pub fn video(source: DataSource, media_type: Option<String>) -> Self {
        Self {
            id: 0,
            role: Role::User,
            tag: "video".into(),
            content: Content::Video(Video { source, media_type }),
        }
    }

    pub fn document(source: DataSource, media_type: Option<String>) -> Self {
        Self {
            id: 0,
            role: Role::User,
            tag: "document".into(),
            content: Content::Document(Document { source, media_type }),
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

    /// Full text content suitable for external consumption (embedding, logging).
    ///
    /// Returns the complete text content for all Fragment types:
    /// - Text/ToolResult/Hitch: the actual text content
    /// - ToolCall: `tool_call: name(arguments)`
    /// - Multi-modal (Image, Audio, Video, Document): placeholder tags like
    ///   `"<image>"`, `"<audio>"`. These are **not** suitable for direct
    ///   embedding — downstream consumers (RL observations, prompt builders)
    ///   should replace placeholders with actual content or metadata from
    ///   the Content variant as needed.
    pub fn content_as_text(&self) -> String {
        match &self.content {
            Content::Text(t) => t.text.clone(),
            Content::ToolCall(tc) => format!("tool_call: {}({})", tc.name, tc.arguments),
            Content::ToolResult(tr) => tr.content.clone(),
            Content::Hitch { message, .. } => message.clone(),
            Content::Image(_) => "<image>".into(),
            Content::Audio(_) => "<audio>".into(),
            Content::Video(_) => "<video>".into(),
            Content::Document(_) => "<document>".into(),
        }
    }
}
