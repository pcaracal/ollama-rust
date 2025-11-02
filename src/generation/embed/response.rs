// Translation of https://github.com/ollama/ollama/blob/main/api/types.go into Rust

use std::time::Duration;

use crate::generation::embed::request::EmbedInput;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
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

impl std::fmt::Debug for EmbedResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmbedResponse")
            .field("Model", &self.model)
            .field(
                "Embeddings",
                &format!(
                    "<{} items flattened>",
                    &self.embeddings.iter().flatten().collect::<Vec<_>>().len()
                ),
            )
            .field(
                "Total",
                &Duration::from_nanos(self.total_duration.unwrap_or_default()),
            )
            .field(
                "Load",
                &Duration::from_nanos(self.load_duration.unwrap_or_default()),
            )
            .field(
                "Generation",
                &(Duration::from_nanos(self.total_duration.unwrap_or_default())
                    - Duration::from_nanos(self.load_duration.unwrap_or_default())),
            )
            .field("Processed Input Tokens", &self.prompt_eval_count)
            .field("Input Text", &self.input_text)
            .finish()
    }
}
