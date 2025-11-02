use std::pin::Pin;

use async_stream::stream;
use reqwest::Response;
use tokio_stream::{Stream, StreamExt};

use crate::{
    OllamaError,
    generation::chat::{history::History, request::ChatRequest, response::ChatResponse},
    ollama::Ollama,
};

pub mod history;
pub mod message;
pub mod request;
pub mod response;

pub type ChatResponseStream = Pin<Box<dyn Stream<Item = crate::Result<ChatResponse>>>>;
type ChatResponseStreamInner = Pin<Box<dyn Stream<Item = crate::Result<Vec<ChatResponse>>>>>;

impl Ollama {
    /// Ollama's `/api/chat` endpoint. Returns a stream of `ChatResponse`.
    /// If the request has `stream` set to false, the returning stream will only have one item.
    ///
    /// # Errors
    ///
    /// If Ollama rejects the request, e.g. the Model does not support thinking.
    /// If the response cannot be parsed.
    pub fn chat(
        &self,
        request: ChatRequest,
        history: History,
    ) -> crate::Result<ChatResponseStream> {
        let ollama = self.clone();

        Ok(Box::pin(stream! {
            let mut request = request.clone();

            loop {
                if request.messages.is_empty() {
                    break;
                }

                history.extend(&request.messages);
                request.messages = history.messages();

                let response = ollama.post(&request).await?;
                request.messages.clear();

                let mut stream = Self::stream_request_and_filter(response);

                while let Some(res) = stream.next().await {
                    if let Ok(responses) = res {
                        for response in responses {
                            history.push(&response.message);
                            yield Ok(response);
                        }
                    }
                }

                if let Some(last) = history.last() {
                    let mut tool_messages = vec![];

                    for tc in &last.tool_calls {
                        for tool in &request.tools {
                            if tool.tool_function().name == tc.function.name {
                                match tool.execute(tc.function.arguments.clone()).await {
                                    Ok(res) => tool_messages.push(message::Message::tool(res)),
                                    Err(err) => tool_messages.push(message::Message::tool(err)),
                                }
                            }
                        }
                    }

                    request.messages = tool_messages;
                }
            }
        }))
    }

    async fn post(&self, request: &ChatRequest) -> crate::Result<Response> {
        let url = self.url.join("/api/chat")?;
        let response = self.client.post(url).json(request).send().await?;

        if !response.status().is_success() {
            return Err(crate::OllamaError::Other(format!(
                "Error {}:\n{}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        Ok(response)
    }

    fn stream_request_and_filter(response: Response) -> ChatResponseStreamInner {
        let stream = response.bytes_stream().map(move |r| match r {
            Ok(bytes) => {
                let iter = serde_json::Deserializer::from_slice(&bytes).into_iter::<ChatResponse>();
                let res = iter
                    .filter_map(|a| {
                        if let Ok(mut cr) = a {
                            cr.message.done = cr.done;
                            Some(cr)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<ChatResponse>>();

                Ok(res)
            }
            Err(e) => Err(OllamaError::Other(format!("Failed to parse response: {e}"))),
        });

        Box::pin(stream)
    }
}
