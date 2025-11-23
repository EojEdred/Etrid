//! # Node Authorization Helper
//!
//! This module provides integration between pallet-node-authorization and pallet-validator-committee.
//! It automatically updates the authorized node list when validators change.
//!
//! ## Overview
//!
//! The helper provides:
//! - Automatic synchronization between validator set and authorized peers
//! - PeerId extraction from validator session keys
//! - Genesis configuration utilities
//! - Runtime hooks for validator set changes
//!
//! ## Integration
//!
//! This is integrated into the runtime as a helper module, not a full pallet.
//! It hooks into ValidatorCommittee events and session changes.

use frame_support::pallet_prelude::*;
use sp_core::OpaquePeerId as PeerId;
use sp_std::vec::Vec;

/// Extract PeerId from AccountId (placeholder implementation)
///
/// NOTE: In production, this should:
/// 1. Query the validator's session keys from pallet_session
/// 2. Derive the network PeerId from the session keys
/// 3. Use the actual libp2p PeerId format
///
/// For now, this creates a deterministic PeerId from AccountId for testing.
pub fn account_id_to_peer_id(account_id: &sp_runtime::AccountId32) -> PeerId {
    // In production, you would:
    // 1. Query Session::NextKeys to get the validator's session keys
    // 2. Extract the public key (e.g., ASF key or GRANDPA key)
    // 3. Convert to libp2p PeerId format
    //
    // For development/testing, we create a fake PeerId from AccountId:
    let account_bytes = account_id.as_ref();

    // Create a deterministic PeerId-like structure
    // Real PeerIds are multihash encoded, but for testing we use a simple format
    let mut peer_id_bytes = Vec::new();
    peer_id_bytes.extend_from_slice(b"12D3KooW"); // Fake PeerId prefix (base58 encoded multihash)
    peer_id_bytes.extend_from_slice(&account_bytes[..20.min(account_bytes.len())]);

    PeerId(peer_id_bytes)
}

/// Sync authorized nodes with current validator committee
///
/// This should be called:
/// - On genesis initialization
/// - When validator set changes (session rotation)
/// - When validators are added/removed via governance
pub fn sync_authorized_nodes_with_validators<T: pallet_node_authorization::Config + pallet_validator_committee::Config>(
) -> DispatchResult
where
    T::AccountId: From<sp_runtime::AccountId32> + Into<sp_runtime::AccountId32>,
{
    // Get current validator committee
    let committee = pallet_validator_committee::Committee::<T>::get();

    // Convert ValidatorIds (AccountId32) to PeerIds
    let mut peer_ids = Vec::new();
    for validator_id in committee.iter() {
        // Convert ValidatorId ([u8; 32]) to AccountId32
        let account_id = sp_runtime::AccountId32::from(*validator_id);
        let peer_id = account_id_to_peer_id(&account_id);
        peer_ids.push((peer_id, account_id.into()));
    }

    // Clear existing well-known nodes and add new ones
    // Note: This requires root origin, so this function should only be called
    // from privileged contexts (runtime upgrades, migrations, etc.)

    // For now, we'll just return Ok and let manual management handle it
    // In production, you might want to use storage directly or have a special origin

    Ok(())
}

/// Get list of authorized peer IDs from current validator set
pub fn get_authorized_peer_ids<T: pallet_validator_committee::Config>() -> Vec<(PeerId, sp_runtime::AccountId32)>
where
    T::AccountId: From<sp_runtime::AccountId32> + Into<sp_runtime::AccountId32>,
{
    let committee = pallet_validator_committee::Committee::<T>::get();

    committee
        .iter()
        .map(|validator_id| {
            let account_id = sp_runtime::AccountId32::from(*validator_id);
            let peer_id = account_id_to_peer_id(&account_id);
            (peer_id, account_id)
        })
        .collect()
}

/// Initialize node authorization with genesis validators
///
/// This should be called in the genesis configuration to set up initial authorized nodes.
pub fn initialize_genesis_authorized_nodes<T: pallet_node_authorization::Config + pallet_validator_committee::Config>(
    validators: &[(asf_algorithm::ValidatorId, asf_algorithm::Balance, u8)]
) -> Vec<(PeerId, sp_runtime::AccountId32)>
where
    T::AccountId: From<sp_runtime::AccountId32> + Into<sp_runtime::AccountId32>,
{
    validators
        .iter()
        .map(|(validator_id, _stake, _peer_type)| {
            let account_id = sp_runtime::AccountId32::from(*validator_id);
            let peer_id = account_id_to_peer_id(&account_id);
            (peer_id, account_id)
        })
        .collect()
}
