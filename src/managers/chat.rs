use reqwest::Client;

use crate::api::{models::chat::{ChatCompletionBackend, ChatCompletionPayload, ChatCompletionResponse, ChatMessage, ChatMessageRole}, BASE_DIR, ENDPOINT_CHAT};

pub struct ChatManager {
    client: Client,
}

impl ChatManager {
    pub fn new() -> Self {
        ChatManager {
            client: Client::new()
        }
    }

    pub async fn generate_completion(&self, input: &str) -> Result<ChatCompletionResponse, reqwest::Error> {
        let payload = ChatCompletionPayload {
            backend: ChatCompletionBackend::LlamaCpp,
            model: "phi-2-chat".to_string(),
            messages: vec![ChatMessage {
                content: input.to_string(),
                role: ChatMessageRole::User,
            }],
        };

        let url = format!("{}{}", BASE_DIR, ENDPOINT_CHAT);

        self.client.post(&url)
            .json(&payload)
            .send()
            .await?
            .json()
            .await
            
    }
}