use serde::{Deserialize, Serialize};
use chrono::DateTime;
use chrono::Utc;
use serde_json::Value;

// Ollama API types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<i32>>,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: String,
    pub message: Message,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsResponse {
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub models: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
    pub digest: String,
    pub details: ModelDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDetails {
    pub format: String,
    pub family: String,
    pub families: Vec<String>,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionResponse {
    pub version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanRequest {
    pub tr_id: String,
    pub ai_profile: AiProfile,
    pub metadata: Metadata,
    pub contents: Vec<Content>,
}

impl ScanResponse {
    pub fn default_safe_response() -> Self {
        Self {
            report_id: String::new(),
            scan_id: uuid::Uuid::default(),
            tr_id: None,
            profile_id: None,
            profile_name: None,
            category: "benign".to_string(),
            action: "allow".to_string(),
            prompt_detected: PromptDetected::default(),
            response_detected: ResponseDetected::default(),
            created_at: None,
            completed_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AiProfile {
    pub profile_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Metadata {
    pub app_name: String,
    pub app_user: String,
    pub ai_model: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Content {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScanResponse {
    #[serde(default)]
    pub report_id: String,
    #[serde(default)]
    pub scan_id: uuid::Uuid,
    #[serde(default)]
    pub tr_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<uuid::Uuid>,
    #[serde(default)]
    pub profile_name: Option<String>,
    pub category: String,
    pub action: String,
    #[serde(default)]
    pub prompt_detected: PromptDetected,
    #[serde(default)]
    pub response_detected: ResponseDetected,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PromptDetected {
    #[serde(default)]
    pub url_cats: bool,
    #[serde(default)]
    pub dlp: bool,
    #[serde(default)]
    pub injection: bool,
    #[serde(default)]
    pub toxic_content: bool,
    #[serde(default)]
    pub malicious_code: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ResponseDetected {
    #[serde(default)]
    pub url_cats: bool,
    #[serde(default)]
    pub dlp: bool,
    #[serde(default)]
    pub db_security: bool,
    #[serde(default)]
    pub toxic_content: bool,
    #[serde(default)]
    pub malicious_code: bool,
}