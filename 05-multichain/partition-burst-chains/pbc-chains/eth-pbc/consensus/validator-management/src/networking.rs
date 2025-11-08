//! # Validator Networking
//!
//! This module handles validator networking, peering, and communication
//! for the consensus protocol.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use codec::{Decode, Encode};

use crate::{ValidatorError, ValidatorId, ValidatorResult};

// ═══════════════════════════════════════════════════════════════════════════════
// PEER CONNECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Connection status for a peer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum ConnectionStatus {
    /// Not connected
    Disconnected,
    /// Connection in progress
    Connecting,
    /// Successfully connected
    Connected,
    /// Connection failed
    Failed,
}

/// Peer connection information
#[derive(Debug, Clone)]
pub struct PeerConnection {
    /// Validator ID
    pub validator: ValidatorId,

    /// Connection status
    pub status: ConnectionStatus,

    /// Last successful connection timestamp
    pub last_connected: u64,

    /// Connection attempt count
    pub attempts: u32,

    /// Network latency (milliseconds)
    pub latency_ms: Option<u64>,
}

impl PeerConnection {
    /// Create a new peer connection
    pub fn new(validator: ValidatorId) -> Self {
        Self {
            validator,
            status: ConnectionStatus::Disconnected,
            last_connected: 0,
            attempts: 0,
            latency_ms: None,
        }
    }

    /// Mark as connected
    pub fn mark_connected(&mut self, timestamp: u64, latency: u64) {
        self.status = ConnectionStatus::Connected;
        self.last_connected = timestamp;
        self.latency_ms = Some(latency);
    }

    /// Mark as disconnected
    pub fn mark_disconnected(&mut self) {
        self.status = ConnectionStatus::Disconnected;
        self.latency_ms = None;
    }

    /// Record connection attempt
    pub fn record_attempt(&mut self) {
        self.attempts += 1;
        self.status = ConnectionStatus::Connecting;
    }

    /// Mark connection as failed
    pub fn mark_failed(&mut self) {
        self.status = ConnectionStatus::Failed;
    }

    /// Check if currently connected
    pub fn is_connected(&self) -> bool {
        self.status == ConnectionStatus::Connected
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// NETWORK MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Manages validator network connections
#[derive(Debug, Clone)]
pub struct NetworkManager {
    /// All peer connections
    peers: BTreeMap<ValidatorId, PeerConnection>,

    /// Maximum peers to connect to
    max_peers: usize,

    /// Connection timeout (milliseconds)
    connection_timeout: u64,

    /// Maximum connection attempts before giving up
    max_attempts: u32,
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new(max_peers: usize, connection_timeout: u64) -> Self {
        Self {
            peers: BTreeMap::new(),
            max_peers,
            connection_timeout,
            max_attempts: 5,
        }
    }

    /// Add a peer to track
    pub fn add_peer(&mut self, validator: ValidatorId) -> ValidatorResult<()> {
        if self.peers.len() >= self.max_peers {
            return Err(ValidatorError::NetworkError("Max peers reached"));
        }

        let connection = PeerConnection::new(validator.clone());
        self.peers.insert(validator, connection);
        Ok(())
    }

    /// Remove a peer
    pub fn remove_peer(&mut self, validator: &ValidatorId) {
        self.peers.remove(validator);
    }

    /// Get peer connection info
    pub fn get_peer(&self, validator: &ValidatorId) -> Option<&PeerConnection> {
        self.peers.get(validator)
    }

    /// Get mutable peer connection
    pub fn get_peer_mut(&mut self, validator: &ValidatorId) -> Option<&mut PeerConnection> {
        self.peers.get_mut(validator)
    }

    /// Mark peer as connected
    pub fn mark_connected(
        &mut self,
        validator: &ValidatorId,
        timestamp: u64,
        latency: u64,
    ) -> ValidatorResult<()> {
        self.peers
            .get_mut(validator)
            .ok_or(ValidatorError::NotFound)?
            .mark_connected(timestamp, latency);
        Ok(())
    }

    /// Mark peer as disconnected
    pub fn mark_disconnected(&mut self, validator: &ValidatorId) -> ValidatorResult<()> {
        self.peers
            .get_mut(validator)
            .ok_or(ValidatorError::NotFound)?
            .mark_disconnected();
        Ok(())
    }

    /// Record connection attempt
    pub fn record_attempt(&mut self, validator: &ValidatorId) -> ValidatorResult<()> {
        self.peers
            .get_mut(validator)
            .ok_or(ValidatorError::NotFound)?
            .record_attempt();
        Ok(())
    }

    /// Get connected peers
    pub fn connected_peers(&self) -> Vec<ValidatorId> {
        self.peers
            .iter()
            .filter(|(_, conn)| conn.is_connected())
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Get connection count
    pub fn connection_count(&self) -> usize {
        self.peers.values().filter(|c| c.is_connected()).count()
    }

    /// Get average latency
    pub fn average_latency(&self) -> Option<u64> {
        let latencies: Vec<u64> = self
            .peers
            .values()
            .filter_map(|p| p.latency_ms)
            .collect();

        if latencies.is_empty() {
            return None;
        }

        let sum: u64 = latencies.iter().sum();
        Some(sum / latencies.len() as u64)
    }

    /// Get peers that need connection retry
    pub fn needs_retry(&self) -> Vec<ValidatorId> {
        self.peers
            .iter()
            .filter(|(_, conn)| {
                matches!(conn.status, ConnectionStatus::Failed | ConnectionStatus::Disconnected)
                    && conn.attempts < self.max_attempts
            })
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Check network health (percentage of connected peers)
    pub fn network_health(&self) -> u8 {
        if self.peers.is_empty() {
            return 100;
        }

        let connected = self.connection_count();
        let total = self.peers.len();
        ((connected * 100) / total) as u8
    }

    /// Get network statistics
    pub fn network_stats(&self) -> NetworkStats {
        NetworkStats {
            total_peers: self.peers.len(),
            connected_peers: self.connection_count(),
            average_latency: self.average_latency(),
            health_score: self.network_health(),
        }
    }

    /// Clear all connections
    pub fn clear(&mut self) {
        self.peers.clear();
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self::new(100, 30000) // 100 max peers, 30s timeout
    }
}

/// Type alias for use in coordinator
pub type NetworkCoordinator = NetworkManager;

// ═══════════════════════════════════════════════════════════════════════════════
// NETWORK STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════

/// Network health and statistics
#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    /// Total peers tracked
    pub total_peers: usize,

    /// Currently connected peers
    pub connected_peers: usize,

    /// Average network latency (ms)
    pub average_latency: Option<u64>,

    /// Network health score (0-100)
    pub health_score: u8,
}

impl NetworkStats {
    /// Calculate connection ratio
    pub fn connection_ratio(&self) -> f64 {
        if self.total_peers == 0 {
            return 1.0;
        }
        self.connected_peers as f64 / self.total_peers as f64
    }

    /// Check if network is healthy (>= 66% connected)
    pub fn is_healthy(&self) -> bool {
        self.health_score >= 66
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MESSAGE TYPES (for future use with libp2p)
// ═══════════════════════════════════════════════════════════════════════════════

/// Network message types
#[derive(Debug, Clone, Encode, Decode)]
pub enum NetworkMessage {
    /// Ping message
    Ping,

    /// Pong response
    Pong,

    /// Request validator info
    RequestValidatorInfo,

    /// Response with validator info
    ValidatorInfo(Vec<u8>), // Encoded ValidatorInfo

    /// Consensus vote
    Vote(Vec<u8>), // Encoded Vote

    /// Validity certificate
    Certificate(Vec<u8>), // Encoded Certificate

    /// State sync request
    StateSyncRequest { from_block: u64 },

    /// State sync response
    StateSyncResponse { data: Vec<u8> },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_validator(id: u8) -> ValidatorId {
        ValidatorId::from([id; 32])
    }

    #[test]
    fn test_peer_connection_creation() {
        let validator = create_test_validator(1);
        let conn = PeerConnection::new(validator.clone());

        assert_eq!(conn.validator, validator);
        assert_eq!(conn.status, ConnectionStatus::Disconnected);
        assert!(!conn.is_connected());
    }

    #[test]
    fn test_peer_connection_lifecycle() {
        let validator = create_test_validator(1);
        let mut conn = PeerConnection::new(validator);

        // Connect
        conn.mark_connected(1000, 50);
        assert_eq!(conn.status, ConnectionStatus::Connected);
        assert!(conn.is_connected());
        assert_eq!(conn.latency_ms, Some(50));

        // Disconnect
        conn.mark_disconnected();
        assert_eq!(conn.status, ConnectionStatus::Disconnected);
        assert!(!conn.is_connected());
        assert_eq!(conn.latency_ms, None);
    }

    #[test]
    fn test_network_manager_creation() {
        let manager = NetworkManager::new(100, 30000);
        assert_eq!(manager.max_peers, 100);
        assert_eq!(manager.connection_count(), 0);
    }

    #[test]
    fn test_add_peer() {
        let mut manager = NetworkManager::new(100, 30000);
        let validator = create_test_validator(1);

        assert!(manager.add_peer(validator.clone()).is_ok());
        assert!(manager.get_peer(&validator).is_some());
    }

    #[test]
    fn test_max_peers_limit() {
        let mut manager = NetworkManager::new(2, 30000);

        assert!(manager.add_peer(create_test_validator(1)).is_ok());
        assert!(manager.add_peer(create_test_validator(2)).is_ok());
        assert!(manager.add_peer(create_test_validator(3)).is_err()); // Should fail
    }

    #[test]
    fn test_peer_connection_tracking() {
        let mut manager = NetworkManager::new(100, 30000);
        let validator = create_test_validator(1);

        manager.add_peer(validator.clone()).unwrap();

        assert_eq!(manager.connection_count(), 0);

        manager.mark_connected(&validator, 1000, 50).unwrap();
        assert_eq!(manager.connection_count(), 1);

        manager.mark_disconnected(&validator).unwrap();
        assert_eq!(manager.connection_count(), 0);
    }

    #[test]
    fn test_connected_peers() {
        let mut manager = NetworkManager::new(100, 30000);

        let v1 = create_test_validator(1);
        let v2 = create_test_validator(2);
        let v3 = create_test_validator(3);

        manager.add_peer(v1.clone()).unwrap();
        manager.add_peer(v2.clone()).unwrap();
        manager.add_peer(v3.clone()).unwrap();

        manager.mark_connected(&v1, 1000, 50).unwrap();
        manager.mark_connected(&v3, 1000, 50).unwrap();

        let connected = manager.connected_peers();
        assert_eq!(connected.len(), 2);
        assert!(connected.contains(&v1));
        assert!(connected.contains(&v3));
    }

    #[test]
    fn test_average_latency() {
        let mut manager = NetworkManager::new(100, 30000);

        let v1 = create_test_validator(1);
        let v2 = create_test_validator(2);
        let v3 = create_test_validator(3);

        manager.add_peer(v1.clone()).unwrap();
        manager.add_peer(v2.clone()).unwrap();
        manager.add_peer(v3.clone()).unwrap();

        manager.mark_connected(&v1, 1000, 50).unwrap();
        manager.mark_connected(&v2, 1000, 100).unwrap();
        manager.mark_connected(&v3, 1000, 150).unwrap();

        assert_eq!(manager.average_latency(), Some(100)); // (50 + 100 + 150) / 3
    }

    #[test]
    fn test_network_health() {
        let mut manager = NetworkManager::new(100, 30000);

        for i in 0..10 {
            manager.add_peer(create_test_validator(i)).unwrap();
        }

        // Connect 7 out of 10 (70%)
        for i in 0..7 {
            manager.mark_connected(&create_test_validator(i), 1000, 50).unwrap();
        }

        assert_eq!(manager.network_health(), 70);
    }

    #[test]
    fn test_network_stats() {
        let mut manager = NetworkManager::new(100, 30000);

        for i in 0..5 {
            manager.add_peer(create_test_validator(i)).unwrap();
        }

        for i in 0..3 {
            manager.mark_connected(&create_test_validator(i), 1000, 50).unwrap();
        }

        let stats = manager.network_stats();
        assert_eq!(stats.total_peers, 5);
        assert_eq!(stats.connected_peers, 3);
        assert_eq!(stats.health_score, 60); // 3/5 = 60%
    }

    #[test]
    fn test_needs_retry() {
        let mut manager = NetworkManager::new(100, 30000);
        let validator = create_test_validator(1);

        manager.add_peer(validator.clone()).unwrap();

        // Initially should need retry
        let needs = manager.needs_retry();
        assert_eq!(needs.len(), 1);

        // After max attempts, should not need retry
        for _ in 0..5 {
            manager.record_attempt(&validator).unwrap();
        }

        let needs = manager.needs_retry();
        assert_eq!(needs.len(), 0);
    }
}
