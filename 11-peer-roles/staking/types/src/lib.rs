//! Ëtrid Peer-Roles — Staking & Role Type Definitions
//!
//! This module contains all shared data structures used by the staking,
//! role assignment, and validator-class pallets across the Ëtrid network.

#![cfg_attr(not(feature = "std"), no_std)]

use scale::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

/// Minimal stake (in ËTR) required for each participant class.
#[derive(Clone, Copy, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum StakeRequirement {
    /// ≥ 128 ËTR — qualifies as a Decentralized Director.
    Director = 128,
    /// ≥ 64 ËTR — qualifies as a Validity Node.
    Validity = 64,
    /// ≥ 1 ËTR — qualifies as a Common Stake Peer.
    CommonStake = 1,
    /// 0 ËTR — Common Peer or Community Developer.
    None = 0,
}

/// Role identifiers recognised by the Ivory protocol.
#[derive(Clone, Copy, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum Role {
    FlareNode,
    ValidityNode,
    CommonStakePeer,
    CommonPeer,
    DecentralizedDirector,
    CommunityDeveloper,
}

/// On-chain record describing an account’s active role and stake status.
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct RoleRecord<AccountId, Balance> {
    /// Account owning this role.
    pub account: AccountId,
    /// Assigned network role.
    pub role: Role,
    /// Total amount staked for this role.
    pub stake: Balance,
    /// Unix timestamp when the role was last updated.
    pub last_update: u64,
    /// Whether this role is currently active (not chilled / revoked).
    pub active: bool,
}

/// Enumeration of staking or role-related events.
/// Pallets in 11-peer-roles will emit these.
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum RoleEvent<AccountId, Balance> {
    RoleAssigned(AccountId, Role),
    RoleRevoked(AccountId, Role),
    StakeIncreased(AccountId, Balance),
    StakeDecreased(AccountId, Balance),
    StakeSlashed(AccountId, Balance),
}

/// Runtime trait for querying roles and stake requirements.
pub trait RoleInterface<AccountId, Balance> {
    /// Return the role for a given account (if any).
    fn get_role(account: &AccountId) -> Option<Role>;

    /// Return the current stake balance for an account.
    fn get_stake(account: &AccountId) -> Option<Balance>;

    /// Return the minimal stake required for a given role.
    fn stake_requirement(role: &Role) -> StakeRequirement;

    /// Check whether an account satisfies its stake requirement.
    fn meets_requirement(account: &AccountId, balance: Balance, role: &Role) -> bool {
        let needed = match role {
            Role::DecentralizedDirector => StakeRequirement::Director as u128,
            Role::ValidityNode => StakeRequirement::Validity as u128,
            Role::CommonStakePeer => StakeRequirement::CommonStake as u128,
            _ => StakeRequirement::None as u128,
        };
        balance >= needed
    }
}

/// Utility alias for convenient use in runtime code.
pub type RoleList<AccountId, Balance> = Vec<RoleRecord<AccountId, Balance>>;

/// Default constants for genesis configuration.
pub mod defaults {
    pub const DIRECTOR_STAKE: u128 = 128;
    pub const VALIDITY_STAKE: u128 = 64;
    pub const COMMON_STAKE: u128 = 1;
}