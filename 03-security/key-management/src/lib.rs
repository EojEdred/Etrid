//! Key Management
//!
//! Secure key storage, rotation, and backup/restore functionality.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
// use ed25519_dalek::{SigningKey, VerifyingKey};

/// Key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub key_id: String,
    pub key_type: String,
    pub created_at: u64,
    pub rotated_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub algorithm: String,
    pub active: bool,
}

impl KeyMetadata {
    pub fn new(key_id: String, algorithm: String) -> Self {
        Self {
            key_id,
            key_type: "Ed25519".to_string(),
            created_at: timestamp_secs(),
            rotated_at: None,
            expires_at: None,
            algorithm,
            active: true,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(false, |exp| exp <= timestamp_secs())
    }
}

/// Encrypted key entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedKey {
    pub key_id: String,
    pub encrypted_key: Vec<u8>,
    pub nonce: Vec<u8>,
    pub metadata: KeyMetadata,
}

/// Key store trait
pub trait KeyStore: Send + Sync {
    fn store_key(&self, key_id: String, key_data: Vec<u8>) -> impl std::future::Future<Output = Result<(), String>> + Send;
    fn retrieve_key(&self, key_id: &str) -> impl std::future::Future<Output = Result<Vec<u8>, String>> + Send;
    fn delete_key(&self, key_id: &str) -> impl std::future::Future<Output = Result<(), String>> + Send;
}

/// File-based key store (production: would encrypt to disk)
pub struct FileKeyStore {
    keys: Arc<RwLock<HashMap<String, EncryptedKey>>>,
    metadata: Arc<RwLock<HashMap<String, KeyMetadata>>>,
}

impl FileKeyStore {
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Store key with metadata
    pub async fn store(&self, key_id: String, key_data: Vec<u8>, algorithm: String) -> Result<(), String> {
        let metadata = KeyMetadata::new(key_id.clone(), algorithm);
        
        let encrypted_key = EncryptedKey {
            key_id: key_id.clone(),
            encrypted_key: key_data, // In production, encrypt this
            nonce: vec![0u8; 12], // In production, generate real nonce
            metadata: metadata.clone(),
        };

        let mut keys = self.keys.write().await;
        let mut meta = self.metadata.write().await;

        keys.insert(key_id.clone(), encrypted_key);
        meta.insert(key_id, metadata);

        Ok(())
    }

    /// Retrieve key
    pub async fn retrieve(&self, key_id: &str) -> Result<Vec<u8>, String> {
        let keys = self.keys.read().await;
        let key = keys.get(key_id).ok_or("Key not found")?;

        let meta = self.metadata.read().await;
        if let Some(m) = meta.get(key_id) {
            if !m.active {
                return Err("Key is inactive".to_string());
            }
            if m.is_expired() {
                return Err("Key has expired".to_string());
            }
        }

        Ok(key.encrypted_key.clone())
    }

    /// Delete key
    pub async fn delete(&self, key_id: &str) -> Result<(), String> {
        let mut keys = self.keys.write().await;
        let mut meta = self.metadata.write().await;

        keys.remove(key_id).ok_or("Key not found")?;
        meta.remove(key_id);

        Ok(())
    }

    /// Rotate key
    pub async fn rotate_key(&self, key_id: &str, new_key_data: Vec<u8>) -> Result<(), String> {
        let mut keys = self.keys.write().await;
        let mut meta = self.metadata.write().await;

        let old_key = keys.get_mut(key_id).ok_or("Key not found")?;
        let old_meta = meta.get_mut(key_id).ok_or("Metadata not found")?;

        old_key.encrypted_key = new_key_data;
        old_meta.rotated_at = Some(timestamp_secs());

        Ok(())
    }

    /// Set key expiration
    pub async fn set_expiration(&self, key_id: &str, expires_at: u64) -> Result<(), String> {
        let mut meta = self.metadata.write().await;
        let entry = meta.get_mut(key_id).ok_or("Key not found")?;
        entry.expires_at = Some(expires_at);
        Ok(())
    }

    /// Deactivate key
    pub async fn deactivate(&self, key_id: &str) -> Result<(), String> {
        let mut meta = self.metadata.write().await;
        let entry = meta.get_mut(key_id).ok_or("Key not found")?;
        entry.active = false;
        Ok(())
    }

    /// Activate key
    pub async fn activate(&self, key_id: &str) -> Result<(), String> {
        let mut meta = self.metadata.write().await;
        let entry = meta.get_mut(key_id).ok_or("Key not found")?;
        entry.active = true;
        Ok(())
    }

    /// Get key metadata
    pub async fn get_metadata(&self, key_id: &str) -> Option<KeyMetadata> {
        let meta = self.metadata.read().await;
        meta.get(key_id).cloned()
    }

    /// List all key IDs
    pub async fn list_keys(&self) -> Vec<String> {
        let keys = self.keys.read().await;
        keys.keys().cloned().collect()
    }

    /// List active keys
    pub async fn list_active_keys(&self) -> Vec<String> {
        let meta = self.metadata.read().await;
        meta
            .iter()
            .filter(|(_, m)| m.active && !m.is_expired())
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Cleanup expired keys
    pub async fn cleanup_expired(&self) -> usize {
        let mut keys = self.keys.write().await;
        let mut meta = self.metadata.write().await;

        let expired: Vec<_> = meta
            .iter()
            .filter(|(_, m)| m.is_expired())
            .map(|(id, _)| id.clone())
            .collect();

        for key_id in expired {
            keys.remove(&key_id);
            meta.remove(&key_id);
        }

        meta.len()
    }

    /// Export key for backup
    pub async fn export_key(&self, key_id: &str) -> Result<String, String> {
        use base64::{Engine as _, engine::general_purpose};
        let key = self.retrieve(key_id).await?;
        let encoded = general_purpose::STANDARD.encode(&key);
        Ok(encoded)
    }

    /// Import key from backup
    pub async fn import_key(&self, key_id: String, backup: &str, algorithm: String) -> Result<(), String> {
        use base64::{Engine as _, engine::general_purpose};
        let decoded = general_purpose::STANDARD.decode(backup).map_err(|_| "Invalid base64")?;
        self.store(key_id, decoded, algorithm).await
    }

    /// Get store statistics
    pub async fn stats(&self) -> KeyStoreStats {
        let keys = self.keys.read().await;
        let meta = self.metadata.read().await;

        let total = keys.len();
        let active = meta.values().filter(|m| m.active && !m.is_expired()).count();
        let expired = meta.values().filter(|m| m.is_expired()).count();

        KeyStoreStats {
            total_keys: total,
            active_keys: active,
            expired_keys: expired,
        }
    }
}

impl KeyStore for FileKeyStore {
    fn store_key(&self, key_id: String, key_data: Vec<u8>) -> impl std::future::Future<Output = Result<(), String>> + Send {
        async move {
            self.store(key_id, key_data, "Ed25519".to_string()).await
        }
    }

    fn retrieve_key(&self, key_id: &str) -> impl std::future::Future<Output = Result<Vec<u8>, String>> + Send {
        let key_id = key_id.to_string();
        async move {
            self.retrieve(&key_id).await
        }
    }

    fn delete_key(&self, key_id: &str) -> impl std::future::Future<Output = Result<(), String>> + Send {
        let key_id = key_id.to_string();
        async move {
            self.delete(&key_id).await
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyStoreStats {
    pub total_keys: usize,
    pub active_keys: usize,
    pub expired_keys: usize,
}

fn timestamp_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_and_retrieve_key() {
        let store = FileKeyStore::new();
        store.store("key1".to_string(), vec![1, 2, 3], "Ed25519".to_string()).await.unwrap();
        
        let retrieved = store.retrieve("key1").await.unwrap();
        assert_eq!(retrieved, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_delete_key() {
        let store = FileKeyStore::new();
        store.store("key1".to_string(), vec![1, 2, 3], "Ed25519".to_string()).await.unwrap();
        
        assert!(store.delete("key1").await.is_ok());
        assert!(store.retrieve("key1").await.is_err());
    }

    #[tokio::test]
    async fn test_rotate_key() {
        let store = FileKeyStore::new();
        store.store("key1".to_string(), vec![1, 2, 3], "Ed25519".to_string()).await.unwrap();
        
        store.rotate_key("key1", vec![4, 5, 6]).await.unwrap();
        let retrieved = store.retrieve("key1").await.unwrap();
        assert_eq!(retrieved, vec![4, 5, 6]);
    }

    #[tokio::test]
    async fn test_deactivate_key() {
        let store = FileKeyStore::new();
        store.store("key1".to_string(), vec![1, 2, 3], "Ed25519".to_string()).await.unwrap();
        
        store.deactivate("key1").await.unwrap();
        assert!(store.retrieve("key1").await.is_err());
    }

    #[tokio::test]
    async fn test_list_active_keys() {
        let store = FileKeyStore::new();
        store.store("key1".to_string(), vec![1, 2, 3], "Ed25519".to_string()).await.unwrap();
        store.store("key2".to_string(), vec![4, 5, 6], "Ed25519".to_string()).await.unwrap();
        
        let active = store.list_active_keys().await;
        assert_eq!(active.len(), 2);
    }

    #[tokio::test]
    async fn test_get_metadata() {
        let store = FileKeyStore::new();
        store.store("key1".to_string(), vec![1, 2, 3], "Ed25519".to_string()).await.unwrap();
        
        let meta = store.get_metadata("key1").await;
        assert!(meta.is_some());
        assert_eq!(meta.unwrap().key_id, "key1");
    }

    #[tokio::test]
    async fn test_stats() {
        let store = FileKeyStore::new();
        store.store("key1".to_string(), vec![1, 2, 3], "Ed25519".to_string()).await.unwrap();
        store.store("key2".to_string(), vec![4, 5, 6], "Ed25519".to_string()).await.unwrap();
        
        let stats = store.stats().await;
        assert_eq!(stats.total_keys, 2);
        assert_eq!(stats.active_keys, 2);
    }
}
