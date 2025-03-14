use axum::{
    extract::State,
    response::Response,
    Json,
};
use tracing::debug;

use crate::AppState;
use crate::types::EmbeddingsRequest;
use crate::handlers::ApiError;

pub async fn handle_embeddings(
    State(state): State<AppState>,
    Json(request): Json<EmbeddingsRequest>,
) -> Result<Response, ApiError> {
    debug!("Received embeddings request for model: {}", request.model);
    
    // Assess the prompt with the updated method signature
    let assessment = state.security_client.assess_content(
        &request.prompt, 
        &request.model,
        true // This is a prompt
    ).await?;
    
    if !assessment.is_safe {
        return Err(ApiError::SecurityIssue(format!(
            "Embedding prompt violates security policy. Category: {}, Action: {}", 
            assessment.category, assessment.action
        )));
    }
    
    // Forward to Ollama
    let response = state.ollama_client.forward("/api/embeddings", &request).await?;
    let body = response.bytes().await.map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap())
}
