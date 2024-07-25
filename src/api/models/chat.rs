use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ChatCompletionResponse {
    pub choices: Vec<ChatCompletionChoice>,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChoice {
    pub message: ChatMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub content: String,
    pub role: ChatMessageRole,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ChatMessageRole {
    User,
    Assistant,
}

#[derive(Serialize, Debug)]
pub struct ChatCompletionPayload {
    pub model: String,
    pub backend: ChatCompletionBackend,
    pub messages: Vec<ChatMessage>,
}

#[derive(Serialize, Debug)]
pub enum ChatCompletionBackend {
    #[serde(rename = "llama-cpp")]
    LlamaCpp,
}
