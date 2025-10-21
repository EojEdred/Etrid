//! Common PBC Runtime Code
//!
//! This crate contains all shared code across Partition Burst Chains (PBCs).
//! Individual PBCs only need to specify their bridge-specific configuration.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use pbc_common::*;
//!
//! // Import bridge pallet
//! pub use pallet_bitcoin_bridge;
//!
//! // Configure bridge
//! impl pallet_bitcoin_bridge::Config for Runtime {
//!     // Bridge-specific config
//! }
//!
//! // Use common macros
//! impl_common_pallet_configs!(Runtime);
//! construct_pbc_runtime!(Runtime, BitcoinBridge: pallet_bitcoin_bridge);
//! impl_pbc_runtime_apis!(Runtime);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

// Re-export all common dependencies that PBCs will need
pub use codec::{Decode, Encode, MaxEncodedLen};
pub use scale_info::TypeInfo;

// Substrate Core
pub use sp_api::impl_runtime_apis;
pub use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
pub use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{
        AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, One, Verify,
    },
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};
pub use sp_std::prelude::*;
#[cfg(feature = "std")]
pub use sp_version::NativeVersion;
pub use sp_version::RuntimeVersion;

// FRAME
pub use frame_executive;
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{
        ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness,
        StorageInfo,
    },
    weights::{
        constants::{
            BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
        },
        IdentityFee, Weight,
    },
    StorageValue,
};
pub use frame_system::{
    limits::{BlockLength, BlockWeights},
    EnsureRoot,
};

// Pallets
pub use pallet_balances;
pub use pallet_grandpa;
pub use pallet_insecure_randomness_collective_flip;
pub use pallet_sudo;
pub use pallet_timestamp;
pub use pallet_transaction_payment::{self, ConstFeeMultiplier, FeeDetails, Multiplier, RuntimeDispatchInfo};

// Ëtrid
pub use etrid_primitives::{AccountId, Balance, BlockNumber, Hash, Moment, Nonce, Signature};
pub use pallet_consensus;
pub use pallet_lightning_channels;

// Module exports
pub mod config;
pub mod opaque;

// Re-export modules
pub use config::*;
// Note: NOT exporting types::* to avoid conflicts with runtime-specific type definitions
// Note: NOT exporting opaque::* to avoid conflicts - each PBC defines its own opaque module
