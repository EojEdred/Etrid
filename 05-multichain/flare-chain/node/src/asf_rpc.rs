//! # ASF Consensus RPC Endpoints
//!
//! This module provides RPC endpoints for querying ASF consensus state:
//! - Finality levels for blocks
//! - Validator committee information
//! - Certificate counts
//! - Vote submission (for external validators)
//!
//! ## RPC Methods
//!
//! - `asf_getFinalityLevel(block_hash)` - Get finality level (0-4) for a block
//! - `asf_getValidatorSet()` - Get current active validator committee
//! - `asf_getCertificateCount(block_hash)` - Get certificate count for a block
//! - `asf_submitVote(vote, signature)` - Submit a vote (for external validators)
//! - `asf_getBlockStatus(block_hash)` - Get comprehensive status of a block
//! - `asf_getCommitteeInfo()` - Get detailed committee information
//! - `asf_getSlashingHistory(limit)` - Get recent slashing events

use std::sync::Arc;
use jsonrpsee::{
    core::{async_trait, RpcResult},
    proc_macros::rpc,
    types::ErrorObjectOwned,
};
use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use codec::{Encode, Decode};

// ASF algorithm types
use asf_algorithm::{
    FinalityLevel, ValidatorId, Hash, BlockNumber,
    Vote, Certificate, ConsensusPhase,
};

// ═══════════════════════════════════════════════════════════════════════════════
// RPC TRAIT DEFINITION
// ═══════════════════════════════════════════════════════════════════════════════

/// ASF RPC API
#[rpc(client, server)]
pub trait AsfRpcApi {
    /// Get finality level for a block
    #[method(name = "asf_getFinalityLevel")]
    async fn get_finality_level(&self, block_hash: Hash) -> RpcResult<FinalityLevelInfo>;

    /// Get current validator committee
    #[method(name = "asf_getValidatorSet")]
    async fn get_validator_set(&self) -> RpcResult<Vec<ValidatorInfo>>;

    /// Get certificate count for a block
    #[method(name = "asf_getCertificateCount")]
    async fn get_certificate_count(&self, block_hash: Hash) -> RpcResult<u32>;

    /// Submit a vote (for external validators)
    #[method(name = "asf_submitVote")]
    async fn submit_vote(&self, vote_hex: String, signature_hex: String) -> RpcResult<VoteSubmissionResult>;

    /// Get comprehensive block status
    #[method(name = "asf_getBlockStatus")]
    async fn get_block_status(&self, block_hash: Hash) -> RpcResult<BlockStatus>;

    /// Get detailed committee information
    #[method(name = "asf_getCommitteeInfo")]
    async fn get_committee_info(&self) -> RpcResult<CommitteeInfo>;

    /// Get recent slashing events
    #[method(name = "asf_getSlashingHistory")]
    async fn get_slashing_history(&self, limit: Option<u32>) -> RpcResult<Vec<SlashingEventInfo>>;
}

// ═══════════════════════════════════════════════════════════════════════════════
// RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Finality level information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalityLevelInfo {
    /// Block hash
    pub block_hash: String,
    /// Block number
    pub block_number: BlockNumber,
    /// Finality level (0-4)
    pub level: u8,
    /// Level name
    pub level_name: String,
    /// Certificate count
    pub certificate_count: u32,
    /// Is finalized (level > 0)
    pub is_finalized: bool,
    /// Time to finalization (ms, if available)
    pub time_to_finalization_ms: Option<u64>,
}

/// Validator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    /// Validator ID (hex)
    pub validator_id: String,
    /// Stake weight
    pub stake: u128,
    /// Is active in current committee
    pub is_active: bool,
    /// Is excluded (slashed)
    pub is_excluded: bool,
    /// Current role (if any)
    pub role: Option<String>,
}

/// Vote submission result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteSubmissionResult {
    /// Success status
    pub success: bool,
    /// Message
    pub message: String,
    /// Vote hash (if successful)
    pub vote_hash: Option<String>,
}

/// Comprehensive block status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStatus {
    /// Block hash
    pub block_hash: String,
    /// Block number
    pub block_number: BlockNumber,
    /// Finality level
    pub finality_level: FinalityLevelInfo,
    /// Certificates by phase
    pub certificates_by_phase: CertificatesByPhase,
    /// Total vote count across all phases
    pub total_votes: u32,
    /// Proposer ID
    pub proposer: Option<String>,
}

/// Certificates grouped by consensus phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatesByPhase {
    pub prepare: u32,
    pub pre_commit: u32,
    pub commit: u32,
    pub decide: u32,
}

/// Committee information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeInfo {
    /// Current epoch
    pub epoch: u32,
    /// Committee size
    pub size: u32,
    /// Total active stake
    pub total_stake: u128,
    /// Active validators
    pub validators: Vec<ValidatorInfo>,
    /// BFT threshold (2/3 + 1)
    pub bft_threshold: u32,
    /// Stake threshold
    pub stake_threshold: u128,
}

/// Slashing event information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingEventInfo {
    /// Validator ID
    pub validator_id: String,
    /// Severity level
    pub severity: String,
    /// Reason
    pub reason: String,
    /// Amount slashed
    pub amount_slashed: u128,
    /// Epoch
    pub epoch: u32,
    /// Timestamp
    pub timestamp: u64,
    /// Was excluded
    pub excluded: bool,
}

// ═══════════════════════════════════════════════════════════════════════════════
// RPC IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════════

/// ASF RPC handler
pub struct AsfRpc<C, Block> {
    client: Arc<C>,
    _phantom: std::marker::PhantomData<Block>,
}

impl<C, Block> AsfRpc<C, Block> {
    /// Create a new ASF RPC handler
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Convert FinalityLevel to level name
    fn level_to_name(level: FinalityLevel) -> &'static str {
        match level {
            FinalityLevel::None => "None",
            FinalityLevel::Weak => "Weak",
            FinalityLevel::Moderate => "Moderate",
            FinalityLevel::Strong => "Strong",
            FinalityLevel::Irreversible => "Irreversible",
        }
    }

    /// Convert FinalityLevel to numeric value
    fn level_to_u8(level: FinalityLevel) -> u8 {
        match level {
            FinalityLevel::None => 0,
            FinalityLevel::Weak => 1,
            FinalityLevel::Moderate => 2,
            FinalityLevel::Strong => 3,
            FinalityLevel::Irreversible => 4,
        }
    }
}

#[async_trait]
impl<C, Block> AsfRpcApiServer for AsfRpc<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: pallet_validator_committee_runtime_api::ValidatorCommitteeApi<Block>,
{
    async fn get_finality_level(&self, block_hash: Hash) -> RpcResult<FinalityLevelInfo> {
        // TODO: Query finality tracker from runtime or local state
        // For now, return a mock response

        // In production, this would:
        // 1. Query the FinalityTracker from runtime storage
        // 2. Get certificate count for the block
        // 3. Calculate finality level

        let certificate_count = 0u32; // TODO: Get from runtime
        let level = FinalityLevel::from(certificate_count);

        Ok(FinalityLevelInfo {
            block_hash: hex::encode(block_hash.as_ref()),
            block_number: 0, // TODO: Get from header
            level: Self::level_to_u8(level),
            level_name: Self::level_to_name(level).to_string(),
            certificate_count,
            is_finalized: level.is_finalized(),
            time_to_finalization_ms: None,
        })
    }

    async fn get_validator_set(&self) -> RpcResult<Vec<ValidatorInfo>> {
        let best_hash = self.client.info().best_hash;

        // Query runtime API for validator committee
        let validators = self
            .client
            .runtime_api()
            .validator_committee(best_hash)
            .map_err(|e| {
                ErrorObjectOwned::owned(
                    1,
                    "Failed to query validator committee",
                    Some(format!("{:?}", e)),
                )
            })?;

        // Convert to ValidatorInfo format
        let validator_infos: Vec<ValidatorInfo> = validators
            .iter()
            .map(|v| ValidatorInfo {
                validator_id: hex::encode(&v.validator_id.encode()),
                stake: v.stake,
                is_active: v.is_active,
                is_excluded: false, // TODO: Query from slashing module
                role: None, // TODO: Determine role (Leader, Director, etc.)
            })
            .collect();

        Ok(validator_infos)
    }

    async fn get_certificate_count(&self, block_hash: Hash) -> RpcResult<u32> {
        // TODO: Query certificate collection from runtime storage
        // For now, return 0
        Ok(0)
    }

    async fn submit_vote(&self, vote_hex: String, signature_hex: String) -> RpcResult<VoteSubmissionResult> {
        // Decode vote from hex
        let vote_bytes = hex::decode(&vote_hex).map_err(|e| {
            ErrorObjectOwned::owned(2, "Invalid vote hex", Some(e.to_string()))
        })?;

        let vote: Vote = Decode::decode(&mut &vote_bytes[..]).map_err(|e| {
            ErrorObjectOwned::owned(3, "Failed to decode vote", Some(e.to_string()))
        })?;

        // TODO: Validate and process vote
        // This would:
        // 1. Verify signature matches validator
        // 2. Check validator is in committee
        // 3. Add vote to vote collection
        // 4. Check if threshold reached for certificate generation

        Ok(VoteSubmissionResult {
            success: true,
            message: "Vote submitted successfully (mock)".to_string(),
            vote_hash: Some(hex::encode(vote.block_hash.as_ref())),
        })
    }

    async fn get_block_status(&self, block_hash: Hash) -> RpcResult<BlockStatus> {
        let finality_level = self.get_finality_level(block_hash).await?;

        Ok(BlockStatus {
            block_hash: hex::encode(block_hash.as_ref()),
            block_number: finality_level.block_number,
            finality_level,
            certificates_by_phase: CertificatesByPhase {
                prepare: 0,
                pre_commit: 0,
                commit: 0,
                decide: 0,
            },
            total_votes: 0,
            proposer: None,
        })
    }

    async fn get_committee_info(&self) -> RpcResult<CommitteeInfo> {
        let validators = self.get_validator_set().await?;
        let size = validators.len() as u32;
        let total_stake: u128 = validators.iter().map(|v| v.stake).sum();

        Ok(CommitteeInfo {
            epoch: 0, // TODO: Get from runtime
            size,
            total_stake,
            validators,
            bft_threshold: asf_algorithm::bft_threshold(size),
            stake_threshold: asf_algorithm::bft_stake_threshold(total_stake),
        })
    }

    async fn get_slashing_history(&self, limit: Option<u32>) -> RpcResult<Vec<SlashingEventInfo>> {
        // TODO: Query slashing events from runtime storage
        // For now, return empty array
        Ok(vec![])
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Create ASF RPC extension
pub fn create_asf_rpc<C, Block>(
    client: Arc<C>,
) -> jsonrpsee::RpcModule<AsfRpc<C, Block>>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: pallet_validator_committee_runtime_api::ValidatorCommitteeApi<Block>,
{
    AsfRpc::new(client).into_rpc()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finality_level_conversion() {
        assert_eq!(AsfRpc::<(), ()>::level_to_u8(FinalityLevel::None), 0);
        assert_eq!(AsfRpc::<(), ()>::level_to_u8(FinalityLevel::Weak), 1);
        assert_eq!(AsfRpc::<(), ()>::level_to_u8(FinalityLevel::Moderate), 2);
        assert_eq!(AsfRpc::<(), ()>::level_to_u8(FinalityLevel::Strong), 3);
        assert_eq!(AsfRpc::<(), ()>::level_to_u8(FinalityLevel::Irreversible), 4);
    }

    #[test]
    fn test_level_names() {
        assert_eq!(AsfRpc::<(), ()>::level_to_name(FinalityLevel::None), "None");
        assert_eq!(AsfRpc::<(), ()>::level_to_name(FinalityLevel::Weak), "Weak");
        assert_eq!(AsfRpc::<(), ()>::level_to_name(FinalityLevel::Moderate), "Moderate");
        assert_eq!(AsfRpc::<(), ()>::level_to_name(FinalityLevel::Strong), "Strong");
        assert_eq!(AsfRpc::<(), ()>::level_to_name(FinalityLevel::Irreversible), "Irreversible");
    }
}
