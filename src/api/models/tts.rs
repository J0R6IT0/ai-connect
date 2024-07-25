use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct TTSPayload {
    pub input: String,
    pub model: String,
    pub backend: TTSBackend
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TTSBackend {
    Piper,
}