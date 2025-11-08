use crate::ollama::Ollama;

#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct ListModelsResponse {
    pub models: Vec<ModelInfo>,
}

#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct ModelInfo {
    pub name: String,

    pub model: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_model: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_host: Option<String>,

    #[serde(default)]
    pub modified_at: String,

    #[serde(default)]
    pub size: i64,

    #[serde(default)]
    pub digest: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ModelDetails>,
}

#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct ModelDetails {
    pub parent_model: String,
    pub format: String,
    pub family: String,
    pub families: Vec<String>,
    pub parameter_size: String,
    pub quantization_level: String,
}

impl Ollama {
    /// Ollama's `/api/tags` endpoint.
    ///
    /// # Errors
    ///
    /// Ollama side errors
    pub async fn list_models(&self) -> crate::Result<ListModelsResponse> {
        let url = self.url.join("/api/tags")?;
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(crate::OllamaError::Other(format!(
                "Error {}:\n{}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let models = response.json::<ListModelsResponse>().await?;

        Ok(models)
    }
}
