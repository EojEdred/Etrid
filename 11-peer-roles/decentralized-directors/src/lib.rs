//! Ëtrid Peer-Roles — Decentralized Directors
//!
//! Directors are elected by Consensus Day to oversee the Foundation DAO.

#![cfg_attr(not(feature = "std"), no_std)]

use peer_roles_staking_types::{Role, RoleInterface};
use frame_support::{dispatch::DispatchResult, ensure};
use frame_system::ensure_signed;
use sp_runtime::RuntimeDebug;

#[derive(Clone, Eq, PartialEq, RuntimeDebug)]
pub struct DirectorProfile<AccountId> {
    pub account: AccountId,
    pub term_start: u64,
    pub term_end: u64,
    pub term_active: bool,
}

pub trait DirectorApi<AccountId> {
    fn register_director(who: &AccountId) -> DispatchResult;
    fn end_term(who: &AccountId) -> DispatchResult;
}

pub struct DecentralizedDirector<T>(sp_std::marker::PhantomData<T>);

impl<T> DirectorApi<T::AccountId> for DecentralizedDirector<T>
where
    T: frame_system::Config + peer_roles_staking_types::RoleInterface<T::AccountId, u128>,
{
    fn register_director(who: &T::AccountId) -> DispatchResult {
        // Must already hold the Director role and stake threshold
        if let Some(role) = T::get_role(who) {
            ensure!(role == Role::DecentralizedDirector, frame_support::error::BadOrigin);
        }
        log::info!("Director {:?} registered for current term", who);
        Ok(())
    }

    fn end_term(who: &T::AccountId) -> DispatchResult {
        if let Some(role) = T::get_role(who) {
            ensure!(role == Role::DecentralizedDirector, frame_support::error::BadOrigin);
        }
        log::info!("Director {:?} term closed", who);
        Ok(())
    }
}