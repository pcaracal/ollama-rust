#[derive(Debug, Clone, serde::Deserialize)]
pub struct GenerateResponse {
    /// Model name
    pub model: String,

    /// Name of the upstream model that generated the response.
    pub remote_model: Option<String>,

    /// URL of the upstream Ollama host that generated the response.
    pub remote_host: Option<String>,

    /// ISO 8601 timestamp of response creation
    pub created_at: String,

    /// The model's generated text response
    pub response: String,

    /// The model's generated thinking output
    pub thinking: Option<String>,

    /// Indicates whether generation has finished
    pub done: bool,

    /// Reason the generation stopped
    pub done_reason: Option<String>,

    /// Encoding of the conversation used in this response
    /// This can be sent in the next request to keep a conversational memory.
    pub context: Option<Vec<i32>>,

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

    // TODO: Tool calls made during generation
    /// Debug information for template rendering
    pub debug_info: Option<DebugInfo>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DebugInfo {
    /// The rendered template used for generation
    pub rendered_template: String,

    /// Number of images included in the generation
    pub image_count: Option<u32>,
}
