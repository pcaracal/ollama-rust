use std::sync::Arc;

use ollama_rust::{
    generation::{
        chat::{history::History, message::Message, request::ChatRequest},
        tools::{Tool, ToolFunction},
    },
    model::ModelOptions,
    ollama::Ollama,
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, stdin, stdout};
use tokio_stream::StreamExt;

pub mod common;

#[derive(Debug, Clone)]
pub struct DateTimeTool;

impl Tool for DateTimeTool {
    fn tool_function(&self) -> ollama_rust::generation::tools::ToolFunction {
        ToolFunction::new(
            "date_time_info",
            "This tool will provide the current date and time in the RFC 2822 format.",
        )
    }

    fn execute(
        &self,
        _: ollama_rust::generation::tools::ToolCallArguments,
    ) -> ollama_rust::Result<String> {
        Ok(chrono::Local::now().to_rfc2822())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ollama = Ollama::default();
    let history = History::default();

    let system = Message::system(
        "You are a helpful assistant with access to the tool date_time_info.\n\
             You must use this tool if the user asks for the current date or time.",
    );

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

        let mut stream = ollama
            .chat(
                ChatRequest::new(
                    crate::common::QWEN3_4B_I,
                    vec![system.clone(), Message::user(line)],
                )
                .options(ModelOptions::default().seed(1).num_ctx(8192))
                // .think(ollama_rust::generation::parameters::Think::Enabled)
                .tool(Arc::new(DateTimeTool))
                .stream(true),
                history.clone(),
            )
            .await?;

        let mut did_think = false;
        let mut message_started = false;

        while let Some(res) = stream.next().await {
            match res {
                Ok(res) => {
                    if let Some(think) = &res.message.thinking {
                        if !did_think {
                            did_think = true;
                            stdout.write_all(b"<think>\n").await?;
                        }

                        stdout.write_all(think.as_bytes()).await?;
                    } else {
                        if did_think && !message_started {
                            message_started = true;
                            did_think = false;
                            stdout.write_all(b"</think>\n").await?;
                        }

                        stdout.write_all(res.message.content.as_bytes()).await?;
                    }

                    stdout.flush().await?;
                }
                Err(e) => println!("Error: {e}"),
            }
        }
    }

    Ok(())
}
