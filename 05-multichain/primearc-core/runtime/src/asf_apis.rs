//! # ASF Runtime APIs
//!
//! Runtime APIs for the Adaptive Scale of Finality (ASF) consensus protocol.
//! These APIs allow clients and nodes to query consensus state without direct
//! runtime storage access.

use sp_api::decl_runtime_apis;
use sp_std::vec::Vec;
use asf_algorithm::{FinalityLevel, ValidatorId, Hash, BlockNumber};

/// ASF consensus runtime APIs
decl_runtime_apis! {
    /// ASF Consensus API - provides access to ASF consensus state
    pub trait AsfApi {
        /// Get the current active validator committee
        ///
        /// Returns a list of all validators currently participating in consensus.
        /// This includes both validators in the PPFA rotation and reserve validators.
        fn get_committee() -> Vec<ValidatorId>;

        /// Get the authorized proposer for a given slot
        ///
        /// # Parameters
        /// - `slot`: The slot number (typically block number)
        ///
        /// # Returns
        /// The ValidatorId of the authorized proposer, or None if slot is invalid
        fn get_proposer(slot: u64) -> Option<ValidatorId>;

        /// Get the finality level for a specific block
        ///
        /// Returns the Ascending Scale of Finality level (0-4) for the given block:
        /// - Level 0 (None): 0-9 validity certificates
        /// - Level 1 (Weak): 10-19 validity certificates
        /// - Level 2 (Moderate): 20-49 validity certificates
        /// - Level 3 (Strong): 50-99 validity certificates
        /// - Level 4 (Irreversible): 100+ validity certificates
        ///
        /// # Parameters
        /// - `block_hash`: Hash of the block to query
        ///
        /// # Returns
        /// The finality level for the block
        fn get_finality_level(block_hash: Hash) -> FinalityLevel;

        /// Check if a validator is currently excluded from consensus
        ///
        /// Validators can be excluded due to:
        /// - Slashing events (equivocation, double-signing)
        /// - Low reputation score
        /// - Insufficient stake
        /// - Manual exclusion by governance
        ///
        /// # Parameters
        /// - `validator`: The validator's AccountId
        ///
        /// # Returns
        /// `true` if validator is excluded, `false` if active
        fn is_validator_excluded(validator: ValidatorId) -> bool;

        /// Get the current epoch number
        ///
        /// Epochs define validator rotation periods. Each epoch lasts a
        /// configured number of blocks (default: 2400 blocks = 4 hours at 6s blocks).
        ///
        /// # Returns
        /// Current epoch number
        fn get_current_epoch() -> u64;

        /// Get the total stake in the current validator set
        ///
        /// # Returns
        /// Total stake weight across all active validators
        fn get_total_stake() -> u128;

        /// Get validator info including stake and status
        ///
        /// # Parameters
        /// - `validator`: The validator's AccountId
        ///
        /// # Returns
        /// Tuple of (stake, is_active, reputation_score) or None if not found
        fn get_validator_info(validator: ValidatorId) -> Option<(u128, bool, u32)>;

        /// Get the PPFA (Probabilistic Permissioned Finality Authorities) index
        /// for a given block number
        ///
        /// The PPFA rotation determines which subset of validators are active
        /// for finality voting at any given block.
        ///
        /// # Parameters
        /// - `block_number`: The block number to query
        ///
        /// # Returns
        /// The PPFA index (0-20 for 21-member committee)
        fn get_ppfa_index(block_number: BlockNumber) -> u32;

        /// Get the number of validity certificates accumulated for a block
        ///
        /// # Parameters
        /// - `block_hash`: Hash of the block to query
        ///
        /// # Returns
        /// Number of validity certificates (used to determine finality level)
        fn get_certificate_count(block_hash: Hash) -> u32;

        /// Check if a block has reached BFT finality threshold
        ///
        /// BFT threshold is 2/3 + 1 of the active validator set.
        ///
        /// # Parameters
        /// - `block_hash`: Hash of the block to query
        ///
        /// # Returns
        /// `true` if block has reached BFT threshold (finalized)
        fn has_bft_finality(block_hash: Hash) -> bool;

        /// Get the ASF key for a specific validator from session storage
        ///
        /// Queries `pallet_session::NextKeys` storage to retrieve the ASF public key
        /// published by a validator via `session.setKeys()` extrinsic.
        ///
        /// This allows the authority set to query ASF keys from all validators
        /// instead of using placeholder keys, fixing V25's signature verification failure.
        ///
        /// # Parameters
        /// - `account_id`: The validator's AccountId
        ///
        /// # Returns
        /// - `Some(AsfId)`: The validator's ASF public key if published
        /// - `None`: If the validator hasn't published session keys yet
        fn get_validator_asf_key(account_id: ValidatorId) -> Option<Vec<u8>>;

        /// Get all validator ASF keys from session storage
        ///
        /// Queries all validators in the current session and returns their ASF keys.
        /// This provides a complete mapping of AccountId -> AsfId for the authority set.
        ///
        /// Validators without published session keys are excluded from the results.
        ///
        /// # Returns
        /// Vector of (AccountId, AsfId) tuples for all validators with published keys
        fn get_all_validator_asf_keys() -> Vec<(ValidatorId, Vec<u8>)>;
    }
}
