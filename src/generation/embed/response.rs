// Translation of https://github.com/ollama/ollama/blob/main/api/types.go into Rust

use crate::generation::embed::request::EmbedInput;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmbedResponse {
    /// Model that produced the embeddings
    pub model: String,

    /// Array of vector embeddings
    pub embeddings: Vec<Vec<f32>>,

    /// Total time spent generating in nanoseconds
    pub total_duration: Option<u64>,

    /// Load time in nanoseconds
    pub load_duration: Option<u64>,

    /// Number of input tokens processed to generate embeddings
    pub prompt_eval_count: Option<u64>,

    /// Input Text for the Embeddings; Not from Ollama API
    pub input_text: Option<EmbedInput>,
}
