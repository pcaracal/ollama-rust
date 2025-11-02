use std::pin::Pin;

use tokio_stream::{Stream, StreamExt};

use crate::{
    OllamaError,
    generation::generate::{request::GenerateRequest, response::GenerateResponse},
    ollama::Ollama,
};

pub mod request;
pub mod response;

pub type GenerateResponseStream = Pin<Box<dyn Stream<Item = crate::Result<Vec<GenerateResponse>>>>>;

impl Ollama {
    /// Ollama's `/api/generate` endpoint. Returns a stream of `GenerateResponse`.
    /// If the request has `stream` set to false, the returning stream will only have one item.
    ///
    /// # Errors
    ///
    /// If Ollama rejects the request, e.g. the Model does not support thinking.
    /// If the response cannot be parsed.
    pub async fn generate(
        &self,
        request: GenerateRequest,
    ) -> crate::Result<GenerateResponseStream> {
        let url = self.url.join("/api/generate")?;
        let response = self.client.post(url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(crate::OllamaError::Other(format!(
                "Error {}:\n{}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let stream = response.bytes_stream().map(|r| match r {
            Ok(bytes) => {
                let iter = serde_json::Deserializer::from_slice(&bytes).into_iter();
                let res = iter
                    .filter_map(Result::ok)
                    .collect::<Vec<GenerateResponse>>();

                Ok(res)
            }
            Err(e) => Err(OllamaError::Other(format!("Failed to parse response: {e}"))),
        });

        Ok(Box::pin(stream))
    }
}
