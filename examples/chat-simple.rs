use std::sync::Arc;

use ollama_rust::{
    generation::{
        chat::{history::History, message::Message, request::ChatRequest},
        tools::{Tool, ToolFunction},
    },
    model::ModelOptions,
    ollama::Ollama,
};
use tokio::io::{AsyncWriteExt, stdout};
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

    let messages = vec![
        Message::system(
            "You are a helpful assistant with access to the tool date_time_info.\n\
             You must use this tool if the user asks for the current date or time.",
        ),
        Message::user("What is the current time, and when should I feed my cats?"),
    ];

    let mut stream = ollama
        .chat(
            ChatRequest::new(crate::common::QWEN3_4B_I, messages)
                .options(ModelOptions::default().seed(1).num_ctx(8192))
                // .think(ollama_rust::generation::parameters::Think::Enabled)
                .tool(Arc::new(DateTimeTool))
                .stream(true),
            history.clone(),
        )
        .await?;

    let mut stdout = stdout();
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

    println!("\n\n[Done]");

    Ok(())
}
