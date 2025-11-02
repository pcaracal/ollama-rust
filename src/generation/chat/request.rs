// Translation of https://github.com/ollama/ollama/blob/main/api/types.go into Rust

use std::sync::Arc;

use crate::{
    generation::{
        chat::message::Message,
        parameters,
        tools::{Tool, ToolInfo, ToolType},
    },
    model::ModelOptions,
};

#[derive(Debug, Clone, serde::Serialize)]
pub struct ChatRequest {
    pub model: String,

    pub messages: Vec<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<parameters::KeepAlive>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "tools")]
    pub tool_infos: Vec<ToolInfo>,

    #[serde(skip)]
    #[serde(default)]
    pub tools: Vec<Arc<dyn Tool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ModelOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<parameters::Think>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_render_only: Option<bool>,
}

#[allow(clippy::doc_markdown)]
impl ChatRequest {
    pub fn new<S: Into<String>>(model: S, messages: Vec<Message>) -> Self {
        Self {
            model: model.into(),
            messages,
            stream: None,
            format: None,
            keep_alive: None,
            tool_infos: vec![],
            tools: vec![],
            options: None,
            think: None,
            truncate: None,
            shift: None,
            debug_render_only: None,
        }
    }

    /// Stream specifies whether the response is streaming; it is true by default.
    #[must_use]
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
    /// The format to return the response in (e.g. "json").
    #[must_use]
    pub fn format(mut self, format: serde_json::Value) -> Self {
        self.format = Some(format);
        self
    }

    /// KeepAlive controls how long the model will stay loaded in memory following this request.
    #[must_use]
    pub fn keep_alive(mut self, keep_alive: parameters::KeepAlive) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    /// Adds a tool that the model has access to.
    #[must_use]
    pub fn tool(mut self, tool: Arc<dyn Tool>) -> Self {
        self.tool_infos.push(ToolInfo {
            tool_type: ToolType::Function,
            function: tool.tool_function(),
        });

        self.tools.push(tool);

        self
    }

    /// Options lists model-specific options. For example, temperature can be
    /// set through this field, if the model supports it.
    #[must_use]
    pub fn options(mut self, options: crate::model::ModelOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Think controls whether thinking/reasoning models will think before responding.
    /// Can be a boolean (true/false) or a string ("high", "medium", "low") for supported models.
    #[must_use]
    pub fn think(mut self, think: parameters::Think) -> Self {
        self.think = Some(think);
        self
    }

    /// Truncate is a boolean that, when set to true, truncates the chat history messages
    /// if the rendered prompt exceeds the context length limit.
    #[must_use]
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = Some(truncate);
        self
    }

    /// Shift is a boolean that, when set to true, shifts the chat history
    /// when hitting the context length limit instead of erroring.
    #[must_use]
    pub fn shift(mut self, shift: bool) -> Self {
        self.shift = Some(shift);
        self
    }

    /// DebugRenderOnly is a debug option that, when set to true, returns the rendered
    /// template instead of calling the model.
    #[must_use]
    pub fn debug_render_only(mut self, debug_render_only: bool) -> Self {
        self.debug_render_only = Some(debug_render_only);
        self
    }
}
