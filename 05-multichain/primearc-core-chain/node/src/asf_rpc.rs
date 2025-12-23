#![allow(dead_code)]
#![allow(unused_imports)]
//! ASF Finality RPC Endpoints
//!
//! Provides RPC methods for querying ASF finality state.
//! Used by block explorers and monitoring tools.

use jsonrpsee::{
    core::{async_trait, RpcResult},
    proc_macros::rpc,
    types::error::{ErrorCode, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};
use std::sync::Arc;
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// ASF Finality RPC API
#[rpc(server, client, namespace = "asf")]
pub trait AsfFinalityApi<BlockHash> {
    /// Get the last finalized block info
    #[method(name = "lastFinalized")]
    async fn last_finalized(&self) -> RpcResult<FinalizedBlockInfo>;

    /// Get finality status for a specific block
    #[method(name = "getFinalityStatus")]
    async fn get_finality_status(&self, hash: BlockHash) -> RpcResult<BlockFinalityStatus>;

    /// Get the current finality lag (best - finalized)
    #[method(name = "finalityLag")]
    async fn finality_lag(&self) -> RpcResult<u32>;

    /// Check if a block is finalized
    #[method(name = "isFinalized")]
    async fn is_finalized(&self, hash: BlockHash) -> RpcResult<bool>;

    /// Get certificates for a block
    #[method(name = "getCertificates")]
    async fn get_certificates(&self, hash: BlockHash) -> RpcResult<Vec<CertificateInfo>>;

    /// Get current vote status for the ongoing view
    #[method(name = "voteStatus")]
    async fn vote_status(&self) -> RpcResult<VoteStatusInfo>;

    /// Get checkpoint info
    #[method(name = "getCheckpoint")]
    async fn get_checkpoint(&self, block_number: u32) -> RpcResult<Option<CheckpointInfo>>;

    /// Get latest checkpoint
    #[method(name = "latestCheckpoint")]
    async fn latest_checkpoint(&self) -> RpcResult<Option<CheckpointInfo>>;

    /// Get equivocation reports
    #[method(name = "getEquivocations")]
    async fn get_equivocations(&self) -> RpcResult<Vec<EquivocationInfo>>;

    /// Get ASF finality statistics
    #[method(name = "stats")]
    async fn stats(&self) -> RpcResult<AsfStats>;

    /// Get implicit finality status
    #[method(name = "implicitFinalityStatus")]
    async fn implicit_finality_status(&self, block_number: u32) -> RpcResult<ImplicitFinalityInfo>;
}

/// Information about a finalized block
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FinalizedBlockInfo {
    pub hash: String,
    pub number: u32,
    pub finalized_at: u64,
    pub certificate_count: u32,
    pub finality_level: String,
}

/// Finality status for a block
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockFinalityStatus {
    pub hash: String,
    pub number: u32,
    pub state: String, // "Pending", "PreCommitted", "Committed", "Finalized", "Checkpointed"
    pub certificates: u32,
    pub is_finalized: bool,
    pub depth: u32,
    pub confidence: f64,
}

/// Certificate information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CertificateInfo {
    pub view: u64,
    pub block_hash: String,
    pub block_number: u32,
    pub signature_count: u32,
    pub created_at: u64,
    pub validators: Vec<String>,
}

/// Current vote status
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoteStatusInfo {
    pub current_view: u64,
    pub block_hash: Option<String>,
    pub block_number: Option<u32>,
    pub votes_received: u32,
    pub votes_needed: u32,
    pub validators_voted: Vec<String>,
    pub validators_pending: Vec<String>,
    pub time_in_view_ms: u64,
}

/// Checkpoint information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckpointInfo {
    pub block_number: u32,
    pub block_hash: String,
    pub state_root: String,
    pub signature_count: u32,
    pub created_at: u64,
    pub parent_checkpoint: Option<u32>,
}

/// Equivocation report
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EquivocationInfo {
    pub validator_id: String,
    pub view: u64,
    pub first_block: String,
    pub second_block: String,
    pub detected_at: u64,
    pub slashed: bool,
    pub slash_amount: Option<u128>,
}

/// ASF statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AsfStats {
    pub current_view: u64,
    pub last_finalized_block: u32,
    pub last_finalized_hash: String,
    pub finality_lag: u32,
    pub total_certificates: u64,
    pub total_checkpoints: u32,
    pub total_equivocations: u32,
    pub active_validators: u32,
    pub implicit_finalized: u32,
    pub average_finality_time_ms: u64,
}

/// Implicit finality information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImplicitFinalityInfo {
    pub block_number: u32,
    pub is_implicitly_final: bool,
    pub depth: u32,
    pub confidence: f64,
    pub confirmations_needed: u32,
    pub explicit_finality: bool,
}

/// ASF RPC implementation
pub struct AsfFinality<C, Block> {
    client: Arc<C>,
    /// ASF finality state
    finality_state: Arc<AsfFinalityState>,
    _phantom: std::marker::PhantomData<Block>,
}

/// Shared finality state
pub struct AsfFinalityState {
    pub current_view: std::sync::atomic::AtomicU64,
    pub last_finalized_number: std::sync::atomic::AtomicU32,
    pub last_finalized_hash: tokio::sync::RwLock<Option<[u8; 32]>>,
    pub total_certificates: std::sync::atomic::AtomicU64,
    pub total_checkpoints: std::sync::atomic::AtomicU32,
    pub equivocations: tokio::sync::RwLock<Vec<EquivocationInfo>>,
    pub vote_status: tokio::sync::RwLock<Option<VoteStatusInfo>>,
    pub best_block: std::sync::atomic::AtomicU32,
    pub confirmation_depth: u32,
}

impl AsfFinalityState {
    pub fn new(confirmation_depth: u32) -> Self {
        Self {
            current_view: std::sync::atomic::AtomicU64::new(0),
            last_finalized_number: std::sync::atomic::AtomicU32::new(0),
            last_finalized_hash: tokio::sync::RwLock::new(None),
            total_certificates: std::sync::atomic::AtomicU64::new(0),
            total_checkpoints: std::sync::atomic::AtomicU32::new(0),
            equivocations: tokio::sync::RwLock::new(Vec::new()),
            vote_status: tokio::sync::RwLock::new(None),
            best_block: std::sync::atomic::AtomicU32::new(0),
            confirmation_depth,
        }
    }

    pub async fn update_finalized(&self, number: u32, hash: [u8; 32]) {
        self.last_finalized_number.store(number, std::sync::atomic::Ordering::SeqCst);
        *self.last_finalized_hash.write().await = Some(hash);
    }

    pub fn update_view(&self, view: u64) {
        self.current_view.store(view, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn increment_certificates(&self) {
        self.total_certificates.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn update_best(&self, number: u32) {
        self.best_block.store(number, std::sync::atomic::Ordering::SeqCst);
    }
}

impl<C, Block> AsfFinality<C, Block>
where
    Block: BlockT,
    C: HeaderBackend<Block> + Send + Sync + 'static,
{
    /// Create a new ASF RPC handler
    pub fn new(client: Arc<C>, finality_state: Arc<AsfFinalityState>) -> Self {
        Self {
            client,
            finality_state,
            _phantom: std::marker::PhantomData,
        }
    }

    fn err(msg: &str) -> ErrorObject<'static> {
        ErrorObject::owned(
            ErrorCode::InternalError.code(),
            msg.to_string(),
            None::<()>,
        )
    }
}

use sp_runtime::traits::NumberFor;

#[async_trait]
impl<C, Block> AsfFinalityApiServer<Block::Hash> for AsfFinality<C, Block>
where
    Block: BlockT,
    C: HeaderBackend<Block> + Send + Sync + 'static,
    NumberFor<Block>: Into<u32>,
{
    async fn last_finalized(&self) -> RpcResult<FinalizedBlockInfo> {
        let number = self.finality_state.last_finalized_number.load(std::sync::atomic::Ordering::SeqCst);
        let hash = self.finality_state.last_finalized_hash.read().await
            .map(|h| hex::encode(h))
            .unwrap_or_default();

        Ok(FinalizedBlockInfo {
            hash,
            number,
            finalized_at: 0, // TODO: Track timestamp
            certificate_count: 3, // Finalized = 3 certs
            finality_level: "Finalized".to_string(),
        })
    }

    async fn get_finality_status(&self, hash: Block::Hash) -> RpcResult<BlockFinalityStatus> {
        let header = self.client.header(hash)
            .map_err(|e| Self::err(&format!("Failed to get header: {:?}", e)))?
            .ok_or_else(|| Self::err("Block not found"))?;

        let number: u32 = (*header.number()).into();
        let finalized = self.finality_state.last_finalized_number.load(std::sync::atomic::Ordering::SeqCst);
        let best = self.finality_state.best_block.load(std::sync::atomic::Ordering::SeqCst);
        let depth = best.saturating_sub(number);

        let (state, is_finalized, certificates) = if number <= finalized {
            ("Finalized".to_string(), true, 3)
        } else {
            ("Pending".to_string(), false, 0)
        };

        let confidence = match depth {
            0 => 0.5,
            1 => 0.75,
            2 => 0.875,
            3 => 0.9375,
            4 => 0.96875,
            5 => 0.984375,
            _ => 0.99,
        };

        Ok(BlockFinalityStatus {
            hash: format!("{:?}", hash),
            number,
            state,
            certificates,
            is_finalized,
            depth,
            confidence,
        })
    }

    async fn finality_lag(&self) -> RpcResult<u32> {
        let best = self.finality_state.best_block.load(std::sync::atomic::Ordering::SeqCst);
        let finalized = self.finality_state.last_finalized_number.load(std::sync::atomic::Ordering::SeqCst);
        Ok(best.saturating_sub(finalized))
    }

    async fn is_finalized(&self, hash: Block::Hash) -> RpcResult<bool> {
        let header = self.client.header(hash)
            .map_err(|e| Self::err(&format!("Failed to get header: {:?}", e)))?
            .ok_or_else(|| Self::err("Block not found"))?;

        let number: u32 = (*header.number()).into();
        let finalized = self.finality_state.last_finalized_number.load(std::sync::atomic::Ordering::SeqCst);

        Ok(number <= finalized)
    }

    async fn get_certificates(&self, _hash: Block::Hash) -> RpcResult<Vec<CertificateInfo>> {
        // TODO: Implement certificate storage lookup
        Ok(Vec::new())
    }

    async fn vote_status(&self) -> RpcResult<VoteStatusInfo> {
        let status = self.finality_state.vote_status.read().await;
        Ok(status.clone().unwrap_or(VoteStatusInfo {
            current_view: self.finality_state.current_view.load(std::sync::atomic::Ordering::SeqCst),
            block_hash: None,
            block_number: None,
            votes_received: 0,
            votes_needed: 15,
            validators_voted: Vec::new(),
            validators_pending: Vec::new(),
            time_in_view_ms: 0,
        }))
    }

    async fn get_checkpoint(&self, _block_number: u32) -> RpcResult<Option<CheckpointInfo>> {
        // TODO: Implement checkpoint lookup
        Ok(None)
    }

    async fn latest_checkpoint(&self) -> RpcResult<Option<CheckpointInfo>> {
        // TODO: Implement checkpoint lookup
        Ok(None)
    }

    async fn get_equivocations(&self) -> RpcResult<Vec<EquivocationInfo>> {
        Ok(self.finality_state.equivocations.read().await.clone())
    }

    async fn stats(&self) -> RpcResult<AsfStats> {
        let best = self.finality_state.best_block.load(std::sync::atomic::Ordering::SeqCst);
        let finalized = self.finality_state.last_finalized_number.load(std::sync::atomic::Ordering::SeqCst);
        let hash = self.finality_state.last_finalized_hash.read().await
            .map(|h| hex::encode(h))
            .unwrap_or_default();

        Ok(AsfStats {
            current_view: self.finality_state.current_view.load(std::sync::atomic::Ordering::SeqCst),
            last_finalized_block: finalized,
            last_finalized_hash: hash,
            finality_lag: best.saturating_sub(finalized),
            total_certificates: self.finality_state.total_certificates.load(std::sync::atomic::Ordering::SeqCst),
            total_checkpoints: self.finality_state.total_checkpoints.load(std::sync::atomic::Ordering::SeqCst),
            total_equivocations: self.finality_state.equivocations.read().await.len() as u32,
            active_validators: 21,
            implicit_finalized: best.saturating_sub(self.finality_state.confirmation_depth),
            average_finality_time_ms: 6000, // ~6 seconds typical
        })
    }

    async fn implicit_finality_status(&self, block_number: u32) -> RpcResult<ImplicitFinalityInfo> {
        let best = self.finality_state.best_block.load(std::sync::atomic::Ordering::SeqCst);
        let finalized = self.finality_state.last_finalized_number.load(std::sync::atomic::Ordering::SeqCst);
        let depth = best.saturating_sub(block_number);
        let conf_depth = self.finality_state.confirmation_depth;

        let is_implicit = depth >= conf_depth;
        let confidence = match depth {
            0 => 0.5,
            1 => 0.75,
            2 => 0.875,
            3 => 0.9375,
            4 => 0.96875,
            5 => 0.984375,
            _ => 0.99,
        };

        Ok(ImplicitFinalityInfo {
            block_number,
            is_implicitly_final: is_implicit,
            depth,
            confidence,
            confirmations_needed: conf_depth.saturating_sub(depth),
            explicit_finality: block_number <= finalized,
        })
    }
}

/// Create ASF finality RPC extension
pub fn create_asf_rpc<C, Block>(
    client: Arc<C>,
    finality_state: Arc<AsfFinalityState>,
) -> Result<jsonrpsee::RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    Block: BlockT,
    C: HeaderBackend<Block> + Send + Sync + 'static,
    NumberFor<Block>: Into<u32>,
{
    use AsfFinalityApiServer;
    let asf = AsfFinality::<C, Block>::new(client, finality_state);
    let mut module = jsonrpsee::RpcModule::new(());
    module.merge(asf.into_rpc())?;
    Ok(module)
}
