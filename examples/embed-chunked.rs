use std::time::Duration;

use colored::Colorize;
use ollama_rust::{
    generation::{embed::request::EmbedRequest, parameters::KeepAlive},
    ollama::Ollama,
};
use tokio_stream::StreamExt;

use crate::common::Airport;

pub mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ollama = Ollama::default();

    let airports = Airport::load()?;

    let total_size = 1000;
    let chunk_size = 50;

    let mut stream = ollama.generate_embeddings_chunked(
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
        chunk_size,
    )?;

    let mut embeddings = vec![];

    let mut gen_sum = Duration::ZERO;

    while let Some(item) = stream.next().await {
        let item = item?;
        println!("Received chunk with {} embeddings", item.embeddings.len());
        embeddings.extend(item.embeddings);

        println!(
            "\n{}",
            "================================================================================"
                .blue()
        );
        println!("Embeddings: {}", embeddings.len());
        let total = Duration::from_nanos(item.total_duration.unwrap_or_default());
        let load = Duration::from_nanos(item.load_duration.unwrap_or_default());
        let generate = total.saturating_sub(load);
        gen_sum += generate;
        println!("Generation Duration: {generate:?}");
        println!(
            "Input tokens processed: {:?}",
            item.prompt_eval_count.unwrap_or_default()
        );
    }

    println!(
        "\n{}",
        "================================================================================".blue()
    );
    println!("Total embeddings received: {}", embeddings.len());
    println!("Total generation duration: {gen_sum:?}");

    Ok(())
}
