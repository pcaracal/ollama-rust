use ollama_rust::{
    OllamaError,
    generation::{generate::GenerateRequest, response::GenerateResponse},
    model::ModelOptions,
    ollama::Ollama,
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ollama = Ollama::default();

    let response = ollama
        .generate(
            GenerateRequest::new("gemma3:latest", "Why do cats always land on their feet?")
                .options(ModelOptions::default().temperature(0.4))
                .think(ollama_rust::generation::parameters::Think::Disabled),
        )
        .await?;

    let mut stream = response.bytes_stream().map(|r| match r {
        Ok(bytes) => {
            let iter = serde_json::Deserializer::from_slice(&bytes).into_iter();
            let res = iter
                .filter_map(Result::ok)
                .collect::<Vec<GenerateResponse>>();

            Ok(res)
        }
        Err(e) => Err(OllamaError::Other(format!("Failed to parse response: {e}"))),
    });

    let mut did_think = false;

    while let Some(a) = stream.next().await {
        match a {
            Ok(res) => {
                for part in res {
                    if let Some(thinking) = part.thinking {
                        if !did_think {
                            println!("Thinking:\n");
                        }

                        print!("{thinking}");
                        did_think = true;
                    } else {
                        if did_think {
                            println!("Response:\n");
                            did_think = false;
                        }

                        print!("{}", part.response);
                    }
                }
            }
            Err(e) => println!("Error: {e}"),
        }
    }

    println!();

    Ok(())
}
