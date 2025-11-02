use std::collections::HashMap;

// Chatting models
pub const QWEN3_4B_I: &str = "qwen3:4b-instruct-2507-q4_K_M";
pub const QWEN3_4B_T: &str = "qwen3:4b-thinking-2507-q4_K_M";

// Embedding
pub const QWEN3_EMBED_4B_2560D: &str = "qwen3-embedding:4b";
pub const QWEN3_EMBED_8B_4096D: &str = "qwen3-embedding:8b";

// Rerank
pub const QWEN3_RERANK_8B: &str = "dengcao/Qwen3-Reranker-8B:Q4_K_M";

// Testing data
#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub struct Airport {
    pub icao: String,
    pub iata: String,
    pub name: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub elevation: i32,
    pub lat: f64,
    pub lon: f64,
    pub tz: String,
}

impl Airport {
    /// Loads around 29'294 `Airport` objects
    pub fn load() -> anyhow::Result<HashMap<String, Self>> {
        let airports = include_str!("../testdata/Airports/airports.json");
        Ok(serde_json::from_str(airports)?)
    }
}
