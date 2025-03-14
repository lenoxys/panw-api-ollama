use crate::types::{AiProfile, Content, Metadata, ScanRequest, PromptFindings, ResponseFindings, ScanResponse};
use reqwest::Client;
use thiserror::Error;
use tracing::{debug, error};
use uuid::Uuid;

use tracing::warn;

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Security assessment error: {0}")]
    AssessmentError(String),
    
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    // Add this new variant
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
    pub fn new(base_url: &str, api_key: &str, profile_name: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            profile_name: profile_name.to_string(),
        }
    }

    pub async fn assess_content(
        &self,
        content: &str,
        model_name: &str,
        is_prompt: bool,
    ) -> Result<Assessment, SecurityError> {
        // Skip assessment for empty content
        if content.trim().is_empty() {
            debug!("Skipping assessment for empty content");
            return Ok(Assessment {
                is_safe: true,
                category: "benign".to_string(),
                action: "allow".to_string(),
                details: ScanResponse {
                    report_id: "".to_string(),
                    scan_id: uuid::Uuid::default(), // Use proper UUID initialization
                    transaction_id: None,           // Previously tr_id
                    profile_id: None,
                    profile_name: None,
                    category: "benign".to_string(),
                    action: "allow".to_string(),
                    prompt_findings: PromptFindings {
                        url_cats: false,
                        dlp: false,
                        injection: false,
                        toxic_content: false,
                        malicious_code: false,
                    },
                    response_findings: ResponseFindings {
                        url_cats: false,
                        dlp: false,
                        db_security: false,
                        toxic_content: false,
                        malicious_code: false,
                    },
                    created_at: None,
                    completed_at: None,
                },
            });
        }

        // Create the content object based on whether it's a prompt or response
        let content_obj = if is_prompt {
            Content::new(Some(content.to_string()), None)
        } else {
            Content::new(None, Some(content.to_string()))
        }
        .expect("Invalid content state");

        // Create the request payload
        let payload = ScanRequest {
            tr_id: Uuid::new_v4().to_string(),
            ai_profile: AiProfile {
                profile_name: self.profile_name.clone(),
            },
            metadata: Metadata {
                app_name: "panw-api-ollama".to_string(),
                app_user: "ollama-proxy".to_string(),
                ai_model: model_name.to_string(),
            },
            contents: vec![content_obj],
        };

        let request = self
            .client
            .post(&format!("{}/v1/scan/sync/request", self.base_url))
            .header("Content-Type", "application/json")
            .header("x-pan-token", &self.api_key)
            .json(&payload)
            .build()
            .map_err(|e| {
                error!("Failed to build request: {}", e);
                SecurityError::RequestError(e)
            })?;

        // Log request details
        debug!("Request method: {}", request.method());
        debug!("Request URL: {}", request.url());
        debug!("Request headers:");
        for (name, value) in request.headers() {
            let value_str = if name == "x-pan-token" {
                "***REDACTED***".to_string()
            } else {
                value.to_str().unwrap_or_default().to_string()
            };
            debug!("  {}: {}", name, value_str);
        }

        // Log the JSON body (manually serialize the payload)
        debug!(
            "Request body:\n{}",
            serde_json::to_string_pretty(&payload)
                .unwrap_or_else(|_| "Failed to serialize payload".to_string())
        );

        // Send the request to the security API
        let response = self
            .client
            .post(&format!("{}/v1/scan/sync/request", self.base_url))
            .header("Content-Type", "application/json")
            .header("x-pan-token", &self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                error!("Security assessment request failed: {}", e);
                e
            })?;

        // Get the status first without consuming the response
        let status = response.status();

        // Now consume the response body
        let body_text = response.text().await.map_err(|e| {
            error!("Failed to read response body: {}", e);
            SecurityError::RequestError(e)  // Changed to RequestError
        })?;

        // Debug the raw response body
        debug!("Raw response body:\n{}", body_text);

        // Now handle the status check using the already obtained status
        if !status.is_success() {
            error!("Security assessment error: {} - {}", status, body_text);
            return Err(SecurityError::AssessmentError(format!(
                "{}: {}",
                status, body_text
            )));
        }

        // In your security assessment function
        let scan_result: ScanResponse = serde_json::from_str(&body_text).map_err(|e| {
            error!("Failed to parse security assessment response");
            SecurityError::JsonError(e)  // Keep as JsonError for serde errors
        })?;

        // Example usage
        if scan_result.action == "block" {
            warn!(
                "Security threat detected! Category: {}, Findings: {:#?}",
                scan_result.category, scan_result.prompt_findings
            );
            return Err(SecurityError::BlockedContent);
        }

        // Create an Assessment based on the ScanResponse
        let assessment = Assessment {
            is_safe: scan_result.category == "benign",
            category: scan_result.category.clone(),
            action: scan_result.action.clone(),
            details: scan_result,
        };

        Ok(assessment)
    }
}
