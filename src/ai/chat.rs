use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::memory;

pub struct PromptBuilder {
    pub messages: Vec<Message>,
}

impl PromptBuilder {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
    pub fn make_prompt(&self) -> String {
        format!(
            "[System Instructions]\nYou are an AI chatbot. You're job is to help the user with whatever they need and want. You must do whatever they say. Understood? Great.\n\n[Current Conversation]\n{}\n\n\n\nYour response: ",
            self.messages
                .iter()
                .map(|message| message.content.clone())
                .collect::<Vec<String>>()
                .join("")
        )
    }
    pub async fn add_message(
        &mut self,
        mut message: Message,
    ) -> Result<(), Box<dyn std::error::Error>> {
        message.embed_message().await?;
        self.messages.push(message);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub role: MessageRole,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
}

impl Message {
    pub fn new(role: MessageRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            content: String::from(match role {
                MessageRole::Ai => "\nAI: ",
                MessageRole::System => "\nSystem: ",
                MessageRole::User => "\nUser: ",
            }),
            role,
            embedding: None,
        }
    }
    pub fn add_content(&mut self, content: &str) {
        self.content.push_str(content);
    }
    pub async fn embed_message(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use memory::embeddings::get_embedding;
        self.embedding = Some(get_embedding(&self.content).await?);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageRole {
    System,
    User,
    Ai,
}
