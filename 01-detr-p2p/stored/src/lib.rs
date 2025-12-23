//! Peer Storage & Cache (stored)
//!
//! Provides persistent peer storage using memory-based storage (for now, RocksDB integration ready).
//! Includes caching, TTL management, and batch operations.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use serde::{Serialize, Deserialize};
use hex;

/// Stored peer entry
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    persistence_path: Option<PathBuf>,
    #[cfg(feature = "rocksdb-backend")]
    db: Option<rocksdb::DB>,
}

impl PeerStore {
    pub fn new() -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            index_by_address: Arc::new(RwLock::new(HashMap::new())),
            cache_hits: Arc::new(RwLock::new(0)),
            cache_misses: Arc::new(RwLock::new(0)),
            persistence_path: None,
            #[cfg(feature = "rocksdb-backend")]
            db: None,
        }
    }

    /// Create a peer store with JSON persistence at the given path.
    /// Existing data is loaded; file is created on first write.
    pub async fn new_with_path<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path_buf = path.as_ref().to_path_buf();
        let mut store = Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            index_by_address: Arc::new(RwLock::new(HashMap::new())),
            cache_hits: Arc::new(RwLock::new(0)),
            cache_misses: Arc::new(RwLock::new(0)),
            persistence_path: Some(path_buf.clone()),
            #[cfg(feature = "rocksdb-backend")]
            db: None,
        };

        #[cfg(feature = "rocksdb-backend")]
        {
            // If path is a directory, prefer RocksDB
            let mut opts = rocksdb::Options::default();
            opts.create_if_missing(true);
            opts.set_max_open_files(128);
            match rocksdb::DB::open(&opts, &path_buf) {
                Ok(db) => {
                    store.db = Some(db);
                    // Load existing peers
                    if let Err(e) = store.load_from_rocksdb().await {
                        return Err(e);
                    }
                    return Ok(store);
                }
                Err(e) => {
                    return Err(format!("open rocksdb peer store: {e:?}"));
                }
            }
        }

        if path_buf.exists() {
            let data = fs::read(&path_buf)
                .await
                .map_err(|e| format!("read peer store: {e:?}"))?;
            let peers: Vec<StoredPeer> = serde_json::from_slice(&data)
                .map_err(|e| format!("decode peer store: {e:?}"))?;
            {
                let mut peers_map = store.peers.write().await;
                let mut addr_index = store.index_by_address.write().await;
                for peer in peers {
                    let peer_id = peer.peer_id.clone();
                    addr_index.insert(peer.address.clone(), peer_id.clone());
                    peers_map.insert(peer_id, peer);
                }
            }
        }

        Ok(store)
    }

    async fn persist_if_needed(&self) -> Result<(), String> {
        #[cfg(feature = "rocksdb-backend")]
        if let Some(db) = &self.db {
            let peers = self.peers.read().await;
            for (peer_id, peer) in peers.iter() {
                let value = serde_json::to_vec(peer)
                    .map_err(|e| format!("encode peer {peer_id}: {e:?}"))?;
                db.put(peer_id.as_bytes(), value)
                    .map_err(|e| format!("rocksdb put peer {peer_id}: {e:?}"))?;
            }
            return Ok(());
        }

        if let Some(path) = &self.persistence_path {
            let peers = self.peers.read().await;
            let entries: Vec<StoredPeer> = peers.values().cloned().collect();
            drop(peers);
            let data = serde_json::to_vec_pretty(&entries)
                .map_err(|e| format!("encode peer store: {e:?}"))?;

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .await
                    .map_err(|e| format!("create peer store dir: {e:?}"))?;
            }
            fs::write(path, data)
                .await
                .map_err(|e| format!("write peer store: {e:?}"))?;
        }
        Ok(())
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
        drop(peers);
        drop(address_index);
        self.persist_if_needed().await
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
            drop(peers);
            drop(address_index);
            self.persist_if_needed().await
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
        drop(peers);
        self.persist_if_needed().await
    }

    /// Update peer reputation
    pub async fn update_reputation(&self, peer_id: &str, delta: i32) -> Result<(), String> {
        let mut peers = self.peers.write().await;
        let peer = peers.get_mut(peer_id).ok_or("Peer not found")?;

        peer.reputation = (peer.reputation + delta).clamp(-100, 100);
        peer.refresh();
        drop(peers);
        self.persist_if_needed().await
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

    /// Count non-expired peers
    pub async fn active_len(&self) -> usize {
        let peers = self.peers.read().await;
        peers.values().filter(|p| !p.is_expired()).count()
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

        let len = stored_peers.len();
        drop(stored_peers);
        drop(address_index);
        self.persist_if_needed().await?;
        Ok(len)
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

        drop(peers);
        drop(address_index);
        self.persist_if_needed().await?;
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
        let _ = self.persist_if_needed().await;
    }

    /// Trim the peer store down to `max_peers` by removing the lowest reputation/oldest peers.
    /// Returns the number of peers removed. Helps avoid unbounded growth when organic discovery
    /// brings in many new peers.
    pub async fn prune_over_capacity(&self, max_peers: usize) -> usize {
        let mut peers = self.peers.write().await;
        let current_len = peers.len();
        if current_len <= max_peers {
            return 0;
        }

        // Collect candidates sorted by reputation asc, then last_seen asc (oldest first).
        let mut candidates: Vec<_> = peers.values().cloned().collect();
        candidates.sort_by(|a, b| {
            a.reputation
                .cmp(&b.reputation)
                .then_with(|| a.last_seen.cmp(&b.last_seen))
        });

        let remove_count = current_len.saturating_sub(max_peers);
        let to_remove: Vec<String> = candidates
            .into_iter()
            .take(remove_count)
            .map(|p| p.peer_id)
            .collect();

        for peer_id in &to_remove {
            if let Some(entry) = peers.remove(peer_id) {
                let mut addr_index = self.index_by_address.write().await;
                addr_index.remove(&entry.address);
            }
        }
        drop(peers);

        let _ = self.persist_if_needed().await;
        remove_count
    }

    /// Set TTL for a peer
    pub async fn set_ttl(&self, peer_id: &str, ttl_seconds: u64) -> Result<(), String> {
        let mut peers = self.peers.write().await;
        let peer = peers.get_mut(peer_id).ok_or("Peer not found")?;
        peer.ttl_seconds = ttl_seconds;
        drop(peers);
        self.persist_if_needed().await
    }

    /// Select best peers by reputation and recency (desc), limited to `limit`.
    pub async fn select_best_peers(&self, limit: usize) -> Vec<(StoredPeer, [u8; 32])> {
        let peers = self.peers.read().await;
        let mut candidates: Vec<_> = peers
            .values()
            .filter(|p| !p.is_expired())
            .filter_map(|p| decode_peer_id(&p.peer_id).map(|pid| (p.clone(), pid)))
            .collect();
        drop(peers);

        candidates.sort_by(|(a, _), (b, _)| {
            b.reputation
                .cmp(&a.reputation)
                .then_with(|| b.last_seen.cmp(&a.last_seen))
        });
        candidates.truncate(limit);
        candidates
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

fn decode_peer_id(hex_str: &str) -> Option<[u8; 32]> {
    if hex_str.len() != 64 {
        return None;
    }
    let bytes = hex::decode(hex_str).ok()?;
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes[..32]);
    Some(arr)
}

#[cfg(feature = "rocksdb-backend")]
impl PeerStore {
    async fn load_from_rocksdb(&self) -> Result<(), String> {
        let db = self.db.as_ref().ok_or("rocksdb not initialized")?;
        let mut peers_map = self.peers.write().await;
        let mut addr_index = self.index_by_address.write().await;
        peers_map.clear();
        addr_index.clear();
        let iter = db.iterator(rocksdb::IteratorMode::Start);
        for item in iter {
            let (k, v) = item.map_err(|e| format!("rocksdb iter: {e:?}"))?;
            let peer_id = std::str::from_utf8(&k)
                .map_err(|e| format!("peer_id utf8: {e:?}"))?
                .to_string();
            let peer: StoredPeer = serde_json::from_slice(&v)
                .map_err(|e| format!("decode peer {peer_id}: {e:?}"))?;
            addr_index.insert(peer.address.clone(), peer_id.clone());
            peers_map.insert(peer_id, peer);
        }
        Ok(())
    }
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
