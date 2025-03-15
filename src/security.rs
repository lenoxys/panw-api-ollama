use crate::types::{
    AiProfile, Content, Metadata, ScanRequest, ScanResponse,
};
use reqwest::Client;
use thiserror::Error;
use tracing::{debug, error, warn};
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Security assessment error: {0}")]
    AssessmentError(String),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Content blocked by security policy")]
    BlockedContent,
}

#[derive(Debug, Clone)]
pub struct Assessment {
    pub is_safe: bool,
    pub category: String,
    pub action: String,
    pub details: ScanResponse,
}

#[derive(Clone)]
pub struct SecurityClient {
    client: Client,
    base_url: String,
    api_key: String,
    profile_name: String,
    app_name: String,
    app_user: String,
}

impl Content {
    pub fn new(prompt: Option<String>, response: Option<String>) -> Result<Self, &'static str> {
        match (prompt.is_some(), response.is_some()) {
            (true, false) | (false, true) => Ok(Self { prompt, response }),
            _ => Err("Content must have either prompt or response, not both or none"),
        }
    }
}

impl SecurityClient {
    pub fn new(
        base_url: &str,
        api_key: &str,
        profile_name: &str,
        app_name: &str,
        app_user: &str,
    ) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            profile_name: profile_name.to_string(),
            app_name: app_name.to_string(),
            app_user: app_user.to_string(),
        }
    }

    pub async fn assess_content(
        &self,
        content: &str,
        model_name: &str,
        is_prompt: bool,
    ) -> Result<Assessment, SecurityError> {
        // Skip assessment for empty content early
        if content.trim().is_empty() {
            debug!("Skipping assessment for empty content");
            return Ok(Assessment {
                is_safe: true,
                category: "benign".to_string(),
                action: "allow".to_string(),
                details: ScanResponse::default_safe_response(),
            });
        }

        // Create the content object
        let content_obj = if is_prompt {
            Content::new(Some(content.to_string()), None)
        } else {
            Content::new(None, Some(content.to_string()))
        }
        .map_err(|e| SecurityError::AssessmentError(e.to_string()))?;

        // Create and send the request payload
        let payload = self.create_scan_request(content_obj, model_name);
        let scan_result = self.send_security_request(&payload).await?;

        // Create assessment and check if content should be blocked
        let assessment = Assessment {
            is_safe: scan_result.category == "benign",
            category: scan_result.category.clone(),
            action: scan_result.action.clone(),
            details: scan_result,
        };

        if assessment.action == "block" {
            warn!(
                "Security threat detected! Category: {}, Findings: {:#?}",
                assessment.category, assessment.details.prompt_detected
            );
            return Err(SecurityError::BlockedContent);
        }

        Ok(assessment)
    }

    // Helper function to create scan request
    fn create_scan_request(&self, content_obj: Content, model_name: &str) -> ScanRequest {
        ScanRequest {
            tr_id: Uuid::new_v4().to_string(),
            ai_profile: AiProfile {
                profile_name: self.profile_name.clone(),
            },
            metadata: Metadata {
                app_name: self.app_name.to_string(),
                app_user: self.app_user.to_string(),
                ai_model: model_name.to_string(),
            },
            contents: vec![content_obj],
        }
    }

    // Helper to send the request and handle errors
    async fn send_security_request(
        &self,
        payload: &ScanRequest,
    ) -> Result<ScanResponse, SecurityError> {
        let response = self
            .client
            .post(&format!("{}/v1/scan/sync/request", self.base_url))
            .header("Content-Type", "application/json")
            .header("x-pan-token", &self.api_key)
            .json(payload)
            .send()
            .await
            .map_err(|e| {
                error!("Security assessment request failed: {}", e);
                SecurityError::RequestError(e)
            })?;

        let status = response.status();
        let body_text = response.text().await.map_err(|e| {
            error!("Failed to read response body: {}", e);
            SecurityError::RequestError(e)
        })?;

        // Debug the raw response only when debug is enabled
        debug!("Raw response body:\n{}", body_text);

        if !status.is_success() {
            error!("Security assessment error: {} - {}", status, body_text);
            return Err(SecurityError::AssessmentError(format!(
                "{}: {}",
                status, body_text
            )));
        }

        serde_json::from_str(&body_text).map_err(|e| {
            error!("Failed to parse security assessment response");
            SecurityError::JsonError(e)
        })
    }
}
