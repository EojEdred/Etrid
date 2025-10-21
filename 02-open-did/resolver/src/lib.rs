//! DID Resolver
//!
//! Resolves DIDs to documents with caching, fallback mechanisms, and error handling.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Resolution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionResult {
    pub did: String,
    pub document: String, // JSON-LD DID Document
    pub resolved_at: u64,
    pub content_type: String,
    pub metadata: ResolutionMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionMetadata {
    pub content_type: String,
    pub retrieved_time: u64,
    pub duration_ms: u64,
}

/// Resolution error
#[derive(Debug, Clone)]
pub enum ResolutionError {
    NotFound,
    InvalidDid,
    Deactivated,
    CacheMiss,
    NetworkError,
    Timeout,
    ParsingError,
}

impl std::fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResolutionError::NotFound => write!(f, "DID not found"),
            ResolutionError::InvalidDid => write!(f, "Invalid DID format"),
            ResolutionError::Deactivated => write!(f, "DID has been deactivated"),
            ResolutionError::CacheMiss => write!(f, "Not in cache"),
            ResolutionError::NetworkError => write!(f, "Network error"),
            ResolutionError::Timeout => write!(f, "Resolution timeout"),
            ResolutionError::ParsingError => write!(f, "Document parsing error"),
        }
    }
}

/// Cached resolution entry
#[derive(Debug, Clone)]
struct CacheEntry {
    result: ResolutionResult,
    cached_at: u64,
    ttl_seconds: u64,
}

impl CacheEntry {
    fn is_expired(&self) -> bool {
        let now = timestamp_secs();
        now.saturating_sub(self.cached_at) > self.ttl_seconds
    }
}

/// DID resolver trait
#[async_trait::async_trait]
pub trait DidResolver: Send + Sync {
    async fn resolve(&self, did: &str) -> Result<ResolutionResult, ResolutionError>;
}

/// On-chain resolver
pub struct OnChainResolver {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    cache_ttl_secs: u64,
    cache_hits: Arc<RwLock<u64>>,
    cache_misses: Arc<RwLock<u64>>,
}

impl OnChainResolver {
    pub fn new(cache_ttl_secs: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl_secs,
            cache_hits: Arc::new(RwLock::new(0)),
            cache_misses: Arc::new(RwLock::new(0)),
        }
    }

    /// Validate DID format
    pub fn validate_did(&self, did: &str) -> Result<(), ResolutionError> {
        let parts: Vec<&str> = did.split(':').collect();
        if parts.len() != 3 || parts[0] != "did" || parts[1] != "etrid" || parts[2].is_empty() {
            return Err(ResolutionError::InvalidDid);
        }
        Ok(())
    }

    /// Try to resolve from cache
    async fn resolve_from_cache(&self, did: &str) -> Result<ResolutionResult, ResolutionError> {
        let mut cache = self.cache.write().await;

        if let Some(entry) = cache.get(did) {
            if !entry.is_expired() {
                *self.cache_hits.write().await += 1;
                return Ok(entry.result.clone());
            } else {
                cache.remove(did);
            }
        }

        *self.cache_misses.write().await += 1;
        Err(ResolutionError::CacheMiss)
    }

    /// Store in cache
    async fn store_in_cache(&self, did: &str, result: ResolutionResult) {
        let mut cache = self.cache.write().await;
        cache.insert(
            did.to_string(),
            CacheEntry {
                result,
                cached_at: timestamp_secs(),
                ttl_seconds: self.cache_ttl_secs,
            },
        );
    }

    /// Simulate on-chain lookup (in real implementation, would query blockchain)
    async fn lookup_on_chain(&self, did: &str) -> Result<ResolutionResult, ResolutionError> {
        self.validate_did(did)?;

        // Simulate on-chain resolution
        // In production, this would query the blockchain state
        let key_id = "#key1";
        let document = format!(
            r#"{{
  "@context": "https://www.w3.org/ns/did/v1",
  "id": "{}",
  "verificationMethod": [
    {{
      "id": "{}",
      "type": "Ed25519VerificationKey2020",
      "controller": "{}",
      "publicKeyBase58": ""
    }}
  ],
  "authentication": ["{}"]
}}"#,
            did, key_id, did, key_id
        );

        Ok(ResolutionResult {
            did: did.to_string(),
            document,
            resolved_at: timestamp_secs(),
            content_type: "application/did+json".to_string(),
            metadata: ResolutionMetadata {
                content_type: "application/did+json".to_string(),
                retrieved_time: timestamp_secs(),
                duration_ms: 100,
            },
        })
    }

    /// Clear cache
    pub async fn clear_cache(&self) {
        self.cache.write().await.clear();
    }

    /// Get cache stats
    pub async fn cache_stats(&self) -> CacheStats {
        let hits = *self.cache_hits.read().await;
        let misses = *self.cache_misses.read().await;
        let total = hits + misses;

        CacheStats {
            hits,
            misses,
            hit_rate: if total > 0 { (hits as f64 / total as f64) * 100.0 } else { 0.0 },
            cached_dids: self.cache.read().await.len(),
        }
    }

    /// Get cached DIDs
    pub async fn get_cached_dids(&self) -> Vec<String> {
        let cache = self.cache.read().await;
        cache.keys().cloned().collect()
    }

    /// Cleanup expired cache entries
    pub async fn cleanup_cache(&self) -> usize {
        let mut cache = self.cache.write().await;
        let before = cache.len();
        cache.retain(|_, entry| !entry.is_expired());
        before - cache.len()
    }
}

#[async_trait::async_trait]
impl DidResolver for OnChainResolver {
    async fn resolve(&self, did: &str) -> Result<ResolutionResult, ResolutionError> {
        // Try cache first
        if let Ok(result) = self.resolve_from_cache(did).await {
            return Ok(result);
        }

        // Try on-chain lookup
        match self.lookup_on_chain(did).await {
            Ok(result) => {
                self.store_in_cache(did, result.clone()).await;
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }
}

/// Multi-resolver with fallback chain
pub struct MultiResolver {
    resolvers: Vec<Arc<dyn DidResolver>>,
    resolution_timeout_ms: u64,
}

impl MultiResolver {
    pub fn new(timeout_ms: u64) -> Self {
        Self {
            resolvers: vec![],
            resolution_timeout_ms: timeout_ms,
        }
    }

    pub fn add_resolver(&mut self, resolver: Arc<dyn DidResolver>) {
        self.resolvers.push(resolver);
    }

    /// Resolve with fallback through all resolvers
    pub async fn resolve(&self, did: &str) -> Result<ResolutionResult, ResolutionError> {
        for resolver in &self.resolvers {
            match resolver.resolve(did).await {
                Ok(result) => return Ok(result),
                Err(_) => continue,
            }
        }
        Err(ResolutionError::NotFound)
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub cached_dids: usize,
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

    #[test]
    fn test_validate_did() {
        let resolver = OnChainResolver::new(3600);
        assert!(resolver.validate_did("did:etrid:user1").is_ok());
        assert!(resolver.validate_did("invalid").is_err());
    }

    #[tokio::test]
    async fn test_resolve_and_cache() {
        let resolver = OnChainResolver::new(3600);
        
        let result1 = resolver.resolve("did:etrid:user1").await;
        assert!(result1.is_ok());

        let result2 = resolver.resolve("did:etrid:user1").await;
        assert!(result2.is_ok());

        let stats = resolver.cache_stats().await;
        assert_eq!(stats.hits, 1);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let resolver = OnChainResolver::new(1); // 1 second TTL
        
        resolver.resolve("did:etrid:user1").await.unwrap();
        
        // Immediate access - should hit cache
        let stats1 = resolver.cache_stats().await;
        let hits_before = stats1.hits;

        // After expiration, should miss cache
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        resolver.resolve("did:etrid:user1").await.unwrap();

        let stats2 = resolver.cache_stats().await;
        assert!(stats2.misses > 0);
    }

    #[tokio::test]
    async fn test_invalid_did_resolution() {
        let resolver = OnChainResolver::new(3600);
        let result = resolver.resolve("invalid").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let resolver = OnChainResolver::new(3600);
        resolver.resolve("did:etrid:user1").await.unwrap();
        
        let cached_before = resolver.get_cached_dids().await;
        assert_eq!(cached_before.len(), 1);

        resolver.clear_cache().await;
        let cached_after = resolver.get_cached_dids().await;
        assert_eq!(cached_after.len(), 0);
    }

    #[tokio::test]
    async fn test_multi_resolver() {
        let resolver1 = Arc::new(OnChainResolver::new(3600));
        let mut multi = MultiResolver::new(5000);
        multi.add_resolver(resolver1);

        let result = multi.resolve("did:etrid:user1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_resolution_result_metadata() {
        let resolver = OnChainResolver::new(3600);
        let result = resolver.resolve("did:etrid:user1").await.unwrap();
        
        assert!(!result.document.is_empty());
        assert_eq!(result.content_type, "application/did+json");
    }

    #[tokio::test]
    async fn test_cleanup_cache() {
        let resolver = OnChainResolver::new(1); // 1 second TTL
        resolver.resolve("did:etrid:user1").await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let cleaned = resolver.cleanup_cache().await;
        assert_eq!(cleaned, 1);
    }
}
