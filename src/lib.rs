use thiserror::Error;

use crate::generation::chat::history::HistoryPoisonError;

pub mod generation;
pub mod llama;
pub mod misc;
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

impl<'a> From<HistoryPoisonError<'a>> for OllamaError {
    fn from(err: HistoryPoisonError<'a>) -> Self {
        OllamaError::Other(format!("History lock poisoned: {err}"))
    }
}
