use crate::{
    generation::{
        generate::{request::GenerateRequest, response::GenerateResponse},
        parameters::KeepAlive,
    },
    model::ModelOptions,
    ollama::Ollama,
};

impl Ollama {
    /// Rerank a single document.
    /// Given a query and a document, returns a relevance score from 0 to 1.
    ///
    /// # Arguments
    ///
    /// * `model` - The model to use.
    /// * `query` - The query to rerank against.
    /// * `document` - The document to rerank.
    /// * `keep_alive` - Optional keep alive setting. `None` to use Ollama's default.
    /// * `options` - Model options.
    ///
    /// # Errors
    ///
    /// If something went wrong. Or lots of things.
    pub async fn rerank(
        &self,
        model: &str,
        query: &str,
        document: &str,
        keep_alive: Option<KeepAlive>,
        options: ModelOptions,
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

        // ModelOptions::default()
        //     .temperature(0.)
        //     .top_k(1)
        //     .top_p(1.)
        //     .repeat_penalty(1.)
        //     .stop(vec!["\n".to_string()]),

        let mut req = GenerateRequest::new(model, &prompt)
            .think(crate::generation::parameters::Think::Disabled)
            .stream(false)
            .options(options);
        if let Some(ka) = keep_alive {
            req = req.keep_alive(ka);
        }

        self.generate_without_stream(req).await
    }
}
