//! Ëtrid Peer-Roles — Flare Nodes
//!
//! Root-chain validators responsible for finality attestation and state aggregation.
//! Interfaces directly with the staking pallet to confirm eligibility.

#![cfg_attr(not(feature = "std"), no_std)]

use peer_roles_staking_types::{Role, RoleInterface};
use frame_support::dispatch::DispatchResult;
use frame_system::ensure_signed;
use sp_runtime::RuntimeDebug;

#[derive(Clone, Eq, PartialEq, RuntimeDebug)]
pub struct AttestationRecord<AccountId> {
    pub account: AccountId,
    pub block_number: u64,
    pub state_root: [u8; 32],
    pub signature: [u8; 64],
}

pub trait FlareNodeApi<AccountId> {
    fn submit_attestation(
        who: &AccountId,
        state_root: [u8; 32],
        signature: [u8; 64],
    ) -> DispatchResult;
}

pub struct FlareNode<T>(sp_std::marker::PhantomData<T>);

impl<T> FlareNodeApi<T::AccountId> for FlareNode<T>
where
    T: frame_system::Config + peer_roles_staking_types::RoleInterface<T::AccountId, u128>,
{
    fn submit_attestation(
        who: &T::AccountId,
        state_root: [u8; 32],
        signature: [u8; 64],
    ) -> DispatchResult {
        // Verify the caller is a registered FlareNode
        if let Some(role) = T::get_role(who) {
            ensure!(role == Role::FlareNode, frame_support::error::BadOrigin);
        }
        // Here, the real ASF pipeline will validate signature + propagate to PPFA
        log::info!("Flare attestation received from {:?}", who);
        Ok(())
    }
}