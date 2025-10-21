//! Ëtrid Peer-Roles — Staking & Role Type Definitions
//!
//! This module contains all shared data structures used by the staking,
//! role assignment, and validator-class pallets across the Ëtrid network.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

/// Minimal stake (in ËTR) required for each participant class.
#[derive(Clone, Copy, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
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
#[derive(Clone, Copy, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Role {
    FlareNode = 0,
    ValidityNode = 1,
    CommonStakePeer = 2,
    CommonPeer = 3,
    DecentralizedDirector = 4,
    CommunityDeveloper = 5,
}

impl Role {
    /// Convert from u8 for use in call parameters (avoids DecodeWithMemTracking issues)
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::FlareNode),
            1 => Some(Self::ValidityNode),
            2 => Some(Self::CommonStakePeer),
            3 => Some(Self::CommonPeer),
            4 => Some(Self::DecentralizedDirector),
            5 => Some(Self::CommunityDeveloper),
            _ => None,
        }
    }
}

/// On-chain record describing an account's active role and stake status.
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(AccountId, Balance))]
pub struct RoleRecord<AccountId: MaxEncodedLen, Balance: MaxEncodedLen> {
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
    ///
    /// NOTE: Default implementation always returns true. Runtime implementations should
    /// override this with proper balance comparison logic.
    fn meets_requirement(_account: &AccountId, _balance: Balance, _role: &Role) -> bool {
        // Default implementation - override in pallet with proper balance comparison
        true
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