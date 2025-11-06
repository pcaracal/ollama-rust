use crate::{
    generation::rerank::{request::RerankRequest, response::RerankResponse},
    llama::Llama,
};

pub mod request;
pub mod response;

impl Llama {
    /// Llama.cpp `/rerank` endpoint. Returns one `RerankResponse`.
    ///
    /// # Errors
    ///
    /// If Llama.cpp rejects the request, e.g. the Model does not support reranking.
    /// If the response cannot be parsed.
    pub async fn rerank(&self, request: RerankRequest) -> crate::Result<RerankResponse> {
        let url = self.url.join("/rerank")?;
        let response = self.client.post(url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(crate::OllamaError::Other(format!(
                "Error {}:\n{}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        Ok(response.json::<RerankResponse>().await?)
    }
}
