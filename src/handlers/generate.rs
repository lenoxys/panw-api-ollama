use axum::{
    extract::State,
    response::Response,
    Json,
};
use bytes::Bytes;
use futures_util::stream::StreamExt;
use serde_json::json;
use tracing::{debug, info, error};
use http_body_util::StreamBody;
use axum::body::Body;

use crate::AppState;
use crate::types::GenerateRequest;
use crate::stream::{SecurityAssessedStream, SecurityAssessable};
use crate::handlers::ApiError;

impl SecurityAssessable for crate::types::GenerateResponse {
    fn get_content_for_assessment(&self) -> Option<(&str, &str)> {
        Some((&self.response, "generate_response"))
    }
}

pub async fn handle_generate(
    State(state): State<AppState>,
    Json(request): Json<GenerateRequest>,
) -> Result<Response, ApiError> {
    debug!("Received generate request for model: {}", request.model);
    
    // Assess the prompt - now specify this is a prompt (true)
    let assessment = state.security_client.assess_content(
        &request.prompt, 
        &request.model,
        true // This is a prompt
    ).await?;
    
    if !assessment.is_safe {
        info!("Security issue detected in prompt: category={}, action={}", 
              assessment.category, assessment.action);
        return Err(ApiError::SecurityIssue(format!(
            "Content violates security policy. Category: {}, Action: {}", 
            assessment.category, assessment.action
        )));
    }
    
    // Handle streaming requests
    if request.stream.unwrap_or(false) {
        debug!("Handling streaming generate request");
        return handle_streaming_generate(state, request).await;
    }
    
    // Handle non-streaming requests
    debug!("Handling non-streaming generate request");
    let response = state.ollama_client.forward("/api/generate", &request).await?;
    let body_bytes = response.bytes().await.map_err(|e| {
        error!("Failed to read response body: {}", e);
        ApiError::InternalError("Failed to read response body".to_string())
    })?;
    
    let response_body: crate::types::GenerateResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
        error!("Failed to parse response: {}", e);
        ApiError::InternalError("Failed to parse response".to_string())
    })?;
    
    // Assess response content - now specify this is a response (false)
    let assessment = state.security_client.assess_content(
        &response_body.response, 
        &request.model,
        false // This is a response
    ).await?;
    
    if !assessment.is_safe {
        info!("Security issue detected in response: category={}, action={}", 
              assessment.category, assessment.action);
        return Err(ApiError::SecurityIssue(format!(
            "Response content violates security policy. Category: {}, Action: {}", 
            assessment.category, assessment.action
        )));
    }
    
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(body_bytes))
        .unwrap())
}

async fn handle_streaming_generate(
    state: AppState,
    request: GenerateRequest,
) -> Result<Response<Body>, ApiError> {
    let stream = state.ollama_client.stream("/api/generate", &request).await?;
    
    let assessed_stream = SecurityAssessedStream::<_, crate::types::GenerateResponse>::new(
        stream, 
        state.security_client.clone(), 
        request.model.clone()
    );
    
    let stream_body = StreamBody::new(assessed_stream.map(|result| {
        match result {
            Ok(bytes) => Ok::<_, std::convert::Infallible>(bytes),
            Err(e) => {
                error!("Error in stream: {:?}", e);
                // In a real implementation, you'd want to handle this better
                Ok(Bytes::from(json!({ "error": "Stream processing error" }).to_string()))
            }
        }
    }));
    
    // Convert to the expected Body type
    let body = Body::from_stream(stream_body);
    
    let response = Response::builder()
        .header("Content-Type", "application/json")
        .body(body)
        .map_err(|e| ApiError::InternalError(format!("Failed to create response: {}", e)))?;
    
    Ok(response)
}
