use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

use crate::models::{Channel, Payment, NetworkStats};
use crate::lightning_service::LightningService;

/// Shared application state (v2 with Lightning service)
#[derive(Clone)]
pub struct AppStateV2 {
    pub inner: Arc<RwLock<AppStateInner>>,
    pub lightning: Arc<LightningService>,
}

pub struct AppStateInner {
    /// In-memory payment storage
    pub payments: Vec<Payment>,

    /// Network statistics
    pub stats: NetworkStats,

    /// WebSocket connections
    pub ws_connections: Vec<tokio::sync::mpsc::UnboundedSender<String>>,
}

impl AppStateV2 {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(AppStateInner {
                payments: Vec::new(),
                stats: NetworkStats {
                    total_channels: 0,
                    total_capacity: "0".to_string(),
                    average_channel_size: "0".to_string(),
                    active_chains: 14,
                    recent_payments: 0,
                    success_rate: 99.2,
                },
                ws_connections: Vec::new(),
            })),
            lightning: Arc::new(LightningService::new()),
        }
    }

    /// Get Lightning service
    pub fn lightning_service(&self) -> Arc<LightningService> {
        self.lightning.clone()
    }

    /// Add a payment
    pub async fn add_payment(&self, payment: Payment) {
        let mut state = self.inner.write().await;
        state.payments.push(payment);
        state.stats.recent_payments = state.payments.len() as u64;
    }

    /// Get all payments
    pub async fn get_payments(&self) -> Vec<Payment> {
        let state = self.inner.read().await;
        state.payments.clone()
    }

    /// Get network stats
    pub async fn get_stats(&self) -> NetworkStats {
        let mut state = self.inner.write().await;

        // Update stats from Lightning service
        let channels = self.lightning.get_channels().await;
        state.stats.total_channels = channels.len() as u64;

        state.stats.clone()
    }

    /// Broadcast event to all WebSocket connections
    pub async fn broadcast_event(&self, event: String) {
        let mut state = self.inner.write().await;

        // Remove closed connections
        state.ws_connections.retain(|tx| !tx.is_closed());

        // Broadcast to all active connections
        for tx in &state.ws_connections {
            let _ = tx.send(event.clone());
        }
    }

    /// Add WebSocket connection
    pub async fn add_ws_connection(&self, tx: tokio::sync::mpsc::UnboundedSender<String>) {
        let mut state = self.inner.write().await;
        state.ws_connections.push(tx);
    }
}

impl Default for AppStateV2 {
    fn default() -> Self {
        Self::new()
    }
}
