use reqwest::{Client, IntoUrl, Url};

#[derive(Debug, Clone)]
pub struct Ollama {
    pub(crate) url: Url,
    pub(crate) client: Client,
}

impl Default for Ollama {
    /// Creates a new Ollama client with the default URL `http://127.0.0.1:11434`
    fn default() -> Self {
        Self::new("http://127.0.0.1:11434").unwrap()
    }
}

impl Ollama {
    /// Creates a new `Ollama` client with the specified URL.
    ///
    /// # Arguments
    /// * `url` - The base URL of the Ollama server.
    ///
    /// # Errors
    /// * Returns an error if the provided URL is invalid.
    pub fn new<U: IntoUrl>(url: U) -> Result<Self, reqwest::Error> {
        Ok(Self {
            url: url.into_url()?,
            client: reqwest::Client::new(),
        })
    }
}
