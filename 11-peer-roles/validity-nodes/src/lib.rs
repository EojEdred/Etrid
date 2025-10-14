//! Ëtrid Peer-Roles — Validity Nodes
//!
//! Partition-Burst-Chain validators executing consensus on side chains.

#![cfg_attr(not(feature = "std"), no_std)]

use peer_roles_staking_types::{Role, RoleInterface};
use frame_support::dispatch::DispatchResult;
use frame_system::ensure_signed;
use sp_runtime::RuntimeDebug;

#[derive(Clone, Eq, PartialEq, RuntimeDebug)]
pub struct ValidationReport<AccountId> {
    pub account: AccountId,
    pub chain_id: u32,
    pub block_number: u64,
    pub valid: bool,
}

pub trait ValidityNodeApi<AccountId> {
    fn report_validation(
        who: &AccountId,
        chain_id: u32,
        block_number: u64,
        valid: bool,
    ) -> DispatchResult;
}

pub struct ValidityNode<T>(sp_std::marker::PhantomData<T>);

impl<T> ValidityNodeApi<T::AccountId> for ValidityNode<T>
where
    T: frame_system::Config + peer_roles_staking_types::RoleInterface<T::AccountId, u128>,
{
    fn report_validation(
        who: &T::AccountId,
        chain_id: u32,
        block_number: u64,
        valid: bool,
    ) -> DispatchResult {
        if let Some(role) = T::get_role(who) {
            ensure!(role == Role::ValidityNode, frame_support::error::BadOrigin);
        }
        log::info!(
            "Validity node {:?} reported block {} on chain {} as {:?}",
            who,
            block_number,
            chain_id,
            valid
        );
        Ok(())
    }
}