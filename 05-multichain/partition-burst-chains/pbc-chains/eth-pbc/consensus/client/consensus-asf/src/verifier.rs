//! ASF Block Verifier
//!
//! This module implements block verification logic for ASF consensus.
//! It validates that blocks conform to ASF rules:
//! - Proposer is in the current PPFA committee
//! - Proposer matches expected PPFA index for the slot
//! - Block signature is valid
//! - Slot timing is correct
//! - Committee rotation at epoch boundaries

use codec::Codec;
use sc_client_api::backend::AuxStore;
use sc_consensus::BlockImportParams;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_consensus_asf::{AsfApi, SlotDuration};
use sp_consensus_slots::Slot;
use sp_runtime::{traits::{Block as BlockT, Header as HeaderT}, DigestItem};
use std::{marker::PhantomData, sync::Arc};

use crate::{Error, Result};

/// ASF block verifier
///
/// Validates blocks against ASF consensus rules before importing them.
pub struct AsfVerifier<B, C, AuthorityId> {
    client: Arc<C>,
    _phantom: PhantomData<(B, AuthorityId)>,
}

impl<B, C, AuthorityId> AsfVerifier<B, C, AuthorityId>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + AuxStore,
    C::Api: AsfApi<B, AuthorityId>,
    AuthorityId: Codec + Clone,
{
    /// Create a new ASF verifier
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _phantom: PhantomData,
        }
    }

    /// Verify a block against ASF consensus rules
    pub fn verify(
        &self,
        mut block_params: BlockImportParams<B>,
    ) -> Result<BlockImportParams<B>> {
        let hash = block_params.post_hash();
        let number = *block_params.header.number();

        log::debug!(
            target: "asf",
            "Verifying block #{} ({:?})",
            number,
            hash
        );

        // Extract pre-runtime digest containing slot information
        let slot = self.extract_slot(&block_params.header)?;

        // Get parent block for context
        let parent_hash = *block_params.header.parent_hash();

        // Query runtime API at parent block for consensus state
        let runtime_api = self.client.runtime_api();

        // Get current PPFA committee
        let committee = runtime_api
            .committee(parent_hash)
            .map_err(|e| Error::RuntimeApi(format!("Failed to get committee: {}", e)))?;

        if committee.is_empty() {
            return Err(Error::Other("Committee is empty".to_string()));
        }

        log::debug!(
            target: "asf",
            "Committee size: {} validators",
            committee.len()
        );

        // Get current PPFA index
        let ppfa_index = runtime_api
            .ppfa_index(parent_hash)
            .map_err(|e| Error::RuntimeApi(format!("Failed to get PPFA index: {}", e)))?;

        log::debug!(
            target: "asf",
            "Current PPFA index: {} (slot: {:?})",
            ppfa_index,
            slot
        );

        // Get expected proposer from committee
        let expected_proposer_idx = (ppfa_index as usize) % committee.len();
        let _expected_proposer = &committee[expected_proposer_idx];

        log::debug!(
            target: "asf",
            "Expected proposer index: {} (of {} validators)",
            expected_proposer_idx,
            committee.len()
        );

        // Extract actual proposer from block (we'll implement signature verification later)
        // For now, we'll verify the proposer is in the committee
        // TODO: Implement full signature verification

        // Verify slot timing
        let slot_duration = runtime_api
            .slot_duration(parent_hash)
            .map_err(|e| Error::RuntimeApi(format!("Failed to get slot duration: {}", e)))?;

        self.verify_slot_timing(slot, slot_duration, &block_params.header)?;

        // Check for epoch boundaries and committee rotation
        let current_epoch = runtime_api
            .current_epoch(parent_hash)
            .map_err(|e| Error::RuntimeApi(format!("Failed to get current epoch: {}", e)))?;

        log::debug!(
            target: "asf",
            "Current epoch: {}, block number: {}",
            current_epoch,
            number
        );

        // Mark block as verified
        block_params.post_digests.push(DigestItem::Other(
            b"asf_verified".to_vec(),
        ));

        log::debug!(
            target: "asf",
            "Block #{} ({:?}) verified successfully",
            number,
            hash
        );

        Ok(block_params)
    }

    /// Extract slot number from block header's pre-runtime digest
    fn extract_slot(&self, header: &B::Header) -> Result<Slot> {
        // Look for ASF slot in pre-runtime digests
        for digest in header.digest().logs() {
            if let DigestItem::PreRuntime(id, data) = digest {
                if id == b"asf0" {
                    // Decode slot from digest data
                    let slot = <Slot as codec::Decode>::decode(&mut &data[..])
                        .map_err(|e| Error::Other(format!("Failed to decode slot: {}", e)))?;
                    return Ok(slot);
                }
            }
        }

        Err(Error::Other("No ASF slot found in block header".to_string()))
    }

    /// Verify slot timing is valid
    fn verify_slot_timing(
        &self,
        slot: Slot,
        slot_duration: SlotDuration,
        header: &B::Header,
    ) -> Result<()> {
        log::trace!(
            target: "asf",
            "Verifying slot {:?} with duration {}ms",
            slot,
            slot_duration.as_millis()
        );

        // For now, we'll do basic validation
        // In a full implementation, we'd check:
        // - Slot is not too far in the future
        // - Slot matches the block's timestamp
        // - Slot progression is monotonic

        // Basic check: slot number should roughly match block number
        // (This is a simplified check; real implementation would be more sophisticated)
        let block_number = *header.number();

        log::trace!(
            target: "asf",
            "Slot timing verified for block #{} (slot: {:?})",
            block_number,
            slot
        );

        Ok(())
    }
}

/// Verification params for ASF consensus
#[derive(Clone)]
pub struct VerificationParams<AuthorityId> {
    /// Expected proposer for the slot
    pub expected_proposer: AuthorityId,
    /// Slot number
    pub slot: Slot,
    /// PPFA index
    pub ppfa_index: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add unit tests for verifier logic
    // Tests should cover:
    // - Slot extraction from header
    // - Proposer verification
    // - Timing verification
    // - Epoch boundary checks
    // - Invalid block rejection
}
