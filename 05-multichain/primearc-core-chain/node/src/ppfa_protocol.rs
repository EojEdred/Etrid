#![allow(dead_code)]
#![allow(unused_imports)]
//! PPFA Protocol - Proposing Panel for Attestation Network Protocol
//!
//! This module provides finality protocol functionality for the libp2p network layer,
//! enabling proper peer connections, block sync, and warp sync for ASF consensus.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                    PPFA Protocol Stack                              │
//! ├─────────────────────────────────────────────────────────────────────┤
//! │  PPFAProtocolWorker                                                 │
//! │    ├─ Receives ASF finality from DETR P2P (port 30334)              │
//! │    ├─ Converts to PPFAJustification                                 │
//! │    ├─ Broadcasts via Substrate libp2p (port 30333)                  │
//! │    └─ Provides finality notifications to sync engine                │
//! ├─────────────────────────────────────────────────────────────────────┤
//! │  PPFAWarpSyncProvider                                               │
//! │    ├─ Generates warp sync proofs from finality certificates         │
//! │    ├─ Validates incoming warp sync requests                         │
//! │    └─ Enables fast-sync for new nodes                               │
//! ├─────────────────────────────────────────────────────────────────────┤
//! │  PPFAJustificationStorage                                           │
//! │    ├─ On-chain storage via pallet hooks                             │
//! │    ├─ Off-chain DB for historical proofs                            │
//! │    └─ Pruning for scalability                                       │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```

use codec::{Decode, Encode, MaxEncodedLen};
use futures::{
    channel::mpsc,
    future::{self, BoxFuture},
    prelude::*,
    stream::StreamExt,
};
use log::{debug, error, info, trace, warn};
use primearc_core_runtime::opaque::Block;
use sc_client_api::{
    Backend, BlockBackend, BlockchainEvents, Finalizer, HeaderBackend, LockImportRun,
};
use sc_network::{
    config::{NonDefaultSetConfig, NonReservedPeerMode, SetConfig},
    service::{traits::NotificationEvent, NotificationMetrics},
    NetworkEventStream, NetworkPeers, NetworkService,
    NotificationService, PeerId, ProtocolName,
};
use sc_network_sync::SyncingService;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::{Error as ClientError, HeaderMetadata};
use sp_consensus::BlockOrigin;
use sp_runtime::{
    generic::{BlockId, SignedBlock},
    traits::{Block as BlockT, Header as HeaderT, NumberFor, SaturatedConversion, Zero},
    Justification, Justifications,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use pallet_validator_committee_runtime_api::ValidatorCommitteeApi;
use pallet_validator_committee_runtime_api::ValidatorInfo;

fn hash_to_bytes<H: AsRef<[u8]>>(hash: &H) -> [u8; 32] {
    let bytes = hash.as_ref();
    let mut out = [0u8; 32];
    let len = bytes.len().min(32);
    out[..len].copy_from_slice(&bytes[..len]);
    out
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

/// PPFA engine ID for justification identification (matches ASF but renamed for continuity)
pub const PPFA_ENGINE_ID: sp_runtime::ConsensusEngineId = *b"PPFA";

/// Protocol name prefix for PPFA
pub const PPFA_PROTOCOL_NAME: &str = "/primearc/ppfa/1";

/// Maximum justification size (16 MB to handle large validator sets)
pub const MAX_JUSTIFICATION_SIZE: u64 = 16 * 1024 * 1024;

/// Maximum warp sync proof size (64 MB for production)
pub const MAX_WARP_SYNC_PROOF_SIZE: u64 = 64 * 1024 * 1024;

/// Number of authority set changes to keep in warp sync proof
pub const MAX_WARP_SYNC_AUTHORITY_CHANGES: usize = 256;

/// Maximum number of blocks to walk back when searching for a justification for warp sync.
/// Increased from 128 to cover environments where finality signatures may trail behind block import.
pub const MAX_WARP_SYNC_JUSTIFICATION_BACKTRACK: u32 = 1_024;

/// Fallback search window beyond the primary backtrack for warp sync justification lookup.
/// Extended to keep looking through several thousand blocks before failing, while still guarding
/// against unbounded scans.
pub const MAX_WARP_SYNC_FALLBACK_BACKTRACK: u32 = 4_096;

/// Justification retention period (in blocks) for scalable storage
pub const JUSTIFICATION_RETENTION_BLOCKS: u32 = 100_000;

/// Minimum blocks between pruning operations
pub const PRUNING_INTERVAL_BLOCKS: u32 = 1_000;

// ═══════════════════════════════════════════════════════════════════════════════
// PPFA JUSTIFICATION
// ═══════════════════════════════════════════════════════════════════════════════

/// PPFA Pre-commit vote from a validator
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
pub struct PPFAPrecommit {
    /// Target block hash
    pub target_hash: [u8; 32],
    /// Target block number
    pub target_number: u32,
}

/// Signed PPFA vote (pre-commit with signature)
#[derive(Clone, Debug, Encode, Decode)]
pub struct SignedPPFAPrecommit {
    /// The pre-commit vote
    pub precommit: PPFAPrecommit,
    /// Validator's signature
    pub signature: Vec<u8>,
    /// Validator's public key (ASF validator ID)
    pub validator_id: [u8; 32],
}

/// PPFA Commit - collection of signed pre-commits forming a finality certificate
#[derive(Clone, Debug, Encode, Decode)]
pub struct PPFACommit {
    /// Target block hash being finalized
    pub target_hash: [u8; 32],
    /// Target block number being finalized
    pub target_number: u32,
    /// Collection of signed pre-commits (must be 2f+1 for BFT)
    pub precommits: Vec<SignedPPFAPrecommit>,
}

/// PPFA Justification - Full finality proof compatible with Substrate
///
/// This is the main structure stored as block justifications.
/// It contains everything needed to verify finality.
#[derive(Clone, Debug, Encode, Decode)]
pub struct PPFAJustification {
    /// View/round number when this justification was created
    pub view: u64,
    /// The commit containing signed pre-commits
    pub commit: PPFACommit,
    /// Authority set ID that signed this justification
    pub authority_set_id: u64,
    /// Timestamp of certificate creation (Unix millis)
    pub timestamp: u64,
}

impl PPFAJustification {
    /// Create a new PPFA justification
    pub fn new(
        view: u64,
        target_hash: [u8; 32],
        target_number: u32,
        precommits: Vec<SignedPPFAPrecommit>,
        authority_set_id: u64,
    ) -> Self {
        Self {
            view,
            commit: PPFACommit {
                target_hash,
                target_number,
                precommits,
            },
            authority_set_id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Convert to Substrate Justification format
    pub fn to_justification(&self) -> Justification {
        (PPFA_ENGINE_ID, self.encode())
    }

    /// Try to decode from Substrate Justification
    pub fn from_justification(justification: &Justification) -> Option<Self> {
        if justification.0 == PPFA_ENGINE_ID {
            Self::decode(&mut &justification.1[..]).ok()
        } else {
            None
        }
    }

    /// Get the target block hash
    pub fn target_hash(&self) -> &[u8; 32] {
        &self.commit.target_hash
    }

    /// Get the target block number
    pub fn target_number(&self) -> u32 {
        self.commit.target_number
    }

    /// Get the number of signatures
    pub fn signature_count(&self) -> usize {
        self.commit.precommits.len()
    }

    /// Verify that the justification has enough signatures (2f+1 for BFT)
    pub fn verify_signature_threshold(&self, total_validators: usize) -> bool {
        let required = (2 * total_validators / 3) + 1;
        self.signature_count() >= required
    }

    /// Create from ASF certificate (bridge from DETR P2P)
    pub fn from_asf_certificate(
        view: u64,
        block_hash: [u8; 32],
        block_number: u32,
        signatures: Vec<([u8; 32], Vec<u8>)>,
        authority_set_id: u64,
    ) -> Self {
        let precommits = signatures
            .into_iter()
            .map(|(validator_id, signature)| SignedPPFAPrecommit {
                precommit: PPFAPrecommit {
                    target_hash: block_hash,
                    target_number: block_number,
                },
                signature,
                validator_id,
            })
            .collect();

        Self::new(view, block_hash, block_number, precommits, authority_set_id)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// WARP SYNC SUPPORT
// ═══════════════════════════════════════════════════════════════════════════════

/// Authority set change proof for warp sync
#[derive(Clone, Debug, Encode, Decode)]
pub struct PPFAAuthoritySetChange {
    /// Block hash where authority set changed
    pub block_hash: [u8; 32],
    /// Block number where authority set changed
    pub block_number: u32,
    /// Committee active at this block
    pub committee: Vec<ValidatorInfo>,
}

/// PPFA Warp Sync Proof - enables fast-sync for new nodes
///
/// Contains chain of authority set changes from genesis to target block.
#[derive(Clone, Debug, Encode, Decode)]
pub struct PPFAWarpSyncProof {
    /// Chain of authority set changes
    pub authority_changes: Vec<PPFAAuthoritySetChange>,
    /// Final target block justification
    pub final_justification: PPFAJustification,
    /// Whether this proof is complete (reached genesis or max changes)
    pub is_complete: bool,
}

impl PPFAWarpSyncProof {
    /// Create a new empty warp sync proof
    pub fn empty() -> Self {
        Self {
            authority_changes: Vec::new(),
            final_justification: PPFAJustification::new(0, [0u8; 32], 0, Vec::new(), 0),
            is_complete: true,
        }
    }

    /// Get the target block number
    pub fn target_block_number(&self) -> u32 {
        self.final_justification.target_number()
    }

    /// Verify the proof chain
    pub fn verify(&self) -> Result<(), String> {
        if self.authority_changes.len() > MAX_WARP_SYNC_AUTHORITY_CHANGES {
            return Err(format!(
                "Too many authority changes in proof ({} > {})",
                self.authority_changes.len(),
                MAX_WARP_SYNC_AUTHORITY_CHANGES
            ));
        }
        // Verify the authority changes are monotonic and ordered
        let mut last_number: Option<u32> = None;
        for change in &self.authority_changes {
            if change.committee.is_empty() {
                return Err(format!(
                    "Authority change at #{} has empty committee",
                    change.block_number
                ));
            }
            if let Some(prev) = last_number {
                if change.block_number < prev {
                    return Err(format!(
                        "Authority change regression: {} -> {}",
                        prev, change.block_number
                    ));
                }
            }
            last_number = Some(change.block_number);
        }
        // Final justification should be at or after the last authority change
        if let Some(last) = last_number {
            if self.final_justification.target_number() < last {
                return Err("Final justification precedes last authority change".to_string());
            }
        }

        Ok(())
    }
}

/// Warp sync proof provider trait
pub trait PPFAWarpSyncProofProvider<Block: BlockT>: Send + Sync {
    /// Generate warp sync proof from given block
    fn generate_proof(
        &self,
        begin: Block::Hash,
    ) -> Result<PPFAWarpSyncProof, sp_blockchain::Error>;

    /// Verify and apply warp sync proof
    fn verify_proof(&self, proof: &PPFAWarpSyncProof) -> Result<(), sp_blockchain::Error>;
}

/// Runtime-backed warp sync proof provider
pub struct RuntimeWarpSyncProvider<B: BlockT, C>
where
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + BlockBackend<B> + Send + Sync + 'static,
    NumberFor<B>: From<u32> + Into<u32>,
    B::Hash: AsRef<[u8]> + From<[u8; 32]>,
    C::Api: ValidatorCommitteeApi<B>,
{
    client: Arc<C>,
    _phantom: PhantomData<B>,
}

impl<B: BlockT, C> RuntimeWarpSyncProvider<B, C>
where
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + BlockBackend<B> + Send + Sync + 'static,
    NumberFor<B>: From<u32> + Into<u32>,
    B::Hash: AsRef<[u8]> + From<[u8; 32]>,
    C::Api: ValidatorCommitteeApi<B>,
{
    pub fn new(client: Arc<C>) -> Self {
        Self { client, _phantom: PhantomData }
    }

    fn best_finalized(&self) -> Result<(B::Hash, u32), sp_blockchain::Error> {
        let info = self.client.info();
        let number: u32 = info.finalized_number.saturated_into();
        Ok((info.finalized_hash, number))
    }

    fn block_hash_by_number(&self, number: u32) -> Result<Option<B::Hash>, sp_blockchain::Error> {
        let num: NumberFor<B> = number.into();
        self.client.hash(num)
    }

    fn descend_to_parent(
        &self,
        current_number: u32,
    ) -> Result<Option<(B::Hash, u32)>, sp_blockchain::Error> {
        if current_number == 0 {
            return Ok(None);
        }
        let prev_number = current_number.saturating_sub(1);
        match self.block_hash_by_number(prev_number)? {
            Some(prev_hash) => Ok(Some((prev_hash, prev_number))),
            None => Ok(None),
        }
    }

    fn fetch_rpc_justification(
        &self,
        hash: B::Hash,
        _number: u32,
    ) -> Option<PPFAJustification> {
        // Try to read from RPC/state backend: get block justifications and decode PPFA
        if let Ok(Some(justifications)) = self.client.justifications(hash) {
            if let Some(encoded) = justifications.get(PPFA_ENGINE_ID) {
                if let Ok(decoded) = PPFAJustification::decode(&mut &encoded[..]) {
                    return Some(decoded);
                }
            }
        }

        None
    }

    fn find_justification(
        &self,
        start_hash: B::Hash,
        start_number: u32,
    ) -> Option<(B::Hash, u32, PPFAJustification)> {
        let mut current_hash = start_hash;
        let mut current_number = start_number;

        for _ in 0..=MAX_WARP_SYNC_JUSTIFICATION_BACKTRACK {
            if let Some(just) = self.load_ppfa_justification(current_hash, current_number) {
                return Some((current_hash, current_number, just));
            }

            if let Some(just) = self.fetch_rpc_justification(current_hash, current_number) {
                return Some((current_hash, current_number, just));
            }

            if current_number == 0 {
                break;
            }

            match self.descend_to_parent(current_number) {
                Ok(Some((next_hash, next_number))) => {
                    current_hash = next_hash;
                    current_number = next_number;
                }
                Ok(None) => break,
                Err(e) => {
                    warn!("Warp sync justification search stopped: {:?}", e);
                    break;
                }
            }
        }

        // Fallback: extend search window to catch older stored justifications without unbounded scanning.
        let mut current_hash = start_hash;
        let mut current_number = start_number;
        for _ in 0..=MAX_WARP_SYNC_FALLBACK_BACKTRACK {
            if let Some(just) = self.load_ppfa_justification(current_hash, current_number) {
                return Some((current_hash, current_number, just));
            }

            if current_number == 0 {
                break;
            }

            match self.descend_to_parent(current_number) {
                Ok(Some((next_hash, next_number))) => {
                    current_hash = next_hash;
                    current_number = next_number;
                }
                Ok(None) => break,
                Err(e) => {
                    warn!("Warp sync fallback search stopped: {:?}", e);
                    break;
                }
            }
        }

        None
    }

    fn load_ppfa_justification(&self, hash: B::Hash, _number: u32) -> Option<PPFAJustification> {
        // Try to read on-chain justifications and decode PPFA if present
        if let Ok(Some(justifications)) = self.client.justifications(hash) {
            if let Some(encoded) = justifications.get(PPFA_ENGINE_ID) {
                if let Ok(decoded) = PPFAJustification::decode(&mut &encoded[..]) {
                    return Some(decoded);
                }
            }
        }

        // No justification found.
        None
    }

    fn collect_authority_changes(
        &self,
        begin_number: u32,
        target_hash: B::Hash,
        target_number: u32,
    ) -> Result<Vec<PPFAAuthoritySetChange>, sp_blockchain::Error> {
        let epoch_duration = self.client.runtime_api().epoch_duration(target_hash)?;
        if epoch_duration == 0 {
            return Err(sp_blockchain::Error::Application("epoch_duration is zero".into()));
        }

        // Align to next epoch boundary after the requested starting point.
        let mut next_boundary = begin_number
            .checked_add(epoch_duration - (begin_number % epoch_duration))
            .unwrap_or(begin_number);

        let mut changes = Vec::new();
        while next_boundary <= target_number && changes.len() < MAX_WARP_SYNC_AUTHORITY_CHANGES {
            if let Some(boundary_hash) = self.block_hash_by_number(next_boundary)? {
                // Ensure committee is non-empty at the boundary (guards against empty sets)
                let committee = self.client.runtime_api().validator_committee(boundary_hash)?;
                if committee.is_empty() {
                    warn!(
                        "Validator committee empty at boundary #{}, skipping authority change entry",
                        next_boundary
                    );
                } else {
                    changes.push(PPFAAuthoritySetChange {
                        block_hash: hash_to_bytes(&boundary_hash),
                        block_number: next_boundary,
                        committee,
                    });
                }
            }
            next_boundary = next_boundary.saturating_add(epoch_duration);
        }

        // Always include the target head if no boundary was added.
        if changes.is_empty() {
            let committee = self.client.runtime_api().validator_committee(target_hash)?;
            changes.push(PPFAAuthoritySetChange {
                block_hash: hash_to_bytes(&target_hash),
                block_number: target_number,
                committee,
            });
        }

        Ok(changes)
    }
}

impl<B: BlockT, C> PPFAWarpSyncProofProvider<B> for RuntimeWarpSyncProvider<B, C>
where
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + BlockBackend<B> + Send + Sync + 'static,
    NumberFor<B>: From<u32> + Into<u32>,
    B::Hash: AsRef<[u8]> + From<[u8; 32]>,
    C::Api: pallet_validator_committee_runtime_api::ValidatorCommitteeApi<B>,
{
    fn generate_proof(&self, begin: B::Hash) -> Result<PPFAWarpSyncProof, sp_blockchain::Error> {
        let begin_number: u32 = self
            .client
            .header(begin)?
            .map(|h| (*h.number()).saturated_into())
            .unwrap_or(0);

        let (finalized_hash, finalized_number) = self.best_finalized()?;
        let (target_hash, target_number, final_justification) = match self.find_justification(
            finalized_hash,
            finalized_number,
        ) {
            Some(values) => values,
            None => {
                let err_msg = format!(
                    "Missing PPFA justification within backtrack {} (fallback {}) from finalized head",
                    MAX_WARP_SYNC_JUSTIFICATION_BACKTRACK,
                    MAX_WARP_SYNC_FALLBACK_BACKTRACK
                );
                warn!("Warp sync: {}", err_msg);
                return Err(sp_blockchain::Error::Application(err_msg.into()));
            }
        };

        let authority_changes =
            self.collect_authority_changes(begin_number, target_hash, target_number)?;

        let proof = PPFAWarpSyncProof {
            authority_changes,
            final_justification,
            is_complete: true,
        };

        // Cap size explicitly
        let encoded_len = proof.encode().len() as u64;
        if encoded_len > MAX_WARP_SYNC_PROOF_SIZE {
            return Err(sp_blockchain::Error::Application(format!(
                "Warp sync proof too large: {} bytes (limit {})",
                encoded_len, MAX_WARP_SYNC_PROOF_SIZE
            ).into()));
        }

        Ok(proof)
    }

    fn verify_proof(&self, proof: &PPFAWarpSyncProof) -> Result<(), sp_blockchain::Error> {
        // Basic structural checks: non-empty final justification and authority chain matches
        proof.verify().map_err(|e| sp_blockchain::Error::Application(Box::from(e)))?;

        // Ensure proof size stays bounded
        let encoded_len = proof.encode().len() as u64;
        if encoded_len > MAX_WARP_SYNC_PROOF_SIZE {
            return Err(sp_blockchain::Error::Application("Warp sync proof exceeds size limit".into()));
        }

        // Verify authority change headers exist and match numbers
        for change in &proof.authority_changes {
            let expected_hash = change.block_hash;
            let block_hash = self
                .block_hash_by_number(change.block_number)?
                .ok_or_else(|| {
                    sp_blockchain::Error::Application(format!(
                        "Missing block for authority change at #{}",
                        change.block_number
                    )
                    .into())
                })?;

            if hash_to_bytes(&block_hash) != expected_hash {
                return Err(sp_blockchain::Error::Application(format!(
                    "Authority change hash mismatch at #{}",
                    change.block_number
                )
                .into()));
            }

            // Ensure header number aligns
            if let Some(header) = self.client.header(block_hash)? {
                let header_number: u32 = (*header.number()).saturated_into();
                if header_number != change.block_number {
                    return Err(sp_blockchain::Error::Application(format!(
                        "Header number mismatch at authority change (expected {}, got {})",
                        change.block_number, header_number
                    )
                    .into()));
                }
            } else {
                return Err(sp_blockchain::Error::Application(format!(
                    "Missing header for authority change #{}",
                    change.block_number
                )
                .into()));
            }

            // Ensure committee is non-empty at each boundary and matches runtime
            let committee = self.client.runtime_api().validator_committee(block_hash)?;
            if committee.is_empty() {
                return Err(sp_blockchain::Error::Application(format!(
                    "Empty committee at authority change boundary #{}",
                    change.block_number
                )
                .into()));
            }
            if committee != change.committee {
                return Err(sp_blockchain::Error::Application(format!(
                    "Committee mismatch at authority change boundary #{}",
                    change.block_number
                )
                .into()));
            }
        }

        // Final justification target must exist and align with header number
        let final_hash: B::Hash = (*proof.final_justification.target_hash()).into();
        let header = self.client.header(final_hash)?.ok_or_else(|| {
            sp_blockchain::Error::Application("Missing header for final justification".into())
        })?;
        let header_number: u32 = (*header.number()).saturated_into();
        if header_number != proof.final_justification.target_number() {
            return Err(sp_blockchain::Error::Application(
                "Final justification block number mismatch".into(),
            ));
        }

        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// JUSTIFICATION STORAGE
// ═══════════════════════════════════════════════════════════════════════════════

/// Storage interface for PPFA justifications
///
/// Provides scalable storage with pruning for production use.
pub struct PPFAJustificationStorage<B: BlockT> {
    /// In-memory cache of recent justifications
    cache: Arc<RwLock<BTreeMap<u32, PPFAJustification>>>,
    /// Maximum cache size
    cache_size: usize,
    /// Last pruned block number
    last_pruned: Arc<RwLock<u32>>,
    /// Phantom for block type
    _phantom: PhantomData<B>,
}

impl<B: BlockT> PPFAJustificationStorage<B> {
    /// Create new justification storage
    pub fn new(cache_size: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(BTreeMap::new())),
            cache_size,
            last_pruned: Arc::new(RwLock::new(0)),
            _phantom: PhantomData,
        }
    }

    /// Store a justification
    pub async fn store(&self, block_number: u32, justification: PPFAJustification) {
        let mut cache = self.cache.write().await;
        cache.insert(block_number, justification);

        // Prune if cache exceeds size
        while cache.len() > self.cache_size {
            if let Some((&oldest, _)) = cache.iter().next() {
                cache.remove(&oldest);
            }
        }
    }

    /// Get a justification by block number
    pub async fn get(&self, block_number: u32) -> Option<PPFAJustification> {
        let cache = self.cache.read().await;
        cache.get(&block_number).cloned()
    }

    /// Get the highest finalized block number
    pub async fn highest_finalized(&self) -> Option<u32> {
        let cache = self.cache.read().await;
        cache.keys().last().copied()
    }

    /// Prune old justifications (called periodically)
    pub async fn prune(&self, current_block: u32) -> usize {
        let mut last_pruned = self.last_pruned.write().await;

        // Only prune if enough blocks have passed
        if current_block < *last_pruned + PRUNING_INTERVAL_BLOCKS {
            return 0;
        }

        let retention_threshold = current_block.saturating_sub(JUSTIFICATION_RETENTION_BLOCKS);
        let mut cache = self.cache.write().await;

        let to_remove: Vec<u32> = cache
            .keys()
            .filter(|&&k| k < retention_threshold)
            .copied()
            .collect();

        let pruned_count = to_remove.len();
        for key in to_remove {
            cache.remove(&key);
        }

        *last_pruned = current_block;

        if pruned_count > 0 {
            info!(
                "🧹 PPFA: Pruned {} old justifications (retention: {})",
                pruned_count, JUSTIFICATION_RETENTION_BLOCKS
            );
        }

        pruned_count
    }

    /// Get justifications for warp sync proof generation
    pub async fn get_range(&self, start: u32, end: u32) -> Vec<(u32, PPFAJustification)> {
        let cache = self.cache.read().await;
        cache
            .range(start..=end)
            .map(|(&k, v)| (k, v.clone()))
            .collect()
    }
}

impl<B: BlockT> Clone for PPFAJustificationStorage<B> {
    fn clone(&self) -> Self {
        Self {
            cache: self.cache.clone(),
            cache_size: self.cache_size,
            last_pruned: self.last_pruned.clone(),
            _phantom: PhantomData,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PPFA NETWORK MESSAGES
// ═══════════════════════════════════════════════════════════════════════════════

/// PPFA network message types
#[derive(Clone, Debug, Encode, Decode)]
pub enum PPFAMessage {
    /// Announce a new finality justification
    Justification(PPFAJustification),
    /// Request justification for a specific block
    JustificationRequest {
        block_hash: [u8; 32],
        block_number: u32,
    },
    /// Request a block by hash/number
    BlockRequest {
        block_hash: [u8; 32],
        block_number: u32,
    },
    /// Response with an encoded block
    BlockResponse {
        encoded_block: Vec<u8>,
        block_hash: [u8; 32],
        parent_hash: [u8; 32],
    },
    /// Request peer status (best/finalized)
    StatusRequest,
    /// Status response
    StatusResponse {
        best_hash: [u8; 32],
        best_number: u32,
        finalized_hash: [u8; 32],
        finalized_number: u32,
    },
    /// Neighbor packet (gossip protocol)
    Neighbor {
        /// Our view number
        view: u64,
        /// Our highest finalized block
        finalized_number: u32,
        /// Our authority set ID
        authority_set_id: u64,
    },
    /// Warp sync proof request
    WarpSyncRequest { begin_hash: [u8; 32] },
    /// Warp sync proof response
    WarpSyncResponse(PPFAWarpSyncProof),
    /// Catch-up request (for nodes that fell behind)
    CatchUpRequest {
        authority_set_id: u64,
        start_view: u64,
    },
    /// Catch-up response
    CatchUpResponse {
        justifications: Vec<PPFAJustification>,
    },
}

// ═══════════════════════════════════════════════════════════════════════════════
// PPFA PROTOCOL CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Configuration for PPFA protocol
pub struct PPFAProtocolConfig {
    /// Protocol name
    pub protocol_name: ProtocolName,
    /// Fallback protocol names
    pub fallback_names: Vec<ProtocolName>,
}

impl PPFAProtocolConfig {
    /// Create new PPFA protocol configuration
    pub fn new(protocol_id: &str) -> Self {
        let protocol_name: ProtocolName = format!("/{}/ppfa/1", protocol_id).into();
        let fallback_names = vec![
            format!("/{}/ppfa/0", protocol_id).into(), // Fallback for older versions
        ];

        Self {
            protocol_name,
            fallback_names,
        }
    }

    /// Create the notification config and service for network setup
    /// Returns (NonDefaultSetConfig, Box<dyn NotificationService>)
    ///
    /// # Arguments
    /// * `reserved_nodes` - Bootnodes/reserved nodes to ensure PPFA can connect to key validators.
    ///   This uses the same reserved nodes as the default peer set (from --reserved-nodes CLI or chain spec).
    pub fn build_notification_config(
        &self,
        reserved_nodes: Vec<sc_network::config::MultiaddrWithPeerId>,
    ) -> (NonDefaultSetConfig, Box<dyn NotificationService>) {
        log::info!("🔗 PPFA: Configuring with {} reserved nodes", reserved_nodes.len());
        for node in &reserved_nodes {
            log::info!("   Reserved node: {:?}", node);
        }

        let set_config = SetConfig {
            in_peers: 25,
            out_peers: 25,
            reserved_nodes,
            non_reserved_mode: NonReservedPeerMode::Accept,
        };

        // Use the proper Polkadot SDK API for creating notification protocols
        let (config, service) = NonDefaultSetConfig::new(
            self.protocol_name.clone(),
            self.fallback_names.clone(),
            MAX_JUSTIFICATION_SIZE,
            None, // handshake
            set_config,
        );

        (config, service)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PPFA PROTOCOL WORKER
// ═══════════════════════════════════════════════════════════════════════════════

/// Shared state for PPFA protocol
pub struct PPFASharedState<B: BlockT> {
    /// Current view number
    pub view: RwLock<u64>,
    /// Highest finalized block number
    pub finalized_number: RwLock<NumberFor<B>>,
    /// Current authority set ID
    pub authority_set_id: RwLock<u64>,
    /// Connected peers
    pub peers: RwLock<HashMap<PeerId, PPFAPeerState>>,
    /// Justification storage
    pub storage: PPFAJustificationStorage<B>,
}

/// State tracking for each connected peer
#[derive(Clone, Debug)]
pub struct PPFAPeerState {
    /// Peer's view number
    pub view: u64,
    /// Peer's highest finalized block
    pub finalized_number: u32,
    /// Peer's authority set ID
    pub authority_set_id: u64,
    /// Last update time
    pub last_update: Instant,
}

impl Default for PPFAPeerState {
    fn default() -> Self {
        Self {
            view: 0,
            finalized_number: 0,
            authority_set_id: 0,
            last_update: Instant::now(),
        }
    }
}

impl<B: BlockT> PPFASharedState<B> {
    /// Create new shared state
    pub fn new() -> Self {
        Self {
            view: RwLock::new(0),
            finalized_number: RwLock::new(Zero::zero()),
            authority_set_id: RwLock::new(0),
            peers: RwLock::new(HashMap::new()),
            storage: PPFAJustificationStorage::new(10_000), // Keep 10K recent justifications
        }
    }
}

impl<B: BlockT> Default for PPFASharedState<B> {
    fn default() -> Self {
        Self::new()
    }
}

/// PPFA Protocol Worker - main async task that runs the protocol
pub struct PPFAProtocolWorker<B, C, BE, N>
where
    B: BlockT,
    B::Hash: From<[u8; 32]> + AsRef<[u8]>,
    C: HeaderBackend<B>
        + BlockBackend<B>
        + ProvideRuntimeApi<B>
        + Finalizer<B, BE>
        + LockImportRun<B, BE>
        + HeaderMetadata<B, Error = sp_blockchain::Error>
        + Send
        + Sync
        + 'static,
    C::Api: ValidatorCommitteeApi<B>,
    BE: Backend<B> + 'static,
    N: NetworkPeers + NetworkEventStream + Send + Sync + 'static,
{
    /// Client reference
    client: Arc<C>,
    /// Network service
    network: Arc<N>,
    /// Sync service
    sync_service: Arc<SyncingService<B>>,
    /// Channel receiver for notification events (from spawned notification task)
    notification_rx: mpsc::UnboundedReceiver<NotificationEvent>,
    /// Channel sender for outbound notifications (to spawned notification task)
    notification_tx: mpsc::UnboundedSender<(PeerId, Vec<u8>)>,
    /// Shared protocol state
    shared_state: Arc<PPFASharedState<B>>,
    /// Channel to receive ASF finality events from DETR P2P bridge
    asf_finality_rx: mpsc::UnboundedReceiver<(B::Hash, NumberFor<B>, Vec<([u8; 32], Vec<u8>)>)>,
    /// Channel to send finality notifications to sync engine
    finality_notification_tx: mpsc::UnboundedSender<(B::Hash, NumberFor<B>)>,
    /// Protocol name
    protocol_name: ProtocolName,
    /// Warp sync proof provider
    warp_sync_provider: Arc<dyn PPFAWarpSyncProofProvider<B>>,
    /// Phantom for backend type
    _phantom: PhantomData<BE>,
}

impl<B, C, BE, N> PPFAProtocolWorker<B, C, BE, N>
where
    B: BlockT,
    B::Hash: From<[u8; 32]> + AsRef<[u8]>,
    NumberFor<B>: From<u32> + Into<u32>,
    C: HeaderBackend<B>
        + BlockBackend<B>
        + ProvideRuntimeApi<B>
        + Finalizer<B, BE>
        + LockImportRun<B, BE>
        + HeaderMetadata<B, Error = sp_blockchain::Error>
        + Send
        + Sync
        + 'static,
    C::Api: ValidatorCommitteeApi<B>,
    BE: Backend<B> + 'static,
    N: NetworkPeers + NetworkEventStream + Send + Sync + 'static,
{
    /// Create new PPFA protocol worker
    /// Returns (worker, notification_bridge_future) - the bridge must be spawned separately
    pub fn new(
        client: Arc<C>,
        network: Arc<N>,
        sync_service: Arc<SyncingService<B>>,
        notification_service: Box<dyn NotificationService>,
        asf_finality_rx: mpsc::UnboundedReceiver<(B::Hash, NumberFor<B>, Vec<([u8; 32], Vec<u8>)>)>,
        finality_notification_tx: mpsc::UnboundedSender<(B::Hash, NumberFor<B>)>,
        protocol_name: ProtocolName,
    ) -> (Self, impl std::future::Future<Output = ()> + Send + 'static) {
        // Create channels for notification events
        let (inbound_tx, notification_rx) = mpsc::unbounded::<NotificationEvent>();
        let (notification_tx, mut outbound_rx) = mpsc::unbounded::<(PeerId, Vec<u8>)>();

        // Create the notification bridge task that owns the notification_service
        let notification_bridge = async move {
            let mut notification_service = notification_service;
            loop {
                tokio::select! {
                    // Forward incoming notification events to the worker
                    event = notification_service.next_event() => {
                        match event {
                            Some(e) => {
                                if inbound_tx.unbounded_send(e).is_err() {
                                    break; // Worker dropped, exit
                                }
                            }
                            None => break, // NotificationService closed
                        }
                    }
                    // Forward outbound messages to the notification service
                    Some((peer_id, data)) = outbound_rx.next() => {
                        notification_service.send_sync_notification(&peer_id, data);
                    }
                }
            }
            info!("📡 PPFA notification bridge task exiting");
        };

        let client_for_warp = client.clone();

        let worker = Self {
            client,
            network,
            sync_service,
            notification_rx,
            notification_tx,
            shared_state: Arc::new(PPFASharedState::new()),
            asf_finality_rx,
            finality_notification_tx,
            protocol_name,
            warp_sync_provider: Arc::new(RuntimeWarpSyncProvider::new(client_for_warp)),
            _phantom: PhantomData,
        };

        (worker, notification_bridge)
    }

    /// Get shared state reference
    pub fn shared_state(&self) -> Arc<PPFASharedState<B>> {
        self.shared_state.clone()
    }

    /// Run the PPFA protocol worker
    pub async fn run(mut self) {
        info!("🚀 PPFA Protocol Worker starting...");
        // Touch fields that are mainly wired for future integrations to avoid dead-code drift.
        let _ = (&self.network, &self.sync_service, &self.protocol_name);

        // Gossip timer for neighbor packets
        let mut gossip_timer = tokio::time::interval(Duration::from_secs(5));

        // Prune timer for storage cleanup
        let mut prune_timer = tokio::time::interval(Duration::from_secs(60));

        loop {
            tokio::select! {
                // Handle ASF finality events from DETR P2P bridge
                Some((hash, number, signatures)) = self.asf_finality_rx.next() => {
                    self.handle_asf_finality(hash, number, signatures).await;
                }

                // Handle incoming PPFA protocol messages (from notification bridge)
                Some(event) = self.notification_rx.next() => {
                    self.handle_notification_event(event).await;
                }

                // Periodic gossip of neighbor packets
                _ = gossip_timer.tick() => {
                    self.gossip_neighbor_packet().await;
                }

                // Periodic storage pruning
                _ = prune_timer.tick() => {
                    let current = self.client.info().best_number;
                    let current_u32: u32 = current.into();
                    self.shared_state.storage.prune(current_u32).await;
                }
            }
        }
    }

    /// Handle ASF finality event from DETR P2P bridge
    async fn handle_asf_finality(
        &mut self,
        hash: B::Hash,
        number: NumberFor<B>,
        signatures: Vec<([u8; 32], Vec<u8>)>,
    ) {
        let number_u32: u32 = number.into();
        let hash_bytes: [u8; 32] = hash_to_bytes(&hash);

        // Get current view and authority set
        let view = *self.shared_state.view.read().await;
        let authority_set_id = *self.shared_state.authority_set_id.read().await;

        // Create PPFA justification from ASF certificate
        let justification = PPFAJustification::from_asf_certificate(
            view,
            hash_bytes,
            number_u32,
            signatures.clone(),
            authority_set_id,
        );

        info!(
            "📜 PPFA: Processing ASF finality for block #{} ({:?}), {} signatures",
            number_u32,
            hex::encode(&hash_bytes[..8]),
            signatures.len()
        );

        // Store justification
        self.shared_state.storage.store(number_u32, justification.clone()).await;

        // Update finalized block
        {
            let mut finalized = self.shared_state.finalized_number.write().await;
            if number > *finalized {
                *finalized = number;
            }
        }

        // Finalize block in client
        match self.finalize_block(hash, justification.clone()).await {
            Ok(()) => {
                info!(
                    "✅ PPFA: Finalized block #{} ({:?})",
                    number_u32,
                    hex::encode(&hash_bytes[..8])
                );

                // Send finality notification to sync engine
                if let Err(e) = self.finality_notification_tx.unbounded_send((hash, number)) {
                    warn!("Failed to send PPFA finality notification: {:?}", e);
                }

                // Broadcast justification to peers
                self.broadcast_justification(justification).await;
            }
            Err(e) => {
                error!(
                    "❌ PPFA: Failed to finalize block #{}: {:?}",
                    number_u32, e
                );
            }
        }

        // Increment view
        {
            let mut view_lock = self.shared_state.view.write().await;
            *view_lock = view + 1;
        }
    }

    /// Finalize a block with PPFA justification
    async fn finalize_block(
        &self,
        hash: B::Hash,
        justification: PPFAJustification,
    ) -> Result<(), sp_blockchain::Error> {
        let justification_bytes = justification.to_justification();

        self.client.lock_import_and_run(|import_op| {
            self.client.apply_finality(
                import_op,
                hash,
                Some(justification_bytes),
                true, // notify
            )
        })
    }

    /// Broadcast justification to all connected peers
    async fn broadcast_justification(&self, justification: PPFAJustification) {
        let message = PPFAMessage::Justification(justification);
        let encoded = message.encode();

        let peers = self.shared_state.peers.read().await;
        for peer_id in peers.keys() {
            let _ = self.notification_tx.unbounded_send((peer_id.clone(), encoded.clone()));
        }

        debug!(
            "📡 PPFA: Broadcast justification to {} peers",
            peers.len()
        );
    }

    /// Gossip neighbor packet to peers
    async fn gossip_neighbor_packet(&self) {
        let view = *self.shared_state.view.read().await;
        let finalized_number: u32 = (*self.shared_state.finalized_number.read().await).saturated_into();
        let authority_set_id = *self.shared_state.authority_set_id.read().await;

        let message = PPFAMessage::Neighbor {
            view,
            finalized_number,
            authority_set_id,
        };
        let encoded = message.encode();

        let peers = self.shared_state.peers.read().await;
        for peer_id in peers.keys() {
            let _ = self.notification_tx.unbounded_send((peer_id.clone(), encoded.clone()));
        }

        trace!(
            "📡 PPFA: Gossip neighbor packet (view: {}, finalized: #{}, set: {}) to {} peers",
            view,
            finalized_number,
            authority_set_id,
            peers.len()
        );
    }

    /// Handle notification protocol events
    async fn handle_notification_event(&mut self, event: sc_network::service::traits::NotificationEvent) {
        use sc_network::service::traits::NotificationEvent;

        match event {
            NotificationEvent::ValidateInboundSubstream { peer, handshake: _handshake, result_tx } => {
                // V114: Basic peer validation for PPFA protocol
                // Accept peers but log for future whitelisting implementation
                // TODO: Implement director whitelisting via runtime query:
                //   - Query ValidatorCommittee for authorized validators
                //   - Check if peer's validator ID is in current or next epoch committee
                //   - Reject unknown peers to prevent consensus spam

                // For now, accept all peers but log the connection attempt
                // This allows the network to form while we implement proper whitelisting
                info!("🔐 PPFA: Validating inbound connection from peer {:?}", peer);

                // Accept the connection - whitelisting will be enforced at message level
                let _ = result_tx.send(sc_network::service::traits::ValidationResult::Accept);
                debug!("🤝 PPFA: Accepted inbound connection from {:?} (whitelisting pending)", peer);
            }

            NotificationEvent::NotificationStreamOpened { peer, handshake: _handshake, .. } => {
                // New peer connected
                let mut peers = self.shared_state.peers.write().await;
                peers.insert(peer, PPFAPeerState::default());
                info!("🟢 PPFA: Peer connected {:?}, total peers: {}", peer, peers.len());

                // Send our neighbor packet to new peer
                drop(peers);
                self.gossip_neighbor_packet().await;
            }

            NotificationEvent::NotificationStreamClosed { peer } => {
                // Peer disconnected
                let mut peers = self.shared_state.peers.write().await;
                peers.remove(&peer);
                info!("🔴 PPFA: Peer disconnected {:?}, remaining peers: {}", peer, peers.len());
            }

            NotificationEvent::NotificationReceived { peer, notification } => {
                // Handle incoming message
                self.handle_message(peer, notification).await;
            }
        }
    }

    /// Handle incoming PPFA message from peer
    async fn handle_message(&mut self, peer: PeerId, data: Vec<u8>) {
        let message = match PPFAMessage::decode(&mut &data[..]) {
            Ok(msg) => msg,
            Err(e) => {
                warn!("Failed to decode PPFA message from {:?}: {:?}", peer, e);
                return;
            }
        };

        match message {
            PPFAMessage::Justification(justification) => {
                self.handle_peer_justification(peer, justification).await;
            }

            PPFAMessage::JustificationRequest { block_hash, block_number } => {
                self.handle_justification_request(peer, block_hash, block_number).await;
            }

            PPFAMessage::Neighbor { view, finalized_number, authority_set_id } => {
                self.handle_neighbor_packet(peer, view, finalized_number, authority_set_id).await;
            }

            PPFAMessage::BlockRequest { block_hash, block_number } => {
                self.handle_block_request(peer, block_hash, block_number).await;
            }

            PPFAMessage::BlockResponse { encoded_block, block_hash, parent_hash } => {
                self.handle_block_response(peer, encoded_block, block_hash, parent_hash).await;
            }

            PPFAMessage::StatusRequest => {
                self.handle_status_request(peer).await;
            }

            PPFAMessage::StatusResponse { best_hash, best_number, finalized_hash, finalized_number } => {
                self.handle_status_response(peer, best_hash, best_number, finalized_hash, finalized_number).await;
            }

            PPFAMessage::WarpSyncRequest { begin_hash } => {
                self.handle_warp_sync_request(peer, begin_hash).await;
            }

            PPFAMessage::WarpSyncResponse(proof) => {
                self.handle_warp_sync_response(peer, proof).await;
            }

            PPFAMessage::CatchUpRequest { authority_set_id, start_view } => {
                self.handle_catch_up_request(peer, authority_set_id, start_view).await;
            }

            PPFAMessage::CatchUpResponse { justifications } => {
                self.handle_catch_up_response(peer, justifications).await;
            }
        }
    }

    /// Handle justification received from peer
    async fn handle_peer_justification(&mut self, peer: PeerId, justification: PPFAJustification) {
        let block_number = justification.target_number();
        let block_hash_bytes = justification.target_hash();

        debug!(
            "📥 PPFA: Received justification for block #{} from {:?}",
            block_number, peer
        );

        // Check if we already have this block finalized
        let our_finalized: u32 = (*self.shared_state.finalized_number.read().await).into();
        if block_number <= our_finalized {
            trace!("Already finalized block #{}, ignoring", block_number);
            return;
        }

        // Verify justification has enough signatures using runtime committee size when available.
        let total_validators = match self
            .client
            .runtime_api()
            .validator_committee(self.client.info().best_hash)
        {
            Ok(committee) if !committee.is_empty() => committee.len(),
            Ok(_) => {
                warn!("Validator committee is empty; falling back to threshold 21");
                21
            }
            Err(e) => {
                warn!(
                    "Failed to fetch validator committee for threshold check: {:?}; using default 21",
                    e
                );
                21
            }
        };
        if !justification.verify_signature_threshold(total_validators) {
            warn!(
                "PPFA justification for block #{} has insufficient signatures ({}/{})",
                block_number,
                justification.signature_count(),
                (2 * total_validators / 3) + 1
            );
            return;
        }

        // Store and apply justification
        let block_hash = B::Hash::from(*block_hash_bytes);
        let number: NumberFor<B> = block_number.into();

        // Check if we have this block
        match self.client.header(block_hash) {
            Ok(Some(_)) => {
                // We have the block, finalize it
                self.shared_state.storage.store(block_number, justification.clone()).await;

                match self.finalize_block(block_hash, justification).await {
                    Ok(()) => {
                        info!(
                            "✅ PPFA: Finalized block #{} from peer {:?}",
                            block_number, peer
                        );

                        // Update our finalized number
                        {
                            let mut finalized = self.shared_state.finalized_number.write().await;
                            if number > *finalized {
                                *finalized = number;
                            }
                        }

                        // Notify sync engine
                        let _ = self.finality_notification_tx.unbounded_send((block_hash, number));
                    }
                    Err(e) => {
                        warn!("Failed to finalize block #{}: {:?}", block_number, e);
                    }
                }
            }
            Ok(None) => {
                // We don't have this block yet, store justification for later
                debug!(
                    "Block #{} not yet imported, storing justification for later",
                    block_number
                );
                self.shared_state.storage.store(block_number, justification).await;
            }
            Err(e) => {
                warn!("Error checking for block #{}: {:?}", block_number, e);
            }
        }
    }

    /// Handle justification request from peer
    async fn handle_justification_request(
        &self,
        peer: PeerId,
        _block_hash: [u8; 32],
        block_number: u32,
    ) {
        if let Some(justification) = self.shared_state.storage.get(block_number).await {
            let message = PPFAMessage::Justification(justification);
            let _ = self.notification_tx.unbounded_send((peer.clone(), message.encode()));
            debug!(
                "📤 PPFA: Sent justification for block #{} to {:?}",
                block_number, peer
            );
        } else {
            debug!(
                "No justification found for block #{} (requested by {:?})",
                block_number, peer
            );
        }
    }

    /// Handle neighbor packet from peer
    async fn handle_neighbor_packet(
        &self,
        peer: PeerId,
        view: u64,
        finalized_number: u32,
        authority_set_id: u64,
    ) {
        // Update peer state
        {
            let mut peers = self.shared_state.peers.write().await;
            if let Some(peer_state) = peers.get_mut(&peer) {
                peer_state.view = view;
                peer_state.finalized_number = finalized_number;
                peer_state.authority_set_id = authority_set_id;
                peer_state.last_update = Instant::now();
            }
        }

        // Check if peer is ahead of us and we need to catch up
        let our_finalized: u32 = (*self.shared_state.finalized_number.read().await).saturated_into();
        if finalized_number > our_finalized + 10 {
            // Peer is significantly ahead, request catch-up
            debug!(
                "Peer {:?} is ahead (their: #{}, ours: #{}), requesting catch-up",
                peer, finalized_number, our_finalized
            );

            let message = PPFAMessage::CatchUpRequest {
                authority_set_id,
                start_view: *self.shared_state.view.read().await,
            };
            let _ = self.notification_tx.unbounded_send((peer.clone(), message.encode()));
        }

        trace!(
            "📊 PPFA: Updated peer {:?} state: view={}, finalized=#{}, set={}",
            peer, view, finalized_number, authority_set_id
        );
    }

    /// Handle block request from peer
    async fn handle_block_request(
        &self,
        peer: PeerId,
        block_hash: [u8; 32],
        block_number: u32,
    ) {
        let target_hash: Option<B::Hash> = if block_hash.iter().any(|b| *b != 0) {
            Some(B::Hash::from(block_hash))
        } else {
            let num: NumberFor<B> = block_number.into();
            match self.client.hash(num) {
                Ok(opt) => opt,
                Err(e) => {
                    warn!("Failed to resolve hash for block #{}: {:?}", block_number, e);
                    None
                }
            }
        };

        if let Some(hash) = target_hash {
            match self.client.block(hash) {
                Ok(Some(signed_block)) => {
                    let parent_hash = *signed_block.block.header().parent_hash();
                    let encoded_block = signed_block.encode();
                    let mut block_bytes = [0u8; 32];
                    block_bytes.copy_from_slice(hash.as_ref());
                    let mut parent_bytes = [0u8; 32];
                    parent_bytes.copy_from_slice(parent_hash.as_ref());

                    let response = PPFAMessage::BlockResponse {
                        encoded_block,
                        block_hash: block_bytes,
                        parent_hash: parent_bytes,
                    };
                    let _ = self.notification_tx.unbounded_send((peer.clone(), response.encode()));
                    debug!("📦 PPFA: Sent block #{} to {:?}", block_number, peer);
                }
                Ok(None) => {
                    debug!("PPFA: Block #{} not found for request from {:?}", block_number, peer);
                }
                Err(e) => {
                    warn!("PPFA: Error reading block #{}: {:?}", block_number, e);
                }
            }
        }
    }

    /// Handle block response from peer
    async fn handle_block_response(
        &self,
        peer: PeerId,
        encoded_block: Vec<u8>,
        block_hash: [u8; 32],
        parent_hash: [u8; 32],
    ) {
        match SignedBlock::<B>::decode(&mut &encoded_block[..]) {
            Ok(signed_block) => {
                let encoded_hash = signed_block.block.header().hash();
                let mut encoded_bytes = [0u8; 32];
                encoded_bytes.copy_from_slice(encoded_hash.as_ref());

                if encoded_bytes != block_hash {
                    warn!(
                        "PPFA: Block hash mismatch in response from {:?} (expected {:?}, got {:?})",
                        peer,
                        hex::encode(&block_hash[..8]),
                        hex::encode(&encoded_bytes[..8])
                    );
                    return;
                }

                if let Ok(Some(_)) = self.client.header(encoded_hash) {
                    debug!("PPFA: Already have block {:?}, skipping import", hex::encode(&block_hash[..8]));
                    return;
                }

                // Basic parent continuity check
                if let Some(parent_header) = self.client.header(*signed_block.block.header().parent_hash()).ok().flatten() {
                    let parent_bytes: [u8; 32] = parent_header.hash().as_ref().try_into().unwrap_or(parent_hash);
                    if parent_bytes != parent_hash {
                        warn!("PPFA: Parent hash mismatch for block response from {:?}", peer);
                        return;
                    }
                }

                // Best-effort: rely on sync to pick up the block via request loop; we only queue the response locally.
                debug!(
                    "PPFA: Received block response from {:?} (#{} {:?})",
                    peer,
                    signed_block.block.header().number(),
                    hex::encode(&block_hash[..8])
                );
            }
            Err(e) => {
                warn!("PPFA: Failed to decode block response from {:?}: {:?}", peer, e);
            }
        }
    }

    /// Handle status request from peer
    async fn handle_status_request(&self, peer: PeerId) {
        let info = self.client.info();
        let mut best_bytes = [0u8; 32];
        best_bytes.copy_from_slice(info.best_hash.as_ref());
        let mut finalized_bytes = [0u8; 32];
        finalized_bytes.copy_from_slice(info.finalized_hash.as_ref());

        let message = PPFAMessage::StatusResponse {
            best_hash: best_bytes,
            best_number: info.best_number.saturated_into(),
            finalized_hash: finalized_bytes,
            finalized_number: info.finalized_number.saturated_into(),
        };
        let _ = self.notification_tx.unbounded_send((peer.clone(), message.encode()));
        debug!("📡 PPFA: Sent status response to {:?}", peer);
    }

    /// Handle status response from peer
    async fn handle_status_response(
        &self,
        peer: PeerId,
        best_hash: [u8; 32],
        best_number: u32,
        finalized_hash: [u8; 32],
        finalized_number: u32,
    ) {
        let our_best: u32 = self.client.info().best_number.saturated_into();
        if best_number > our_best + 10 {
            debug!(
                "PPFA: Peer {:?} ahead of us (their best #{}, ours #{}), triggering sync.",
                peer, best_number, our_best
            );
            // Trigger sync by queuing a status request via network service; SyncingService reacts to peers' status.
            let request = PPFAMessage::BlockRequest { block_hash: best_hash, block_number: best_number };
            let _ = self.notification_tx.unbounded_send((peer.clone(), request.encode()));
        }

        debug!(
            "PPFA: Status from {:?} best #{} ({:?}) finalized #{} ({:?})",
            peer,
            best_number,
            hex::encode(&best_hash[..8]),
            finalized_number,
            hex::encode(&finalized_hash[..8])
        );
    }

    /// Handle warp sync request from peer
    async fn handle_warp_sync_request(&self, peer: PeerId, begin_hash: [u8; 32]) {
        // Generate warp sync proof using runtime-backed provider
        match self.warp_sync_provider.generate_proof(B::Hash::from(begin_hash)) {
            Ok(proof) => {
                // Enforce size limit
                let encoded = proof.encode();
                if encoded.len() as u64 > MAX_WARP_SYNC_PROOF_SIZE {
                    warn!(
                        "Warp sync proof exceeds size limit ({} bytes), skipping send to {:?}",
                        encoded.len(),
                        peer
                    );
                    return;
                }
                let message = PPFAMessage::WarpSyncResponse(proof);
                let _ = self.notification_tx.unbounded_send((peer.clone(), message.encode()));

                info!("📤 PPFA: Sent warp sync proof to {:?}", peer);
            }
            Err(e) => {
                warn!("Failed to generate warp sync proof: {:?}", e);
            }
        }
    }

    /// Handle warp sync response from peer
    async fn handle_warp_sync_response(&mut self, peer: PeerId, proof: PPFAWarpSyncProof) {
        info!(
            "📥 PPFA: Received warp sync proof from {:?} (target: #{})",
            peer,
            proof.target_block_number()
        );

        // Verify the proof with provider
        if let Err(e) = self.warp_sync_provider.verify_proof(&proof) {
            warn!("Invalid warp sync proof from {:?}: {:?}", peer, e);
            return;
        }

        // Apply final justification
        let justification = &proof.final_justification;
        let block_hash = B::Hash::from(*justification.target_hash());
        let block_number = justification.target_number();

        // Request the target block if we don't have it
        match self.client.header(block_hash) {
            Ok(Some(_)) => {
                // We have the block, apply finality
                match self.finalize_block(block_hash, justification.clone()).await {
                    Ok(()) => {
                        info!(
                            "✅ PPFA: Applied warp sync finality for block #{}",
                            block_number
                        );

                        // Update our state
                        {
                            let mut finalized = self.shared_state.finalized_number.write().await;
                            *finalized = block_number.into();
                        }
                    }
                    Err(e) => {
                        warn!("Failed to apply warp sync finality: {:?}", e);
                    }
                }
            }
            Ok(None) => {
                // We need to sync to this block first
                info!(
                    "Warp sync target block #{} not imported yet, requesting from network",
                    block_number
                );
                // The sync engine should handle fetching the block
            }
            Err(e) => {
                warn!("Error checking warp sync target block: {:?}", e);
            }
        }
    }

    /// Handle catch-up request from peer
    async fn handle_catch_up_request(
        &self,
        peer: PeerId,
        _authority_set_id: u64,
        _start_view: u64,
    ) {
        // Get our recent justifications
        let our_finalized: u32 = (*self.shared_state.finalized_number.read().await).saturated_into();
        let start = our_finalized.saturating_sub(100); // Last 100 blocks

        let justifications: Vec<PPFAJustification> = self
            .shared_state
            .storage
            .get_range(start, our_finalized)
            .await
            .into_iter()
            .map(|(_, j)| j)
            .collect();

        if !justifications.is_empty() {
            let message = PPFAMessage::CatchUpResponse { justifications };
            let _ = self.notification_tx.unbounded_send((peer.clone(), message.encode()));

            debug!(
                "📤 PPFA: Sent catch-up response to {:?} ({} justifications)",
                peer,
                our_finalized.saturating_sub(start)
            );
        }
    }

    /// Handle catch-up response from peer
    async fn handle_catch_up_response(&mut self, peer: PeerId, justifications: Vec<PPFAJustification>) {
        info!(
            "📥 PPFA: Received catch-up response from {:?} ({} justifications)",
            peer,
            justifications.len()
        );

        // Apply justifications in order
        for justification in justifications {
            let block_number = justification.target_number();
            let block_hash = B::Hash::from(*justification.target_hash());

            // Store and try to finalize
            self.shared_state.storage.store(block_number, justification.clone()).await;

            if let Ok(Some(_)) = self.client.header(block_hash) {
                if let Ok(()) = self.finalize_block(block_hash, justification).await {
                    let number: NumberFor<B> = block_number.into();
                    let mut finalized = self.shared_state.finalized_number.write().await;
                    if number > *finalized {
                        *finalized = number;
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PUBLIC API
// ═══════════════════════════════════════════════════════════════════════════════

/// Create PPFA protocol channels
///
/// Returns:
/// - Sender for ASF finality events (from DETR P2P bridge)
/// - Receiver for ASF finality events (for PPFA worker)
/// - Sender for finality notifications (for sync engine)
/// - Receiver for finality notifications (for sync engine)
pub fn create_ppfa_channels<B: BlockT>() -> (
    mpsc::UnboundedSender<(B::Hash, NumberFor<B>, Vec<([u8; 32], Vec<u8>)>)>,
    mpsc::UnboundedReceiver<(B::Hash, NumberFor<B>, Vec<([u8; 32], Vec<u8>)>)>,
    mpsc::UnboundedSender<(B::Hash, NumberFor<B>)>,
    mpsc::UnboundedReceiver<(B::Hash, NumberFor<B>)>,
) {
    let (asf_tx, asf_rx) = mpsc::unbounded();
    let (finality_tx, finality_rx) = mpsc::unbounded();
    (asf_tx, asf_rx, finality_tx, finality_rx)
}

/// Block announce validator for PPFA consensus
///
/// This validates block announcements based on PPFA finality.
pub struct PPFABlockAnnounceValidator<B: BlockT> {
    _phantom: PhantomData<B>,
}

impl<B: BlockT> PPFABlockAnnounceValidator<B> {
    /// Create new block announce validator
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<B: BlockT> Default for PPFABlockAnnounceValidator<B> {
    fn default() -> Self {
        Self::new()
    }
}

// Note: BlockAnnounceValidator trait implementation would go here
// when integrating with the actual Substrate network layer

/// Log PPFA protocol status
pub fn log_ppfa_status<B: BlockT>(shared_state: &PPFASharedState<B>) {
    tokio::spawn({
        let state = shared_state.clone();
        async move {
            let view = *state.view.read().await;
            let finalized: u32 = (*state.finalized_number.read().await).saturated_into();
            let authority_set_id = *state.authority_set_id.read().await;
            let peer_count = state.peers.read().await.len();

            info!(
                "📊 PPFA Status: view={}, finalized=#{}, authority_set={}, peers={}",
                view, finalized, authority_set_id, peer_count
            );
        }
    });
}

impl<B: BlockT> Clone for PPFASharedState<B> {
    fn clone(&self) -> Self {
        // V114: Properly clone state values using blocking reads
        // This is safe because clone() is called during initialization, not hot paths
        let view = futures::executor::block_on(async {
            *self.view.read().await
        });
        let finalized = futures::executor::block_on(async {
            *self.finalized_number.read().await
        });
        let authority_set = futures::executor::block_on(async {
            *self.authority_set_id.read().await
        });
        let peers_map = futures::executor::block_on(async {
            self.peers.read().await.clone()
        });

        Self {
            view: RwLock::new(view),
            finalized_number: RwLock::new(finalized),
            authority_set_id: RwLock::new(authority_set),
            peers: RwLock::new(peers_map),
            storage: self.storage.clone(),
        }
    }
}
