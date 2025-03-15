mod config;
mod ollama;
mod security;
mod handlers;
mod types;
mod stream;

use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::handlers::*;
use crate::ollama::OllamaClient;
use crate::security::SecurityClient;

#[derive(Clone)]
pub struct AppState {
    ollama_client: OllamaClient,
    security_client: SecurityClient,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting Ollama Proxy");
    
    // Load configuration
    let config = config::load_config("config.yaml")?;
    
    // Create application state
    let state = AppState {
        ollama_client: OllamaClient::new(&config.ollama.base_url),
        security_client: SecurityClient::new(
            &config.security.base_url,
            &config.security.api_key,
            &config.security.profile_name,
            &config.security.app_name,
            &config.security.app_user,
        ),
    };
    
    // Build router with all the Ollama API endpoints
    let app = Router::new()
        .route("/api/generate", post(generate::handle_generate))
        .route("/api/chat", post(chat::handle_chat))
        .route("/api/tags", get(models::handle_list_models))
        .route("/api/show", post(models::handle_show_model))
        .route("/api/create", post(models::handle_create_model))
        .route("/api/copy", post(models::handle_copy_model))
        .route("/api/delete", post(models::handle_delete_model))
        .route("/api/pull", post(models::handle_pull_model))
        .route("/api/push", post(models::handle_push_model))
        .route("/api/embeddings", post(embeddings::handle_embeddings))
        .route("/api/version", get(version::handle_version))
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    
    // Start the server using the new Axum 0.7 API
    let addr = SocketAddr::new(
        IpAddr::from_str(&config.server.host)?, 
        config.server.port
    );
    info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
