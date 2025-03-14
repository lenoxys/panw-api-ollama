use axum::{
    extract::State,
    response::Response,
};
use tracing::debug;

use crate::AppState;
use crate::handlers::ApiError;

pub async fn handle_version(
    State(state): State<AppState>,
) -> Result<Response, ApiError> {
    debug!("Forwarding version request");
    let response = state.ollama_client.forward_get("/api/version").await?;
    let body = response.bytes().await.map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap())
}
