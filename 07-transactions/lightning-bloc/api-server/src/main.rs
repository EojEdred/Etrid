use axum::{
    extract::{Path, Query, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};
use std::net::SocketAddr;

mod handlers;
mod models;
mod state;
mod websocket;

use state::AppState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("lightning_bloc_api=debug,tower_http=debug")
        .init();

    info!("Starting Lightning-Bloc API Server");

    // Create shared application state
    let state = AppState::new();

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))

        // Lightning endpoints
        .route("/lightning/channels", get(handlers::get_channels))
        .route("/lightning/channels/open", post(handlers::open_channel))
        .route("/lightning/channels/:id/close", post(handlers::close_channel))

        .route("/lightning/payments", get(handlers::get_payments))
        .route("/lightning/route", post(handlers::find_route))
        .route("/lightning/send", post(handlers::send_payment))

        .route("/lightning/stats", get(handlers::get_stats))
        .route("/lightning/rates", get(handlers::get_exchange_rate))

        // WebSocket endpoint
        .route("/lightning/ws", get(websocket::ws_handler))

        // Add state and CORS
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 9944));
    info!("Lightning API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "lightning-bloc-api",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
