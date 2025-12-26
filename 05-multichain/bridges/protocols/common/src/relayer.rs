//! Relayer authorization types and traits
//!
//! Defines common relayer roles and authorization hooks used by bridge pallets.

use parity_scale_codec::{Decode, Encode, MaxEncodedLen, DecodeWithMemTracking};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Relayer roles with distinct permissions
#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub enum RelayerRole {
    /// Can process burns (highest privilege)
    Oracle,
    /// Can confirm deposits
    RelayNode,
    /// Multi-sig approvals
    Custodian,
}

impl DecodeWithMemTracking for RelayerRole {}

/// Operations that require relayer authorization
#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub enum RelayerOperation {
    /// Confirm deposits or other low-risk relay actions
    ConfirmDeposit,
    /// Process burns and unlocks
    ProcessBurn,
}

impl DecodeWithMemTracking for RelayerOperation {}

/// Authorization interface for bridge relayers
pub trait RelayerAuthorization<AccountId> {
    /// Returns true if relayer is registered and active
    fn is_authorized_relayer(relayer: &AccountId) -> bool;
    /// Returns true if relayer can confirm deposits
    fn can_confirm_deposit(relayer: &AccountId) -> bool;
    /// Returns true if relayer can process burns
    fn can_process_burn(relayer: &AccountId) -> bool;
}
