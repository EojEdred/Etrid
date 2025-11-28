//! DID Registry
//!
//! Manages on-chain DID registration, updates, revocation, and access control.
//! Implements registration pallet for ÉTRID blockchain.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// DID registration entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidRegistration {
    pub did: String,
    pub owner: String,
    pub controller: String,
    pub document_hash: Vec<u8>,
    pub registered_at: u64,
    pub updated_at: u64,
    pub expires_at: Option<u64>,
    pub revoked: bool,
}

impl DidRegistration {
    pub fn new(did: String, owner: String, controller: String, document_hash: Vec<u8>) -> Self {
        let now = timestamp_secs();
        Self {
            did,
            owner,
            controller,
            document_hash,
            registered_at: now,
            updated_at: now,
            expires_at: None,
            revoked: false,
        }
    }

    pub fn is_active(&self) -> bool {
        !self.revoked && (self.expires_at.is_none() || self.expires_at.unwrap() > timestamp_secs())
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(false, |exp| exp <= timestamp_secs())
    }
}

/// Access control entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessLevel {
    None,
    Reader,
    Writer,
    Admin,
}

#[derive(Debug, Clone)]
pub struct AccessControl {
    pub did: String,
    pub agent: String,
    pub level: AccessLevel,
    pub granted_at: u64,
}

impl AccessControl {
    pub fn new(did: String, agent: String, level: AccessLevel) -> Self {
        Self {
            did,
            agent,
            level,
            granted_at: timestamp_secs(),
        }
    }
}

/// DID Registry storage
pub struct DidRegistry {
    registrations: Arc<RwLock<HashMap<String, DidRegistration>>>,
    access_control: Arc<RwLock<HashMap<String, Vec<AccessControl>>>>, // did -> access list
    nonce_counter: Arc<RwLock<u64>>,
    operation_history: Arc<RwLock<Vec<RegistryOperation>>>,
}

/// Registry operation for audit trail
#[derive(Debug, Clone)]
pub enum RegistryOperation {
    Register { did: String, at: u64 },
    Update { did: String, at: u64 },
    Revoke { did: String, at: u64 },
    Transfer { did: String, new_owner: String, at: u64 },
    GrantAccess { did: String, agent: String, level: String, at: u64 },
}

impl DidRegistry {
    pub fn new() -> Self {
        Self {
            registrations: Arc::new(RwLock::new(HashMap::new())),
            access_control: Arc::new(RwLock::new(HashMap::new())),
            nonce_counter: Arc::new(RwLock::new(0)),
            operation_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register a new DID
    pub async fn register(&self, did: String, owner: String, controller: String, document_hash: Vec<u8>) -> Result<u64, String> {
        let mut regs = self.registrations.write().await;

        if regs.contains_key(&did) {
            return Err("DID already registered".to_string());
        }

        let registration = DidRegistration::new(did.clone(), owner, controller, document_hash);
        regs.insert(did.clone(), registration);

        let mut history = self.operation_history.write().await;
        history.push(RegistryOperation::Register { did: did.clone(), at: timestamp_secs() });

        let mut nonce = self.nonce_counter.write().await;
        *nonce += 1;
        Ok(*nonce)
    }

    /// Get DID registration
    pub async fn get(&self, did: &str) -> Option<DidRegistration> {
        let regs = self.registrations.read().await;
        regs.get(did).cloned()
    }

    /// Update DID document hash
    pub async fn update(&self, did: &str, new_document_hash: Vec<u8>, requestor: &str) -> Result<(), String> {
        let mut regs = self.registrations.write().await;
        let reg = regs.get_mut(did).ok_or("DID not found")?;

        // Check authorization
        if reg.owner != requestor && reg.controller != requestor {
            return Err("Not authorized to update".to_string());
        }

        reg.document_hash = new_document_hash;
        reg.updated_at = timestamp_secs();

        let mut history = self.operation_history.write().await;
        history.push(RegistryOperation::Update { did: did.to_string(), at: timestamp_secs() });

        Ok(())
    }

    /// Revoke DID
    pub async fn revoke(&self, did: &str, requestor: &str) -> Result<(), String> {
        let mut regs = self.registrations.write().await;
        let reg = regs.get_mut(did).ok_or("DID not found")?;

        if reg.owner != requestor {
            return Err("Only owner can revoke".to_string());
        }

        reg.revoked = true;

        let mut history = self.operation_history.write().await;
        history.push(RegistryOperation::Revoke { did: did.to_string(), at: timestamp_secs() });

        Ok(())
    }

    /// Transfer ownership
    pub async fn transfer_ownership(&self, did: &str, new_owner: String, requestor: &str) -> Result<(), String> {
        let mut regs = self.registrations.write().await;
        let reg = regs.get_mut(did).ok_or("DID not found")?;

        if reg.owner != requestor {
            return Err("Only owner can transfer".to_string());
        }

        reg.owner = new_owner.clone();
        reg.updated_at = timestamp_secs();

        let mut history = self.operation_history.write().await;
        history.push(RegistryOperation::Transfer { did: did.to_string(), new_owner, at: timestamp_secs() });

        Ok(())
    }

    /// Set DID expiration
    pub async fn set_expiration(&self, did: &str, expires_at: u64, requestor: &str) -> Result<(), String> {
        let mut regs = self.registrations.write().await;
        let reg = regs.get_mut(did).ok_or("DID not found")?;

        if reg.owner != requestor && reg.controller != requestor {
            return Err("Not authorized".to_string());
        }

        if expires_at <= timestamp_secs() {
            return Err("Expiration must be in future".to_string());
        }

        reg.expires_at = Some(expires_at);
        Ok(())
    }

    /// Grant access to DID
    pub async fn grant_access(&self, did: &str, agent: String, level: AccessLevel, requestor: &str) -> Result<(), String> {
        let regs = self.registrations.read().await;
        let reg = regs.get(did).ok_or("DID not found")?;

        if reg.owner != requestor {
            return Err("Only owner can grant access".to_string());
        }

        drop(regs); // Release read lock

        let mut access = self.access_control.write().await;
        let acl = access.entry(did.to_string()).or_insert_with(Vec::new);

        // Remove existing access for this agent
        acl.retain(|ac| ac.agent != agent);

        // Add new access
        acl.push(AccessControl::new(did.to_string(), agent.clone(), level.clone()));

        let mut history = self.operation_history.write().await;
        history.push(RegistryOperation::GrantAccess {
            did: did.to_string(),
            agent,
            level: format!("{:?}", level),
            at: timestamp_secs(),
        });

        Ok(())
    }

    /// Revoke access
    pub async fn revoke_access(&self, did: &str, agent: &str, requestor: &str) -> Result<(), String> {
        let regs = self.registrations.read().await;
        let reg = regs.get(did).ok_or("DID not found")?;

        if reg.owner != requestor {
            return Err("Only owner can revoke access".to_string());
        }

        drop(regs);

        let mut access = self.access_control.write().await;
        if let Some(acl) = access.get_mut(did) {
            acl.retain(|ac| ac.agent != agent);
        }

        Ok(())
    }

    /// Check access level
    pub async fn check_access(&self, did: &str, agent: &str) -> AccessLevel {
        let access = self.access_control.read().await;
        if let Some(acl) = access.get(did) {
            if let Some(ac) = acl.iter().find(|ac| ac.agent == agent) {
                return ac.level.clone();
            }
        }
        AccessLevel::None
    }

    /// Get all DIDs for owner
    pub async fn get_owner_dids(&self, owner: &str) -> Vec<String> {
        let regs = self.registrations.read().await;
        regs
            .iter()
            .filter(|(_, reg)| reg.owner == owner && !reg.revoked)
            .map(|(did, _)| did.clone())
            .collect()
    }

    /// Get active DIDs count
    pub async fn active_count(&self) -> usize {
        let regs = self.registrations.read().await;
        regs.values().filter(|r| r.is_active()).count()
    }

    /// Get total registrations
    pub async fn total_count(&self) -> usize {
        self.registrations.read().await.len()
    }

    /// Get operation history
    pub async fn get_history(&self, limit: usize) -> Vec<RegistryOperation> {
        let history = self.operation_history.read().await;
        history
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// Cleanup expired DIDs
    pub async fn cleanup_expired(&self) -> usize {
        let mut regs = self.registrations.write().await;
        let before = regs.len();

        regs.retain(|_, reg| !reg.is_expired());

        before - regs.len()
    }

    /// Get registry statistics
    pub async fn stats(&self) -> RegistryStats {
        let regs = self.registrations.read().await;
        let total = regs.len();
        let active = regs.values().filter(|r| r.is_active()).count();
        let revoked = regs.values().filter(|r| r.revoked).count();

        RegistryStats {
            total_dids: total,
            active_dids: active,
            revoked_dids: revoked,
            operations_count: self.operation_history.read().await.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegistryStats {
    pub total_dids: usize,
    pub active_dids: usize,
    pub revoked_dids: usize,
    pub operations_count: usize,
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
    async fn test_register_did() {
        let registry = DidRegistry::new();
        let result = registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_duplicate_did() {
        let registry = DidRegistry::new();
        registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await.unwrap();

        let result = registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_did() {
        let registry = DidRegistry::new();
        registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await.unwrap();

        let did = registry.get("did:etrid:user1").await;
        assert!(did.is_some());
    }

    #[tokio::test]
    async fn test_update_did() {
        let registry = DidRegistry::new();
        registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await.unwrap();

        let result = registry.update("did:etrid:user1", vec![4, 5, 6], "owner1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_unauthorized() {
        let registry = DidRegistry::new();
        registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await.unwrap();

        let result = registry.update("did:etrid:user1", vec![4, 5, 6], "other").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_revoke_did() {
        let registry = DidRegistry::new();
        registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await.unwrap();

        assert!(registry.revoke("did:etrid:user1", "owner1").await.is_ok());

        let did = registry.get("did:etrid:user1").await.unwrap();
        assert!(did.revoked);
    }

    #[tokio::test]
    async fn test_transfer_ownership() {
        let registry = DidRegistry::new();
        registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await.unwrap();

        registry.transfer_ownership("did:etrid:user1", "owner2".to_string(), "owner1").await.unwrap();

        let did = registry.get("did:etrid:user1").await.unwrap();
        assert_eq!(did.owner, "owner2");
    }

    #[tokio::test]
    async fn test_grant_access() {
        let registry = DidRegistry::new();
        registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await.unwrap();

        assert!(registry.grant_access("did:etrid:user1", "agent1".to_string(), AccessLevel::Reader, "owner1").await.is_ok());
    }

    #[tokio::test]
    async fn test_check_access() {
        let registry = DidRegistry::new();
        registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await.unwrap();

        registry.grant_access("did:etrid:user1", "agent1".to_string(), AccessLevel::Writer, "owner1").await.unwrap();

        let access = registry.check_access("did:etrid:user1", "agent1").await;
        assert_eq!(access, AccessLevel::Writer);
    }

    #[tokio::test]
    async fn test_revoke_access() {
        let registry = DidRegistry::new();
        registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await.unwrap();

        registry.grant_access("did:etrid:user1", "agent1".to_string(), AccessLevel::Reader, "owner1").await.unwrap();
        registry.revoke_access("did:etrid:user1", "agent1", "owner1").await.unwrap();

        let access = registry.check_access("did:etrid:user1", "agent1").await;
        assert_eq!(access, AccessLevel::None);
    }

    #[tokio::test]
    async fn test_get_owner_dids() {
        let registry = DidRegistry::new();
        registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await.unwrap();

        registry.register(
            "did:etrid:user2".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await.unwrap();

        let dids = registry.get_owner_dids("owner1").await;
        assert_eq!(dids.len(), 2);
    }

    #[tokio::test]
    async fn test_stats() {
        let registry = DidRegistry::new();
        registry.register(
            "did:etrid:user1".to_string(),
            "owner1".to_string(),
            "controller1".to_string(),
            vec![1, 2, 3],
        ).await.unwrap();

        let stats = registry.stats().await;
        assert_eq!(stats.total_dids, 1);
        assert_eq!(stats.active_dids, 1);
    }
}
