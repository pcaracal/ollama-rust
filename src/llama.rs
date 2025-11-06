use reqwest::{Client, IntoUrl, Url};

#[derive(Debug, Clone)]
pub struct Llama {
    pub(crate) url: Url,
    pub(crate) client: Client,
}

impl Default for Llama {
    /// Creates a new Llama.cpp client with the default URL `http://127.0.0.1:8012`
    fn default() -> Self {
        Self {
            url: Url::parse("http://127.0.0.1:8012").unwrap(),
            client: Client::new(),
        }
    }
}

impl Llama {
    /// Set the base URL for the Llama.cpp API
    /// # Errors
    /// * Returns an error if the provided URL is invalid.
    pub fn with_url<U: IntoUrl>(mut self, url: U) -> crate::Result<Self> {
        self.url = url.into_url()?;
        Ok(self)
    }

    /// Set the `reqwest::Client` to be used
    pub fn with_client(mut self, client: reqwest::Client) {
        self.client = client;
    }
}
