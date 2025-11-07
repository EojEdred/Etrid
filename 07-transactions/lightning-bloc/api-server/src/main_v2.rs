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
mod handlers_v2;
mod models;
mod state;
mod state_v2;
mod websocket;
mod lightning_service;
mod auth;

use state_v2::AppStateV2;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("lightning_bloc_api=debug,tower_http=debug")
        .init();

    info!("Starting Lightning-Bloc API Server v2");
    info!("Integrated with Lightning-Bloc Core Modules");

    // Create shared application state with Lightning service
    let state = AppStateV2::new();

    // Build router with Lightning-integrated handlers
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))

        // Lightning endpoints (v2 with Lightning service)
        .route("/lightning/channels", get(handlers_v2::get_channels_v2))
        .route("/lightning/channels/open", post(handlers_v2::open_channel_v2))
        .route("/lightning/channels/:id/close", post(handlers_v2::close_channel_v2))

        .route("/lightning/payments", get(get_payments))
        .route("/lightning/route", post(handlers_v2::find_route_v2))
        .route("/lightning/send", post(handlers_v2::send_payment_v2))

        .route("/lightning/stats", get(get_stats))
        .route("/lightning/rates", get(handlers_v2::get_exchange_rate_v2))

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
    info!("WebSocket endpoint: ws://{}/lightning/ws", addr);
    info!("Health check: http://{}/health", addr);

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
        "features": ["cross-pbc-routing", "oracle-integration", "websocket-events"],
    }))
}

async fn get_payments(
    State(state): State<AppStateV2>,
) -> Result<Json<models::PaymentsResponse>, Response> {
    let payments = state.get_payments().await;
    Ok(Json(models::PaymentsResponse { payments }))
}

async fn get_stats(
    State(state): State<AppStateV2>,
) -> Result<Json<models::StatsResponse>, Response> {
    let stats = state.get_stats().await;
    Ok(Json(models::StatsResponse { stats }))
}
