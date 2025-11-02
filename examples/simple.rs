use ollama_rust::{
    generation::generate::request::GenerateRequest, model::ModelOptions, ollama::Ollama,
};
use tokio::io::{AsyncWriteExt, stdout};
use tokio_stream::StreamExt;

use crate::common::QWEN3_4B;

pub mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ollama = Ollama::default();

    let mut stream = ollama
        .generate(
            GenerateRequest::new(QWEN3_4B, "Why do cats always land on their feet?")
                .options(ModelOptions::default())
                .think(ollama_rust::generation::parameters::Think::Disabled),
        )
        .await?;

    let mut stdout = stdout();

    stdout.write_all(b"Response:\n\n").await?;
    while let Some(a) = stream.next().await {
        match a {
            Ok(res) => {
                for res in res {
                    stdout.write_all(res.response.as_bytes()).await?;
                    stdout.flush().await?;
                }
            }
            Err(e) => println!(">> Error: {e}"),
        }
    }

    println!("\n\n[Done]");

    Ok(())
}
