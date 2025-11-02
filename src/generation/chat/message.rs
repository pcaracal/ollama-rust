use crate::generation::tools::ToolCall;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
}

/// Message is a single message in a chat sequence. The message contains the
/// role ("system", "user", or "assistant"), the content and an optional list
/// of images.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    /// Role of the sender
    pub role: Role,

    /// Main text content of the message.
    pub content: String,

    /// Text that was inside thinking tags in the
    /// original model output when thinking is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<String>,

    /// List of images associated with the message.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub images: Vec<String>,

    /// List of tool calls associated with the message.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,

    /// Name of the tool used in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
}

impl Message {
    /// User prompt
    pub fn user<S: Into<String>>(content: S) -> Self {
        Self::new(content, Role::User)
    }

    /// System prompt
    pub fn system<S: Into<String>>(content: S) -> Self {
        Self::new(content, Role::System)
    }

    fn new<S: Into<String>>(content: S, role: Role) -> Self {
        Self {
            role,
            content: content.into(),
            thinking: None,
            images: vec![],
            tool_calls: vec![],
            tool_name: None,
        }
    }
}
