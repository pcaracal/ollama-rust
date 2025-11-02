// Translation of https://github.com/ollama/ollama/blob/main/api/types.go into Rust

use crate::generation::chat::message::Message;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ChatResponse {
    /// Model name
    pub model: String,

    /// Name of the upstream model that generated the response.
    pub remote_model: Option<String>,

    /// URL of the upstream Ollama host that generated the response.
    pub remote_host: Option<String>,

    /// ISO 8601 timestamp of response creation
    pub created_at: String,

    /// Message or part of a message from the model
    pub message: Message,

    /// Indicates whether generation has finished
    pub done: bool,

    /// Reason the generation stopped
    pub done_reason: Option<String>,

    /// Debug information for template rendering
    pub debug_info: Option<DebugInfo>,

    // -- metrics below --
    /// Time spent generating the response in nanoseconds
    pub total_duration: Option<u64>,

    /// Time spent loading the model in nanoseconds
    pub load_duration: Option<u64>,

    /// Number of input tokens in the prompt
    pub prompt_eval_count: Option<u64>,

    /// Time spent evaluating the prompt in nanoseconds
    pub prompt_eval_duration: Option<u64>,

    /// Number of output tokens generated in the response
    pub eval_count: Option<u64>,

    /// Time spent generating tokens in nanoseconds
    pub eval_duration: Option<u64>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DebugInfo {
    /// The rendered template used for generation
    pub rendered_template: String,

    /// Number of images included in the generation
    pub image_count: Option<u32>,
}
