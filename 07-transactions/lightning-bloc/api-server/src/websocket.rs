use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::state::AppState;

/// WebSocket handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Handle individual WebSocket connection
async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    // Create channel for this connection
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Add connection to state
    state.add_ws_connection(tx).await;

    info!("New WebSocket connection established");

    // Send initial welcome message
    let welcome = serde_json::json!({
        "type": "connected",
        "message": "Connected to Lightning-Bloc API",
    });

    if let Err(e) = sender.send(axum::extract::ws::Message::Text(welcome.to_string())).await {
        warn!("Failed to send welcome message: {}", e);
        return;
    }

    // Spawn task to handle outgoing messages
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(axum::extract::ws::Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                axum::extract::ws::Message::Text(text) => {
                    info!("Received message: {}", text);
                    // Handle incoming messages (ping, subscribe, etc.)
                }
                axum::extract::ws::Message::Close(_) => {
                    info!("WebSocket connection closed");
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    info!("WebSocket connection terminated");
}
