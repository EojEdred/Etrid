#![allow(dead_code)]
#![allow(unused_imports)]
//! ASF Justification Import and Sync Link
//!
//! This module provides ASF consensus integration with Substrate's syncing engine.
//! It replaces GRANDPA's justification system with ASF's certificate-based finality.
//!
//! ## Key Components
//!
//! - `AsfJustificationImport`: Imports ASF certificates as block justifications
//! - `AsfSyncLink`: Provides the sync link interface for the SyncingEngine
//!
//! ## Why This Is Needed
//!
//! Substrate's SyncingEngine uses `select_next_some()` on internal streams that
//! expect finality-related events. Without proper justification handling, these
//! streams terminate and cause panics:
//!   "SelectNextSome polled after terminated"
//!
//! ASF provides its own finality (3-level: Pre-commit → Commit → Finalized),
//! so we need to bridge ASF finality events to Substrate's sync engine.

use codec::{Decode, Encode};
use futures::channel::mpsc;
use futures::stream::StreamExt;
use primearc_core_runtime::opaque::Block;
use sc_client_api::{Backend, BlockBackend, BlockchainEvents, HeaderBackend};
use sc_consensus::BlockImport;
use sp_api::ProvideRuntimeApi;
use sp_core::ByteArray;
use sp_consensus::BlockOrigin;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT, NumberFor};
use sp_runtime::Justification;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::RwLock;

/// ASF Certificate encoded as a Substrate Justification
///
/// This allows ASF finality proofs to be stored and imported as standard
/// Substrate justifications, maintaining compatibility with the sync engine.
#[derive(Clone, Debug, Encode, Decode)]
pub struct AsfCertificateJustification {
    /// View number when certificate was created
    pub view: u64,
    /// Block hash this certificate finalizes
    pub block_hash: [u8; 32],
    /// Signatures from validators (validator_id, signature)
    pub signatures: Vec<([u8; 32], Vec<u8>)>,
    /// Timestamp of certificate creation
    pub timestamp: u64,
}

/// ASF engine ID for justification identification
pub const ASF_ENGINE_ID: sp_runtime::ConsensusEngineId = *b"ASFJ";

impl AsfCertificateJustification {
    /// Convert to Substrate Justification format
    pub fn to_justification(&self) -> Justification {
        (ASF_ENGINE_ID, self.encode())
    }

    /// Try to decode from Substrate Justification
    pub fn from_justification(justification: &Justification) -> Option<Self> {
        if justification.0 == ASF_ENGINE_ID {
            Self::decode(&mut &justification.1[..]).ok()
        } else {
            None
        }
    }
}

/// ASF Justification Import
///
/// Handles importing ASF certificates as block justifications.
/// This integrates ASF finality with Substrate's import pipeline.
pub struct AsfJustificationImport<B, C, BE>
where
    B: BlockT,
    C: HeaderBackend<B> + BlockBackend<B> + ProvideRuntimeApi<B> + Send + Sync,
    BE: Backend<B>,
{
    client: Arc<C>,
    /// Channel to send finality notifications
    finality_tx: mpsc::UnboundedSender<(B::Hash, NumberFor<B>)>,
    /// Pending justifications awaiting import
    pending: Arc<RwLock<HashMap<B::Hash, AsfCertificateJustification>>>,
    _phantom: PhantomData<(B, BE)>,
}

impl<B, C, BE> AsfJustificationImport<B, C, BE>
where
    B: BlockT,
    C: HeaderBackend<B> + BlockBackend<B> + ProvideRuntimeApi<B> + Send + Sync,
    BE: Backend<B>,
{
    /// Create a new ASF justification import handler
    pub fn new(
        client: Arc<C>,
        finality_tx: mpsc::UnboundedSender<(B::Hash, NumberFor<B>)>,
    ) -> Self {
        Self {
            client,
            finality_tx,
            pending: Arc::new(RwLock::new(HashMap::new())),
            _phantom: PhantomData,
        }
    }

    /// Import an ASF certificate as a justification
    pub async fn import_certificate(&self, cert: AsfCertificateJustification, hash: B::Hash) -> Result<(), String> {
        // V114: Strictly enforce BFT signature threshold
        // Fixed ASF guardrail: committee size 21, threshold 15
        const COMMITTEE_SIZE: usize = 21;
        const BFT_THRESHOLD: usize = 15;

        if cert.signatures.len() < BFT_THRESHOLD {
            log::error!(
                "ASF certificate REJECTED: insufficient signatures {} < {} (BFT threshold for {} validators)",
                cert.signatures.len(),
                BFT_THRESHOLD,
                COMMITTEE_SIZE
            );
            return Err(format!(
                "Insufficient signatures: {} < {} required for BFT consensus",
                cert.signatures.len(),
                BFT_THRESHOLD
            ));
        }

        // Validate signer uniqueness and signature correctness against the expected payload
        use sp_core::sr25519;
        use sp_runtime::traits::Verify;
        use sp_runtime::MultiSignature;

        let mut seen = std::collections::BTreeSet::new();
        let mut valid_count = 0usize;

        // Payload: view + block_hash + timestamp
        let mut payload = Vec::with_capacity(8 + 32 + 8);
        payload.extend_from_slice(&cert.view.to_le_bytes());
        payload.extend_from_slice(&cert.block_hash);
        payload.extend_from_slice(&cert.timestamp.to_le_bytes());

        for (validator_id, sig_bytes) in cert.signatures.iter() {
            if validator_id.len() != 32 {
                log::warn!("Skipping signer with invalid id length {}", validator_id.len());
                continue;
            }
            if !seen.insert(*validator_id) {
                log::warn!("Duplicate signer detected: {:?}", hex::encode(&validator_id[..8]));
                continue;
            }
            let pubkey = sr25519::Public::from_raw(*validator_id);
            let account_id = sp_runtime::AccountId32::from(pubkey);
            if let Ok(sig) = sr25519::Signature::try_from(sig_bytes.as_slice()) {
                let multi_sig = MultiSignature::from(sig);
                if multi_sig.verify(payload.as_slice(), &account_id) {
                    valid_count += 1;
                } else {
                    log::warn!("Invalid signature from signer {:?}", hex::encode(&validator_id[..8]));
                }
            } else {
                log::warn!("Malformed signature from signer {:?}", hex::encode(&validator_id[..8]));
            }
        }

        if valid_count < BFT_THRESHOLD {
            return Err(format!(
                "ASF certificate REJECTED: only {} valid signatures < required {}",
                valid_count, BFT_THRESHOLD
            ));
        }

        // Store pending justification
        {
            let mut pending = self.pending.write().await;
            pending.insert(hash, cert.clone());
        }

        // Get block number for the finality notification
        match self.client.header(hash) {
            Ok(Some(header)) => {
                let number = *header.number();
                log::info!(
                    "🔒 ASF certificate imported for block #{} ({:?}), {} signatures",
                    number,
                    hash,
                    cert.signatures.len()
                );

                // Send finality notification
                if let Err(e) = self.finality_tx.unbounded_send((hash, number)) {
                    log::warn!("Failed to send ASF finality notification: {:?}", e);
                }

                Ok(())
            }
            Ok(None) => {
                log::warn!("ASF certificate for unknown block {:?}", hash);
                Err("Block not found".to_string())
            }
            Err(e) => {
                log::error!("Error looking up block {:?}: {:?}", hash, e);
                Err(format!("Client error: {:?}", e))
            }
        }
    }

    /// Get pending justification for a block
    pub async fn get_pending(&self, hash: &B::Hash) -> Option<AsfCertificateJustification> {
        let pending = self.pending.read().await;
        pending.get(hash).cloned()
    }
}

/// Clone implementation for AsfJustificationImport
impl<B, C, BE> Clone for AsfJustificationImport<B, C, BE>
where
    B: BlockT,
    C: HeaderBackend<B> + BlockBackend<B> + ProvideRuntimeApi<B> + Send + Sync,
    BE: Backend<B>,
{
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            finality_tx: self.finality_tx.clone(),
            pending: self.pending.clone(),
            _phantom: PhantomData,
        }
    }
}

/// ASF Sync Oracle
///
/// Provides sync status information to the network layer.
/// Reports whether the node is synced based on ASF finality state.
#[derive(Clone)]
    pub struct AsfSyncOracle<C> {
        client: Arc<C>,
        /// Receiver for finality notifications
        last_finalized: Arc<RwLock<Option<u32>>>,
        /// Tracks whether we have seen any finality (used to avoid false-healthy)
        has_finality: Arc<RwLock<bool>>,
    }

impl<C> AsfSyncOracle<C> {
    /// Create a new ASF sync oracle
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            last_finalized: Arc::new(RwLock::new(None)),
            has_finality: Arc::new(RwLock::new(false)),
        }
    }

    /// Update the last finalized block number
    pub async fn set_finalized(&self, number: u32) {
        let mut last = self.last_finalized.write().await;
        *last = Some(number);
        let mut seen = self.has_finality.write().await;
        *seen = true;
    }
}

impl<C> sp_consensus::SyncOracle for AsfSyncOracle<C>
where
    C: Send + Sync,
{
    fn is_major_syncing(&self) -> bool {
        // Report syncing until we have observed at least one finality update.
        let has_finality = futures::executor::block_on(async {
            *self.has_finality.read().await
        });
        !has_finality
    }

    fn is_offline(&self) -> bool {
        // Consider offline if we have never finalized and best number is zero.
        // This is a conservative heuristic to avoid false "healthy".
        let best_zero = futures::executor::block_on(async {
            let number = self.last_finalized.read().await;
            number.is_none()
        });
        best_zero
    }
}

/// Create ASF finality notification channels
///
/// Returns:
/// - Sender for finality notifications (used by AsfJustificationImport)
/// - Receiver stream (used by tasks that need finality updates)
pub fn create_finality_channels<B: BlockT>() -> (
    mpsc::UnboundedSender<(B::Hash, NumberFor<B>)>,
    mpsc::UnboundedReceiver<(B::Hash, NumberFor<B>)>,
) {
    mpsc::unbounded()
}

/// ASF Block Import Wrapper
///
/// Wraps the client's block import to add ASF-specific finality handling.
/// This ensures blocks are imported with proper ASF justification support.
pub struct AsfBlockImportWrapper<I, C>
where
    I: BlockImport<Block>,
    C: HeaderBackend<Block> + Send + Sync,
{
    inner: I,
    client: Arc<C>,
    finality_tx: mpsc::UnboundedSender<(<Block as BlockT>::Hash, NumberFor<Block>)>,
}

impl<I, C> AsfBlockImportWrapper<I, C>
where
    I: BlockImport<Block>,
    C: HeaderBackend<Block> + Send + Sync,
{
    /// Create a new ASF block import wrapper
    pub fn new(
        inner: I,
        client: Arc<C>,
        finality_tx: mpsc::UnboundedSender<(<Block as BlockT>::Hash, NumberFor<Block>)>,
    ) -> Self {
        Self {
            inner,
            client,
            finality_tx,
        }
    }
}

#[async_trait::async_trait]
impl<I, C> BlockImport<Block> for AsfBlockImportWrapper<I, C>
where
    I: BlockImport<Block> + Send + Sync,
    C: HeaderBackend<Block> + Send + Sync,
{
    type Error = I::Error;

    async fn check_block(
        &self,
        block: sc_consensus::BlockCheckParams<Block>,
    ) -> Result<sc_consensus::ImportResult, Self::Error> {
        self.inner.check_block(block).await
    }

    async fn import_block(
        &self,
        block: sc_consensus::BlockImportParams<Block>,
    ) -> Result<sc_consensus::ImportResult, Self::Error> {
        let hash = block.post_hash();
        let number = *block.header.number();
        let has_justification = block.justifications.is_some();

        // Import the block
        let result = self.inner.import_block(block).await?;

        // If block was imported successfully and has ASF justification,
        // send finality notification
        if matches!(result, sc_consensus::ImportResult::Imported(_)) {
            if has_justification {
                log::debug!(
                    "📦 Block #{} ({:?}) imported with ASF justification",
                    number,
                    hash
                );
                // V114: Send finality notification with proper error logging
                if let Err(e) = self.finality_tx.unbounded_send((hash, number)) {
                    log::warn!(
                        "⚠️ Failed to send ASF finality notification for block #{}: {:?}",
                        number,
                        e
                    );
                }
            } else {
                log::trace!(
                    "📦 Block #{} ({:?}) imported without justification",
                    number,
                    hash
                );
            }
        }

        Ok(result)
    }
}

impl<I, C> Clone for AsfBlockImportWrapper<I, C>
where
    I: BlockImport<Block> + Clone,
    C: HeaderBackend<Block> + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            client: self.client.clone(),
            finality_tx: self.finality_tx.clone(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// NO-OP JUSTIFICATION IMPORT
// ═══════════════════════════════════════════════════════════════════════════════

use sc_consensus::JustificationImport;

/// No-op justification import that satisfies Substrate's type requirements
/// without doing anything. ASF handles finality through DETR P2P independently.
///
/// ## Security Note
/// This is NOT a security component. It's purely a compatibility shim.
/// All finality security is enforced by the ASF Finality Engine which:
/// - Verifies Sr25519 signatures on votes
/// - Requires 2f+1 (15/21) signatures for certificates
/// - Implements equivocation detection and slashing
/// - Uses 3-certificate finality for defense in depth
pub struct NoOpJustificationImport<B: BlockT> {
    _phantom: PhantomData<B>,
}

impl<B: BlockT> NoOpJustificationImport<B> {
    /// Create a new no-op justification import
    pub fn new() -> Self {
        log::info!("🔗 ASF: Using NoOpJustificationImport - finality handled by ASF engine");
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<B: BlockT> Default for NoOpJustificationImport<B> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl<B: BlockT> JustificationImport<B> for NoOpJustificationImport<B> {
    type Error = sp_consensus::Error;

    async fn on_start(&mut self) -> Vec<(B::Hash, NumberFor<B>)> {
        // No-op: ASF doesn't request justifications on start
        // Certificate sync happens through DETR P2P protocol
        log::debug!("ASF: NoOpJustificationImport started - using DETR P2P for finality");
        Vec::new()
    }

    async fn import_justification(
        &mut self,
        _hash: B::Hash,
        _number: NumberFor<B>,
        _justification: Justification,
    ) -> Result<(), Self::Error> {
        // No-op: ASF doesn't use Substrate's justification import path
        // Certificates are received and verified through DETR P2P layer
        log::trace!("ASF: Ignoring Substrate justification import - using DETR P2P");
        Ok(())
    }
}

/// Type alias for the boxed justification import used in BasicQueue
pub type BoxedJustificationImport<B> = Box<dyn JustificationImport<B, Error = sp_consensus::Error> + Send + Sync>;

/// Create a boxed no-op justification import for use with BasicQueue
pub fn create_noop_justification_import<B: BlockT>() -> BoxedJustificationImport<B> {
    Box::new(NoOpJustificationImport::<B>::new())
}
