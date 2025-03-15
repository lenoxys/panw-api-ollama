mod config;
mod handlers;
mod ollama;
mod security;
mod stream;
mod types;

use std::net::{IpAddr, SocketAddr};
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

impl AppState {
    pub fn builder() -> AppStateBuilder {
        AppStateBuilder::default()
    }
}

#[derive(Default)]
pub struct AppStateBuilder {
    ollama_client: Option<OllamaClient>,
    security_client: Option<SecurityClient>,
}

impl AppStateBuilder {
    pub fn with_ollama_client(mut self, client: OllamaClient) -> Self {
        self.ollama_client = Some(client);
        self
    }

    pub fn with_security_client(mut self, client: SecurityClient) -> Self {
        self.security_client = Some(client);
        self
    }

    pub fn build(self) -> Result<AppState, &'static str> {
        let ollama_client = self.ollama_client.ok_or("OllamaClient is required")?;
        let security_client = self.security_client.ok_or("SecurityClient is required")?;

        Ok(AppState {
            ollama_client,
            security_client,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting panw-api-ollama server");

    // Load configuration
    let config = config::load_config("config.yaml").map_err(|e| {
        eprintln!("Failed to load configuration: {}", e);
        e
    })?;

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
    let addr = SocketAddr::new(IpAddr::from_str(&config.server.host)?, config.server.port);
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
