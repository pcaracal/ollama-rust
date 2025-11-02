use std::pin::Pin;

use async_stream::stream;
use tokio_stream::Stream;

use crate::{
    generation::embed::{request::EmbedRequest, response::EmbedResponse},
    ollama::Ollama,
};

pub mod request;
pub mod response;

pub type EmbedResponseStream = Pin<Box<dyn Stream<Item = crate::Result<EmbedResponse>>>>;

impl Ollama {
    /// Ollama's `/api/embed` endpoint. Returns an `EmbedResponse`.
    ///
    /// # Errors
    ///
    /// If Ollama rejects the request, e.g. bad parameters.
    /// If the response cannot be parsed.
    pub async fn generate_embeddings(&self, request: EmbedRequest) -> crate::Result<EmbedResponse> {
        let url = self.url.join("/api/embed")?;
        let response = self.client.post(url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(crate::OllamaError::Other(format!(
                "Error {}:\n{}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        Ok(response.json::<EmbedResponse>().await?)
    }

    /// Ollama's `/api/embed` endpoint. Returns a stream of `EmbedResponse`.
    ///
    /// The `chunk_size` parameter specifies how many items to include in each chunk of the response.
    ///
    /// # Notes
    ///
    /// The performance difference between using chunked and non-chunked requests is negligible.
    /// From a few benchmarks it seems, that the performance is pretty much identical +-0.05s...
    /// However, having a streamed response can be nice.
    ///
    /// # Errors
    ///
    /// If Ollama rejects the request, e.g. bad parameters.
    /// If the response cannot be parsed.
    pub fn generate_embeddings_chunked(
        &self,
        request: EmbedRequest,
        chunk_size: usize,
    ) -> crate::Result<EmbedResponseStream> {
        let url = self.url.join("/api/embed")?;

        let mut chunks = vec![];

        match &request.input {
            request::EmbedInput::Single(item) => {
                chunks.push(vec![item.clone()]);
            }
            request::EmbedInput::Multiple(items) => {
                for chunk in items.chunks(chunk_size) {
                    chunks.push(chunk.to_vec());
                }
            }
        }

        let ollama = self.clone();
        let stream = Box::pin(stream! {
            for chunk in chunks {
                let mut request = request.clone();
                request.input = request::EmbedInput::Multiple(chunk);
                let response = ollama.client.post(url.clone()).json(&request).send().await?;

                if !response.status().is_success() {
                    yield Err(crate::OllamaError::Other(format!(
                        "Error {}:\n{}",
                        response.status(),
                        response.text().await.unwrap_or_default()
                    )));

                    continue;
                }

                yield Ok(response.json::<EmbedResponse>().await?);
            }
        });

        Ok(stream)
    }
}
