use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Debug)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

pub async fn get_embedding(prompt: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "http://localhost:11434/api/embeddings";

    let res = client
        .post(url)
        .json(&json!({
            "model": "nomic-embed-text",
            "prompt": prompt,
        }))
        .send()
        .await?
        .error_for_status()?
        .json::<EmbeddingResponse>()
        .await?;

    Ok(res.embedding)
}
