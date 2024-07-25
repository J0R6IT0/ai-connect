use reqwest::Client;

use crate::api::{models::tts::{TTSBackend, TTSPayload}, BASE_DIR, ENDPOINT_TTS};

pub struct TTSManager {
    client: Client,
}

impl TTSManager {
    pub fn new() -> Self {
        TTSManager {
            client: Client::new()
        }
    }

    pub async fn generate_audio(&self, input: &str) -> Result<Vec<u8>, reqwest::Error> {
        let payload = TTSPayload {
            model: "en-us-ryan-medium.onnx".to_string(),
            input: input.to_string(),
            backend: TTSBackend::Piper,
        };

        let url = format!("{}{}", BASE_DIR, ENDPOINT_TTS);

        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await?;

        let audio_bytes = response.bytes().await?;

        Ok(audio_bytes.to_vec())
    }
}