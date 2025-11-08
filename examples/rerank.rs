use ollama_rust::{generation::rerank::request::RerankRequest, llama::Llama};

pub mod common;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let documents = vec![
        "Switzerland is known for its mountains.",
        "The capital of the United States is Washington, D.C.",
        "The capital of China is not Berlin.",
        "The capital of China is Beijing.",
        "Cats are popular pets.",
        "China is a large country in Asia.",
        "Paris is the capital of France.",
    ]
    .into_iter()
    .map(std::string::ToString::to_string)
    .collect::<Vec<_>>();

    let query = "What is the capital of China?";

    println!("\nQueston: {query}\n");

    let llama = Llama::default();
    // println!("Individual Document Scores:");
    // for (i, doc) in documents.iter().enumerate() {
    //     let response = llama
    //         .rerank(RerankRequest::new_single("", query, &doc.clone()))
    //         .await
    //         .expect("Rerank failed");
    //     println!(
    //         "Document {}: Score {:.4}",
    //         i + 1,
    //         response.results[0].relevance_score
    //     );
    // }

    let response = llama
        .rerank(RerankRequest::new("", query, documents.clone()).top_n(4))
        .await
        .expect("Rerank failed");
    println!("\nTop Results:");
    for (i, result) in response.results.iter().enumerate() {
        println!(
            "Rank {}: Score {:.4}\nDocument: {:?}\n",
            i + 1,
            result.relevance_score,
            documents[result.index]
        );
    }

    Ok(())
}
