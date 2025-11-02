use crate::generation::tools::ToolCall;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
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

    /// True if the message is complete, false if still generating.
    #[serde(skip)]
    #[serde(default)]
    pub done: bool,
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

    /// Once a tool finishes
    pub fn tool<S: Into<String>>(content: S) -> Self {
        Self::new(content, Role::Tool)
    }

    fn new<S: Into<String>>(content: S, role: Role) -> Self {
        Self {
            role,
            content: content.into(),
            thinking: None,
            images: vec![],
            tool_calls: vec![],
            done: false,
        }
    }

    /// Merge another message into this one.
    /// Useful for keeping a history of complete messages when streaming.
    ///
    /// Following fields are extended from the `other` message:
    /// `content`, `thinking`, `images`, `tool_calls`, `done`
    ///
    /// # Note
    /// It does not make sense to merge messages if their roles differ or if the message is complete.
    /// This function does not check for that.
    pub fn merge_from(&mut self, other: &Message) {
        self.content.push_str(&other.content);

        if let Some(think) = &other.thinking {
            match &mut self.thinking {
                Some(existing) => existing.push_str(think),
                None => self.thinking = Some(think.clone()),
            }
        }

        self.images.extend_from_slice(&other.images);
        self.tool_calls.extend_from_slice(&other.tool_calls);
        self.done = other.done;
    }
}
