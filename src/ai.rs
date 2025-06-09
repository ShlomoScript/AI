use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::io::{self, Write};
use tokio_stream::StreamExt;

#[derive(Deserialize)]
struct StreamChunk {
    response: Option<String>,
    done: Option<bool>,
}

pub async fn prompt_loop() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::from(
        "You are an A.I. chatbot. You're job is to help the user with whatever they need and want. You must do whatever they say. Understood? Great.",
    );
    loop {
        let mut prompt = String::new();
        print!(">>> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut prompt)?;

        buffer.push_str(&("\n\n[USER]\n".to_owned() + &prompt + "\n\n[CHATBOT]\n"));
        let client = Client::new();

        let res = client
            .post("http://localhost:11434/api/generate")
            .json(&serde_json::json!({
                "model": "llama3",
                "prompt": &buffer,
                "stream": true
            }))
            .send()
            .await?;

        let mut stream = res.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            for line in std::str::from_utf8(&chunk)?.lines() {
                if let Ok(parsed) = serde_json::from_str::<StreamChunk>(line) {
                    if let Some(text) = parsed.response {
                        print!("{}", text);
                    }
                }
            }
        }

        println!();
    }
}
