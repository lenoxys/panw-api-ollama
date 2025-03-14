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

// Fonction générique pour gérer les requêtes de transfert POST avec un corps
async fn forward_request<T: Serialize>(
    state: &AppState,
    endpoint: &str,
    body: &T,
    log_message: &str,
) -> Result<Response, ApiError> {
    debug!("{}", log_message);
    let response = state.ollama_client.forward(endpoint, body).await?;
    let body = response.bytes().await.map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap())
}

// Fonction générique pour gérer les requêtes GET
async fn forward_get_request(
    state: &AppState,
    endpoint: &str,
    log_message: &str,
) -> Result<Response, ApiError> {
    debug!("{}", log_message);
    let response = state.ollama_client.forward_get(endpoint).await?;
    let body = response.bytes().await.map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap())
}

// Implémentation des points de terminaison avec les fonctions génériques
pub async fn handle_list_models(
    State(state): State<AppState>,
) -> Result<Response, ApiError> {
    forward_get_request(&state, "/api/tags", "Forwarding list models request").await
}

pub async fn handle_show_model(
    State(state): State<AppState>,
    Json(request): Json<ModelRequest>,
) -> Result<Response, ApiError> {
    let log_message = format!("Forwarding show model request for: {}", request.name);
    forward_request(&state, "/api/show", &request, &log_message).await
}

pub async fn handle_create_model(
    State(state): State<AppState>,
    Json(request): Json<Value>,
) -> Result<Response, ApiError> {
    forward_request(&state, "/api/create", &request, "Forwarding create model request").await
}

pub async fn handle_copy_model(
    State(state): State<AppState>,
    Json(request): Json<Value>,
) -> Result<Response, ApiError> {
    forward_request(&state, "/api/copy", &request, "Forwarding copy model request").await
}

pub async fn handle_delete_model(
    State(state): State<AppState>,
    Json(request): Json<ModelRequest>,
) -> Result<Response, ApiError> {
    let log_message = format!("Forwarding delete model request for: {}", request.name);
    forward_request(&state, "/api/delete", &request, &log_message).await
}

pub async fn handle_pull_model(
    State(state): State<AppState>,
    Json(request): Json<ModelRequest>,
) -> Result<Response, ApiError> {
    let log_message = format!("Forwarding pull model request for: {}", request.name);
    forward_request(&state, "/api/pull", &request, &log_message).await
}

pub async fn handle_push_model(
    State(state): State<AppState>,
    Json(request): Json<ModelRequest>,
) -> Result<Response, ApiError> {
    let log_message = format!("Forwarding push model request for: {}", request.name);
    forward_request(&state, "/api/push", &request, &log_message).await
}