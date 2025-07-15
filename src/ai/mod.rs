use chat::*;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::io::{self, Write};
use tokio_stream::StreamExt;

mod chat;
mod memory;

#[derive(Deserialize)]
struct StreamChunk {
    response: Option<String>,
    done: Option<bool>,
}

pub async fn prompt_loop() -> Result<(), Box<dyn Error>> {
    let mut prompt_builder = PromptBuilder::new();
    loop {
        let mut prompt = String::new();
        print!(">>> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut prompt)?;

        let message = Message::new(MessageRole::User);
        message.add_content(&prompt);
        message.content = message.content.trim_end().to_string();

        prompt_builder.add_message(message);

        let client = Client::new();

        let res = client
            .post("http://localhost:11434/api/generate")
            .json(&serde_json::json!({
                "model": "llama3",
                "prompt": prompt_builder.make_prompt(),
                "stream": true
            }))
            .send()
            .await?;

        let mut stream = res.bytes_stream();

        let message = Message::new(MessageRole::Ai);
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            for line in std::str::from_utf8(&chunk)?.lines() {
                if let Ok(parsed) = serde_json::from_str::<StreamChunk>(line) {
                    if let Some(text) = parsed.response {
                        print!("{}", text);
                        message.add_content(&text);
                    }
                }
            }
        }
        message.content = message.content.trim_end().to_string();
        prompt_builder.add_message(message);

        println!();
    }
}
