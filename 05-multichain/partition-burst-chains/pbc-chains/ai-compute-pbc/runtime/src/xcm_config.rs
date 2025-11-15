//! # XCM Configuration for AI-Compute-PBC
//!
//! This module configures cross-chain messaging (XCM) to enable communication
//! between AI-Compute-PBC and FlareChain.
//!
//! ## Message Types
//! - Checkpoint submission (AI-PBC → FlareChain every 256 blocks)
//! - Asset transfers (ËDSC, ËTRD between chains)
//! - Job result verification requests
//! - Governance messages (runtime upgrades, parameter changes)

use frame_support::{
    parameter_types,
    traits::{Everything, Nothing},
};
use xcm::latest::prelude::*;
use xcm_builder::{
    AccountId32Aliases, AllowTopLevelPaidExecutionFrom, CurrencyAdapter as XcmCurrencyAdapter,
    FixedWeightBounds, FungiblesAdapter, IsConcrete, LocationInverter, ParentIsPreset,
    RelayChainAsNative, SiblingParamachainAsNative, SignedAccountId32AsNative,
    SiblingParachainConvertsVia, SignedToAccountId32, SovereignSignedViaLocation,
    TakeWeightCredit,
};
use xcm_executor::{Config as XcmConfig, XcmExecutor};

// Our runtime types
use crate::{AccountId, Balance, Balances, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin};

parameter_types! {
    /// FlareChain (relay chain) location
    pub const RelayLocation: MultiLocation = MultiLocation::parent();

    /// AI-Compute-PBC para ID
    pub const AiComputePbcParaId: u32 = 2015; // 15th PBC

    /// Native token (ËDSC) as MultiAsset
    pub const NativeTokenId: MultiLocation = MultiLocation::here();

    /// Universal location (Polkadot → FlareChain → AI-Compute-PBC)
    pub UniversalLocation: InteriorMultiLocation = X2(
        GlobalConsensus(NetworkId::Polkadot),
        Parachain(AiComputePbcParaId::get())
    );
}

/// How we convert AccountId to MultiLocation
pub type LocationToAccountId = (
    // Relay chain (FlareChain) account
    ParentIsPreset<AccountId>,
    // Sibling parachain accounts
    SiblingParachainConvertsVia<polkadot_parachain::primitives::Sibling, AccountId>,
    // Local AccountId32
    AccountId32Aliases<RelayNetwork, AccountId>,
);

/// How we convert MultiLocation back to local AccountId
pub type LocalAssetTransactor = XcmCurrencyAdapter<
    // Use Balances pallet
    Balances,
    // Match native token
    IsConcrete<NativeTokenId>,
    // Convert location to account
    LocationToAccountId,
    // AccountId type
    AccountId,
    // No teleporter (we use reserve-based transfers)
    (),
>;

/// Origin converter
pub type LocalOriginConverter = (
    // Relay chain (FlareChain) as native origin
    RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
    // Sibling parachains
    SiblingParamachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
    // Signed origins
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    // Sovereign accounts
    SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
);

parameter_types! {
    /// Relay network (FlareChain uses Polkadot's NetworkId)
    pub const RelayNetwork: NetworkId = NetworkId::Polkadot;

    /// Ancestry (our location relative to relay)
    pub Ancestry: MultiLocation = Parachain(AiComputePbcParaId::get()).into();

    /// Max XCM instructions per message
    pub const MaxInstructions: u32 = 100;

    /// Base XCM execution weight
    pub const BaseXcmWeight: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);

    /// Max assets in XCM message
    pub const MaxAssetsIntoHolding: u32 = 64;
}

/// XCM weight trader (pays for execution in ËDSC)
pub type Trader = (
    // Pay with native token (ËDSC)
    xcm_builder::UsingComponents<
        IdentityFee<Balance>,
        NativeTokenId,
        AccountId,
        Balances,
        (),
    >,
);

/// Barrier - what XCM messages are allowed
pub type Barrier = (
    // Allow paid executions from FlareChain
    AllowTopLevelPaidExecutionFrom<Everything>,
    // Allow messages that consume weight credit
    TakeWeightCredit,
);

/// Main XCM configuration
pub struct XcmConfigImpl;

impl XcmConfig for XcmConfigImpl {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = LocalOriginConverter;
    type IsReserve = (); // No reserve assets (we use native ËDSC)
    type IsTeleporter = (); // No teleportation
    type LocationInverter = LocationInverter<Ancestry>;
    type Barrier = Barrier;
    type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
    type Trader = Trader;
    type ResponseHandler = (); // No async responses needed
    type AssetTrap = (); // Don't trap assets
    type AssetClaims = (); // No claims
    type SubscriptionService = (); // No subscriptions
    type AssetLocker = (); // No locking
    type AssetExchanger = (); // No exchanges
    type PalletInstancesInfo = (); // No pallet instances
    type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
    type FeeManager = (); // Simple fee handling
    type MessageExporter = (); // No cross-consensus messaging
    type UniversalAliases = Nothing; // No aliases
    type CallDispatcher = RuntimeCall;
    type SafeCallFilter = Everything;
}

/// XCM router - how to send messages
pub type XcmRouter = (
    // Route to FlareChain (upward messages)
    cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, ()>,
    // Route to sibling parachains (horizontal messages)
    // (Add HRMP channel configuration if needed)
);

// ============================================
// CHECKPOINT SUBMISSION TO FLARECHAIN
// ============================================

/// Helper function to submit state checkpoint to FlareChain
pub fn submit_checkpoint_to_flarechain(block_number: u32, state_root: [u8; 32]) -> DispatchResult {
    use xcm::latest::prelude::*;

    // Construct XCM message
    let message = Xcm(vec![
        // Step 1: Withdraw fee for execution
        WithdrawAsset((Here, 1_000_000_000_000u128).into()), // 1 ËDSC

        // Step 2: Buy execution on FlareChain
        BuyExecution {
            fees: (Here, 1_000_000_000_000u128).into(),
            weight_limit: Unlimited,
        },

        // Step 3: Transact - call FlareChain's checkpoint pallet
        Transact {
            origin_kind: OriginKind::SovereignAccount,
            require_weight_at_most: Weight::from_parts(1_000_000_000, 64 * 1024),
            call: checkpoint_call_data(block_number, state_root).into(),
        },
    ]);

    // Send to FlareChain (parent chain)
    let dest = MultiLocation::parent();

    PolkadotXcm::send_xcm(Here, dest, message)
        .map_err(|_| Error::<Runtime>::XcmSendFailed)?;

    Ok(())
}

/// Encode checkpoint submission call for FlareChain
fn checkpoint_call_data(block_number: u32, state_root: [u8; 32]) -> Vec<u8> {
    // This would be the actual FlareChain runtime call
    // Format: pallet_index (u8) + call_index (u8) + params

    let mut call = vec![
        50, // FlareChain checkpoint pallet index (example)
        0,  // submit_checkpoint call index
    ];

    // Encode AI-Compute-PBC para ID
    call.extend_from_slice(&AiComputePbcParaId::get().to_le_bytes());

    // Encode block number
    call.extend_from_slice(&block_number.to_le_bytes());

    // Encode state root
    call.extend_from_slice(&state_root);

    call
}

// ============================================
// ASSET TRANSFERS (ËDSC, ËTRD)
// ============================================

/// Transfer ËDSC from AI-Compute-PBC to another parachain
pub fn transfer_edsc_to_parachain(
    dest_para_id: u32,
    beneficiary: AccountId,
    amount: Balance,
) -> DispatchResult {
    let dest = MultiLocation::new(1, X1(Parachain(dest_para_id)));

    let beneficiary_location = X1(AccountId32 {
        network: Some(RelayNetwork::get()),
        id: beneficiary.into(),
    });

    let asset = MultiAsset {
        id: Concrete(NativeTokenId::get()),
        fun: Fungible(amount),
    };

    PolkadotXcm::limited_reserve_transfer_assets(
        RuntimeOrigin::signed(beneficiary),
        Box::new(dest.into()),
        Box::new(beneficiary_location.into()),
        Box::new(asset.into()),
        0,
        Unlimited,
    )?;

    Ok(())
}

/// Receive ËDSC from FlareChain or sibling parachain
/// (Automatically handled by XcmExecutor when message arrives)

// ============================================
// CUMULUS INTEGRATION (PARACHAIN SYSTEM)
// ============================================

use cumulus_pallet_parachain_system as parachain_system;
use cumulus_pallet_xcm as cumulus_xcm;
use pallet_xcm as polkadot_xcm;

impl parachain_system::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnSystemEvent = ();
    type SelfParaId = AiComputePbcParaId;
    type OutboundXcmpMessageSource = (); // No XCMP yet
    type DmpMessageHandler = cumulus_primitives_utility::DmpQueue;
    type ReservedDmpWeight = (); // TODO: Set reserved weight
    type XcmpMessageHandler = (); // No XCMP yet
    type ReservedXcmpWeight = (); // TODO: Set reserved weight
    type CheckAssociatedRelayNumber = cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
}

impl cumulus_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfigImpl>;
}

impl polkadot_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginConverter>;
    type XcmRouter = XcmRouter;
    type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginConverter>;
    type XcmExecuteFilter = Everything;
    type XcmExecutor = XcmExecutor<XcmConfigImpl>;
    type XcmTeleportFilter = Nothing; // No teleportation
    type XcmReserveTransferFilter = Everything; // Allow reserve transfers
    type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
    type LocationInverter = LocationInverter<Ancestry>;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    type Currency = Balances;
    type CurrencyMatcher = ();
    type TrustedLockers = ();
    type SovereignAccountOf = LocationToAccountId;
    type MaxLockers = ConstU32<8>;
    type WeightInfo = pallet_xcm::TestWeightInfo;
    type AdminOrigin = frame_system::EnsureRoot<AccountId>;
    type MaxRemoteLockConsumers = ConstU32<0>;
    type RemoteLockConsumerIdentifier = ();
}

// ============================================
// EXAMPLE USAGE IN OFF-CHAIN WORKER
// ============================================

/// Off-chain worker submits checkpoint every 256 blocks
pub fn offchain_worker_checkpoint_submission(block_number: u32) {
    if block_number % 256 == 0 {
        // Get state root from block header
        let state_root = sp_io::storage::root(sp_storage::StateVersion::V1);

        // Submit to FlareChain via XCM
        if let Err(e) = submit_checkpoint_to_flarechain(block_number, state_root) {
            log::error!("Failed to submit checkpoint at block {}: {:?}", block_number, e);
        } else {
            log::info!("✅ Checkpoint submitted to FlareChain at block {}", block_number);
        }
    }
}

// Missing imports
use frame_support::dispatch::DispatchResult;
use sp_runtime::traits::ConstU32;
use crate::PolkadotXcm;
use crate::ParachainSystem;
use sp_runtime::DispatchError;
use frame_support::weights::{Weight, IdentityFee};

#[derive(Debug)]
pub enum Error<T> {
    XcmSendFailed,
}

// Relay chain origin type
pub struct RelayChainOrigin;
impl From<RelayChainOrigin> for RuntimeOrigin {
    fn from(_: RelayChainOrigin) -> Self {
        RuntimeOrigin::root()
    }
}
