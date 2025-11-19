// director-node/src/relay.rs
// MESSAGE RELAY SYSTEM
// Handles message forwarding with loop prevention and metrics

use detrp2p::Message;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use blake2::{Blake2b512, Digest};

// ============================================================================
// MESSAGE RELAY
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelayMetrics {
    /// Total messages relayed
    pub total_relayed: u64,

    /// Messages dropped (loops detected)
    pub loops_prevented: u64,

    /// Current seen cache size
    pub cache_size: usize,

    /// Relay latency (average ms)
    pub avg_latency_ms: f64,
}

impl Default for RelayMetrics {
    fn default() -> Self {
        Self {
            total_relayed: 0,
            loops_prevented: 0,
            cache_size: 0,
            avg_latency_ms: 0.0,
        }
    }
}

pub struct MessageRelay {
    /// Track message hashes to prevent loops
    /// Hash = BLAKE2b-512(message.encode())
    seen_messages: Arc<RwLock<HashSet<[u8; 32]>>>,

    /// Relay metrics
    metrics: Arc<RwLock<RelayMetrics>>,

    /// Maximum cache size (LRU eviction when full)
    max_cache_size: usize,
}

impl MessageRelay {
    pub fn new() -> Self {
        Self {
            seen_messages: Arc::new(RwLock::new(HashSet::new())),
            metrics: Arc::new(RwLock::new(RelayMetrics::default())),
            max_cache_size: 10_000, // Store last 10k message hashes
        }
    }

    /// Hash a message for loop detection
    pub fn hash_message(&self, msg: &Message) -> Result<[u8; 32], String> {
        let encoded = msg.encode()?;
        let hash = Blake2b512::digest(&encoded);

        // Take first 32 bytes of BLAKE2b-512 hash
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&hash[..32]);

        Ok(hash_bytes)
    }

    /// Check if message has been seen (loop detection)
    pub async fn is_seen(&self, msg_hash: &[u8; 32]) -> bool {
        self.seen_messages.read().await.contains(msg_hash)
    }

    /// Mark message as seen
    pub async fn mark_seen(&self, msg_hash: [u8; 32]) {
        let mut seen = self.seen_messages.write().await;

        // LRU eviction if cache is full
        if seen.len() >= self.max_cache_size {
            // Simple eviction: clear half the cache
            // In production, use a proper LRU cache
            let to_remove: Vec<[u8; 32]> = seen.iter().take(self.max_cache_size / 2).cloned().collect();
            for hash in to_remove {
                seen.remove(&hash);
            }

            log::debug!(
                "♻️ Evicted {} old messages from relay cache",
                self.max_cache_size / 2
            );
        }

        seen.insert(msg_hash);

        // Update cache size metric
        let mut metrics = self.metrics.write().await;
        metrics.cache_size = seen.len();
    }

    /// Increment relay counter
    pub async fn increment_relay_count(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.total_relayed += 1;
    }

    /// Increment loop prevention counter
    pub async fn increment_loop_prevented(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.loops_prevented += 1;
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> RelayMetrics {
        self.metrics.read().await.clone()
    }

    /// Clear cache (use sparingly - for testing or manual intervention)
    pub async fn clear_cache(&self) {
        let mut seen = self.seen_messages.write().await;
        seen.clear();

        let mut metrics = self.metrics.write().await;
        metrics.cache_size = 0;

        log::info!("🧹 Cleared message relay cache");
    }
}

impl Default for MessageRelay {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_message_relay_loop_prevention() {
        let relay = MessageRelay::new();

        let msg = Message::Vote {
            data: vec![1, 2, 3],
        };

        let hash = relay.hash_message(&msg).unwrap();

        // First time - not seen
        assert!(!relay.is_seen(&hash).await);

        // Mark as seen
        relay.mark_seen(hash).await;

        // Second time - should be seen
        assert!(relay.is_seen(&hash).await);
    }

    #[tokio::test]
    async fn test_relay_metrics() {
        let relay = MessageRelay::new();

        relay.increment_relay_count().await;
        relay.increment_relay_count().await;
        relay.increment_loop_prevented().await;

        let metrics = relay.get_metrics().await;
        assert_eq!(metrics.total_relayed, 2);
        assert_eq!(metrics.loops_prevented, 1);
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let mut relay = MessageRelay::new();
        relay.max_cache_size = 10; // Small cache for testing

        // Fill cache beyond capacity
        for i in 0..15 {
            let hash = [i as u8; 32];
            relay.mark_seen(hash).await;
        }

        let metrics = relay.get_metrics().await;
        // Should have evicted half when full
        assert!(metrics.cache_size <= 10);
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let relay = MessageRelay::new();

        let msg = Message::Vote { data: vec![1, 2, 3] };
        let hash = relay.hash_message(&msg).unwrap();

        relay.mark_seen(hash).await;
        assert!(relay.is_seen(&hash).await);

        relay.clear_cache().await;
        assert!(!relay.is_seen(&hash).await);

        let metrics = relay.get_metrics().await;
        assert_eq!(metrics.cache_size, 0);
    }

    #[test]
    fn test_hash_message_deterministic() {
        let relay = MessageRelay::new();

        let msg = Message::Vote {
            data: vec![1, 2, 3],
        };

        let hash1 = relay.hash_message(&msg).unwrap();
        let hash2 = relay.hash_message(&msg).unwrap();

        // Same message should produce same hash
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_message_different() {
        let relay = MessageRelay::new();

        let msg1 = Message::Vote { data: vec![1, 2, 3] };
        let msg2 = Message::Vote { data: vec![4, 5, 6] };

        let hash1 = relay.hash_message(&msg1).unwrap();
        let hash2 = relay.hash_message(&msg2).unwrap();

        // Different messages should produce different hashes
        assert_ne!(hash1, hash2);
    }
}
