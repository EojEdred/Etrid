//! Peer Storage & Cache (stored)
//!
//! Provides persistent peer storage using memory-based storage (for now, RocksDB integration ready).
//! Includes caching, TTL management, and batch operations.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

/// Stored peer entry
#[derive(Debug, Clone)]
pub struct StoredPeer {
    pub peer_id: String,
    pub address: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub reputation: i32,
    pub created_at: u64,
    pub last_seen: u64,
    pub ttl_seconds: u64,
}

impl StoredPeer {
    pub fn new(peer_id: String, address: String) -> Self {
        let now = timestamp_secs();
        Self {
            peer_id,
            address,
            version: "0.1.0".to_string(),
            capabilities: vec![],
            reputation: 0,
            created_at: now,
            last_seen: now,
            ttl_seconds: 86400, // 24 hours
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = timestamp_secs();
        now.saturating_sub(self.created_at) > self.ttl_seconds
    }

    pub fn is_stale(&self, threshold_secs: u64) -> bool {
        let now = timestamp_secs();
        now.saturating_sub(self.last_seen) > threshold_secs
    }

    pub fn refresh(&mut self) {
        self.last_seen = timestamp_secs();
    }
}

/// In-memory peer storage
pub struct PeerStore {
    peers: Arc<RwLock<HashMap<String, StoredPeer>>>,
    index_by_address: Arc<RwLock<HashMap<String, String>>>, // address -> peer_id
    cache_hits: Arc<RwLock<u64>>,
    cache_misses: Arc<RwLock<u64>>,
}

impl PeerStore {
    pub fn new() -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            index_by_address: Arc::new(RwLock::new(HashMap::new())),
            cache_hits: Arc::new(RwLock::new(0)),
            cache_misses: Arc::new(RwLock::new(0)),
        }
    }

    /// Store a peer
    pub async fn store(&self, peer: StoredPeer) -> Result<(), String> {
        let peer_id = peer.peer_id.clone();
        let address = peer.address.clone();

        let mut peers = self.peers.write().await;
        let mut address_index = self.index_by_address.write().await;

        // Remove old address mapping if exists
        if let Some(old_peer) = peers.get(&peer_id) {
            address_index.remove(&old_peer.address);
        }

        peers.insert(peer_id.clone(), peer);
        address_index.insert(address, peer_id);

        Ok(())
    }

    /// Retrieve a peer by ID
    pub async fn get(&self, peer_id: &str) -> Option<StoredPeer> {
        let peers = self.peers.read().await;

        if let Some(peer) = peers.get(peer_id) {
            if !peer.is_expired() {
                *self.cache_hits.write().await += 1;
                return Some(peer.clone());
            }
        }

        *self.cache_misses.write().await += 1;
        None
    }

    /// Retrieve peer by address
    pub async fn get_by_address(&self, address: &str) -> Option<StoredPeer> {
        let address_index = self.index_by_address.read().await;
        if let Some(peer_id) = address_index.get(address) {
            let peer_id = peer_id.clone();
            drop(address_index);
            self.get(&peer_id).await
        } else {
            *self.cache_misses.write().await += 1;
            None
        }
    }

    /// Remove a peer
    pub async fn delete(&self, peer_id: &str) -> Result<(), String> {
        let mut peers = self.peers.write().await;
        let mut address_index = self.index_by_address.write().await;

        if let Some(peer) = peers.remove(peer_id) {
            address_index.remove(&peer.address);
            Ok(())
        } else {
            Err("Peer not found".to_string())
        }
    }

    /// Update peer information
    pub async fn update(&self, peer_id: &str, version: String, capabilities: Vec<String>) -> Result<(), String> {
        let mut peers = self.peers.write().await;
        let peer = peers.get_mut(peer_id).ok_or("Peer not found")?;

        peer.version = version;
        peer.capabilities = capabilities;
        peer.refresh();

        Ok(())
    }

    /// Update peer reputation
    pub async fn update_reputation(&self, peer_id: &str, delta: i32) -> Result<(), String> {
        let mut peers = self.peers.write().await;
        let peer = peers.get_mut(peer_id).ok_or("Peer not found")?;

        peer.reputation = (peer.reputation + delta).clamp(-100, 100);
        peer.refresh();

        Ok(())
    }

    /// Get all peers
    pub async fn get_all(&self) -> Vec<StoredPeer> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    /// Get all non-expired peers
    pub async fn get_active(&self) -> Vec<StoredPeer> {
        let peers = self.peers.read().await;
        peers
            .values()
            .filter(|p| !p.is_expired())
            .cloned()
            .collect()
    }

    /// Get peers by capability
    pub async fn get_by_capability(&self, capability: &str) -> Vec<StoredPeer> {
        let peers = self.peers.read().await;
        peers
            .values()
            .filter(|p| p.capabilities.contains(&capability.to_string()) && !p.is_expired())
            .cloned()
            .collect()
    }

    /// Batch insert peers
    pub async fn batch_insert(&self, peers: Vec<StoredPeer>) -> Result<usize, String> {
        let mut stored_peers = self.peers.write().await;
        let mut address_index = self.index_by_address.write().await;

        for peer in peers {
            let peer_id = peer.peer_id.clone();
            let address = peer.address.clone();

            // Remove old address mapping if exists
            if let Some(old_peer) = stored_peers.get(&peer_id) {
                address_index.remove(&old_peer.address);
            }

            stored_peers.insert(peer_id.clone(), peer);
            address_index.insert(address, peer_id);
        }

        Ok(stored_peers.len())
    }

    /// Batch delete peers
    pub async fn batch_delete(&self, peer_ids: Vec<String>) -> Result<usize, String> {
        let mut peers = self.peers.write().await;
        let mut address_index = self.index_by_address.write().await;

        let mut deleted = 0;
        for peer_id in peer_ids {
            if let Some(peer) = peers.remove(&peer_id) {
                address_index.remove(&peer.address);
                deleted += 1;
            }
        }

        Ok(deleted)
    }

    /// Query peers with filter
    pub async fn query<F>(&self, filter: F) -> Vec<StoredPeer>
    where
        F: Fn(&StoredPeer) -> bool,
    {
        let peers = self.peers.read().await;
        peers
            .values()
            .filter(|p| filter(p))
            .cloned()
            .collect()
    }

    /// Cleanup expired peers
    pub async fn cleanup_expired(&self) -> usize {
        let mut peers = self.peers.write().await;
        let mut address_index = self.index_by_address.write().await;

        let before = peers.len();
        let expired_ids: Vec<_> = peers
            .iter()
            .filter(|(_, p)| p.is_expired())
            .map(|(id, _)| id.clone())
            .collect();

        for peer_id in expired_ids {
            if let Some(peer) = peers.remove(&peer_id) {
                address_index.remove(&peer.address);
            }
        }

        before - peers.len()
    }

    /// Cleanup stale peers
    pub async fn cleanup_stale(&self, threshold_secs: u64) -> usize {
        let mut peers = self.peers.write().await;
        let mut address_index = self.index_by_address.write().await;

        let before = peers.len();
        let stale_ids: Vec<_> = peers
            .iter()
            .filter(|(_, p)| p.is_stale(threshold_secs))
            .map(|(id, _)| id.clone())
            .collect();

        for peer_id in stale_ids {
            if let Some(peer) = peers.remove(&peer_id) {
                address_index.remove(&peer.address);
            }
        }

        before - peers.len()
    }

    /// Get peer count
    pub async fn count(&self) -> usize {
        self.peers.read().await.len()
    }

    /// Get active peer count
    pub async fn count_active(&self) -> usize {
        let peers = self.peers.read().await;
        peers.values().filter(|p| !p.is_expired()).count()
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> CacheStats {
        let hits = *self.cache_hits.read().await;
        let misses = *self.cache_misses.read().await;
        let total = hits + misses;

        CacheStats {
            hits,
            misses,
            hit_rate: if total > 0 { (hits as f64 / total as f64) * 100.0 } else { 0.0 },
            total_peers: self.count().await,
        }
    }

    /// Clear all peers
    pub async fn clear(&self) {
        self.peers.write().await.clear();
        self.index_by_address.write().await.clear();
    }

    /// Set TTL for a peer
    pub async fn set_ttl(&self, peer_id: &str, ttl_seconds: u64) -> Result<(), String> {
        let mut peers = self.peers.write().await;
        let peer = peers.get_mut(peer_id).ok_or("Peer not found")?;
        peer.ttl_seconds = ttl_seconds;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub total_peers: usize,
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

    #[tokio::test]
    async fn test_store_peer() {
        let store = PeerStore::new();
        let peer = StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        assert!(store.store(peer).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_peer() {
        let store = PeerStore::new();
        let peer = StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        store.store(peer).await.unwrap();

        let retrieved = store.get("peer1").await;
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_get_by_address() {
        let store = PeerStore::new();
        let peer = StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        store.store(peer).await.unwrap();

        let retrieved = store.get_by_address("127.0.0.1:8001").await;
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_delete_peer() {
        let store = PeerStore::new();
        let peer = StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        store.store(peer).await.unwrap();

        assert!(store.delete("peer1").await.is_ok());
        assert!(store.get("peer1").await.is_none());
    }

    #[tokio::test]
    async fn test_update_peer() {
        let store = PeerStore::new();
        let peer = StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        store.store(peer).await.unwrap();

        store.update("peer1", "2.0.0".to_string(), vec!["relay".to_string()]).await.unwrap();

        let updated = store.get("peer1").await.unwrap();
        assert_eq!(updated.version, "2.0.0");
        assert_eq!(updated.capabilities, vec!["relay"]);
    }

    #[tokio::test]
    async fn test_reputation() {
        let store = PeerStore::new();
        let peer = StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        store.store(peer).await.unwrap();

        store.update_reputation("peer1", 50).await.unwrap();
        let updated = store.get("peer1").await.unwrap();
        assert_eq!(updated.reputation, 50);
    }

    #[tokio::test]
    async fn test_get_all_peers() {
        let store = PeerStore::new();
        for i in 0..5 {
            let peer = StoredPeer::new(
                format!("peer{}", i),
                format!("127.0.0.1:{}", 8001 + i),
            );
            store.store(peer).await.unwrap();
        }

        let all = store.get_all().await;
        assert_eq!(all.len(), 5);
    }

    #[tokio::test]
    async fn test_get_by_capability() {
        let store = PeerStore::new();
        let mut peer1 = StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        peer1.capabilities = vec!["relay".to_string()];

        let peer2 = StoredPeer::new("peer2".to_string(), "127.0.0.1:8002".to_string());

        store.store(peer1).await.unwrap();
        store.store(peer2).await.unwrap();

        let with_relay = store.get_by_capability("relay").await;
        assert_eq!(with_relay.len(), 1);
    }

    #[tokio::test]
    async fn test_batch_insert() {
        let store = PeerStore::new();
        let peers = vec![
            StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string()),
            StoredPeer::new("peer2".to_string(), "127.0.0.1:8002".to_string()),
        ];

        let count = store.batch_insert(peers).await.unwrap();
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_batch_delete() {
        let store = PeerStore::new();
        let peers = vec![
            StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string()),
            StoredPeer::new("peer2".to_string(), "127.0.0.1:8002".to_string()),
        ];

        store.batch_insert(peers).await.unwrap();
        let deleted = store.batch_delete(vec!["peer1".to_string()]).await.unwrap();
        assert_eq!(deleted, 1);
        assert_eq!(store.count().await, 1);
    }

    #[tokio::test]
    async fn test_query() {
        let store = PeerStore::new();
        let mut peer = StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        peer.reputation = 50;
        store.store(peer).await.unwrap();

        let results = store.query(|p| p.reputation > 30).await;
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let store = PeerStore::new();
        let peer = StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        store.store(peer).await.unwrap();

        store.get("peer1").await;
        store.get("peer1").await;
        store.get("nonexistent").await;

        let stats = store.cache_stats().await;
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
    }

    #[tokio::test]
    async fn test_cleanup_expired() {
        let store = PeerStore::new();
        let mut peer = StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        peer.ttl_seconds = 0; // Immediately expired
        store.store(peer).await.unwrap();

        let removed = store.cleanup_expired().await;
        assert_eq!(removed, 1);
    }

    #[tokio::test]
    async fn test_set_ttl() {
        let store = PeerStore::new();
        let peer = StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        store.store(peer).await.unwrap();

        store.set_ttl("peer1", 3600).await.unwrap();
        let updated = store.get("peer1").await.unwrap();
        assert_eq!(updated.ttl_seconds, 3600);
    }

    #[tokio::test]
    async fn test_clear() {
        let store = PeerStore::new();
        let peer = StoredPeer::new("peer1".to_string(), "127.0.0.1:8001".to_string());
        store.store(peer).await.unwrap();

        store.clear().await;
        assert_eq!(store.count().await, 0);
    }
}
