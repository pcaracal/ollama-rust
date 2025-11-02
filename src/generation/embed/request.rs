// Translation of https://github.com/ollama/ollama/blob/main/api/types.go into Rust

use crate::{generation::parameters, model::ModelOptions};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EmbedInput {
    Single(String),
    Multiple(Vec<String>),
}

impl From<String> for EmbedInput {
    fn from(value: String) -> Self {
        EmbedInput::Single(value)
    }
}

impl From<&str> for EmbedInput {
    fn from(value: &str) -> Self {
        EmbedInput::Single(value.to_string())
    }
}

impl From<Vec<String>> for EmbedInput {
    fn from(value: Vec<String>) -> Self {
        EmbedInput::Multiple(value)
    }
}

impl IntoIterator for EmbedInput {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            EmbedInput::Single(s) => vec![s].into_iter(),
            EmbedInput::Multiple(v) => v.into_iter(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct EmbedRequest {
    pub model: String,

    pub input: EmbedInput,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<parameters::KeepAlive>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ModelOptions>,
}

impl EmbedRequest {
    pub fn new<S: Into<String>, I: Into<EmbedInput>>(model: S, input: I) -> Self {
        Self {
            model: model.into(),
            input: input.into(),
            keep_alive: None,
            truncate: None,
            dimensions: None,
            options: None,
        }
    }

    /// How long the model will stay loaded in memory following this request.
    #[must_use]
    pub fn keep_alive(mut self, keep_alive: parameters::KeepAlive) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    /// Truncate is a boolean that, when set to true, truncates the chat history messages
    /// if the rendered prompt exceeds the context length limit.
    #[must_use]
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = Some(truncate);
        self
    }

    /// Dimensions truncates the output embedding to the specified dimension.
    #[must_use]
    pub fn dimensions(mut self, dimensions: i32) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    /// Options lists model-specific options. For example, temperature can be
    /// set through this field, if the model supports it.
    #[must_use]
    pub fn options(mut self, options: crate::model::ModelOptions) -> Self {
        self.options = Some(options);
        self
    }
}
