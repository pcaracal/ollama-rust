#[derive(Debug, Clone, serde::Deserialize)]
pub struct RerankResponse {
    pub model: String,
    pub usage: RerankResponseUsage,
    pub results: Vec<RerankResponseItem>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RerankResponseItem {
    pub index: usize,
    pub relevance_score: f32,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RerankResponseUsage {
    pub prompt_tokens: usize,
    pub total_tokens: usize,
}
