use ollama_rust::ollama::Ollama;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ollama = Ollama::default();

    Ok(())
}
