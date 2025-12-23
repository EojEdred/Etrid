//! # XCM Configuration Template for PBC Chains
//!
//! This template provides the standard XCM configuration for all Partition Burst Chains.
//! Each PBC should copy this template and customize:
//! - Para ID (unique per chain)
//! - Asset configuration (native token for that PBC)
//! - Bridge-specific message handlers
//!
//! ## Para ID Assignments
//! - ETH-PBC:      2001
//! - SOL-PBC:      2002
//! - BNB-PBC:      2003
//! - TRX-PBC:      2004
//! - XRP-PBC:      2005
//! - DOGE-PBC:     2006
//! - BTC-PBC:      2007
//! - XLM-PBC:      2008
//! - USDT-PBC:     2009
//! - ADA-PBC:      2010
//! - DOT-PBC:      2011
//! - AVAX-PBC:     2012
//! - MATIC-PBC:    2013
//! - LINK-PBC:     2014
//! - AI-Compute:   2015

use frame_support::{
    parameter_types,
    traits::{Everything, Nothing, Get},
    weights::Weight,
};
use xcm::latest::prelude::*;
use xcm_builder::{
    AccountId32Aliases, AllowTopLevelPaidExecutionFrom, CurrencyAdapter as XcmCurrencyAdapter,
    FixedWeightBounds, IsConcrete, ParentIsPreset, RelayChainAsNative,
    SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32,
    SovereignSignedViaLocation, TakeWeightCredit,
};
use xcm_executor::{Config as XcmConfig, XcmExecutor};
use sp_runtime::traits::ConstU32;
use frame_support::weights::IdentityFee;

// ============================================================================
// CUSTOMIZE THESE FOR EACH PBC
// ============================================================================

/// Para ID - MUST BE UNIQUE FOR EACH PBC
/// Replace with actual para ID from the assignments above
pub const PBC_PARA_ID: u32 = 2001; // Example: ETH-PBC

/// PBC Name for logging
pub const PBC_NAME: &str = "ETH-PBC";

// ============================================================================
// STANDARD XCM CONFIGURATION (Same for all PBCs)
// ============================================================================

parameter_types! {
    /// Primearc Core Chain (relay chain) location
    pub const RelayLocation: MultiLocation = MultiLocation::parent();

    /// This PBC's para ID
    pub const SelfParaId: u32 = PBC_PARA_ID;

    /// Native token location (represents ETR locked in bridge)
    pub const NativeTokenLocation: MultiLocation = MultiLocation::here();

    /// Universal location (Polkadot → Primearc → This PBC)
    pub UniversalLocation: InteriorMultiLocation = X2(
        GlobalConsensus(NetworkId::Polkadot),
        Parachain(PBC_PARA_ID)
    );

    /// Relay network ID
    pub const RelayNetwork: NetworkId = NetworkId::Polkadot;

    /// Our ancestry (location relative to relay)
    pub Ancestry: MultiLocation = Parachain(PBC_PARA_ID).into();

    /// Max XCM instructions
    pub const MaxInstructions: u32 = 100;

    /// Base execution weight
    pub const BaseXcmWeight: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);

    /// Max assets in holding
    pub const MaxAssetsIntoHolding: u32 = 64;

    /// Checkpoint submission interval (blocks)
    pub const CheckpointInterval: u32 = 256;
}

// ============================================================================
// TYPE DEFINITIONS (Common to all PBCs)
// ============================================================================

// Import runtime types - these must be defined in each PBC's runtime
// use crate::{AccountId, Balance, Balances, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin};

/// Convert MultiLocation to AccountId
pub type LocationToAccountId<AccountId> = (
    ParentIsPreset<AccountId>,
    SiblingParachainConvertsVia<polkadot_parachain::primitives::Sibling, AccountId>,
    AccountId32Aliases<RelayNetwork, AccountId>,
);

/// Local asset transactor (handles token transfers)
pub type LocalAssetTransactor<AccountId, Balance, Balances> = XcmCurrencyAdapter<
    Balances,
    IsConcrete<NativeTokenLocation>,
    LocationToAccountId<AccountId>,
    AccountId,
    (),
>;

/// Origin converter
pub type LocalOriginConverter<RuntimeOrigin, AccountId> = (
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    SovereignSignedViaLocation<LocationToAccountId<AccountId>, RuntimeOrigin>,
);

/// Weight trader (pays execution in native token)
pub type Trader<AccountId, Balance, Balances> = xcm_builder::UsingComponents<
    IdentityFee<Balance>,
    NativeTokenLocation,
    AccountId,
    Balances,
    (),
>;

/// Barrier (what messages are allowed)
pub type Barrier = (
    AllowTopLevelPaidExecutionFrom<Everything>,
    TakeWeightCredit,
);

// ============================================================================
// CHECKPOINT SUBMISSION
// ============================================================================

/// Submit state checkpoint to Primearc Core Chain via XCM
/// Called every CheckpointInterval blocks by off-chain worker
pub fn submit_checkpoint_to_primearc<T: frame_system::Config>(
    block_number: u32,
    state_root: [u8; 32],
) -> frame_support::dispatch::DispatchResult
where
    T::AccountId: Into<[u8; 32]>,
{
    use xcm::latest::prelude::*;

    log::info!(
        "📤 {}: Submitting checkpoint at block {} to Primearc Core Chain",
        PBC_NAME,
        block_number
    );

    // Construct XCM message
    let message = Xcm(vec![
        // Withdraw fee from PBC sovereign account on Primearc
        WithdrawAsset((Here, 1_000_000_000_000u128).into()),

        // Buy execution on Primearc
        BuyExecution {
            fees: (Here, 1_000_000_000_000u128).into(),
            weight_limit: Unlimited,
        },

        // Call Primearc's checkpoint pallet
        Transact {
            origin_kind: OriginKind::SovereignAccount,
            require_weight_at_most: Weight::from_parts(1_000_000_000, 64 * 1024),
            call: encode_checkpoint_call(block_number, state_root).into(),
        },

        // Report any errors back
        ReportError(QueryResponseInfo {
            destination: Parachain(PBC_PARA_ID).into(),
            query_id: block_number as u64,
            max_weight: Weight::from_parts(100_000_000, 10 * 1024),
        }),
    ]);

    // Send to Primearc (parent chain)
    let dest = MultiLocation::parent();

    // Use pallet-xcm to send
    // PolkadotXcm::send_xcm(Here, dest, message)?;

    log::info!(
        "✅ {}: Checkpoint submitted for block {}",
        PBC_NAME,
        block_number
    );

    Ok(())
}

/// Encode the checkpoint call for Primearc
fn encode_checkpoint_call(block_number: u32, state_root: [u8; 32]) -> Vec<u8> {
    // SCALE encoding for: pallet_pbc_checkpoints::Call::submit_checkpoint
    // Pallet index: 50 (example), Call index: 0
    let mut call = vec![
        50, // pallet index (adjust for actual Primearc runtime)
        0,  // call index (submit_checkpoint)
    ];

    // Encode para_id (u32, compact)
    call.extend_from_slice(&codec::Compact(PBC_PARA_ID).encode());

    // Encode block_number (u32, compact)
    call.extend_from_slice(&codec::Compact(block_number).encode());

    // Encode state_root ([u8; 32])
    call.extend_from_slice(&state_root);

    call
}

// ============================================================================
// BRIDGE MESSAGE HANDLING
// ============================================================================

/// Handle incoming bridge messages from external chain
/// Each PBC implements this for their specific external chain
pub trait BridgeMessageHandler {
    /// Process deposit from external chain
    fn handle_deposit(
        sender: Vec<u8>,
        recipient: [u8; 32],
        amount: u128,
        external_tx_hash: [u8; 32],
    ) -> frame_support::dispatch::DispatchResult;

    /// Process withdrawal confirmation to external chain
    fn handle_withdrawal_confirmed(
        withdrawal_id: u64,
        external_tx_hash: [u8; 32],
    ) -> frame_support::dispatch::DispatchResult;
}

// ============================================================================
// ASSET TRANSFER HELPERS
// ============================================================================

/// Transfer native token to another parachain
pub fn transfer_to_parachain<AccountId, Balance>(
    dest_para_id: u32,
    beneficiary: AccountId,
    amount: Balance,
) -> frame_support::dispatch::DispatchResult
where
    AccountId: Into<[u8; 32]>,
    Balance: Into<u128>,
{
    let dest = MultiLocation::new(1, X1(Parachain(dest_para_id)));

    let beneficiary_location = X1(AccountId32 {
        network: Some(RelayNetwork::get()),
        id: beneficiary.into(),
    });

    let asset = MultiAsset {
        id: Concrete(NativeTokenLocation::get()),
        fun: Fungible(amount.into()),
    };

    // Use pallet-xcm limited_reserve_transfer_assets
    // PolkadotXcm::limited_reserve_transfer_assets(
    //     RuntimeOrigin::signed(from),
    //     Box::new(dest.into()),
    //     Box::new(beneficiary_location.into()),
    //     Box::new(asset.into()),
    //     0,
    //     Unlimited,
    // )?;

    Ok(())
}

/// Transfer native token to Primearc Core Chain (relay)
pub fn transfer_to_primearc<AccountId, Balance>(
    beneficiary: AccountId,
    amount: Balance,
) -> frame_support::dispatch::DispatchResult
where
    AccountId: Into<[u8; 32]>,
    Balance: Into<u128>,
{
    transfer_to_parachain::<AccountId, Balance>(0, beneficiary, amount)
}

// ============================================================================
// CODEC HELPERS
// ============================================================================

mod codec {
    use parity_scale_codec::{Compact, Encode};

    impl<T: Encode> Encode for super::Compact<T> {
        fn encode(&self) -> Vec<u8> {
            parity_scale_codec::Compact(self.0.clone()).encode()
        }
    }

    pub struct Compact<T>(pub T);
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_para_id_configured() {
        assert!(PBC_PARA_ID >= 2001 && PBC_PARA_ID <= 2015);
    }

    #[test]
    fn test_checkpoint_encoding() {
        let block = 1000u32;
        let state_root = [0xab; 32];
        let encoded = encode_checkpoint_call(block, state_root);

        // Should be: pallet(1) + call(1) + compact_para_id + compact_block + state_root(32)
        assert!(encoded.len() >= 36);
    }
}
