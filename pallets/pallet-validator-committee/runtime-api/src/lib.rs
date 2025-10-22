//! # Validator Committee Runtime API
//!
//! This crate defines the Runtime API for querying the ASF validator committee.
//! It allows the node service layer to query committee state without directly
//! accessing runtime storage.
//!
//! ## Usage
//!
//! From the node service (e.g., `asf_service.rs`):
//!
//! ```rust,ignore
//! use pallet_validator_committee_runtime_api::ValidatorCommitteeApi;
//!
//! // Query active committee
//! let committee = client.runtime_api()
//!     .validator_committee(at_hash)?;
//!
//! // Check if specific validator is active
//! let is_active = client.runtime_api()
//!     .is_validator_active(at_hash, &validator_id)?;
//!
//! // Get current epoch
//! let epoch = client.runtime_api()
//!     .current_epoch(at_hash)?;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_std::vec::Vec;

// Re-export types
pub use asf_algorithm::ValidatorId;
pub use validator_management::ValidatorInfo;

/// Runtime API for validator committee queries
sp_api::decl_runtime_apis! {
    /// API for querying the validator committee state
    pub trait ValidatorCommitteeApi {
        /// Get all active committee members
        ///
        /// Returns a list of ValidatorInfo for all validators in the active committee.
        /// This is used by the PPFA proposer selection logic.
        fn validator_committee() -> Vec<ValidatorInfo>;

        /// Get specific validator information
        ///
        /// Returns ValidatorInfo if the validator exists in storage, None otherwise.
        fn validator_info(validator_id: ValidatorId) -> Option<ValidatorInfo>;

        /// Check if a validator is in the active committee
        ///
        /// Returns true if the validator is currently in the active committee.
        fn is_validator_active(validator_id: ValidatorId) -> bool;

        /// Get the current epoch number
        ///
        /// Returns the current epoch, incremented on each committee rotation.
        fn current_epoch() -> u64;

        /// Get committee size limit
        ///
        /// Returns the maximum allowed committee size.
        fn committee_size_limit() -> u32;

        /// Get next epoch start block
        ///
        /// Returns the block number when the next epoch begins.
        fn next_epoch_start() -> u32;

        /// Get validators for next epoch
        ///
        /// Returns the list of validators scheduled for the next epoch.
        fn next_epoch_validators() -> Vec<ValidatorInfo>;

        /// Check if proposer was authorized for specific block/ppfa_index
        ///
        /// Validates that the given proposer was authorized to produce a block
        /// at the specified block number and PPFA index. This is the core
        /// security check for PPFA block production authorization.
        ///
        /// # Parameters
        /// - `block_number`: The block number being validated
        /// - `ppfa_index`: The PPFA rotation index from the block seal
        /// - `proposer_id`: The validator ID claiming to be the proposer
        ///
        /// # Returns
        /// - `true` if the proposer was authorized for this block/ppfa_index
        /// - `false` if unauthorized (block should be rejected)
        fn is_proposer_authorized(
            block_number: u32,
            ppfa_index: u32,
            proposer_id: ValidatorId,
        ) -> bool;

        /// Get epoch duration in blocks
        ///
        /// Returns the number of blocks per epoch.
        fn epoch_duration() -> u32;
    }
}
