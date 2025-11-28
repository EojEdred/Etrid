//! AI DID - Decentralized Identity for AI Agents
//!
//! This module provides functionality for registering and managing decentralized
//! identities for AI agents on the Ëtrid blockchain.

use async_trait::async_trait;
use crate::{Client, Error, Result};
use crate::types::{Address, AiProfile, Permission, TxHash};
use serde::{Deserialize, Serialize};

/// AI DID wrapper for AI agent identity management
pub struct AiDidWrapper {
    client: Client,
}

/// AI registration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAiParams {
    /// AI agent name
    pub name: String,
    /// Model type (e.g., "GPT-4", "Claude", "Custom")
    pub model_type: String,
    /// Initial permissions
    pub permissions: Vec<Permission>,
    /// Optional metadata
    pub metadata: Option<Vec<u8>>,
}

/// Reputation update parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationUpdate {
    /// AI agent ID
    pub ai_id: String,
    /// Score delta (can be negative)
    pub score_delta: i32,
    /// Reason for update
    pub reason: String,
}

/// Permission grant parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGrant {
    /// AI agent ID
    pub ai_id: String,
    /// Permission to grant
    pub permission: Permission,
    /// Expiration block (None = permanent)
    pub expiration: Option<u64>,
}

impl AiDidWrapper {
    /// Create a new AI DID wrapper instance
    ///
    /// # Arguments
    ///
    /// * `client` - Reference to the Ëtrid RPC client
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::AiDidWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("ws://localhost:9944").await?;
    /// let ai_did = AiDidWrapper::new(client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Register a new AI agent
    ///
    /// # Arguments
    ///
    /// * `params` - Registration parameters
    ///
    /// # Returns
    ///
    /// Returns the AI agent ID and transaction hash
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::{AiDidWrapper, RegisterAiParams}};
    /// # use etrid_sdk::types::Permission;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let ai_did = AiDidWrapper::new(client);
    /// let params = RegisterAiParams {
    ///     name: "MyAI Assistant".to_string(),
    ///     model_type: "GPT-4".to_string(),
    ///     permissions: vec![Permission::Read, Permission::Write],
    ///     metadata: None,
    /// };
    /// let (ai_id, tx_hash) = ai_did.register_ai(params).await?;
    /// println!("AI registered with ID: {}", ai_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn register_ai(&self, params: RegisterAiParams) -> Result<(String, TxHash)> {
        // Validate parameters
        if params.name.is_empty() {
            return Err(Error::AiDid("AI name cannot be empty".to_string()));
        }

        if params.model_type.is_empty() {
            return Err(Error::AiDid("Model type cannot be empty".to_string()));
        }

        // Check for duplicate permissions
        let mut unique_perms = params.permissions.clone();
        unique_perms.sort_by_key(|p| format!("{:?}", p));
        unique_perms.dedup();
        if unique_perms.len() != params.permissions.len() {
            return Err(Error::AiDid("Duplicate permissions not allowed".to_string()));
        }

        // In production, submit register_ai extrinsic
        let ai_id = format!("ai_{}", Self::timestamp());
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok((ai_id, tx_hash))
    }

    /// Get AI agent profile
    ///
    /// # Arguments
    ///
    /// * `ai_id` - AI agent ID
    ///
    /// # Returns
    ///
    /// Returns the AI profile
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::AiDidWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let ai_did = AiDidWrapper::new(client);
    /// let profile = ai_did.get_ai_profile("ai_12345").await?;
    /// println!("AI Name: {}", profile.name);
    /// println!("Reputation: {}", profile.reputation);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_ai_profile(&self, ai_id: &str) -> Result<AiProfile> {
        // In production, query AI profile from storage
        let profile = AiProfile {
            id: ai_id.to_string(),
            name: "Example AI".to_string(),
            model_type: "GPT-4".to_string(),
            reputation: 100,
            permissions: vec![Permission::Read, Permission::Write],
        };

        Ok(profile)
    }

    /// Update AI agent reputation
    ///
    /// # Arguments
    ///
    /// * `update` - Reputation update parameters
    ///
    /// # Returns
    ///
    /// Returns the new reputation score and transaction hash
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::{AiDidWrapper, ReputationUpdate}};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let ai_did = AiDidWrapper::new(client);
    /// let update = ReputationUpdate {
    ///     ai_id: "ai_12345".to_string(),
    ///     score_delta: 10,
    ///     reason: "Successful task completion".to_string(),
    /// };
    /// let (new_score, tx_hash) = ai_did.update_reputation(update).await?;
    /// println!("New reputation: {}", new_score);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_reputation(&self, update: ReputationUpdate) -> Result<(u32, TxHash)> {
        // Validate update
        if update.reason.is_empty() {
            return Err(Error::AiDid("Reason cannot be empty".to_string()));
        }

        // Get current profile
        let profile = self.get_ai_profile(&update.ai_id).await?;

        // Calculate new score (with bounds checking)
        let new_score = if update.score_delta >= 0 {
            profile.reputation.saturating_add(update.score_delta as u32)
        } else {
            profile.reputation.saturating_sub((-update.score_delta) as u32)
        };

        // In production, submit reputation update extrinsic
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok((new_score, tx_hash))
    }

    /// Grant permission to an AI agent
    ///
    /// # Arguments
    ///
    /// * `grant` - Permission grant parameters
    ///
    /// # Returns
    ///
    /// Returns the transaction hash
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::{AiDidWrapper, PermissionGrant}};
    /// # use etrid_sdk::types::Permission;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let ai_did = AiDidWrapper::new(client);
    /// let grant = PermissionGrant {
    ///     ai_id: "ai_12345".to_string(),
    ///     permission: Permission::Execute,
    ///     expiration: Some(1000000), // Expires at block 1000000
    /// };
    /// let tx_hash = ai_did.grant_permission(grant).await?;
    /// println!("Permission granted: {}", tx_hash);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn grant_permission(&self, grant: PermissionGrant) -> Result<TxHash> {
        // Verify AI exists
        let profile = self.get_ai_profile(&grant.ai_id).await?;

        // Check if permission already exists
        if profile.permissions.contains(&grant.permission) {
            return Err(Error::AlreadyExists("Permission already granted".to_string()));
        }

        // Validate expiration block
        if let Some(exp_block) = grant.expiration {
            // Get current block
            let current_block = self.client.get_block_number().await?;
            if exp_block <= current_block {
                return Err(Error::AiDid(
                    "Expiration block must be in the future".to_string(),
                ));
            }
        }

        // In production, submit grant_permission extrinsic
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok(tx_hash)
    }

    /// Revoke permission from an AI agent
    ///
    /// # Arguments
    ///
    /// * `ai_id` - AI agent ID
    /// * `permission` - Permission to revoke
    ///
    /// # Returns
    ///
    /// Returns the transaction hash
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::AiDidWrapper};
    /// # use etrid_sdk::types::Permission;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let ai_did = AiDidWrapper::new(client);
    /// let tx_hash = ai_did.revoke_permission("ai_12345", Permission::Execute).await?;
    /// println!("Permission revoked: {}", tx_hash);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn revoke_permission(&self, ai_id: &str, permission: Permission) -> Result<TxHash> {
        // Verify AI exists
        let profile = self.get_ai_profile(ai_id).await?;

        // Check if permission exists
        if !profile.permissions.contains(&permission) {
            return Err(Error::NotFound("Permission not found".to_string()));
        }

        // In production, submit revoke_permission extrinsic
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok(tx_hash)
    }

    /// List all AI agents registered by an account
    ///
    /// # Arguments
    ///
    /// * `owner` - Owner address
    ///
    /// # Returns
    ///
    /// Returns a vector of AI profiles
    pub async fn list_ai_agents(&self, owner: &str) -> Result<Vec<AiProfile>> {
        // In production, query all AI agents for owner
        Ok(vec![])
    }

    /// Update AI agent metadata
    ///
    /// # Arguments
    ///
    /// * `ai_id` - AI agent ID
    /// * `metadata` - New metadata
    ///
    /// # Returns
    ///
    /// Returns the transaction hash
    pub async fn update_metadata(&self, ai_id: &str, metadata: Vec<u8>) -> Result<TxHash> {
        // Verify AI exists
        let _ = self.get_ai_profile(ai_id).await?;

        // In production, submit update_metadata extrinsic
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok(tx_hash)
    }

    /// Get reputation history for an AI agent
    ///
    /// # Arguments
    ///
    /// * `ai_id` - AI agent ID
    /// * `limit` - Maximum number of entries to return
    ///
    /// # Returns
    ///
    /// Returns reputation history
    pub async fn get_reputation_history(
        &self,
        ai_id: &str,
        limit: u32,
    ) -> Result<Vec<ReputationEntry>> {
        // In production, query reputation history from events/storage
        Ok(vec![])
    }

    /// Check if an AI has a specific permission
    ///
    /// # Arguments
    ///
    /// * `ai_id` - AI agent ID
    /// * `permission` - Permission to check
    ///
    /// # Returns
    ///
    /// Returns true if AI has the permission
    pub async fn has_permission(&self, ai_id: &str, permission: &Permission) -> Result<bool> {
        let profile = self.get_ai_profile(ai_id).await?;
        Ok(profile.permissions.contains(permission))
    }

    /// Timestamp helper
    fn timestamp() -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}

/// Reputation history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationEntry {
    /// Timestamp
    pub timestamp: u64,
    /// Score delta
    pub score_delta: i32,
    /// Reason
    pub reason: String,
    /// Previous score
    pub previous_score: u32,
    /// New score
    pub new_score: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reputation_calculation() {
        let current = 100u32;
        let delta = 10i32;
        let new_score = current.saturating_add(delta as u32);
        assert_eq!(new_score, 110);

        let delta = -50i32;
        let new_score = current.saturating_sub((-delta) as u32);
        assert_eq!(new_score, 50);
    }

    #[test]
    fn test_permission_deduplication() {
        let mut perms = vec![Permission::Read, Permission::Write, Permission::Read];
        perms.sort_by_key(|p| format!("{:?}", p));
        perms.dedup();
        assert_eq!(perms.len(), 2);
    }
}
