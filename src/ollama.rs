use reqwest::{Client, IntoUrl, Url};

#[derive(Debug, Clone)]
pub struct Ollama {
    pub(crate) url: Url,
    pub(crate) client: Client,
}

impl Default for Ollama {
    /// Creates a new Ollama client with the default URL `http://127.0.0.1:11434`
    fn default() -> Self {
        Self {
            url: Url::parse("http://127.0.0.1:11434").unwrap(),
            client: Client::new(),
        }
    }
}

impl Ollama {
    /// Set the base URL for the Ollama API
    /// # Errors
    /// * Returns an error if the provided URL is invalid.
    pub fn with_url<U: IntoUrl>(mut self, url: U) -> crate::Result<Self> {
        self.url = url.into_url()?;
        Ok(self)
    }

    /// Set the `reqwest::Client` to be used
    #[must_use]
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }
}
