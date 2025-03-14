use axum::{
    extract::State,
    response::Response,
    Json,
};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use tracing::debug;

use crate::AppState;
use crate::handlers::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

// Simple forward for model-related endpoints (no content to assess)
pub async fn handle_list_models(
    State(state): State<AppState>,
) -> Result<Response, ApiError> {
    debug!("Forwarding list models request");
    let response = state.ollama_client.forward_get("/api/tags").await?;
    let body = response.bytes().await.map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap())
}

pub async fn handle_show_model(
    State(state): State<AppState>,
    Json(request): Json<ModelRequest>,
) -> Result<Response, ApiError> {
    debug!("Forwarding show model request for: {}", request.name);
    let response = state.ollama_client.forward("/api/show", &request).await?;
    let body = response.bytes().await.map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap())
}

pub async fn handle_create_model(
    State(state): State<AppState>,
    Json(request): Json<Value>,
) -> Result<Response, ApiError> {
    debug!("Forwarding create model request");
    let response = state.ollama_client.forward("/api/create", &request).await?;
    let body = response.bytes().await.map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap())
}

pub async fn handle_copy_model(
    State(state): State<AppState>,
    Json(request): Json<Value>,
) -> Result<Response, ApiError> {
    debug!("Forwarding copy model request");
    let response = state.ollama_client.forward("/api/copy", &request).await?;
    let body = response.bytes().await.map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap())
}

pub async fn handle_delete_model(
    State(state): State<AppState>,
    Json(request): Json<ModelRequest>,
) -> Result<Response, ApiError> {
    debug!("Forwarding delete model request for: {}", request.name);
    let response = state.ollama_client.forward("/api/delete", &request).await?;
    let body = response.bytes().await.map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap())
}

pub async fn handle_pull_model(
    State(state): State<AppState>,
    Json(request): Json<ModelRequest>,
) -> Result<Response, ApiError> {
    debug!("Forwarding pull model request for: {}", request.name);
    let response = state.ollama_client.forward("/api/pull", &request).await?;
    let body = response.bytes().await.map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap())
}

pub async fn handle_push_model(
    State(state): State<AppState>,
    Json(request): Json<ModelRequest>,
) -> Result<Response, ApiError> {
    debug!("Forwarding push model request for: {}", request.name);
    let response = state.ollama_client.forward("/api/push", &request).await?;
    let body = response.bytes().await.map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap())
}
