use ollama_rust::{
    generation::generate::{GenerateResponseStream, request::GenerateRequest},
    model::ModelOptions,
    ollama::Ollama,
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, stdin, stdout};
use tokio_stream::StreamExt;

pub mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ollama = Ollama::default();

    let mut context = vec![];

    let mut stdout = stdout();
    loop {
        stdout.write_all(b"\n > ").await?;
        stdout.flush().await?;

        let reader = BufReader::new(stdin());
        let line = reader.lines().next_line().await?.unwrap_or_default();
        let line = line.trim();
        if line.is_empty() || line.eq_ignore_ascii_case("exit") || line.eq_ignore_ascii_case("quit")
        {
            break;
        }

        let request = GenerateRequest::new(common::QWEN3_4B_I, line)
            .options(ModelOptions::default().num_ctx(16384))
            .context(context.clone());
        let mut stream: GenerateResponseStream = ollama.generate(request).await?;

        while let Some(a) = stream.next().await {
            match a {
                Ok(res) => {
                    for res in res {
                        stdout.write_all(res.response.as_bytes()).await?;
                        stdout.flush().await?;

                        if let Some(new_context) = res.context {
                            context = new_context;
                            stdout
                                .write_all(
                                    format!(
                                        "\n\n>> Done. Updated context length: {}",
                                        context.len()
                                    )
                                    .as_bytes(),
                                )
                                .await?;
                        }
                    }
                }
                Err(e) => println!(">> Error: {e}"),
            }
        }
    }

    Ok(())
}
