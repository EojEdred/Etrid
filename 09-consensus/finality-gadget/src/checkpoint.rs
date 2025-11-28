//! Checkpoint BFT System
//!
//! Creates permanent, irreversible anchor points in the blockchain.
//! Checkpoints provide the strongest finality guarantee.
//!
//! ## Properties
//!
//! - Created every N blocks (configurable)
//! - Requires ALL validator signatures (unanimous)
//! - Forms a chain (each checkpoint references the previous)
//! - Stored both on-chain and externally for disaster recovery
//!
//! ## Use Cases
//!
//! - Disaster recovery: Restore from known-good checkpoint
//! - Long-range attack prevention: Rejects chains not descending from checkpoint
//! - Cross-chain bridges: Provide finality proofs to other chains
//! - Light clients: Sync from recent checkpoint instead of genesis

use codec::{Decode, Encode};
use sp_core::sr25519::{Public, Signature};
use sp_core::Pair as PairT;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for checkpoint BFT
#[derive(Clone, Debug)]
pub struct CheckpointConfig {
    /// Blocks between checkpoints (default: 100)
    pub checkpoint_interval: u32,
    /// Whether checkpointing is enabled
    pub enabled: bool,
    /// Timeout for collecting signatures (seconds)
    pub signature_timeout_secs: u64,
    /// Whether to store checkpoints externally
    pub external_storage_enabled: bool,
}

impl Default for CheckpointConfig {
    fn default() -> Self {
        Self {
            checkpoint_interval: 100,
            enabled: true,
            signature_timeout_secs: 60,
            external_storage_enabled: true,
        }
    }
}

/// A checkpoint - permanent anchor point
#[derive(Clone, Debug, Encode, Decode, PartialEq)]
pub struct Checkpoint {
    /// Block number of this checkpoint
    pub block_number: u32,
    /// Block hash
    pub block_hash: [u8; 32],
    /// State root at this block
    pub state_root: [u8; 32],
    /// Extrinsics root at this block
    pub extrinsics_root: [u8; 32],
    /// Signatures from ALL validators
    pub signatures: Vec<CheckpointSignature>,
    /// Previous checkpoint hash (forms a chain)
    pub parent_checkpoint: Option<[u8; 32]>,
    /// Timestamp of checkpoint creation
    pub created_at: u64,
    /// Checkpoint version for future upgrades
    pub version: u8,
}

/// A signature on a checkpoint
#[derive(Clone, Debug, Encode, Decode, PartialEq)]
pub struct CheckpointSignature {
    /// Validator's public key
    pub validator_id: [u8; 32],
    /// Sr25519 signature
    pub signature: [u8; 64],
    /// Timestamp when signed
    pub signed_at: u64,
}

impl Checkpoint {
    /// Get the hash of this checkpoint
    pub fn hash(&self) -> [u8; 32] {
        use sp_core::hashing::blake2_256;
        // Hash everything except signatures for the signing message
        let data = (
            self.block_number,
            self.block_hash,
            self.state_root,
            self.extrinsics_root,
            self.parent_checkpoint,
            self.version,
        );
        blake2_256(&data.encode())
    }

    /// Get the message to sign
    pub fn signing_message(&self) -> Vec<u8> {
        let mut msg = b"CHECKPOINT-V1:".to_vec();
        msg.extend(self.hash());
        msg
    }

    /// Verify all signatures
    pub fn verify_signatures(&self, expected_validators: &[[u8; 32]]) -> CheckpointVerifyResult {
        // Check we have signatures from all expected validators
        if self.signatures.len() != expected_validators.len() {
            return CheckpointVerifyResult::MissingSignatures {
                have: self.signatures.len(),
                need: expected_validators.len(),
            };
        }

        let message = self.signing_message();
        let mut seen_validators = std::collections::HashSet::new();

        for sig in &self.signatures {
            // Check validator is in expected set
            if !expected_validators.contains(&sig.validator_id) {
                return CheckpointVerifyResult::UnknownValidator(sig.validator_id);
            }

            // Check for duplicates
            if !seen_validators.insert(sig.validator_id) {
                return CheckpointVerifyResult::DuplicateSignature(sig.validator_id);
            }

            // Verify signature
            let public = Public::from_raw(sig.validator_id);
            let signature = Signature::from_raw(sig.signature);

            if !sp_core::sr25519::Pair::verify(&signature, message.as_slice(), &public) {
                return CheckpointVerifyResult::InvalidSignature(sig.validator_id);
            }
        }

        // Check all expected validators signed
        for validator in expected_validators {
            if !seen_validators.contains(validator) {
                return CheckpointVerifyResult::MissingValidator(*validator);
            }
        }

        CheckpointVerifyResult::Valid
    }

    /// Check if this checkpoint is for a given block
    pub fn is_for_block(&self, number: u32, hash: &[u8; 32]) -> bool {
        self.block_number == number && &self.block_hash == hash
    }
}

/// Result of checkpoint verification
#[derive(Debug, Clone)]
pub enum CheckpointVerifyResult {
    Valid,
    MissingSignatures { have: usize, need: usize },
    UnknownValidator([u8; 32]),
    DuplicateSignature([u8; 32]),
    InvalidSignature([u8; 32]),
    MissingValidator([u8; 32]),
}

impl CheckpointVerifyResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, CheckpointVerifyResult::Valid)
    }
}

/// Pending checkpoint being assembled
#[derive(Clone, Debug)]
pub struct PendingCheckpoint {
    /// The checkpoint data (without signatures)
    pub checkpoint: Checkpoint,
    /// Signatures collected so far
    pub collected_signatures: Vec<CheckpointSignature>,
    /// Validators we're still waiting for
    pub pending_validators: Vec<[u8; 32]>,
    /// When we started collecting
    pub started_at: u64,
}

impl PendingCheckpoint {
    /// Create a new pending checkpoint
    pub fn new(checkpoint: Checkpoint, validators: Vec<[u8; 32]>) -> Self {
        Self {
            checkpoint,
            collected_signatures: Vec::new(),
            pending_validators: validators,
            started_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Add a signature
    pub fn add_signature(&mut self, sig: CheckpointSignature) -> bool {
        // Check if we're waiting for this validator
        if let Some(pos) = self.pending_validators.iter().position(|v| v == &sig.validator_id) {
            self.pending_validators.remove(pos);
            self.collected_signatures.push(sig);
            true
        } else {
            false
        }
    }

    /// Check if we have all signatures
    pub fn is_complete(&self) -> bool {
        self.pending_validators.is_empty()
    }

    /// Finalize into a complete checkpoint
    pub fn finalize(mut self) -> Option<Checkpoint> {
        if !self.is_complete() {
            return None;
        }

        self.checkpoint.signatures = self.collected_signatures;
        Some(self.checkpoint)
    }

    /// Check if collection has timed out
    pub fn is_timed_out(&self, timeout_secs: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        now > self.started_at + timeout_secs
    }
}

/// Checkpoint BFT manager
pub struct CheckpointBFT {
    config: CheckpointConfig,
    /// Completed checkpoints by block number
    checkpoints: Arc<RwLock<BTreeMap<u32, Checkpoint>>>,
    /// Currently pending checkpoint (if any)
    pending: Arc<RwLock<Option<PendingCheckpoint>>>,
    /// External storage interface (optional)
    external_storage: Option<Arc<dyn CheckpointStorage + Send + Sync>>,
    /// Last checkpoint block number
    last_checkpoint_number: Arc<std::sync::atomic::AtomicU32>,
}

impl CheckpointBFT {
    /// Create a new checkpoint BFT manager
    pub fn new(
        config: CheckpointConfig,
        external_storage: Option<Arc<dyn CheckpointStorage + Send + Sync>>,
    ) -> Self {
        Self {
            config,
            checkpoints: Arc::new(RwLock::new(BTreeMap::new())),
            pending: Arc::new(RwLock::new(None)),
            external_storage,
            last_checkpoint_number: Arc::new(std::sync::atomic::AtomicU32::new(0)),
        }
    }

    /// Check if a block should be a checkpoint
    pub fn should_checkpoint(&self, block_number: u32) -> bool {
        self.config.enabled &&
        block_number > 0 &&
        block_number % self.config.checkpoint_interval == 0
    }

    /// Initialize a checkpoint for a block
    pub async fn init_checkpoint(
        &self,
        block_number: u32,
        block_hash: [u8; 32],
        state_root: [u8; 32],
        extrinsics_root: [u8; 32],
        validators: Vec<[u8; 32]>,
    ) -> Result<(), CheckpointError> {
        if !self.should_checkpoint(block_number) {
            return Err(CheckpointError::NotCheckpointBlock);
        }

        // Check if we already have a pending checkpoint
        if self.pending.read().await.is_some() {
            return Err(CheckpointError::CheckpointInProgress);
        }

        // Get parent checkpoint
        let parent = self.get_latest_checkpoint().await.map(|c| c.hash());

        let checkpoint = Checkpoint {
            block_number,
            block_hash,
            state_root,
            extrinsics_root,
            signatures: Vec::new(),
            parent_checkpoint: parent,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            version: 1,
        };

        let pending = PendingCheckpoint::new(checkpoint, validators);
        *self.pending.write().await = Some(pending);

        tracing::info!(
            "🏁 Checkpoint initiated for block #{} ({:?})",
            block_number,
            hex::encode(&block_hash[..8])
        );

        Ok(())
    }

    /// Add a signature to pending checkpoint
    pub async fn add_signature(
        &self,
        validator_id: [u8; 32],
        signature: [u8; 64],
    ) -> Result<Option<Checkpoint>, CheckpointError> {
        let mut pending_guard = self.pending.write().await;

        let pending = pending_guard
            .as_mut()
            .ok_or(CheckpointError::NoCheckpointPending)?;

        // Check for timeout
        if pending.is_timed_out(self.config.signature_timeout_secs) {
            *pending_guard = None;
            return Err(CheckpointError::Timeout);
        }

        let sig = CheckpointSignature {
            validator_id,
            signature,
            signed_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        // Verify the signature before adding
        let message = pending.checkpoint.signing_message();
        let public = Public::from_raw(validator_id);
        let sig_obj = Signature::from_raw(signature);

        if !sp_core::sr25519::Pair::verify(&sig_obj, message.as_slice(), &public) {
            return Err(CheckpointError::InvalidSignature);
        }

        if !pending.add_signature(sig) {
            return Err(CheckpointError::UnexpectedValidator);
        }

        tracing::debug!(
            "📝 Checkpoint signature received from {:?}, {} pending",
            hex::encode(&validator_id[..8]),
            pending.pending_validators.len()
        );

        // Check if complete
        if pending.is_complete() {
            let completed = pending_guard.take().unwrap().finalize().unwrap();

            // Store the checkpoint
            self.store_checkpoint(completed.clone()).await?;

            tracing::info!(
                "🏁 CHECKPOINT COMPLETE at block #{} with {} signatures",
                completed.block_number,
                completed.signatures.len()
            );

            return Ok(Some(completed));
        }

        Ok(None)
    }

    /// Store a completed checkpoint
    async fn store_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), CheckpointError> {
        let block_number = checkpoint.block_number;

        // Store locally
        self.checkpoints.write().await.insert(block_number, checkpoint.clone());
        self.last_checkpoint_number.store(block_number, std::sync::atomic::Ordering::SeqCst);

        // Store externally if configured
        if let Some(storage) = &self.external_storage {
            if let Err(e) = storage.store(&checkpoint).await {
                tracing::warn!("Failed to store checkpoint externally: {:?}", e);
                // Don't fail - local storage succeeded
            }
        }

        Ok(())
    }

    /// Get the latest checkpoint
    pub async fn get_latest_checkpoint(&self) -> Option<Checkpoint> {
        self.checkpoints.read().await.values().last().cloned()
    }

    /// Get checkpoint at specific block
    pub async fn get_checkpoint(&self, block_number: u32) -> Option<Checkpoint> {
        self.checkpoints.read().await.get(&block_number).cloned()
    }

    /// Verify a block descends from the latest checkpoint
    ///
    /// This prevents long-range attacks by ensuring all blocks can be traced
    /// back to a known checkpoint signed by all validators.
    pub async fn verify_descends_from_checkpoint(
        &self,
        block_number: u32,
        block_hash: [u8; 32],
        get_parent: impl Fn(u32, [u8; 32]) -> Option<(u32, [u8; 32])>,
    ) -> bool {
        let checkpoint = match self.get_latest_checkpoint().await {
            Some(c) => c,
            None => return true, // No checkpoints yet
        };

        // If this block is before the checkpoint, it's invalid
        if block_number < checkpoint.block_number {
            return false;
        }

        // If this is the checkpoint block, verify hash matches
        if block_number == checkpoint.block_number {
            return block_hash == checkpoint.block_hash;
        }

        // Trace back from block to checkpoint
        let mut current_number = block_number;
        let mut current_hash = block_hash;

        while current_number > checkpoint.block_number {
            match get_parent(current_number, current_hash) {
                Some((parent_number, parent_hash)) => {
                    current_number = parent_number;
                    current_hash = parent_hash;
                }
                None => return false, // Can't find parent - invalid
            }
        }

        // Check if we reached the checkpoint
        current_number == checkpoint.block_number && current_hash == checkpoint.block_hash
    }

    /// Get current state
    pub async fn state(&self) -> CheckpointBFTState {
        let checkpoints = self.checkpoints.read().await;
        let pending = self.pending.read().await;

        CheckpointBFTState {
            total_checkpoints: checkpoints.len(),
            latest_checkpoint_block: checkpoints.keys().last().copied(),
            pending_checkpoint: pending.as_ref().map(|p| p.checkpoint.block_number),
            pending_signatures: pending.as_ref().map(|p| p.collected_signatures.len()),
            pending_validators: pending.as_ref().map(|p| p.pending_validators.len()),
        }
    }

    /// Export checkpoint for external storage or cross-chain verification
    pub async fn export_checkpoint(&self, block_number: u32) -> Result<Vec<u8>, CheckpointError> {
        let checkpoint = self
            .get_checkpoint(block_number)
            .await
            .ok_or(CheckpointError::CheckpointNotFound)?;

        Ok(checkpoint.encode())
    }

    /// Import checkpoint from external source
    pub async fn import_checkpoint(&self, data: &[u8]) -> Result<(), CheckpointError> {
        let checkpoint = Checkpoint::decode(&mut &data[..])
            .map_err(|_| CheckpointError::InvalidCheckpointData)?;

        // Verify checkpoint is not from the future
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if checkpoint.created_at > current_time + 300 {
            // Allow 5 min clock skew
            return Err(CheckpointError::CheckpointFromFuture);
        }

        // Store the checkpoint
        self.store_checkpoint(checkpoint).await?;

        Ok(())
    }

    /// Prune old checkpoints, keeping only the last N
    pub async fn prune_checkpoints(&self, keep_count: usize) -> Result<usize, CheckpointError> {
        let mut checkpoints = self.checkpoints.write().await;

        if checkpoints.len() <= keep_count {
            return Ok(0);
        }

        let to_remove = checkpoints.len() - keep_count;
        let keys_to_remove: Vec<u32> = checkpoints.keys().take(to_remove).copied().collect();

        for key in &keys_to_remove {
            checkpoints.remove(key);
        }

        tracing::info!(
            "🧹 Pruned {} old checkpoints, kept {}",
            keys_to_remove.len(),
            keep_count
        );

        Ok(keys_to_remove.len())
    }

    /// Get checkpoint chain from a block back to genesis checkpoint
    pub async fn get_checkpoint_chain(&self, from_block: u32) -> Vec<Checkpoint> {
        let checkpoints = self.checkpoints.read().await;

        checkpoints
            .values()
            .filter(|c| c.block_number <= from_block)
            .cloned()
            .collect()
    }

    /// Verify checkpoint chain integrity
    pub async fn verify_checkpoint_chain(&self) -> Result<bool, CheckpointError> {
        let checkpoints = self.checkpoints.read().await;

        let chain: Vec<&Checkpoint> = checkpoints.values().collect();

        if chain.is_empty() {
            return Ok(true);
        }

        // Verify first checkpoint has no parent
        if chain[0].parent_checkpoint.is_some() {
            tracing::error!("First checkpoint should not have parent");
            return Ok(false);
        }

        // Verify each checkpoint links to previous
        for i in 1..chain.len() {
            let prev_hash = chain[i - 1].hash();
            match chain[i].parent_checkpoint {
                Some(parent) if parent == prev_hash => continue,
                _ => {
                    tracing::error!(
                        "Checkpoint chain broken at block #{} -> #{}",
                        chain[i - 1].block_number,
                        chain[i].block_number
                    );
                    return Ok(false);
                }
            }
        }

        tracing::info!("✅ Checkpoint chain verified: {} checkpoints", chain.len());
        Ok(true)
    }
}

/// Error types for checkpoint operations
#[derive(Debug, Clone)]
pub enum CheckpointError {
    NotCheckpointBlock,
    CheckpointInProgress,
    NoCheckpointPending,
    InvalidSignature,
    UnexpectedValidator,
    Timeout,
    StorageError(String),
    CheckpointNotFound,
    InvalidCheckpointData,
    CheckpointFromFuture,
}

impl std::fmt::Display for CheckpointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotCheckpointBlock => write!(f, "Block is not a checkpoint block"),
            Self::CheckpointInProgress => write!(f, "Checkpoint already in progress"),
            Self::NoCheckpointPending => write!(f, "No checkpoint pending"),
            Self::InvalidSignature => write!(f, "Invalid signature"),
            Self::UnexpectedValidator => write!(f, "Signature from unexpected validator"),
            Self::Timeout => write!(f, "Checkpoint timed out"),
            Self::StorageError(e) => write!(f, "Storage error: {}", e),
            Self::CheckpointNotFound => write!(f, "Checkpoint not found"),
            Self::InvalidCheckpointData => write!(f, "Invalid checkpoint data"),
            Self::CheckpointFromFuture => write!(f, "Checkpoint timestamp is from the future"),
        }
    }
}

impl std::error::Error for CheckpointError {}

/// Current state of checkpoint BFT
#[derive(Debug, Clone)]
pub struct CheckpointBFTState {
    pub total_checkpoints: usize,
    pub latest_checkpoint_block: Option<u32>,
    pub pending_checkpoint: Option<u32>,
    pub pending_signatures: Option<usize>,
    pub pending_validators: Option<usize>,
}

/// External checkpoint storage interface
#[async_trait::async_trait]
pub trait CheckpointStorage {
    /// Store a checkpoint
    async fn store(&self, checkpoint: &Checkpoint) -> Result<(), String>;
    /// Retrieve a checkpoint
    async fn get(&self, block_number: u32) -> Result<Option<Checkpoint>, String>;
    /// Get the latest checkpoint
    async fn get_latest(&self) -> Result<Option<Checkpoint>, String>;
}

impl Clone for CheckpointBFT {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            checkpoints: self.checkpoints.clone(),
            pending: self.pending.clone(),
            external_storage: self.external_storage.clone(),
            last_checkpoint_number: Arc::new(std::sync::atomic::AtomicU32::new(
                self.last_checkpoint_number.load(std::sync::atomic::Ordering::SeqCst)
            )),
        }
    }
}

/// File-based checkpoint storage implementation
pub struct FileCheckpointStorage {
    base_path: std::path::PathBuf,
}

impl FileCheckpointStorage {
    /// Create a new file-based checkpoint storage
    pub fn new(base_path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    /// Ensure storage directory exists
    async fn ensure_dir(&self) -> Result<(), String> {
        tokio::fs::create_dir_all(&self.base_path)
            .await
            .map_err(|e| format!("Failed to create checkpoint directory: {}", e))
    }

    /// Get path for a checkpoint file
    fn checkpoint_path(&self, block_number: u32) -> std::path::PathBuf {
        self.base_path.join(format!("checkpoint-{}.bin", block_number))
    }
}

#[async_trait::async_trait]
impl CheckpointStorage for FileCheckpointStorage {
    async fn store(&self, checkpoint: &Checkpoint) -> Result<(), String> {
        self.ensure_dir().await?;

        let path = self.checkpoint_path(checkpoint.block_number);
        let data = checkpoint.encode();

        tokio::fs::write(&path, data)
            .await
            .map_err(|e| format!("Failed to write checkpoint: {}", e))?;

        tracing::debug!(
            "💾 Checkpoint #{} saved to {:?}",
            checkpoint.block_number,
            path
        );

        Ok(())
    }

    async fn get(&self, block_number: u32) -> Result<Option<Checkpoint>, String> {
        let path = self.checkpoint_path(block_number);

        if !path.exists() {
            return Ok(None);
        }

        let data = tokio::fs::read(&path)
            .await
            .map_err(|e| format!("Failed to read checkpoint: {}", e))?;

        let checkpoint = Checkpoint::decode(&mut &data[..])
            .map_err(|e| format!("Failed to decode checkpoint: {}", e))?;

        Ok(Some(checkpoint))
    }

    async fn get_latest(&self) -> Result<Option<Checkpoint>, String> {
        self.ensure_dir().await?;

        let mut entries = tokio::fs::read_dir(&self.base_path)
            .await
            .map_err(|e| format!("Failed to read checkpoint directory: {}", e))?;

        let mut latest_number = None;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| format!("Failed to read directory entry: {}", e))?
        {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with("checkpoint-") && name.ends_with(".bin") {
                    if let Some(num_str) = name.strip_prefix("checkpoint-").and_then(|s| s.strip_suffix(".bin")) {
                        if let Ok(num) = num_str.parse::<u32>() {
                            latest_number = Some(latest_number.map(|n: u32| n.max(num)).unwrap_or(num));
                        }
                    }
                }
            }
        }

        match latest_number {
            Some(num) => self.get(num).await,
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_checkpoint() {
        let bft = CheckpointBFT::new(CheckpointConfig::default(), None);

        assert!(!bft.should_checkpoint(0));
        assert!(!bft.should_checkpoint(50));
        assert!(bft.should_checkpoint(100));
        assert!(!bft.should_checkpoint(150));
        assert!(bft.should_checkpoint(200));
    }

    #[test]
    fn test_checkpoint_hash() {
        let checkpoint = Checkpoint {
            block_number: 100,
            block_hash: [1u8; 32],
            state_root: [2u8; 32],
            extrinsics_root: [3u8; 32],
            signatures: Vec::new(),
            parent_checkpoint: None,
            created_at: 1234567890,
            version: 1,
        };

        let hash1 = checkpoint.hash();
        let hash2 = checkpoint.hash();

        // Hash should be deterministic
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_pending_checkpoint_lifecycle() {
        let checkpoint = Checkpoint {
            block_number: 100,
            block_hash: [0u8; 32],
            state_root: [0u8; 32],
            extrinsics_root: [0u8; 32],
            signatures: Vec::new(),
            parent_checkpoint: None,
            created_at: 0,
            version: 1,
        };

        let validators = vec![[1u8; 32], [2u8; 32]];
        let mut pending = PendingCheckpoint::new(checkpoint, validators);

        assert!(!pending.is_complete());

        let sig1 = CheckpointSignature {
            validator_id: [1u8; 32],
            signature: [0u8; 64],
            signed_at: 0,
        };

        assert!(pending.add_signature(sig1));
        assert!(!pending.is_complete());

        let sig2 = CheckpointSignature {
            validator_id: [2u8; 32],
            signature: [0u8; 64],
            signed_at: 0,
        };

        assert!(pending.add_signature(sig2));
        assert!(pending.is_complete());

        let completed = pending.finalize();
        assert!(completed.is_some());
    }

    #[tokio::test]
    async fn test_checkpoint_init() {
        let bft = CheckpointBFT::new(CheckpointConfig::default(), None);

        let validators = vec![[1u8; 32], [2u8; 32], [3u8; 32]];

        let result = bft.init_checkpoint(
            100,
            [0u8; 32],
            [0u8; 32],
            [0u8; 32],
            validators,
        ).await;

        assert!(result.is_ok());

        // Should fail on non-checkpoint block
        let result = bft.init_checkpoint(
            50,
            [0u8; 32],
            [0u8; 32],
            [0u8; 32],
            vec![[1u8; 32]],
        ).await;

        assert!(matches!(result, Err(CheckpointError::NotCheckpointBlock)));
    }

    #[tokio::test]
    async fn test_checkpoint_state() {
        let bft = CheckpointBFT::new(CheckpointConfig::default(), None);

        let state = bft.state().await;
        assert_eq!(state.total_checkpoints, 0);
        assert_eq!(state.latest_checkpoint_block, None);
    }

    #[test]
    fn test_checkpoint_verify_result() {
        let result = CheckpointVerifyResult::Valid;
        assert!(result.is_valid());

        let result = CheckpointVerifyResult::InvalidSignature([0u8; 32]);
        assert!(!result.is_valid());
    }

    #[tokio::test]
    async fn test_verify_descends_from_checkpoint() {
        let bft = CheckpointBFT::new(CheckpointConfig::default(), None);

        // Mock parent function: simple linear chain
        let get_parent = |num: u32, _hash: [u8; 32]| -> Option<(u32, [u8; 32])> {
            if num > 0 {
                Some((num - 1, [0u8; 32]))
            } else {
                None
            }
        };

        // No checkpoints yet - should accept any block
        assert!(bft.verify_descends_from_checkpoint(100, [0u8; 32], get_parent).await);

        // Create a checkpoint at block 100
        let checkpoint = Checkpoint {
            block_number: 100,
            block_hash: [0u8; 32],
            state_root: [0u8; 32],
            extrinsics_root: [0u8; 32],
            signatures: Vec::new(),
            parent_checkpoint: None,
            created_at: 0,
            version: 1,
        };

        bft.checkpoints.write().await.insert(100, checkpoint);

        // Block 150 should descend from checkpoint
        assert!(bft.verify_descends_from_checkpoint(150, [0u8; 32], get_parent).await);

        // Block 100 with matching hash should be valid
        assert!(bft.verify_descends_from_checkpoint(100, [0u8; 32], get_parent).await);

        // Block 100 with different hash should be invalid
        assert!(!bft.verify_descends_from_checkpoint(100, [1u8; 32], get_parent).await);

        // Block before checkpoint should be invalid
        assert!(!bft.verify_descends_from_checkpoint(50, [0u8; 32], get_parent).await);
    }

    #[tokio::test]
    async fn test_checkpoint_chain_verification() {
        let bft = CheckpointBFT::new(CheckpointConfig::default(), None);

        // Empty chain is valid
        assert!(bft.verify_checkpoint_chain().await.unwrap());

        // Create valid chain
        let checkpoint1 = Checkpoint {
            block_number: 100,
            block_hash: [1u8; 32],
            state_root: [0u8; 32],
            extrinsics_root: [0u8; 32],
            signatures: Vec::new(),
            parent_checkpoint: None,
            created_at: 0,
            version: 1,
        };

        bft.checkpoints.write().await.insert(100, checkpoint1.clone());

        assert!(bft.verify_checkpoint_chain().await.unwrap());

        let checkpoint2 = Checkpoint {
            block_number: 200,
            block_hash: [2u8; 32],
            state_root: [0u8; 32],
            extrinsics_root: [0u8; 32],
            signatures: Vec::new(),
            parent_checkpoint: Some(checkpoint1.hash()),
            created_at: 0,
            version: 1,
        };

        bft.checkpoints.write().await.insert(200, checkpoint2);

        assert!(bft.verify_checkpoint_chain().await.unwrap());
    }

    #[tokio::test]
    async fn test_prune_checkpoints() {
        let bft = CheckpointBFT::new(CheckpointConfig::default(), None);

        // Add 5 checkpoints
        for i in 1..=5 {
            let checkpoint = Checkpoint {
                block_number: i * 100,
                block_hash: [i as u8; 32],
                state_root: [0u8; 32],
                extrinsics_root: [0u8; 32],
                signatures: Vec::new(),
                parent_checkpoint: None,
                created_at: 0,
                version: 1,
            };
            bft.checkpoints.write().await.insert(i * 100, checkpoint);
        }

        // Prune to keep only 3
        let pruned = bft.prune_checkpoints(3).await.unwrap();
        assert_eq!(pruned, 2);

        let state = bft.state().await;
        assert_eq!(state.total_checkpoints, 3);
        assert_eq!(state.latest_checkpoint_block, Some(500));
    }
}
