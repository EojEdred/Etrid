//! Distributed Peer Management (dpeers)
//!
//! Manages peer lifecycle, connections, discovery protocol, and metadata storage.
//! Implements connection pooling and peer state machines.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

/// Connection state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Authenticating,
    Authenticated,
    Idle,
    Unhealthy,
    Closing,
}

/// Peer connection information
#[derive(Debug, Clone)]
pub struct PeerConnection {
    pub peer_id: String,
    pub address: String,
    pub state: ConnectionState,
    pub connected_at: u64,
    pub last_activity: u64,
    pub message_count: u64,
    pub error_count: u32,
}

impl PeerConnection {
    pub fn new(peer_id: String, address: String) -> Self {
        let now = timestamp_secs();
        Self {
            peer_id,
            address,
            state: ConnectionState::Disconnected,
            connected_at: now,
            last_activity: now,
            message_count: 0,
            error_count: 0,
        }
    }

    /// Transition to new state
    pub fn transition(&mut self, new_state: ConnectionState) -> Result<(), String> {
        let valid = match (self.state, new_state) {
            (ConnectionState::Disconnected, ConnectionState::Connecting) => true,
            (ConnectionState::Connecting, ConnectionState::Connected) => true,
            (ConnectionState::Connecting, ConnectionState::Disconnected) => true,
            (ConnectionState::Connected, ConnectionState::Authenticating) => true,
            (ConnectionState::Authenticating, ConnectionState::Authenticated) => true,
            (ConnectionState::Authenticating, ConnectionState::Disconnected) => true,
            (ConnectionState::Authenticated, ConnectionState::Idle) => true,
            (ConnectionState::Idle, ConnectionState::Authenticated) => true,
            (ConnectionState::Idle, ConnectionState::Unhealthy) => true,
            (_, ConnectionState::Closing) => true,
            (ConnectionState::Closing, ConnectionState::Disconnected) => true,
            _ => false,
        };

        if valid {
            self.state = new_state;
            self.last_activity = timestamp_secs();
            Ok(())
        } else {
            Err(format!("Invalid transition: {:?} -> {:?}", self.state, new_state))
        }
    }

    pub fn record_message(&mut self) {
        self.message_count += 1;
        self.last_activity = timestamp_secs();
    }

    pub fn record_error(&mut self) {
        self.error_count += 1;
        self.last_activity = timestamp_secs();
        if self.error_count > 5 {
            let _ = self.transition(ConnectionState::Unhealthy);
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.state != ConnectionState::Unhealthy
            && self.state != ConnectionState::Disconnected
            && self.error_count < 5
    }

    pub fn uptime_secs(&self) -> u64 {
        timestamp_secs().saturating_sub(self.connected_at)
    }
}

/// Peer metadata
#[derive(Debug, Clone)]
pub struct PeerMetadata {
    pub peer_id: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub reputation: i32,
    pub first_seen: u64,
    pub last_updated: u64,
}

impl PeerMetadata {
    pub fn new(peer_id: String) -> Self {
        let now = timestamp_secs();
        Self {
            peer_id,
            version: "0.1.0".to_string(),
            capabilities: vec![],
            reputation: 0,
            first_seen: now,
            last_updated: now,
        }
    }

    pub fn update_reputation(&mut self, delta: i32) {
        self.reputation = (self.reputation + delta).clamp(-100, 100);
        self.last_updated = timestamp_secs();
    }
}

/// Peer registry and connection pool
pub struct PeerRegistry {
    connections: Arc<RwLock<HashMap<String, PeerConnection>>>,
    metadata: Arc<RwLock<HashMap<String, PeerMetadata>>>,
    max_connections: usize,
}

impl PeerRegistry {
    pub fn new(max_connections: usize) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
            max_connections,
        }
    }

    /// Register a new peer
    pub async fn register_peer(&self, peer_id: String, address: String) -> Result<(), String> {
        let mut conns = self.connections.write().await;
        
        if conns.len() >= self.max_connections {
            return Err("Connection pool at capacity".to_string());
        }

        if conns.contains_key(&peer_id) {
            return Err("Peer already registered".to_string());
        }

        conns.insert(peer_id.clone(), PeerConnection::new(peer_id.clone(), address));

        let mut meta = self.metadata.write().await;
        meta.insert(peer_id.clone(), PeerMetadata::new(peer_id));

        Ok(())
    }

    /// Unregister a peer
    pub async fn unregister_peer(&self, peer_id: &str) -> Result<(), String> {
        let mut conns = self.connections.write().await;
        conns.remove(peer_id).ok_or("Peer not found".to_string())?;

        let mut meta = self.metadata.write().await;
        meta.remove(peer_id);

        Ok(())
    }

    /// Update connection state
    pub async fn set_connection_state(
        &self,
        peer_id: &str,
        state: ConnectionState,
    ) -> Result<(), String> {
        let mut conns = self.connections.write().await;
        let conn = conns.get_mut(peer_id).ok_or("Peer not found")?;
        conn.transition(state)
    }

    /// Record message activity
    pub async fn record_message(&self, peer_id: &str) -> Result<(), String> {
        let mut conns = self.connections.write().await;
        let conn = conns.get_mut(peer_id).ok_or("Peer not found")?;
        conn.record_message();
        Ok(())
    }

    /// Record error activity
    pub async fn record_error(&self, peer_id: &str) -> Result<(), String> {
        let mut conns = self.connections.write().await;
        let conn = conns.get_mut(peer_id).ok_or("Peer not found")?;
        conn.record_error();
        Ok(())
    }

    /// Get peer connection
    pub async fn get_peer(&self, peer_id: &str) -> Option<PeerConnection> {
        let conns = self.connections.read().await;
        conns.get(peer_id).cloned()
    }

    /// Get all active peers
    pub async fn get_active_peers(&self) -> Vec<PeerConnection> {
        let conns = self.connections.read().await;
        conns
            .values()
            .filter(|c| c.state == ConnectionState::Authenticated)
            .cloned()
            .collect()
    }

    /// Get all unhealthy peers
    pub async fn get_unhealthy_peers(&self) -> Vec<PeerConnection> {
        let conns = self.connections.read().await;
        conns
            .values()
            .filter(|c| !c.is_healthy())
            .cloned()
            .collect()
    }

    /// Get peer count
    pub async fn peer_count(&self) -> usize {
        self.connections.read().await.len()
    }

    /// Get active peer count
    pub async fn active_peer_count(&self) -> usize {
        self.get_active_peers().await.len()
    }

    /// Update peer metadata
    pub async fn update_metadata(&self, peer_id: &str, version: String, capabilities: Vec<String>) -> Result<(), String> {
        let mut meta = self.metadata.write().await;
        let entry = meta.get_mut(peer_id).ok_or("Peer not found")?;
        entry.version = version;
        entry.capabilities = capabilities;
        entry.last_updated = timestamp_secs();
        Ok(())
    }

    /// Update peer reputation
    pub async fn update_reputation(&self, peer_id: &str, delta: i32) -> Result<(), String> {
        let mut meta = self.metadata.write().await;
        let entry = meta.get_mut(peer_id).ok_or("Peer not found")?;
        entry.update_reputation(delta);
        Ok(())
    }

    /// Get peer metadata
    pub async fn get_metadata(&self, peer_id: &str) -> Option<PeerMetadata> {
        let meta = self.metadata.read().await;
        meta.get(peer_id).cloned()
    }

    /// Cleanup disconnected peers
    pub async fn cleanup(&self, idle_secs: u64) -> usize {
        let mut conns = self.connections.write().await;
        let before = conns.len();
        let now = timestamp_secs();

        conns.retain(|_, conn| {
            !(conn.state == ConnectionState::Disconnected
                && now.saturating_sub(conn.last_activity) > idle_secs)
        });

        let mut meta = self.metadata.write().await;
        meta.retain(|k, _| conns.contains_key(k));

        before - conns.len()
    }

    /// Get registry stats
    pub async fn stats(&self) -> PeerRegistryStats {
        let conns = self.connections.read().await;
        let authenticated = conns.values().filter(|c| c.state == ConnectionState::Authenticated).count();
        let total_messages = conns.values().map(|c| c.message_count).sum();
        let total_errors = conns.values().map(|c| c.error_count as u64).sum();

        PeerRegistryStats {
            total_peers: conns.len(),
            authenticated_peers: authenticated,
            max_capacity: self.max_connections,
            total_messages,
            total_errors,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PeerRegistryStats {
    pub total_peers: usize,
    pub authenticated_peers: usize,
    pub max_capacity: usize,
    pub total_messages: u64,
    pub total_errors: u64,
}

/// Peer discovery protocol
pub struct DiscoveryProtocol {
    registry: Arc<PeerRegistry>,
}

impl DiscoveryProtocol {
    pub fn new(registry: Arc<PeerRegistry>) -> Self {
        Self { registry }
    }

    /// Initiate peer discovery
    pub async fn discover_peers(&self, bootstrap_nodes: Vec<(String, String)>) -> Result<usize, String> {
        let mut discovered = 0;

        for (peer_id, address) in bootstrap_nodes {
            match self.registry.register_peer(peer_id.clone(), address.clone()).await {
                Ok(_) => {
                    let _ = self.registry.set_connection_state(&peer_id, ConnectionState::Connecting).await;
                    discovered += 1;
                }
                Err(_) => {} // Already registered or pool full
            }
        }

        Ok(discovered)
    }

    /// Complete peer discovery handshake
    pub async fn complete_discovery(&self, peer_id: &str) -> Result<(), String> {
        self.registry.set_connection_state(peer_id, ConnectionState::Authenticating).await?;
        self.registry.set_connection_state(peer_id, ConnectionState::Authenticated).await
    }
}

fn timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_connection_creation() {
        let conn = PeerConnection::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        assert_eq!(conn.state, ConnectionState::Disconnected);
        assert_eq!(conn.message_count, 0);
    }

    #[test]
    fn test_connection_state_transition() {
        let mut conn = PeerConnection::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        assert!(conn.transition(ConnectionState::Connecting).is_ok());
        assert!(conn.transition(ConnectionState::Connected).is_ok());
        assert!(conn.transition(ConnectionState::Authenticating).is_ok());
    }

    #[test]
    fn test_invalid_state_transition() {
        let mut conn = PeerConnection::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        assert!(conn.transition(ConnectionState::Authenticated).is_err());
    }

    #[test]
    fn test_record_message() {
        let mut conn = PeerConnection::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        conn.record_message();
        assert_eq!(conn.message_count, 1);
    }

    #[test]
    fn test_record_error() {
        let mut conn = PeerConnection::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        conn.record_error();
        assert_eq!(conn.error_count, 1);
    }

    #[test]
    fn test_health_check() {
        let conn = PeerConnection::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        assert!(!conn.is_healthy()); // Disconnected = unhealthy
    }

    #[tokio::test]
    async fn test_registry_register_peer() {
        let registry = PeerRegistry::new(100);
        assert!(registry.register_peer("peer1".to_string(), "127.0.0.1:8001".to_string()).await.is_ok());
        assert_eq!(registry.peer_count().await, 1);
    }

    #[tokio::test]
    async fn test_registry_duplicate_peer() {
        let registry = PeerRegistry::new(100);
        registry.register_peer("peer1".to_string(), "127.0.0.1:8001".to_string()).await.unwrap();
        let result = registry.register_peer("peer1".to_string(), "127.0.0.1:8001".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_registry_max_capacity() {
        let registry = PeerRegistry::new(2);
        registry.register_peer("peer1".to_string(), "127.0.0.1:8001".to_string()).await.unwrap();
        registry.register_peer("peer2".to_string(), "127.0.0.1:8002".to_string()).await.unwrap();

        let result = registry.register_peer("peer3".to_string(), "127.0.0.1:8003".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_registry_unregister() {
        let registry = PeerRegistry::new(100);
        registry.register_peer("peer1".to_string(), "127.0.0.1:8001".to_string()).await.unwrap();
        assert!(registry.unregister_peer("peer1").await.is_ok());
        assert_eq!(registry.peer_count().await, 0);
    }

    #[tokio::test]
    async fn test_set_connection_state() {
        let registry = PeerRegistry::new(100);
        registry.register_peer("peer1".to_string(), "127.0.0.1:8001".to_string()).await.unwrap();
        assert!(registry.set_connection_state("peer1", ConnectionState::Connecting).await.is_ok());

        let peer = registry.get_peer("peer1").await.unwrap();
        assert_eq!(peer.state, ConnectionState::Connecting);
    }

    #[tokio::test]
    async fn test_record_message_activity() {
        let registry = PeerRegistry::new(100);
        registry.register_peer("peer1".to_string(), "127.0.0.1:8001".to_string()).await.unwrap();
        registry.record_message("peer1").await.unwrap();

        let peer = registry.get_peer("peer1").await.unwrap();
        assert_eq!(peer.message_count, 1);
    }

    #[tokio::test]
    async fn test_get_active_peers() {
        let registry = PeerRegistry::new(100);
        registry.register_peer("peer1".to_string(), "127.0.0.1:8001".to_string()).await.unwrap();
        registry.register_peer("peer2".to_string(), "127.0.0.1:8002".to_string()).await.unwrap();

        registry.set_connection_state("peer1", ConnectionState::Authenticated).await.unwrap();
        let active = registry.get_active_peers().await;
        assert_eq!(active.len(), 1);
    }

    #[tokio::test]
    async fn test_peer_metadata() {
        let registry = PeerRegistry::new(100);
        registry.register_peer("peer1".to_string(), "127.0.0.1:8001".to_string()).await.unwrap();
        registry.update_metadata("peer1", "1.0.0".to_string(), vec!["relay".to_string()]).await.unwrap();

        let meta = registry.get_metadata("peer1").await.unwrap();
        assert_eq!(meta.version, "1.0.0");
        assert_eq!(meta.capabilities, vec!["relay"]);
    }

    #[tokio::test]
    async fn test_reputation() {
        let registry = PeerRegistry::new(100);
        registry.register_peer("peer1".to_string(), "127.0.0.1:8001".to_string()).await.unwrap();
        registry.update_reputation("peer1", 10).await.unwrap();

        let meta = registry.get_metadata("peer1").await.unwrap();
        assert_eq!(meta.reputation, 10);
    }

    #[tokio::test]
    async fn test_cleanup() {
        let registry = PeerRegistry::new(100);
        registry.register_peer("peer1".to_string(), "127.0.0.1:8001".to_string()).await.unwrap();
        let removed = registry.cleanup(0).await; // Aggressive cleanup
        assert_eq!(removed, 1);
    }

    #[tokio::test]
    async fn test_stats() {
        let registry = PeerRegistry::new(100);
        registry.register_peer("peer1".to_string(), "127.0.0.1:8001".to_string()).await.unwrap();
        registry.set_connection_state("peer1", ConnectionState::Authenticated).await.unwrap();

        let stats = registry.stats().await;
        assert_eq!(stats.total_peers, 1);
        assert_eq!(stats.authenticated_peers, 1);
    }

    #[tokio::test]
    async fn test_discovery_protocol() {
        let registry = Arc::new(PeerRegistry::new(100));
        let discovery = DiscoveryProtocol::new(registry.clone());

        let discovered = discovery.discover_peers(vec![
            ("peer1".to_string(), "127.0.0.1:8001".to_string()),
            ("peer2".to_string(), "127.0.0.1:8002".to_string()),
        ]).await.unwrap();

        assert_eq!(discovered, 2);
    }
}
