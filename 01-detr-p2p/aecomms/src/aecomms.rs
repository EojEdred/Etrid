#![cfg_attr(not(feature = "std"), no_std)]

//! # Ëtrid Primitives
//!
//! Core types and primitives for the Ëtrid Multichain Network.
//!
//! This crate provides:
//! - Core types (AccountId, Balance, Hash, etc.)
//! - Chain identification types
//! - Cross-chain messaging primitives
//! - VMw (VM Watts) computation metering
//! - FlareChain block structures
//! - PBC block structures with Ants support

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, OpaqueExtrinsic as UncheckedExtrinsic,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// MODULE DECLARATIONS
// ═══════════════════════════════════════════════════════════════════════════

/// VM Watts (VMw) - Virtual Machine computation units
pub mod vmw;

/// FlareChain block structures
pub mod flare_chain_blocks;

/// PBC (Partitioned Burst Chain) block structures
pub mod pbc_blocks;

// ═══════════════════════════════════════════════════════════════════════════
// RE-EXPORTS
// ═══════════════════════════════════════════════════════════════════════════

// Re-export VMw types
pub use vmw::{BlockVMwLimit, VMw, VMwMetering, VMwPrice};

// Re-export FlareChain types
pub use flare_chain_blocks::{
    AttestationRecord, FlareChainBlock, FlareChainBody, FlareChainHeader, PbcStateSubmission,
    PenalizedNode, StakeDeposit, FLARE_CHAIN_ID,
};

// Re-export PBC types
pub use pbc_blocks::{
    AntBlock, BlockValidationResult, PbcBlock, PbcBlockBody, PbcBlockHeader, TransactionRecord,
    MAX_ANTS_DEPTH, MAX_ANTS_PER_BLOCK,
};

// ═══════════════════════════════════════════════════════════════════════════
// CORE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = H256;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// Opaque block header type.
pub type Header = sp_runtime::generic::Header<BlockNumber, BlakeTwo256>;

/// Opaque block type.
pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;

/// Opaque block identifier type.
pub type BlockId = sp_runtime::generic::BlockId<Block>;

// ═══════════════════════════════════════════════════════════════════════════
// CHAIN IDENTIFICATION
// ═══════════════════════════════════════════════════════════════════════════

/// Chain identifier for Ëtrid multichain
#[derive(
    Clone,
    Copy,
    Encode,
    Decode,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    codec::MaxEncodedLen,
    TypeInfo,
    Debug,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ChainId {
    /// FlareChain (root chain)
    Flare,
    /// Partitioned Burst Chain with ID
    Pbc(u8),
}

impl Default for ChainId {
    fn default() -> Self {
        Self::Flare
    }
}

impl ChainId {
    /// Check if this is FlareChain
    pub fn is_flare(&self) -> bool {
        matches!(self, ChainId::Flare)
    }

    /// Check if this is a PBC
    pub fn is_pbc(&self) -> bool {
        matches!(self, ChainId::Pbc(_))
    }

    /// Get PBC ID if this is a PBC
    pub fn pbc_id(&self) -> Option<u8> {
        match self {
            ChainId::Pbc(id) => Some(*id),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TOKEN TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Token types in Ëtrid ecosystem
#[derive(
    Clone,
    Copy,
    Encode,
    Decode,
    Eq,
    PartialEq,
    codec::MaxEncodedLen,
    TypeInfo,
    Debug,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum TokenType {
    /// Ëtrid native token (ËTR)
    Etr,
    /// Ëtrid Dollar stablecoin (ËTD)
    Etd,
}

impl Default for TokenType {
    fn default() -> Self {
        Self::Etr
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CROSS-CHAIN MESSAGING
// ═══════════════════════════════════════════════════════════════════════════

/// Cross-chain message between PBCs and FlareChain
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CrossChainMessage {
    /// Source chain
    pub source_chain: ChainId,
    /// Destination chain
    pub destination_chain: ChainId,
    /// Nonce for ordering
    pub nonce: u64,
    /// Payload data
    pub payload: sp_std::vec::Vec<u8>,
}

/// PBC state root for FlareChain aggregation
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PbcStateRoot {
    /// PBC chain ID
    pub chain_id: ChainId,
    /// Block number
    pub block_number: BlockNumber,
    /// State root hash
    pub state_root: Hash,
    /// Timestamp
    pub timestamp: Moment,
}

// ═══════════════════════════════════════════════════════════════════════════
// ACCOUNT TYPES (From Ivory Papers)
// ═══════════════════════════════════════════════════════════════════════════

/// Account types as defined in Ëtrid Ivory Papers
#[derive(
    Clone,
    Copy,
    Encode,
    Decode,
    Eq,
    PartialEq,
    codec::MaxEncodedLen,
    TypeInfo,
    Debug,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AccountType {
    /// External Blockchain Account (non-Ëtrid key pairs)
    Ebca,
    /// Root Chain Account (FlareChain account)
    Rca,
    /// Root Chain Withdrawal Account (withdrawal-only)
    Rcwa,
    /// Side Chain Account (PBC account)
    Sca,
    /// Smart Side Chain Account (smart contract on PBC)
    Ssca,
}

impl Default for AccountType {
    fn default() -> Self {
        Self::Rca
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_id_works() {
        let flare = ChainId::Flare;
        assert!(flare.is_flare());
        assert!(!flare.is_pbc());

        let pbc = ChainId::Pbc(1);
        assert!(!pbc.is_flare());
        assert!(pbc.is_pbc());
        assert_eq!(pbc.pbc_id(), Some(1));
    }

    #[test]
    fn token_type_default_works() {
        assert_eq!(TokenType::default(), TokenType::Etr);
    }

    #[test]
    fn account_type_default_works() {
        assert_eq!(AccountType::default(), AccountType::Rca);
    }
}
