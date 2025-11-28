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

// ═══════════════════════════════════════════════════════════════════════════════
// AUTHENTICATED MESSAGES (PRODUCTION SECURITY)
// ═══════════════════════════════════════════════════════════════════════════════

/// Authenticated message wrapper with cryptographic signature
#[derive(Debug, Clone, Encode, Decode)]
pub struct AuthenticatedMessage {
    /// The actual message payload
    pub payload: NetworkMessage,

    /// Sender's validator ID
    pub sender: ValidatorId,

    /// Message sequence number (for replay protection)
    pub sequence: u64,

    /// Timestamp (milliseconds since epoch)
    pub timestamp: u64,

    /// Cryptographic signature over (payload, sender, sequence, timestamp)
    pub signature: Vec<u8>, // Encoded asf_algorithm::crypto::Signature
}

impl AuthenticatedMessage {
    /// Create a new authenticated message
    pub fn new(
        payload: NetworkMessage,
        sender: ValidatorId,
        sequence: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            payload,
            sender,
            sequence,
            timestamp,
            signature: Vec::new(), // Will be filled by sign()
        }
    }

    /// Get the message to be signed
    pub fn signing_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.payload.encode());
        bytes.extend_from_slice(&self.sender.encode());
        bytes.extend_from_slice(&self.sequence.to_le_bytes());
        bytes.extend_from_slice(&self.timestamp.to_le_bytes());
        bytes
    }

    /// Sign the message (must be done externally with keypair)
    pub fn attach_signature(&mut self, signature_bytes: Vec<u8>) {
        self.signature = signature_bytes;
    }

    /// Verify message authenticity and timestamp
    pub fn verify(&self, current_time: u64) -> ValidatorResult<()> {
        // Check signature is present
        if self.signature.is_empty() {
            return Err(ValidatorError::NetworkError("Missing signature"));
        }

        // Check timestamp is not too old (prevent replay attacks)
        const MAX_MESSAGE_AGE_MS: u64 = 30_000; // 30 seconds
        if self.timestamp < current_time.saturating_sub(MAX_MESSAGE_AGE_MS) {
            return Err(ValidatorError::NetworkError("Message too old"));
        }

        // Check timestamp is not from future (clock drift tolerance)
        const MAX_CLOCK_DRIFT_MS: u64 = 5_000; // 5 seconds
        if self.timestamp > current_time + MAX_CLOCK_DRIFT_MS {
            return Err(ValidatorError::NetworkError("Message from future"));
        }

        // NOTE: Actual signature verification done externally with crypto module
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// RATE LIMITING (DDoS PROTECTION)
// ═══════════════════════════════════════════════════════════════════════════════

/// Token bucket for rate limiting
#[derive(Debug, Clone)]
pub struct TokenBucket {
    /// Current tokens available
    tokens: f64,

    /// Maximum tokens (capacity)
    capacity: f64,

    /// Tokens per second refill rate
    refill_rate: f64,

    /// Last refill timestamp
    last_refill: u64,
}

impl TokenBucket {
    /// Create a new token bucket
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            tokens: capacity,
            capacity,
            refill_rate,
            last_refill: 0,
        }
    }

    /// Refill tokens based on elapsed time
    pub fn refill(&mut self, current_time: u64) {
        if self.last_refill == 0 {
            self.last_refill = current_time;
            return;
        }

        let elapsed_ms = current_time.saturating_sub(self.last_refill);
        let elapsed_seconds = elapsed_ms as f64 / 1000.0;
        let new_tokens = elapsed_seconds * self.refill_rate;

        self.tokens = (self.tokens + new_tokens).min(self.capacity);
        self.last_refill = current_time;
    }

    /// Try to consume tokens
    pub fn consume(&mut self, amount: f64, current_time: u64) -> bool {
        self.refill(current_time);

        if self.tokens >= amount {
            self.tokens -= amount;
            true
        } else {
            false
        }
    }

    /// Get current token count
    pub fn available(&self) -> f64 {
        self.tokens
    }
}

/// Rate limiter for network messages
#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Per-validator token buckets
    buckets: BTreeMap<ValidatorId, TokenBucket>,

    /// Messages per second limit per validator
    messages_per_second: f64,

    /// Burst capacity (max messages in bucket)
    burst_capacity: f64,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(messages_per_second: f64, burst_capacity: f64) -> Self {
        Self {
            buckets: BTreeMap::new(),
            messages_per_second,
            burst_capacity,
        }
    }

    /// Check if message is allowed (consume 1 token)
    pub fn check_rate_limit(
        &mut self,
        validator: &ValidatorId,
        current_time: u64,
    ) -> ValidatorResult<()> {
        let bucket = self.buckets.entry(validator.clone()).or_insert_with(|| {
            TokenBucket::new(self.burst_capacity, self.messages_per_second)
        });

        if bucket.consume(1.0, current_time) {
            Ok(())
        } else {
            Err(ValidatorError::NetworkError("Rate limit exceeded"))
        }
    }

    /// Get current available tokens for a validator
    pub fn available_tokens(&self, validator: &ValidatorId) -> f64 {
        self.buckets
            .get(validator)
            .map(|b| b.available())
            .unwrap_or(self.burst_capacity)
    }

    /// Clear old buckets (cleanup)
    pub fn cleanup(&mut self) {
        // Remove buckets with full tokens (inactive validators)
        self.buckets
            .retain(|_, bucket| bucket.available() < bucket.capacity);
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        // Default: 10 messages/second, burst of 20
        Self::new(10.0, 20.0)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// REPLAY ATTACK PROTECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Tracks seen message sequences to prevent replay attacks
#[derive(Debug, Clone)]
pub struct ReplayProtector {
    /// Last seen sequence number per validator
    last_sequences: BTreeMap<ValidatorId, u64>,

    /// Maximum sequence number gap allowed
    max_gap: u64,
}

impl ReplayProtector {
    /// Create a new replay protector
    pub fn new(max_gap: u64) -> Self {
        Self {
            last_sequences: BTreeMap::new(),
            max_gap,
        }
    }

    /// Check if message sequence is valid (not a replay)
    pub fn check_sequence(
        &mut self,
        validator: &ValidatorId,
        sequence: u64,
    ) -> ValidatorResult<()> {
        let last_seq = self.last_sequences.entry(validator.clone()).or_insert(0);

        // Sequence must be strictly increasing
        if sequence <= *last_seq {
            return Err(ValidatorError::NetworkError("Replay attack detected"));
        }

        // Check for suspicious gaps (possible attack or network issues)
        if sequence > *last_seq + self.max_gap {
            return Err(ValidatorError::NetworkError(
                "Sequence gap too large",
            ));
        }

        // Update last seen sequence
        *last_seq = sequence;
        Ok(())
    }

    /// Get last sequence for a validator
    pub fn last_sequence(&self, validator: &ValidatorId) -> Option<u64> {
        self.last_sequences.get(validator).copied()
    }

    /// Reset sequence for a validator (e.g., after reconnection)
    pub fn reset_sequence(&mut self, validator: &ValidatorId) {
        self.last_sequences.remove(validator);
    }
}

impl Default for ReplayProtector {
    fn default() -> Self {
        // Default: allow gaps up to 100 messages
        Self::new(100)
    }
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
