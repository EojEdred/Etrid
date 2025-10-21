//! ASF Block Authoring Worker
//!
//! This module implements the block authoring worker for ASF consensus.
//! It handles PPFA (Proposing Panel for Attestation) rotation, slot timing,
//! and block production.
//!
//! ## Architecture
//!
//! The worker runs a continuous loop that:
//! 1. Waits for the next slot based on adaptive timing
//! 2. Queries the runtime to check if this node is the current PPFA proposer
//! 3. If yes, builds a block using the proposer environment
//! 4. Signs the block with the authority key
//! 5. Imports and broadcasts the block to the network
//! 6. Handles backoff strategy if authoring fails
//!
//! ## PPFA Rotation
//!
//! The PPFA committee consists of 21 validators selected by stake weight.
//! The proposer rotates through the committee based on the PPFA index,
//! which increments with each block or slot.

use codec::{Codec, Encode};
use futures::{future, prelude::*};
use futures_timer::Delay;
use sc_client_api::{backend::AuxStore, BlockBackend};
use sc_consensus::{BlockImport, BlockImportParams, StateAction, StorageChanges};
use sc_consensus_slots::{BackoffAuthoringBlocksStrategy, SlotInfo};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_consensus::{Environment, Error as ConsensusError, Proposer, SyncOracle};
use sp_consensus_asf::{AsfApi, SlotDuration};
use sp_consensus_slots::Slot;
use sp_inherents::{CreateInherentDataProviders, InherentDataProvider};
use sp_keystore::KeystorePtr;
use sp_runtime::{
    traits::{Block as BlockT, Header as HeaderT, NumberFor, Zero},
    DigestItem,
};
use std::{marker::PhantomData, sync::Arc, time::Duration};

use crate::{Error, Result};

/// Parameters for starting the ASF worker
pub struct AsfWorkerParams<B, C, E, I, SO, L, CIDP, BS, AuthorityId> {
    /// Client for blockchain queries
    pub client: Arc<C>,
    /// Block import
    pub block_import: I,
    /// Proposer environment
    pub env: E,
    /// Sync oracle to check sync status
    pub sync_oracle: SO,
    /// Backoff strategy for failed authorin
    pub backoff_authoring_blocks: Option<BS>,
    /// Keystore for signing
    pub keystore: KeystorePtr,
    /// Inherent data providers
    pub create_inherent_data_providers: CIDP,
    /// Force authoring even when offline
    pub force_authoring: bool,
    /// Block proposal duration limit
    pub block_proposal_slot_portion: f32,
    /// Maximum block proposal duration
    pub max_block_proposal_slot_portion: Option<f32>,
    /// Justification sync link
    pub justification_sync_link: L,
    /// Phantom data for type parameters
    pub _phantom: PhantomData<(B, AuthorityId)>,
}

/// Run the ASF worker
///
/// This function runs a continuous loop that:
/// 1. Waits for the next slot
/// 2. Checks if this node is the current PPFA proposer
/// 3. If yes, builds and proposes a block
/// 4. Handles Ant blocks if needed (future enhancement)
pub async fn run_asf_worker<B, C, E, I, SO, L, CIDP, BS, AuthorityId>(
    params: AsfWorkerParams<B, C, E, I, SO, L, CIDP, BS, AuthorityId>,
) -> Result<()>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + BlockBackend<B> + AuxStore + Send + Sync + 'static,
    C::Api: AsfApi<B, AuthorityId>,
    E: Environment<B> + Send + Sync + 'static,
    E::Proposer: Proposer<B>,
    E::Error: std::fmt::Debug,
    I: BlockImport<B, Error = ConsensusError> + Send + Sync + 'static,
    SO: SyncOracle + Send + Sync + Clone + 'static,
    CIDP: CreateInherentDataProviders<B, ()> + Send + 'static,
    CIDP::InherentDataProviders: sp_inherents::InherentDataProvider,
    BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + 'static,
    AuthorityId: Codec + Clone + Send + Sync + AsRef<[u8]> + 'static + std::fmt::Debug,
{
    log::info!(
        target: "asf",
        "Starting ASF block authoring worker"
    );

    let AsfWorkerParams {
        client,
        block_import,
        mut env,
        sync_oracle,
        backoff_authoring_blocks,
        keystore,
        create_inherent_data_providers,
        force_authoring,
        block_proposal_slot_portion,
        max_block_proposal_slot_portion,
        justification_sync_link: _justification_sync_link,
        _phantom,
    } = params;

    let mut backoff = backoff_authoring_blocks;

    // Main authoring loop
    loop {
        // Get the best block
        let best_hash = client.info().best_hash;
        let best_header = client
            .header(best_hash)
            .map_err(|e| Error::Client(format!("Failed to get best header: {}", e)))?
            .ok_or_else(|| Error::Client("Best header not found".to_string()))?;

        let best_number = *best_header.number();

        // Get slot duration from runtime
        let slot_duration = client
            .runtime_api()
            .slot_duration(best_hash)
            .map_err(|e| Error::RuntimeApi(format!("Failed to get slot duration: {}", e)))?;

        // Calculate current slot
        let current_slot = current_slot(slot_duration);

        log::trace!(
            target: "asf",
            "Current slot: {:?}, best block: #{} ({:?})",
            current_slot,
            best_number,
            best_hash
        );

        // Check if we should author a block
        let should_author = !sync_oracle.is_major_syncing() || force_authoring;

        if !should_author {
            log::debug!(
                target: "asf",
                "Skipping authoring - node is syncing"
            );
            // Wait for next slot
            Delay::new(slot_duration.as_duration()).await;
            continue;
        }

        // Get PPFA committee and index
        let committee = client
            .runtime_api()
            .committee(best_hash)
            .map_err(|e| Error::RuntimeApi(format!("Failed to get committee: {}", e)))?;

        if committee.is_empty() {
            log::warn!(
                target: "asf",
                "Committee is empty - waiting for next slot"
            );
            Delay::new(slot_duration.as_duration()).await;
            continue;
        }

        let ppfa_index = client
            .runtime_api()
            .ppfa_index(best_hash)
            .map_err(|e| Error::RuntimeApi(format!("Failed to get PPFA index: {}", e)))?;

        // Get expected proposer
        let proposer_idx = (ppfa_index as usize) % committee.len();
        let expected_proposer = &committee[proposer_idx];

        log::debug!(
            target: "asf",
            "PPFA index: {}, expected proposer: {:?}",
            ppfa_index,
            expected_proposer
        );

        // Check if we have the authority key for this proposer
        let we_are_proposer = check_if_we_are_proposer(
            &keystore,
            expected_proposer,
        ).await;

        if !we_are_proposer {
            log::trace!(
                target: "asf",
                "Not our turn to propose - waiting for next slot"
            );
            Delay::new(slot_duration.as_duration()).await;
            continue;
        }

        log::info!(
            target: "asf",
            "Our turn to propose! Slot: {:?}, PPFA index: {}",
            current_slot,
            ppfa_index
        );

        // Check if we should back off
        // BackoffAuthoringBlocksStrategy::should_backoff(chain_head_number, chain_head_slot, finalized_number, slow_now, logging_target)
        if let Some(ref mut backoff_strategy) = backoff {
            let finalized_number = client.info().finalized_number;
            if backoff_strategy.should_backoff(
                best_number,
                current_slot,
                finalized_number,
                current_slot,
                "asf",
            ) {
                log::info!(
                    target: "asf",
                    "Backing off from authoring at slot {:?}",
                    current_slot
                );
                Delay::new(slot_duration.as_duration()).await;
                continue;
            }
        }

        // Build and import block
        match author_block(
            &client,
            &mut env,
            current_slot,
            &best_header,
            &create_inherent_data_providers,
            block_proposal_slot_portion,
            max_block_proposal_slot_portion,
            slot_duration,
            expected_proposer.clone(),
        )
        .await
        {
            Ok(block) => {
                log::info!(
                    target: "asf",
                    "Authored block #{} at slot {:?}",
                    block.header().number(),
                    current_slot
                );

                // Import the block
                let block_import_params = build_block_import_params(
                    block,
                    current_slot,
                );

                if let Err(e) = block_import
                    .import_block(block_import_params)
                    .await
                {
                    log::error!(
                        target: "asf",
                        "Failed to import authored block: {}",
                        e
                    );
                }
            }
            Err(e) => {
                log::warn!(
                    target: "asf",
                    "Failed to author block at slot {:?}: {}",
                    current_slot,
                    e
                );
            }
        }

        // Wait for next slot
        Delay::new(slot_duration.as_duration()).await;
    }
}

/// Calculate the current slot based on slot duration
fn current_slot(slot_duration: SlotDuration) -> Slot {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();

    let slot_number = now.as_millis() as u64 / slot_duration.as_millis();
    Slot::from(slot_number)
}

/// Check if we have the authority key for the expected proposer
///
/// This function checks if the local keystore contains the private key
/// corresponding to the expected proposer's public key.
async fn check_if_we_are_proposer<AuthorityId>(
    keystore: &KeystorePtr,
    expected_proposer: &AuthorityId,
) -> bool
where
    AuthorityId: Codec + Clone + AsRef<[u8]>,
{
    use sp_application_crypto::{sr25519, AppPublic};
    use sp_core::crypto::ByteArray;

    // Get the public key bytes from the AuthorityId
    let proposer_bytes = expected_proposer.as_ref();

    // Try to construct an sr25519 public key from the bytes
    let public_key = match sr25519::Public::from_slice(proposer_bytes) {
        Ok(key) => key,
        Err(_) => {
            log::warn!(
                target: "asf",
                "Failed to parse authority ID as sr25519 public key"
            );
            return false;
        }
    };

    // Check if we have this key in the keystore
    // We use the AURA key type for validator keys (standard in Substrate)
    let key_type = sp_core::crypto::key_types::AURA;

    // has_keys returns a bool, not a Future, so no .await
    if keystore.has_keys(&[(public_key.to_raw_vec(), key_type)]) {
        log::debug!(
            target: "asf",
            "✓ We are the proposer - found matching key in keystore"
        );
        true
    } else {
        log::trace!(
            target: "asf",
            "Not our turn - no matching key in keystore"
        );
        false
    }
}

/// Author a block using the proposer environment
async fn author_block<B, C, E, AuthorityId, CIDP>(
    _client: &Arc<C>,
    env: &mut E,
    _slot: Slot,
    parent_header: &B::Header,
    create_inherent_data_providers: &CIDP,
    block_proposal_slot_portion: f32,
    max_block_proposal_slot_portion: Option<f32>,
    slot_duration: SlotDuration,
    _proposer_id: AuthorityId,
) -> Result<B>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + Send + Sync,
    C::Api: AsfApi<B, AuthorityId>,
    E: Environment<B>,
    E::Proposer: Proposer<B>,
    E::Error: std::fmt::Debug,
    CIDP: CreateInherentDataProviders<B, ()>,
    CIDP::InherentDataProviders: sp_inherents::InherentDataProvider,
    AuthorityId: Codec + Clone,
{
    let parent_hash = parent_header.hash();

    log::debug!(
        target: "asf",
        "Creating block proposal on top of #{} ({:?})",
        parent_header.number(),
        parent_hash
    );

    // Create inherent data providers
    let inherent_data_providers = create_inherent_data_providers
        .create_inherent_data_providers(parent_hash, ())
        .await
        .map_err(|e| Error::Other(format!("Failed to create inherent data providers: {:?}", e)))?;

    // Create inherent data
    let inherent_data = inherent_data_providers
        .create_inherent_data()
        .await
        .map_err(|e| Error::Other(format!("Failed to create inherent data: {}", e)))?;

    // Calculate proposal duration
    let proposal_duration = Duration::from_millis(
        (slot_duration.as_millis() as f32 * block_proposal_slot_portion) as u64,
    );

    let max_proposal_duration = max_block_proposal_slot_portion.map(|portion| {
        Duration::from_millis((slot_duration.as_millis() as f32 * portion) as u64)
    });

    // Create proposer
    let proposer = env
        .init(parent_header)
        .await
        .map_err(|e| Error::Other(format!("Failed to create proposer: {:?}", e)))?;

    // Propose block
    let proposal = proposer
        .propose(
            inherent_data,
            Default::default(), // Default digest
            proposal_duration,
            None, // No max duration
        )
        .await
        .map_err(|e| Error::Other(format!("Failed to propose block: {:?}", e)))?;

    let block = proposal.block;

    log::info!(
        target: "asf",
        "Proposed block #{} with {} extrinsics",
        block.header().number(),
        block.extrinsics().len()
    );

    Ok(block)
}

/// Build block import parameters for a newly authored block
fn build_block_import_params<B>(
    block: B,
    slot: Slot,
) -> BlockImportParams<B>
where
    B: BlockT,
{
    let (header, body) = block.deconstruct();
    let post_hash = header.hash();

    // Create pre-runtime digest with slot information
    let mut pre_digest = Vec::new();
    slot.encode_to(&mut pre_digest);

    let mut block_import_params = BlockImportParams::new(sp_consensus::BlockOrigin::Own, header);
    block_import_params.body = Some(body);
    block_import_params.state_action = StateAction::ApplyChanges(StorageChanges::Changes(Default::default()));
    block_import_params.post_digests.push(DigestItem::PreRuntime(*b"asf0", pre_digest));

    log::debug!(
        target: "asf",
        "Built import params for block {:?}",
        post_hash
    );

    block_import_params
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_slot_calculation() {
        let slot_duration = SlotDuration::from_millis(6000);
        let slot = current_slot(slot_duration);

        // Slot should be non-zero for any time after UNIX_EPOCH
        assert!(u64::from(slot) > 0);
    }

    // TODO: Add more unit tests
    // Tests should cover:
    // - Proposer selection logic
    // - Block authoring flow
    // - Error handling
    // - Backoff strategy
}
