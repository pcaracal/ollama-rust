use std::time::Duration;

use ollama_rust::{
    generation::{embed::request::EmbedRequest, parameters::KeepAlive},
    ollama::Ollama,
};

use crate::common::Airport;

pub mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ollama = Ollama::default();

    let airports = Airport::load()?;

    let total_size = 1000;

    let res = ollama
        .generate_embeddings(
            EmbedRequest::new(
                common::QWEN3_EMBED_4B_2560D,
                airports
                    .iter()
                    .map(|(icao, airport)| format!("{icao}: {airport:#?}"))
                    .take(total_size)
                    .collect::<Vec<_>>(),
            )
            .keep_alive(KeepAlive::Custom("30s".to_string()))
            .dimensions(768),
        )
        .await?;

    println!("Embeddings: {}", res.embeddings.len());
    println!(
        "Dimensions: {}",
        res.embeddings.first().cloned().unwrap_or_default().len()
    );
    let total = Duration::from_nanos(res.total_duration.unwrap_or_default());
    let load = Duration::from_nanos(res.load_duration.unwrap_or_default());
    let generate = total.saturating_sub(load);
    println!("Generation Duration: {generate:?}");
    println!(
        "Input tokens processed: {:?}",
        res.prompt_eval_count.unwrap_or_default()
    );

    Ok(())
}
