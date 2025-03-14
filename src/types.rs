use serde::{Deserialize, Serialize};

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
    pub options: Option<serde_json::Value>,
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
    pub options: Option<serde_json::Value>,
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
    pub options: Option<serde_json::Value>,
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

// Palo Alto Networks API types
#[derive(Debug, Clone, Serialize)]
pub struct ScanRequest {
    pub tr_id: String,
    pub ai_profile: AiProfile,
    pub metadata: Metadata,
    pub contents: Vec<Content>,
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

// Updated Content struct to match Palo Alto API format
#[derive(Debug, Clone, Serialize)]
pub struct Content {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScanResponse {
    #[serde(rename = "report_id")]
    pub report_id: String,
    #[serde(rename = "scan_id")]
    pub scan_id: uuid::Uuid,
    #[serde(rename = "tr_id")]
    pub transaction_id: Option<String>,
    #[serde(rename = "profile_id")]
    pub profile_id: Option<uuid::Uuid>,
    #[serde(rename = "profile_name")]
    pub profile_name: Option<String>,
    pub category: String,
    pub action: String,
    #[serde(rename = "prompt_detected")]
    pub prompt_findings: PromptFindings,
    #[serde(rename = "response_detected")]
    pub response_findings: ResponseFindings,
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "completed_at")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PromptFindings {
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

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseFindings {
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
