#[derive(Debug, Clone, serde::Serialize)]
pub struct RerankRequest {
    pub model: String,
    pub query: String,
    pub documents: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_n: Option<usize>,
}

impl RerankRequest {
    pub fn new<S: Into<String>>(model: S, query: S, documents: Vec<String>) -> Self {
        Self {
            model: model.into(),
            query: query.into(),
            top_n: None,
            documents,
        }
    }

    pub fn new_single<S: Into<String>>(model: S, query: S, documents: S) -> Self {
        Self {
            model: model.into(),
            query: query.into(),
            top_n: None,
            documents: vec![documents.into()],
        }
    }

    #[must_use]
    pub fn top_n(mut self, n: usize) -> Self {
        self.top_n = Some(n);
        self
    }
}
