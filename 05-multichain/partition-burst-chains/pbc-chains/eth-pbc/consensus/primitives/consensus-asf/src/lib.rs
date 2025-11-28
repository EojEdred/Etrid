#![cfg_attr(not(feature = "std"), no_std)]

//! Primitives for ASF (Adaptive Stake-weighted Finality) consensus
//!
//! This crate provides the runtime API interface for ASF consensus,
//! which replaces AURA with PPFA (Proposing Panel for Attestation)
//! committee-based block production.

use codec::{Codec, Decode, Encode};
use scale_info::TypeInfo;
use sp_consensus_slots::Slot;
use sp_std::vec::Vec;

/// ASF consensus slot duration in milliseconds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct SlotDuration(pub u64);

impl SlotDuration {
    /// Create a new slot duration
    pub const fn from_millis(millis: u64) -> Self {
        Self(millis)
    }

    /// Get the duration in milliseconds
    pub const fn as_millis(&self) -> u64 {
        self.0
    }

    /// Get the duration as a Duration (std feature only)
    #[cfg(feature = "std")]
    pub fn as_duration(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.0)
    }
}

/// ASF consensus inherent identifier
pub const INHERENT_IDENTIFIER: sp_inherents::InherentIdentifier = *b"asfslot0";

/// ASF consensus inherent data
#[derive(Debug, Encode, Decode)]
pub struct AsfInherentData {
    /// Current slot number
    pub slot: Slot,
    /// Current PPFA index
    pub ppfa_index: u32,
}

/// Provide inherent data for ASF consensus
#[cfg(feature = "std")]
pub struct InherentDataProvider {
    slot: Slot,
    ppfa_index: u32,
}

#[cfg(feature = "std")]
impl InherentDataProvider {
    /// Create a new inherent data provider
    pub fn new(slot: Slot, ppfa_index: u32) -> Self {
        Self { slot, ppfa_index }
    }

    /// Create inherent data from timestamp and slot duration
    pub fn from_timestamp_and_slot_duration(
        timestamp: sp_timestamp::Timestamp,
        slot_duration: SlotDuration,
        ppfa_index: u32,
    ) -> Self {
        let slot = Slot::from_timestamp(timestamp, slot_duration.into());
        Self::new(slot, ppfa_index)
    }
}

#[cfg(feature = "std")]
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for InherentDataProvider {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        let data = AsfInherentData {
            slot: self.slot,
            ppfa_index: self.ppfa_index,
        };
        inherent_data.put_data(INHERENT_IDENTIFIER, &data)
    }

    async fn try_handle_error(
        &self,
        identifier: &sp_inherents::InherentIdentifier,
        error: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        if *identifier != INHERENT_IDENTIFIER {
            return None;
        }

        Some(Err(sp_inherents::Error::Application(Box::from(
            String::from_utf8_lossy(error),
        ))))
    }
}

// Convert SlotDuration to sp_consensus_slots::SlotDuration
impl From<SlotDuration> for sp_consensus_slots::SlotDuration {
    fn from(duration: SlotDuration) -> Self {
        sp_consensus_slots::SlotDuration::from_millis(duration.0)
    }
}

impl From<sp_consensus_slots::SlotDuration> for SlotDuration {
    fn from(duration: sp_consensus_slots::SlotDuration) -> Self {
        SlotDuration(duration.as_millis())
    }
}

sp_api::decl_runtime_apis! {
    /// API for ASF consensus
    pub trait AsfApi<AuthorityId: Codec> {
        /// Get the current PPFA committee
        ///
        /// Returns the list of validator authority IDs in the current
        /// PPFA committee. The committee size is typically 21 validators.
        fn committee() -> Vec<AuthorityId>;

        /// Get the current PPFA index
        ///
        /// The PPFA index determines which validator in the committee
        /// should propose the next block. It rotates through the committee
        /// members.
        fn ppfa_index() -> u32;

        /// Get the adaptive slot duration
        ///
        /// ASF uses adaptive slot timing (6-18 seconds) based on network
        /// health and performance metrics.
        fn slot_duration() -> SlotDuration;

        /// Check if a validator should propose in the current slot
        ///
        /// Returns true if the given validator is the current proposer
        /// according to the PPFA rotation.
        fn should_propose(validator: AuthorityId) -> bool;

        /// Get the current epoch number
        ///
        /// Epochs last 2400 blocks (~4 hours). Committee rotation
        /// happens at epoch boundaries.
        fn current_epoch() -> u32;

        /// Get validators in the current active set
        ///
        /// Returns all active validators (up to 100), not just the
        /// committee members.
        fn active_validators() -> Vec<AuthorityId>;
    }
}
