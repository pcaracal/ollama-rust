use tokio_stream::StreamExt;

use crate::{
    OllamaError,
    generation::generate::{request::GenerateRequest, response::GenerateResponse},
    model::ModelOptions,
    ollama::Ollama,
};

impl Ollama {
    /// Rerank a single document.
    /// Given a query and a document, returns a relevance score from 0 to 1.
    ///
    /// # Errors
    ///
    /// If something went wrong. Or lots of things.
    pub async fn rerank(
        &self,
        model: &str,
        query: &str,
        document: &str,
    ) -> crate::Result<GenerateResponse> {
        let prompt = format!(
            r#"
<|im_start|>system
Judge whether the Document meets the requirements based on the Query and the Instruct provided.
The answer can ONLY be "yes" or "no", nothing else.
<|im_end|>

<|im_start|>user
<Instruct>: Is this document relevant to the query?
<Query>: {query}
<Document>: {document}
<|im_end|>

<|im_start|>assistant

<think>



</think>

"#
        );

        let mut stream = self
            .generate(
                GenerateRequest::new(model, &prompt)
                    .keep_alive(crate::generation::parameters::KeepAlive::Custom(
                        "30s".to_string(),
                    ))
                    .stream(false)
                    .options(
                        ModelOptions::default()
                            .temperature(0.)
                            .top_k(1)
                            .top_p(1.)
                            .repeat_penalty(1.)
                            .stop(vec!["\n".to_string()]),
                    ),
            )
            .await?;

        if let Some(item) = stream.next().await {
            let a = item.map(|a| a.first().cloned())?;
            if let Some(response) = a {
                return Ok(response);
            }
        }

        Err(OllamaError::Other("Reranking failed.".to_string()))
    }
}
