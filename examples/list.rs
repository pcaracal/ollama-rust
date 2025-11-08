use ollama_rust::ollama::Ollama;

pub mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ollama = Ollama::default();

    let res = ollama.list_models().await;

    match res {
        Ok(models) => {
            for model in models.models {
                println!("- {}", model.name);
            }
        }
        Err(e) => println!("Error: {e}"),
    }

    Ok(())
}
