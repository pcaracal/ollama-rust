// Translation of https://github.com/ollama/ollama/blob/main/api/types.go into Rust

use reqwest::Response;

use crate::{generation::parameters, model::ModelOptions, ollama::Ollama};

#[derive(Debug, Clone, serde::Serialize)]
pub struct GenerateRequest {
    pub model: String,

    pub prompt: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub context: Vec<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<parameters::KeepAlive>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,

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
impl GenerateRequest {
    pub fn new<S: Into<String>>(model: S, prompt: S) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            suffix: None,
            system: None,
            template: None,
            context: Vec::new(),
            stream: None,
            raw: None,
            format: None,
            keep_alive: None,
            images: Vec::new(),
            options: None,
            think: None,
            truncate: None,
            shift: None,
            debug_render_only: None,
        }
    }

    /// Suffix is the text that comes after the inserted text.
    #[must_use]
    pub fn suffix(mut self, suffix: String) -> Self {
        self.suffix = Some(suffix);
        self
    }

    /// System overrides the model's default system message/prompt.
    #[must_use]
    pub fn system(mut self, system: String) -> Self {
        self.system = Some(system);
        self
    }

    /// Template overrides the model's default prompt template.
    #[must_use]
    pub fn template(mut self, template: String) -> Self {
        self.template = Some(template);
        self
    }

    /// Context is the context parameter returned from a previous GenerateRequest.
    /// It can be used to keep a short conversational memory.
    #[must_use]
    pub fn context(mut self, context: Vec<i32>) -> Self {
        self.context = context;
        self
    }

    /// Stream specifies whether the response is streaming; it is true by default.
    #[must_use]
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Raw set to true means that no formatting will be applied to the prompt.
    #[must_use]
    pub fn raw(mut self, raw: bool) -> Self {
        self.raw = Some(raw);
        self
    }

    /// KeepAlive controls how long the model will stay loaded in memory following
    /// this request.
    #[must_use]
    pub fn keep_alive(mut self, keep_alive: parameters::KeepAlive) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    /// Images is an optional list of raw image bytes accompanying this
    /// request, for multimodal models.
    #[must_use]
    pub fn images(mut self, images: Vec<String>) -> Self {
        self.images = images;
        self
    }

    /// Options lists model-specific options. For example, temperature can be
    /// set through this field, if the model supports it.
    #[must_use]
    pub fn options(mut self, options: crate::model::ModelOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Think controls whether thinking/reasoning models will think before
    /// responding. Can be a boolean (true/false) or a string ("high", "medium", "low")
    /// for supported models.
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

impl Ollama {
    pub async fn generate(&self, request: GenerateRequest) -> crate::Result<Response> {
        let url = self.url.join("/api/generate")?;
        let response = self.client.post(url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(crate::OllamaError::Other(format!(
                "Error {}:\n{}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        Ok(response)
    }
}
