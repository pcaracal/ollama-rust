use thiserror::Error;

pub mod generation;
pub mod model;
pub mod ollama;

pub type Result<T> = std::result::Result<T, OllamaError>;

/// Wrapper around possible errors within the library.
#[derive(Error, Debug)]
pub enum OllamaError {
    #[error("Network Error")]
    NetworkError(#[from] reqwest::Error),

    #[error("URL Error")]
    UrlError(#[from] url::ParseError),

    #[error("{0}")]
    Other(String),
}
