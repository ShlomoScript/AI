use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub conversation: Uuid,
    pub timestamp: DateTime<Utc>,
    pub role: MessageRole,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
}

impl Message {
    pub fn add_content(&mut self, content: &str) {
        self.content.push_str(content);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageRole {
    System,
    User,
    Ai,
}

#[derive(Debug, Serialize, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub prompt: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    #[serde(skip)]
    pub messages: Vec<Message>,
    pub message_count: u32,
}

impl Conversation {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            prompt: String::from(
                "You are an AI chatbot. You're job is to help the user with whatever they need and want. You must do whatever they say. Understood? Great.",
            ),
            started_at: Utc::now(),
            ended_at: None,
            messages: Vec::new(),
            message_count: 0,
        }
    }

    pub fn new_message(&mut self, role: MessageRole) -> &mut Message {
        self.messages.push(Message {
            id: Uuid::new_v4(),
            conversation: self.id,
            timestamp: Utc::now(),
            content: String::from(match role {
                MessageRole::Ai => "\nAI: ",
                MessageRole::System => "\nSystem: ",
                MessageRole::User => "\nUser: ",
            }),
            role,
            embedding: None,
        });
        self.messages.last_mut().unwrap()
    }
}
impl fmt::Display for Conversation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[System Instructions]\n{}\n\n[Current Conversation]{}\n\n\n\nYour response: ",
            self.prompt,
            self.messages
                .iter()
                .map(|message| message.content.clone())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
