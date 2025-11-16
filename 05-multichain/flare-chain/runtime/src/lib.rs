#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "512"]

// Make the WASM binary available
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

// EVM moved to ETH-PBC - FlareChain is pure coordination layer

// ═══════════════════════════════════════════════════════════════════════════════
// ASF CONSENSUS MODULES (Phase 1-2: Runtime Integration)
// ═══════════════════════════════════════════════════════════════════════════════
mod asf_apis;
mod asf_config;

use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use pallet_session::disabling::UpToLimitDisablingStrategy;
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{
        AccountIdLookup, AccountIdConversion, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, One, Verify, OpaqueKeys,
    },
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, DispatchResult, FixedU128, MultiSignature, Perbill,
};
use sp_arithmetic::Permill;
use sp_std::prelude::*;
use frame_system::EnsureRoot;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// Codec and scale_info for RuntimeHoldReason
use codec::Encode;
use sp_runtime::RuntimeDebug;

// Substrate and Polkadot dependencies
use frame_support::{
    construct_runtime, derive_impl,
    dispatch::DispatchClass,
    parameter_types,
    traits::{ConstU128, ConstU16, ConstU32, ConstU64, ConstU8},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, WEIGHT_REF_TIME_PER_SECOND},
        IdentityFee,
        Weight,
    },
    PalletId,
};
use frame_system::limits::{BlockLength, BlockWeights};
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ConstFeeMultiplier, FungibleAdapter, Multiplier};

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            // Empty - ASF consensus manages validator rotation internally
            // Custom EmptySessionHandler below satisfies trait bounds
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// EMPTY SESSION HANDLER FOR ASF CONSENSUS
// ═══════════════════════════════════════════════════════════════════════════════
//
// ASF (Ascending Scale of Finality) manages validator rotation internally via
// the ValidatorCommittee pallet. We don't need session keys for consensus, but
// pallet_session requires a SessionHandler implementation.
//
// This empty handler satisfies the trait bounds without requiring any actual
// session key management.

/// Empty session handler for ASF consensus
pub struct EmptySessionHandler;

impl pallet_session::SessionHandler<AccountId> for EmptySessionHandler {
    const KEY_TYPE_IDS: &'static [KeyTypeId] = &[];

    fn on_genesis_session<Ks: OpaqueKeys>(_validators: &[(AccountId, Ks)]) {
        // No-op: ASF handles validator initialization via ValidatorCommittee
    }

    fn on_new_session<Ks: OpaqueKeys>(
        _changed: bool,
        _validators: &[(AccountId, Ks)],
        _queued_validators: &[(AccountId, Ks)],
    ) {
        // No-op: ASF handles validator rotation via ValidatorCommittee
    }

    fn on_disabled(_validator_index: u32) {
        // No-op: ASF handles validator disabling via ValidatorCommittee
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// GENESIS MESSAGE - EMBEDDED IN RUNTIME WASM (Bitcoin-style Easter Egg)
// ═══════════════════════════════════════════════════════════════════════════════
/// "The Ëtrid Network - First pure ASF blockchain - Built by Eoj - November 2025"
///
/// Like Bitcoin's genesis block message, this is permanently embedded in the
/// runtime WASM code stored in System.Code at block 0.
///
/// Consensus Evolution: GRANDPA → ASF (Ascending Scale of Finality)
/// HotStuff 4-phase Byzantine consensus with 5-level finality scale
///
/// This message will remain on-chain forever as part of the genesis runtime blob.
// ═══════════════════════════════════════════════════════════════════════════════

// To learn more about runtime versioning, see:
// https://docs.substrate.io/main-docs/build/upgrade#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("etrid"),
    impl_name: create_runtime_str!("etrid"),
    authoring_version: 1,
    spec_version: 108,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// RUNTIME UPGRADE MIGRATIONS
// ═══════════════════════════════════════════════════════════════════════════════
//
// Migration History:
// - v106: Fixed GRANDPA committee formation (10 validators)
// - v107: Transitioned to ASF primary finality (GRANDPA fallback)
// - v108: Removed GRANDPA entirely - Pure ASF consensus
//
// Current Migration: v108 (Pure ASF Consensus)
// ═══════════════════════════════════════════════════════════════════════════════

pub mod migrations;

/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(10);
/// We allow for 2 seconds of compute with a 6 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
    WEIGHT_REF_TIME_PER_SECOND * 2,
    u64::MAX,
);
/// Maximum length of block.
const MAXIMUM_BLOCK_LENGTH: u32 = 5 * 1024 * 1024;

pub struct Version;
impl frame_support::traits::Get<RuntimeVersion> for Version {
    fn get() -> RuntimeVersion {
        VERSION
    }
}

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub RuntimeBlockLength: BlockLength =
        BlockLength::max_with_normal_ratio(MAXIMUM_BLOCK_LENGTH, NORMAL_DISPATCH_RATIO);
    pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();
}

/// The default types are being injected by [`derive_impl`](`frame_support::derive_impl`) from
/// [`SoloChainDefaultConfig`](`struct@frame_system::config_preludes::SolochainDefaultConfig`),
/// but overridden as needed.
#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = RuntimeBlockWeights;
    type BlockLength = RuntimeBlockLength;
    type AccountId = AccountId;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Block = Block;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = Version;
    type AccountData = pallet_balances::AccountData<Balance>;
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>; // Can be changed to custom prefix
    type MaxConsumers = ConstU32<16>;
}

// RuntimeHoldReason disabled for phase 1 - no holds needed yet
// Phase 2 can enable holds when needed for staking/governance

// GRANDPA removed in v108 - Pure ASF consensus

parameter_types! {
    pub const MinimumPeriod: u64 = 3000; // 3 seconds (half of block time)
}

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

// pallet_session configuration for future validator management
// Following Polkadot's phased approach: infrastructure now, deposits later
parameter_types! {
    pub const Period: u32 = 600; // 600 blocks = 1 hour at 6s blocks
    pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = AccountId;
    // Use ValidatorCommittee's ValidatorIdOf for ËTRID ASF consensus integration
    type ValidatorIdOf = pallet_validator_committee::ValidatorIdOf<Self>;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    // Use ValidatorCommittee as SessionManager for ASF consensus coordination
    // This enables:
    // - Validator set updates every 600 blocks (session period)
    // - ASF validator committee synchronization with active validators
    type SessionManager = ValidatorCommittee;
    // Use EmptySessionHandler since ASF manages validator keys internally
    type SessionHandler = EmptySessionHandler;
    type Keys = opaque::SessionKeys;
    type WeightInfo = ();
    type DisablingStrategy = UpToLimitDisablingStrategy;
    // pallet_session requires Currency and KeyDeposit for validator key management
    // Using Balances pallet satisfies trait bounds without needing RuntimeHoldReason
    type Currency = Balances;
    type KeyDeposit = ConstU128<0>; // No deposit required for session keys
}

/// Existential deposit - minimum balance to keep an account alive
pub const EXISTENTIAL_DEPOSIT: u128 = 500;

impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

parameter_types! {
    pub FeeMultiplier: Multiplier = Multiplier::one();
}

/// Handler for transaction fees - splits 50/50 between treasury and burn
pub struct DealWithFees;
impl frame_support::traits::OnUnbalanced<frame_support::traits::fungible::Credit<AccountId, Balances>> for DealWithFees {
    fn on_unbalanced(amount: frame_support::traits::fungible::Credit<AccountId, Balances>) {
        use frame_support::traits::fungible::Balanced;
        

        // Split the credit into two parts: 50% to treasury, 50% burn
        // We need to resolve the credit to get its value, then handle it
        let treasury_account = EtridTreasury::account_id();

        // Deposit the credit to the treasury account
        // The credit represents fees that should be allocated
        let _ = Balances::resolve(&treasury_account, amount);

        // Note: We're currently depositing all fees to treasury
        // The treasury pallet can then manage fee distribution/burning
    }
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = FungibleAdapter<Balances, DealWithFees>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
    type LengthToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
    type WeightInfo = ();
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = ();
}

use frame_support::traits::WithdrawReasons;

parameter_types! {
    pub const MinVestedTransfer: Balance = 100_000_000_000_000; // 0.0001 ETR (100 million base units)
    pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
        WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}

impl pallet_vesting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BlockNumberToBalance = sp_runtime::traits::ConvertInto;
    type MinVestedTransfer = MinVestedTransfer;
    type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
    type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
    type BlockNumberProvider = System;
    const MAX_VESTING_SCHEDULES: u32 = 28; // Allow multiple vesting schedules per account
}

parameter_types! {
    pub const DepositBase: Balance = 1_000_000_000_000; // 0.001 ETR base deposit
    pub const DepositFactor: Balance = 500_000_000_000; // 0.0005 ETR per signatory
    pub const MaxSignatories: u32 = 10; // Max 10 signatories per multisig
}

impl pallet_multisig::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type Currency = Balances;
    type DepositBase = DepositBase;
    type DepositFactor = DepositFactor;
    type MaxSignatories = MaxSignatories;
    type WeightInfo = pallet_multisig::weights::SubstrateWeight<Runtime>;
    type BlockNumberProvider = System;
}

// Treasury PalletId is defined later with the custom treasury configuration

impl pallet_insecure_randomness_collective_flip::Config for Runtime {}

// ========================================
// ËTRID CUSTOM PALLETS CONFIGURATION
// ========================================

/// Configure the pallet-accounts (ETR/ETD token system)
impl pallet_accounts::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u64;
    type GovernanceOrigin = frame_system::EnsureRoot<AccountId>;
    type WeightInfo = ();
}

parameter_types! {
    pub TreasuryAccountForStaking: AccountId = EtridTreasury::account_id();
}

/// Configure the pallet-etrid-staking (peer roles staking system)
impl pallet_etrid_staking::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type UnbondPeriod = ConstU32<28800>; // ~2 days at 6 second blocks
    type MaxUnbondingEntries = ConstU32<32>; // Max unbonding entries per account
    type TreasuryAccount = TreasuryAccountForStaking;
    type ValidatorRewards = Runtime;
}

/// Configure the pallet-etwasm-vm (smart contract execution)
impl pallet_etwasm_vm::Config for Runtime {
    type MaxCodeSize = ConstU32<1024>;
    type DefaultGasLimit = ConstU64<10_000_000>; // 10 million gas default
    type MaxGasLimit = ConstU64<100_000_000>; // 100 million gas max
    type VmwOperationPrice = ConstU32<1>; // VMW operation price (1 unit per operation)
}

/// Configure the pallet-consensus (ASF consensus - Adaptive Scale of Finality)
impl pallet_consensus::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type RandomnessSource = RandomnessCollectiveFlip;
    type Time = Timestamp;
    type ValidatorReward = ConstU128<100_000_000_000_000_000_000>; // 0.1 ETR per block
    type MinValidityStake = ConstU128<64_000_000_000_000_000_000_000>; // 64 ETR (Validity Node minimum)
    type CommitteeSize = ConstU32<21>; // 21 PPFA committee members (as per Ivory Papers)
    type EpochDuration = ConstU32<2400>; // 2400 blocks per epoch (~4 hours at 6s blocks)
    type BaseSlotDuration = ConstU64<6000>; // 6 seconds base slot duration (adaptive)
}

/// Configure the pallet-governance (DAO governance)
impl pallet_governance::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Time = Timestamp;
    type ProposalDuration = ConstU64<604_800_000>; // 7 days in milliseconds
    type MinProposalStake = ConstU128<10_000_000_000_000>; // 10 ETR
    type GovernanceOrigin = frame_system::EnsureRoot<AccountId>;
}

/// Configure the PBC Router (Partition Burst Chain routing)
impl pallet_pbc_router::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxPbcs = ConstU32<12>; // 12 PBCs (one per bridge)
    type MaxPendingMessages = ConstU32<1000>; // Max pending messages per PBC
    type MaxMessageSize = ConstU32<10240>; // 10KB max message size
    type RegisterOrigin = frame_system::EnsureRoot<AccountId>;
    type CollatorOrigin = frame_system::EnsureSigned<AccountId>;
}

// ========================================
// CONSENSUS DAY PALLETS CONFIGURATION
// ========================================

parameter_types! {
    pub const ConsensusDayRegistrationDeposit: Balance = 1_000_000_000_000; // 0.001 ETR
    pub FoundationTreasuryAccount: AccountId = AccountId::from([0u8; 32]);
    pub DirectorAccounts: Vec<AccountId> = vec![]; // To be populated from staking pallet
    pub ValidatorAccounts: Vec<AccountId> = vec![]; // To be populated from staking pallet
    pub VoterAccounts: Vec<AccountId> = vec![]; // To be populated from Consensus Day registration
    pub const AnnualMintCapPercent: u8 = 5; // 5% annual inflation cap
}

/// Configure Consensus Day Proposal System
impl consensus_day_proposal_system::Config for Runtime {
    type Currency = Balances;
    type RegistrationDeposit = ConsensusDayRegistrationDeposit;
}

/// Configure Consensus Day Voting Protocol
impl consensus_day_voting_protocol::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

/// Configure Consensus Day Distribution
impl consensus_day_distribution::Config for Runtime {
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type FoundationAccount = FoundationTreasuryAccount;
    type Directors = DirectorAccounts;
    type Validators = ValidatorAccounts;
    type Voters = VoterAccounts;
}

/// Configure Consensus Day Minting Logic
impl consensus_day_minting_logic::Config for Runtime {
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type TreasuryAccount = FoundationTreasuryAccount;
    type AnnualMintCapPercent = AnnualMintCapPercent;
}

// ========================================
// TRANSACTION PALLETS CONFIGURATION
// ========================================

/// Configure Transaction Processor
impl pallet_tx_processor::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

// ========================================
// BRIDGE PALLETS CONFIGURATION (12 bridges)
// ========================================

parameter_types! {
    pub const PolygonBridgePalletId: PalletId = PalletId(*b"py/plygn");
    pub const DogeBridgePalletId: PalletId = PalletId(*b"py/dogeb");
    pub const EthBridgePalletId: PalletId = PalletId(*b"py/ethbr");
    pub const SolBridgePalletId: PalletId = PalletId(*b"py/solbr");
    pub const BtcBridgePalletId: PalletId = PalletId(*b"py/btcbr");
    // Bridge authority accounts (use well-known test accounts for now)
    pub BitcoinBridgeAuthority: AccountId = AccountId::from([1u8; 32]);
    pub CardanoBridgeAuthority: AccountId = AccountId::from([2u8; 32]);
    // Doge bridge fee as Perbill (0.1% = 1_000_000 parts per billion)
    pub const DogeBridgeFee: Perbill = Perbill::from_parts(1_000_000);
    // Validator pool accounts for bridges
    pub BtcValidatorPoolAccount: AccountId = BtcBridgePalletId::get().into_account_truncating();
    pub EthValidatorPoolAccount: AccountId = EthBridgePalletId::get().into_account_truncating();
    pub SolValidatorPoolAccount: AccountId = SolBridgePalletId::get().into_account_truncating();
    pub PolygonValidatorPoolAccount: AccountId = PolygonBridgePalletId::get().into_account_truncating();
}

/// Configure ETR Lock Pallet (shared by all bridges)
parameter_types! {
    pub const EtrLockId: [u8; 8] = *b"etr/lock";
}

impl pallet_etr_lock::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BridgeOrigin = EnsureRoot<AccountId>; // Foundation multisig in production
    type MaxLockAmount = ConstU128<250_000_000_000_000_000_000_000_000>; // 250M ETR max lock
    type LockIdentifier = EtrLockId;
}

/// Configure Bitcoin Bridge
impl pallet_bitcoin_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Treasury = BridgeTreasuryInterface;
    type MinConfirmations = ConstU32<6>;
    type MinDepositAmount = ConstU64<1_000_000>; // 0.01 BTC in satoshis
    type MaxDepositAmount = ConstU64<100_000_000_000>; // 1000 BTC in satoshis
    type BridgeAuthority = BitcoinBridgeAuthority;
    type ValidatorPoolAccount = BtcValidatorPoolAccount;
}

/// Configure Ethereum Bridge
impl eth_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Treasury = BridgeTreasuryInterface;
    type MinConfirmations = ConstU32<12>;
    type BridgeFeeRate = ConstU32<10>; // 0.1%
    type MaxGasLimit = ConstU64<10_000_000>;
    type MaxDepositsPerAccount = ConstU32<100>;
    type MaxWithdrawalsPerAccount = ConstU32<100>;
    type ValidatorPoolAccount = EthValidatorPoolAccount;
}

/// Configure Dogecoin Bridge
impl pallet_doge_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BridgeFee = DogeBridgeFee;
    type MinBridgeAmount = ConstU128<100_000_000_000>; // 0.1 ETR
    type MaxBridgeAmount = ConstU128<100_000_000_000_000_000>; // 100,000 ETR
    type PalletId = DogeBridgePalletId;
    type DogeConfirmations = ConstU32<6>;
    type DogeConversionRate = ConstU64<1000>; // 1 DOGE = 0.001 ETR
}

/// Configure Stellar (XLM) Bridge
impl stellar_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = ConstU32<3>;
    type BridgeFeeRate = ConstU32<10>; // 0.1%
    type MaxDepositsPerAccount = ConstU32<100>;
    type MaxWithdrawalsPerAccount = ConstU32<100>;
}

/// Configure XRP Bridge
impl xrp_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MinConfirmations = ConstU32<3>;
    type BridgeFeeRate = ConstU32<10>; // 0.1%
    type MaxFeeDrops = ConstU64<1_000_000>; // 1 XRP
    type MaxDepositsPerAccount = ConstU32<100>;
    type MaxWithdrawalsPerAccount = ConstU32<100>;
}

/// Configure Solana Bridge
impl sol_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Treasury = BridgeTreasuryInterface;
    type MinConfirmations = ConstU32<32>;
    type BridgeFeeRate = ConstU32<10>; // 0.1%
    type MaxPriorityFee = ConstU64<10_000>;
    type MaxComputeUnits = ConstU32<1_400_000>;
    type MaxDepositsPerAccount = ConstU32<100>;
    type MaxWithdrawalsPerAccount = ConstU32<100>;
    type ValidatorPoolAccount = SolValidatorPoolAccount;
}

/// Configure Cardano (ADA) Bridge
impl pallet_cardano_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = ConstU32<15>;
    type MinDepositAmount = ConstU64<1_000_000>; // 1 ADA in lovelaces
    type MaxDepositAmount = ConstU64<1_000_000_000_000>; // 1,000,000 ADA
    type BridgeAuthority = CardanoBridgeAuthority;
}

/// Configure Chainlink (LINK) Bridge
impl chainlink_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = ConstU32<12>;
    type BridgeFeeRate = ConstU32<10>; // 0.1%
    type MaxOracleNodes = ConstU32<100>;
    type MaxDataFeeds = ConstU32<1000>;
    type MaxVRFRequests = ConstU32<1000>;
    type PriceStalenessThreshold = ConstU32<100>; // 100 blocks
}

/// Configure Polygon (MATIC) Bridge
impl polygon_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MinConfirmations = ConstU32<128>;
    type BridgeFeeRate = ConstU32<10>; // 0.1%
    type MaxGasLimit = ConstU64<10_000_000>;
    type MinBridgeAmount = ConstU128<1_000_000_000_000>; // 0.001 ETR
    type MaxDepositsPerAccount = ConstU32<100>;
    type MaxWithdrawalsPerAccount = ConstU32<100>;
    type PalletId = PolygonBridgePalletId;
}

/// Configure BNB Bridge
impl bnb_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MinConfirmations = ConstU32<15>;
    type BridgeFeeRate = ConstU32<10>; // 0.1%
    type MaxGasLimit = ConstU64<10_000_000>;
    type MaxGasPrice = ConstU128<100_000_000_000>; // 100 Gwei
    type MaxDepositsPerAccount = ConstU32<100>;
    type MaxWithdrawalsPerAccount = ConstU32<100>;
}

/// Configure Tron (TRX) Bridge
impl trx_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = ConstU32<19>;
    type BridgeFeeRate = ConstU32<10>; // 0.1%
    type MaxEnergyLimit = ConstU64<100_000_000>;
    type MaxBandwidth = ConstU64<10_000_000>;
    type MaxDepositsPerAccount = ConstU32<100>;
    type MaxWithdrawalsPerAccount = ConstU32<100>;
}

/// Configure USDT Stablecoin Bridge
impl stablecoin_usdt_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BridgeFeeRate = ConstU32<5>; // 0.05% for stablecoins
    type MaxDepositsPerAccount = ConstU32<100>;
    type MaxWithdrawalsPerAccount = ConstU32<100>;
    type MaxCustodians = ConstU32<10>; // Maximum 10 custodians for M-of-N multisig
}

// ========================================
// EDSC PALLETS CONFIGURATION
// ========================================

/// Configure EDSC Token Pallet
impl pallet_edsc_token::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxSupply = ConstU128<1_000_000_000_000_000_000_000>; // 1 billion EDSC (18 decimals)
    type MinBalance = ConstU128<1_000_000_000_000>; // 0.000001 EDSC minimum
    type WeightInfo = ();
}

/// Configure EDSC Receipts Pallet (SBT registry)
impl pallet_edsc_receipts::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxReceiptsPerWallet = ConstU32<1000>; // Max 1000 receipts per wallet
    type ReceiptExpiryPeriod = ConstU32<5_256_000>; // ~1 year (at 6s blocks)
}

parameter_types! {
    // EDSC Redemption Parameters
    pub const MinRedemptionFee: Permill = Permill::from_parts(2_500); // 0.25%
    pub SafetyMultiplier: FixedU128 = FixedU128::from_rational(12u128, 10u128); // 1.2
    pub const Path1DailyLimit: u128 = 50_000_00; // $50,000 in cents
    pub const Path2DailyLimit: u128 = 25_000_00; // $25,000 in cents
    pub const Path3DailyLimit: u128 = 10_000_00; // $10,000 in cents
    pub const HourlyRedemptionCap: Permill = Permill::from_parts(5_000); // 0.5%
    pub const DailyRedemptionCap: Permill = Permill::from_parts(5_000); // 0.5%
    pub ThrottleRedemptionRatio: FixedU128 = FixedU128::from_rational(105u128, 100u128); // 1.05 = 105%
    pub EmergencyRedemptionRatio: FixedU128 = FixedU128::from_rational(100u128, 100u128); // 1.00 = 100%
    pub const MaxRedemptionQueueSize: u32 = 10_000;
}

/// Configure EDSC Redemption Pallet
impl pallet_edsc_redemption::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MinRedemptionFee = MinRedemptionFee;
    type SafetyMultiplier = SafetyMultiplier;
    type Path1DailyLimit = Path1DailyLimit;
    type Path2DailyLimit = Path2DailyLimit;
    type Path3DailyLimit = Path3DailyLimit;
    type HourlyRedemptionCap = HourlyRedemptionCap;
    type DailyRedemptionCap = DailyRedemptionCap;
    type ThrottleReserveRatio = ThrottleRedemptionRatio;
    type EmergencyReserveRatio = EmergencyRedemptionRatio;
    type MaxQueueSize = MaxRedemptionQueueSize;
}

parameter_types! {
    // EDSC Oracle Parameters
    pub const PrimaryTwapWindow: u32 = 14_400; // 24 hours (at 6s blocks)
    pub const FallbackTwapWindow: u32 = 100_800; // 7 days (at 6s blocks)
    pub const MinPriceSources: u32 = 5;
    pub const OutlierThreshold: Permill = Permill::from_parts(20_000); // 2%
    pub const StalenessTimeout: u32 = 100; // ~10 minutes
    pub const MaxPriceHistory: u32 = 10_000; // Keep last 10k price points
}

/// Configure EDSC Oracle Pallet
impl pallet_edsc_oracle::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type PriceCallback = EdscRedemption;
    type PrimaryTwapWindow = PrimaryTwapWindow;
    type FallbackTwapWindow = FallbackTwapWindow;
    type MinPriceSources = MinPriceSources;
    type OutlierThreshold = OutlierThreshold;
    type StalenessTimeout = StalenessTimeout;
    type MaxPriceHistory = MaxPriceHistory;
}

parameter_types! {
    // Reserve Vault Parameters
    pub OptimalReserveMin: FixedU128 = FixedU128::from_rational(110u128, 100u128); // 1.10 = 110%
    pub OptimalReserveMax: FixedU128 = FixedU128::from_rational(130u128, 100u128); // 1.30 = 130%
    pub ThrottleReserveRatio: FixedU128 = FixedU128::from_rational(105u128, 100u128); // 1.05 = 105%
    pub EmergencyReserveRatio: FixedU128 = FixedU128::from_rational(100u128, 100u128); // 1.00 = 100%
}

/// Configure Reserve Vault Pallet
impl pallet_reserve_vault::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OptimalReserveMin = OptimalReserveMin;
    type OptimalReserveMax = OptimalReserveMax;
    type ThrottleReserveRatio = ThrottleReserveRatio;
    type EmergencyReserveRatio = EmergencyReserveRatio;
}

parameter_types! {
    // Custodian Registry Parameters
    pub const MinCustodianBond: Balance = 100_000_000_000_000_000_000_000; // 100,000 ETR
    pub const AttestationFrequency: u32 = 1_314_000; // ~3 months (at 6s blocks)
    pub const MaxMissedAttestations: u32 = 2;
    pub const CustodianSlashPercentage: Permill = Permill::from_percent(50); // 50% slash for non-compliance
}

/// Configure Custodian Registry Pallet
impl pallet_custodian_registry::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinBondAmount = MinCustodianBond;
    type AttestationFrequency = AttestationFrequency;
    type MaxMissedAttestations = MaxMissedAttestations;
    type SlashPercentage = CustodianSlashPercentage;
}

// Reserve Oracle Configuration (FlareChain - aggregates reserve data)
parameter_types! {
    pub const OracleSnapshotInterval: u32 = 100;  // Create snapshot every 100 blocks (~10 minutes)
    pub const MaxOracleSnapshots: u32 = 10_000;  // Max stored snapshots
    pub const ReserveOracleOptimalMin: u16 = 11000;  // 110% (11000 basis points)
    pub const ReserveOracleOptimalMax: u16 = 13000;  // 130% (13000 basis points)
    pub const ReserveOracleThrottleThreshold: u16 = 10500;  // 105% (10500 basis points)
    pub const ReserveOracleCriticalThreshold: u16 = 10000;  // 100% (10000 basis points)
    pub const MaxOraclePriceStaleness: u32 = 1000;  // Max blocks before price is stale
}

impl pallet_reserve_oracle::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type SnapshotInterval = OracleSnapshotInterval;
    type MaxSnapshots = MaxOracleSnapshots;
    type ReserveOptimalMin = ReserveOracleOptimalMin;
    type ReserveOptimalMax = ReserveOracleOptimalMax;
    type ReserveThrottleThreshold = ReserveOracleThrottleThreshold;
    type ReserveCriticalThreshold = ReserveOracleCriticalThreshold;
    type MaxPriceStaleness = MaxOraclePriceStaleness;
    type MaxPriceAge = ConstU32<100>; // Max price age in blocks before considered stale
}

// Multiasset Reserve Configuration (Advanced multi-asset reserve management)
parameter_types! {
    pub const MaxReserveAssets: u32 = 50;  // Maximum number of assets in reserve
    pub const RebalanceIntervalBlocks: u32 = 14_400;  // Rebalance every ~24 hours (assuming 6s blocks)
    pub const RebalanceThreshold: sp_arithmetic::Permill = sp_arithmetic::Permill::from_percent(5);  // 5% threshold
    pub const MultiassetReservePalletId: frame_support::PalletId = frame_support::PalletId(*b"py/marve");
}

impl pallet_multiasset_reserve::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxAssets = MaxReserveAssets;
    type RebalanceInterval = RebalanceIntervalBlocks;
    type RebalanceThreshold = RebalanceThreshold;
    type PalletId = MultiassetReservePalletId;
    type WeightInfo = ();
}

// Reserve-Backed Token Configuration (Synthetic assets backed by reserves)
parameter_types! {
    pub const MaxSyntheticTokens: u32 = 100;  // Maximum number of synthetic tokens
    pub const MaxPositionsPerUser: u32 = 50;  // Maximum positions per user
    pub const MinCollateralAmount: u128 = 1_000_000_000_000;  // Minimum collateral (1 token with 12 decimals)
    pub const LiquidationPenaltyPercent: u16 = 500;  // 5% liquidation penalty (500 basis points)
    pub const ReserveBackedTokenPalletId: frame_support::PalletId = frame_support::PalletId(*b"py/rbtok");
}

impl pallet_reserve_backed_token::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxSynthetics = MaxSyntheticTokens;
    type MaxPositionsPerUser = MaxPositionsPerUser;
    type MinCollateral = MinCollateralAmount;
    type LiquidationPenalty = LiquidationPenaltyPercent;
    type PalletId = ReserveBackedTokenPalletId;
    type WeightInfo = ();
}

// ========================================
// ORACLE NETWORK CONFIGURATION
// ========================================

parameter_types! {
    pub const MinOracleStake: Balance = 1_000_000_000_000_000_000_000; // 1,000 ETR
    pub const MaxOracleStake: Balance = 1_000_000_000_000_000_000_000_000; // 1,000,000 ETR
    pub const OracleSlashPercentage: sp_arithmetic::Permill = sp_arithmetic::Permill::from_percent(5); // 5% slash
    pub const MinReputationThreshold: u8 = 50; // Min reputation to stay active
    pub const OracleReward: Balance = 10_000_000_000_000_000_000; // 10 ETR per submission
    pub const MaxOracles: u32 = 1000; // Maximum 1000 oracles
    pub const MaxDataRequests: u32 = 10000; // Maximum 10000 data requests
}

/// Configure Oracle Network Pallet
impl pallet_oracle_network::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Treasury = OracleTreasuryNotifier;
    type MinimumStake = MinOracleStake;
    type MaximumStake = MaxOracleStake;
    type SlashPercentage = OracleSlashPercentage;
    type MinimumReputation = MinReputationThreshold;
    type SubmissionReward = OracleReward;
    type MaxOracles = MaxOracles;
    type MaxDataRequests = MaxDataRequests;
}

// ========================================
// OPENDID PALLETS CONFIGURATION (Component 02)
// ========================================

/// Configure DID Registry Pallet
impl pallet_did_registry::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxAccessControlEntries = ConstU32<100>;
}

/// Configure AIDID Pallet (World's First AI DID Standard)
impl pallet_aidid::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxAIsPerController = ConstU32<100>;
}

// ========================================
// CROSS-CHAIN INFRASTRUCTURE
// ========================================

// XCM Bridge Configuration (Cross-chain messaging with PBC-EDSC)
parameter_types! {
    pub const FlareChainMaxPayloadSize: u32 = 1024;  // Max message size (bytes)
    pub const FlareChainMessageTimeout: u32 = 1_000;  // Message expiry (blocks)
    pub const FlareChainMaxPendingMessages: u32 = 1_000;  // Max queue size
    pub const FlareChainIdentifier: pallet_xcm_bridge::ChainId = pallet_xcm_bridge::ChainId::FlareChain;
}

impl pallet_xcm_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxPayloadSize = FlareChainMaxPayloadSize;
    type MessageTimeout = FlareChainMessageTimeout;
    type MaxPendingMessages = FlareChainMaxPendingMessages;
    type ChainIdentifier = FlareChainIdentifier;
}

// ════════════════════════════════════════════════════════════════════════════
// Phase 3: External Bridge Protocol (CCTP-style)
// ════════════════════════════════════════════════════════════════════════════

parameter_types! {
    pub const FlareTokenMessengerMaxMessageBodySize: u32 = 512;
    pub const FlareTokenMessengerMaxBurnAmount: u128 = 1_000_000_000_000_000_000_000_000;  // 1M EDSC per tx
    pub const FlareTokenMessengerDailyBurnCap: u128 = 10_000_000_000_000_000_000_000_000;  // 10M EDSC per day
    pub const FlareTokenMessengerMessageTimeout: u32 = 1000;
}

impl pallet_edsc_bridge_token_messenger::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxMessageBodySize = FlareTokenMessengerMaxMessageBodySize;
    type MaxBurnAmount = FlareTokenMessengerMaxBurnAmount;
    type DailyBurnCap = FlareTokenMessengerDailyBurnCap;
    type MessageTimeout = FlareTokenMessengerMessageTimeout;
}

parameter_types! {
    pub const FlareMaxAttesters: u32 = 100;  // Maximum registered attesters
    pub const FlareMaxAttestersPerMessage: u32 = 10;  // Maximum signatures per message
    pub const FlareMinSignatureThreshold: u32 = 3;  // Default M-of-N (3-of-5)
    pub const FlareAttestationMaxAge: u32 = 1000;  // 1000 blocks (~100 minutes)
}

impl pallet_edsc_bridge_attestation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxAttesters = FlareMaxAttesters;
    type MaxAttestersPerMessage = FlareMaxAttestersPerMessage;
    type MinSignatureThreshold = FlareMinSignatureThreshold;
    type AttestationMaxAge = FlareAttestationMaxAge;
}

// ═══════════════════════════════════════════════════════════════════════════════
// ASF CONSENSUS PALLETS
// ═══════════════════════════════════════════════════════════════════════════════

parameter_types! {
    pub const MaxCommitteeSize: u32 = 100;
    pub const MinValidatorStake: u128 = 1_000_000_000_000_000_000; // 1 ËTRID
}

impl pallet_validator_committee::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxCommitteeSize = MaxCommitteeSize;
    type MinValidatorStake = MinValidatorStake;
}

// ═══════════════════════════════════════════════════════════════════════════════
// VALIDATOR REWARDS PALLET
// ═══════════════════════════════════════════════════════════════════════════════

parameter_types! {
    pub const EpochDuration: u32 = 14_400; // ~24 hours at 6s blocks
    pub const AnnualRewardPoolBps: u32 = 300; // 3% annual inflation
    pub const ValidatorShareBps: u32 = 5000; // 50/50 validator/delegator split
}

impl pallet_validator_rewards::Config for Runtime {
    type Currency = Balances;
    type EpochDuration = EpochDuration;
    type AnnualRewardPoolBps = AnnualRewardPoolBps;
    type ValidatorShareBps = ValidatorShareBps;
}

// ═══════════════════════════════════════════════════════════════════════════════
// ETRID TREASURY PALLET
// ═══════════════════════════════════════════════════════════════════════════════

parameter_types! {
    pub const TreasuryDirectorCount: u8 = 9; // 9 directors
    pub const TreasuryApprovalThreshold: u8 = 6; // 6-of-9 for normal disbursements
    pub const TreasuryEmergencyThreshold: u8 = 7; // 7-of-9 for emergency withdrawals
    pub const TreasuryProposalExpiration: BlockNumber = 7 * DAYS; // 7 days
}

impl pallet_treasury_etrid::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type DirectorCount = TreasuryDirectorCount;
    type ApprovalThreshold = TreasuryApprovalThreshold;
    type EmergencyThreshold = TreasuryEmergencyThreshold;
    type ProposalExpiration = TreasuryProposalExpiration;
    type WeightInfo = ();
}

// ═══════════════════════════════════════════════════════════════════════════════
// BRIDGE TREASURY INTERFACES
// ═══════════════════════════════════════════════════════════════════════════════

/// Treasury interface for cross-chain bridges
pub struct BridgeTreasuryInterface;
impl etrid_bridge_common::treasury::TreasuryInterface<sp_runtime::AccountId32, u128> for BridgeTreasuryInterface {
    fn receive_cross_chain_fees(amount: u128) -> sp_runtime::DispatchResult {
        // Bridge fees are tracked but treasury receives them directly in the bridge logic
        // The 10% fee split happens within each bridge pallet's withdrawal logic
        let _ = amount; // Unused but required by trait signature
        Ok(())
    }
}

/// Treasury notifier for oracle network slashing
pub struct OracleTreasuryNotifier;
impl pallet_oracle_network::TreasuryNotifier<u128> for OracleTreasuryNotifier {
    fn notify_slashing_proceeds(amount: u128) -> Result<(), sp_runtime::DispatchError> {
        // Oracle slashing proceeds tracked but handled directly in oracle logic
        // The 50% slash distribution happens within the oracle pallet
        let _ = amount; // Unused but required by trait signature
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONSENSUS DAY PALLET
// ═══════════════════════════════════════════════════════════════════════════════

/// Treasury interface implementation for Consensus Day
pub struct ConsensusDayTreasuryInterface;
impl pallet_consensus_day::TreasuryInterface<AccountId, Balance> for ConsensusDayTreasuryInterface {
    fn fund_treasury(
        from: &AccountId,
        amount: Balance,
        categories: sp_std::vec::Vec<(pallet_consensus_day::BudgetCategory, Balance)>,
    ) -> DispatchResult {
        use frame_support::traits::fungible::Mutate;
        use frame_support::traits::tokens::Preservation;

        // Transfer from Consensus Day pallet to Treasury pallet
        let treasury_account = EtridTreasury::account_id();
        Balances::transfer(from, &treasury_account, amount, Preservation::Preserve)?;

        // Fund treasury with categorized allocations
        EtridTreasury::fund_treasury(
            frame_system::RawOrigin::Root.into(),
            pallet_treasury_etrid::FundingSource::ConsensusDayMinting,
            amount,
        )?;

        // Allocate to categories
        EtridTreasury::allocate_to_categories(
            frame_system::RawOrigin::Root.into(),
            amount,
        )?;

        Ok(())
    }
}

parameter_types! {
    pub const ConsensusRegistrationDuration: u32 = 3_600; // 6 hours at 6s blocks
    pub const ConsensusVotingDuration: u32 = 7_200; // 12 hours
    pub const ConsensusMintingDuration: u32 = 1_800; // 3 hours
    pub const ConsensusDistributionDuration: u32 = 600; // 1 hour
    pub const ConsensusProposalBond: Balance = 10_000 * UNITS; // 10,000 ETR
    pub const ConsensusDirectorMinStake: Balance = 128 * UNITS; // 128 ETR
    pub const ConsensusMaxInflationBps: u32 = 500; // 5% max inflation
    pub const ConsensusMaxProposals: u32 = 100;
    pub const ConsensusMaxTitleLength: u32 = 100;
}

impl pallet_consensus_day::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Treasury = ConsensusDayTreasuryInterface;
    type RegistrationDuration = ConsensusRegistrationDuration;
    type VotingDuration = ConsensusVotingDuration;
    type MintingDuration = ConsensusMintingDuration;
    type DistributionDuration = ConsensusDistributionDuration;
    type ProposalBond = ConsensusProposalBond;
    type DirectorMinStake = ConsensusDirectorMinStake;
    type MaxInflationBps = ConsensusMaxInflationBps;
    type MaxProposals = ConsensusMaxProposals;
    type MaxTitleLength = ConsensusMaxTitleLength;
}

// ═══════════════════════════════════════════════════════════════════════════════
// EDSC STABILITY PALLET
// ═══════════════════════════════════════════════════════════════════════════════

/// Treasury interface for EDSC Stability fees
pub struct EdscStabilityTreasuryInterface;
impl pallet_edsc_stability::TreasuryInterface<AccountId, Balance> for EdscStabilityTreasuryInterface {
    fn receive_stability_fees(amount: Balance) -> Result<(), sp_runtime::DispatchError> {
        use frame_support::traits::fungible::Mutate;

        // Mint stability fees directly to treasury account
        let treasury_account = EtridTreasury::account_id();
        let _ = Balances::mint_into(&treasury_account, amount)?;

        // Record as stability fee income
        let _ = EtridTreasury::fund_treasury(
            RuntimeOrigin::root(),
            pallet_treasury_etrid::FundingSource::StabilityFees,
            amount,
        )?;

        Ok(())
    }
}

parameter_types! {
    pub const MinCollateralRatio: u16 = 15000; // 150%
    pub const LiquidationThreshold: u16 = 12000; // 120%
    pub const LiquidationPenalty: u16 = 500; // 5%
    pub const StabilityRebalanceThreshold: u16 = 500; // 5%
    pub const EmergencyPauseThreshold: u16 = 1000; // 10%
    pub const MinEDSCMint: u128 = 100 * UNITS; // 100 EDSC minimum
    pub const BaseInterestRate: u16 = 300; // 3% annual
    pub const EDSCPalletId: PalletId = PalletId(*b"py/edscs");
}

impl pallet_edsc_stability::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinCollateralRatio = MinCollateralRatio;
    type LiquidationThreshold = LiquidationThreshold;
    type LiquidationPenalty = LiquidationPenalty;
    type RebalanceThreshold = StabilityRebalanceThreshold;
    type EmergencyPauseThreshold = EmergencyPauseThreshold;
    type MinEDSCMint = MinEDSCMint;
    type BaseInterestRate = BaseInterestRate;
    type PalletId = EDSCPalletId;
    type TreasuryAccount = FoundationTreasuryAccount;
    type Treasury = EdscStabilityTreasuryInterface;
    type WeightInfo = ();
}

// ═══════════════════════════════════════════════════════════════════════════════
// CIRCUIT BREAKER PALLET (already in Cargo.toml)
// ═══════════════════════════════════════════════════════════════════════════════

parameter_types! {
    pub const MaxHourlyVolume: u128 = 1_000_000 * UNITS; // 1M ETR per hour
    pub const MaxDailyVolume: u128 = 10_000_000 * UNITS; // 10M ETR per day
    pub const ThrottleThreshold: u16 = 8000; // 80% of max (8000 basis points)
    pub const EmergencyThreshold: u16 = 9500; // 95% of max (9500 basis points)
    pub const BlocksPerHour: u32 = HOURS; // ~600 blocks per hour at 6s blocks
    pub const BlocksPerDay: u32 = DAYS; // ~14400 blocks per day at 6s blocks
}

impl pallet_circuit_breaker::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxHourlyVolume = MaxHourlyVolume;
    type MaxDailyVolume = MaxDailyVolume;
    type ThrottleThreshold = ThrottleThreshold;
    type EmergencyThreshold = EmergencyThreshold;
    type BlocksPerHour = BlocksPerHour;
    type BlocksPerDay = BlocksPerDay;
}

// ═══════════════════════════════════════════════════════════════════════════════
// AI AGENTS PALLET (Component 14)
// ═══════════════════════════════════════════════════════════════════════════════

parameter_types! {
    pub const MinAgentStake: Balance = 100 * UNITS; // 100 ETR minimum stake per agent
    pub const MaxAgentsPerValidator: u32 = 6;
    pub const SlashingThreshold: u32 = 100; // Slash if reputation < 100
    pub const InitialReputation: u32 = 500; // Start with 500 reputation (out of 1000)
}

impl pallet_ai_agents::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinAgentStake = MinAgentStake;
    type MaxAgentsPerValidator = MaxAgentsPerValidator;
    type SlashingThreshold = SlashingThreshold;
    type InitialReputation = InitialReputation;
}

// ═══════════════════════════════════════════════════════════════════════════════
// EVM LAYER MOVED TO ETH-PBC
// ═══════════════════════════════════════════════════════════════════════════════
// FlareChain is the pure coordination layer with:
// - ËtwasmVM for native WebAssembly smart contracts
// - Oracle Network for cross-chain price feeds
// - Governance & Consensus Day
// - Staking & Validator Management
// - PBC Router for coordinating all Partition Burst Chains
//
// Ethereum compatibility (Solidity, MetaMask, etc.) is handled by ETH-PBC
// This architectural separation preserves Ëtrid's unique identity
// ═══════════════════════════════════════════════════════════════════════════════

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub struct Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Session: pallet_session,
        Balances: pallet_balances,
        Vesting: pallet_vesting,
        Multisig: pallet_multisig,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,
        RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip,

        // EVM moved to ETH-PBC - FlareChain stays pure

        // Ëtrid custom pallets
        Accounts: pallet_accounts,
        EtwasmVM: pallet_etwasm_vm,
        Consensus: pallet_consensus,
        Governance: pallet_governance,
        PbcRouter: pallet_pbc_router,

        // Staking pallets
        EtridStaking: pallet_etrid_staking,

        // Consensus Day pallets
        ConsensusDayProposalSystem: consensus_day_proposal_system,
        ConsensusDayVotingProtocol: consensus_day_voting_protocol,
        ConsensusDayDistribution: consensus_day_distribution,
        ConsensusDayMintingLogic: consensus_day_minting_logic,

        // Transaction pallets
        TxProcessor: pallet_tx_processor,

        // Bridge pallets (12 bridges for multichain support)
        BitcoinBridge: pallet_bitcoin_bridge,
        EthereumBridge: eth_bridge,
        DogeBridge: pallet_doge_bridge,
        StellarBridge: stellar_bridge,
        XrpBridge: xrp_bridge,
        SolanaBridge: sol_bridge,
        CardanoBridge: pallet_cardano_bridge,
        ChainlinkBridge: chainlink_bridge,
        PolygonBridge: polygon_bridge,
        BnbBridge: bnb_bridge,
        TronBridge: trx_bridge,
        UsdtBridge: stablecoin_usdt_bridge,

        // ETR Lock (shared by all bridges for ETR token locking)
        EtrLock: pallet_etr_lock,

        // EDSC pallets (Ëtrid Dollar Stablecoin system)
        EdscToken: pallet_edsc_token,
        EdscReceipts: pallet_edsc_receipts,
        EdscRedemption: pallet_edsc_redemption,
        EdscOracle: pallet_edsc_oracle,
        ReserveVault: pallet_reserve_vault,
        CustodianRegistry: pallet_custodian_registry,
        ReserveOracle: pallet_reserve_oracle,
        MultiassetReserve: pallet_multiasset_reserve,
        ReserveBackedToken: pallet_reserve_backed_token,
        XcmBridge: pallet_xcm_bridge,

        // Phase 3: External Bridge Protocol (CCTP-style)
        TokenMessenger: pallet_edsc_bridge_token_messenger,
        BridgeAttestation: pallet_edsc_bridge_attestation,

        // ASF Consensus pallets
        ValidatorCommittee: pallet_validator_committee,
        ValidatorRewards: pallet_validator_rewards,

        // Oracle Network
        OracleNetwork: pallet_oracle_network,

        // OpenDID pallets (Component 02)
        DidRegistry: pallet_did_registry,
        AIDID: pallet_aidid,

        // AI Agents (Component 14)
        AiAgents: pallet_ai_agents,

        // Treasury/Reserve/Stability System
        EtridTreasury: pallet_treasury_etrid,
        ConsensusDayPallet: pallet_consensus_day,
        EdscStability: pallet_edsc_stability,
        CircuitBreaker: pallet_circuit_breaker,
    }
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    migrations::MigrateToAsfPrimary,
>;

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
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The weights we use.
const NORMAL_DISPATCH_RATIO: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(75);
const BLOCK_EXECUTION_WEIGHT: Weight = Weight::from_parts(5_000_000_000, 0);
const EXTRINSIC_BASE_WEIGHT: Weight = Weight::from_parts(125_000_000, 0);

/// Time constants - assuming 6 second blocks
pub const MINUTES: BlockNumber = 60 / 6;
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// Currency units
pub const UNITS: Balance = 1_000_000_000_000_000_000; // 1 ETR with 18 decimals
pub const CENTS: Balance = UNITS / 100;
pub const MILLICENTS: Balance = CENTS / 1_000;

#[cfg(feature = "std")]
pub fn wasm_binary_unwrap() -> &'static [u8] {
    WASM_BINARY.expect(
        "Development wasm binary is not available. This means the client is built with \
         `SKIP_WASM_BUILD` flag and it is only usable for production chains. Please rebuild with \
         the flag disabled.",
    )
}

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> sp_std::vec::Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    // GRANDPA API removed in v108 - Pure ASF consensus
    // Aura API removed in v108 - Pure ASF consensus

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    // ASF Consensus Runtime APIs
    impl pallet_validator_committee_runtime_api::ValidatorCommitteeApi<Block> for Runtime {
        fn validator_committee() -> sp_std::vec::Vec<pallet_validator_committee_runtime_api::ValidatorInfo> {
            ValidatorCommittee::get_committee()
        }

        fn validator_info(validator_id: pallet_validator_committee_runtime_api::ValidatorId) -> Option<pallet_validator_committee_runtime_api::ValidatorInfo> {
            ValidatorCommittee::get_validator(&validator_id)
        }

        fn is_validator_active(validator_id: pallet_validator_committee_runtime_api::ValidatorId) -> bool {
            ValidatorCommittee::is_validator_active(&validator_id)
        }

        fn current_epoch() -> u64 {
            ValidatorCommittee::get_current_epoch()
        }

        fn committee_size_limit() -> u32 {
            ValidatorCommittee::committee_size_limit()
        }

        fn next_epoch_start() -> u32 {
            ValidatorCommittee::next_epoch_start()
        }

        fn next_epoch_validators() -> sp_std::vec::Vec<pallet_validator_committee_runtime_api::ValidatorInfo> {
            ValidatorCommittee::get_next_epoch_validators()
        }

        fn is_proposer_authorized(
            block_number: u32,
            ppfa_index: u32,
            proposer_id: pallet_validator_committee_runtime_api::ValidatorId,
        ) -> bool {
            ValidatorCommittee::is_proposer_authorized(block_number, ppfa_index, &proposer_id)
        }

        fn epoch_duration() -> u32 {
            ValidatorCommittee::get_epoch_duration()
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            frame_support::genesis_builder_helper::build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            frame_support::genesis_builder_helper::get_preset::<RuntimeGenesisConfig>(id, |name| {
                match name.as_ref() {
                    sp_genesis_builder::DEV_RUNTIME_PRESET => {
                        Some(include_bytes!("../presets/development.json").to_vec())
                    },
                    sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => {
                        Some(include_bytes!("../presets/local_testnet.json").to_vec())
                    },
                    "ember_testnet" => {
                        Some(include_bytes!("../presets/ember_testnet.json").to_vec())
                    },
                    "test_2validator" => {
                        Some(include_bytes!("../presets/test_2validator.json").to_vec())
                    },
                    "test_21val" => {
                        Some(include_bytes!("../presets/test_21val.json").to_vec())
                    },
                    "flarechain_mainnet" => {
                        Some(include_bytes!("../presets/flarechain_mainnet.json").to_vec())
                    },
                    "flarechain_mainnet_restart_final" => {
                        Some(include_bytes!("../presets/flarechain_mainnet_restart_final.json").to_vec())
                    },
                    "flarechain_mainnet_asf" => {
                        Some(include_bytes!("../presets/flarechain_mainnet_asf.json").to_vec())
                    },
                    "mainnet_asf_only" => {
                        Some(include_bytes!("../presets/mainnet_asf_only.json").to_vec())
                    },
                    "mainnet_v108_pure_asf" => {
                        Some(include_bytes!("../presets/mainnet_v108_pure_asf.json").to_vec())
                    },
                    "mainnet_hybrid" => {
                        Some(include_bytes!("../presets/mainnet_hybrid.json").to_vec())
                    },
                    _ => None,
                }
            })
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![
                sp_genesis_builder::DEV_RUNTIME_PRESET.into(),
                sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET.into(),
                "ember_testnet".into(),
                "test_2validator".into(),
                "test_21val".into(),
                "flarechain_mainnet".into(),
                "flarechain_mainnet_asf".into(),
                "flarechain_mainnet_restart_final".into(),
                "flarechain_mainnet_session_fixed".into(),
                "mainnet_asf_only".into(),
                "mainnet_v108_pure_asf".into(),
                "mainnet_hybrid".into(),
            ]
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // ASF CONSENSUS RUNTIME APIs (Phase 1-2: Runtime Integration)
    // ═══════════════════════════════════════════════════════════════════════════════

    impl asf_apis::AsfApi<Block> for Runtime {
        fn get_committee() -> Vec<asf_algorithm::ValidatorId> {
            ValidatorCommittee::get_committee()
                .into_iter()
                .map(|info| info.validator_id().clone())
                .collect()
        }

        fn get_proposer(slot: u64) -> Option<asf_algorithm::ValidatorId> {
            ValidatorCommittee::get_proposer_for_slot(slot)
        }

        fn get_finality_level(block_hash: asf_algorithm::Hash) -> asf_algorithm::FinalityLevel {
            // v108: Pure ASF consensus - no GRANDPA
            // Finality is determined by ASF certificate accumulation
            // For now, return None until certificate tracking is fully implemented
            asf_algorithm::FinalityLevel::None
        }

        fn is_validator_excluded(validator: asf_algorithm::ValidatorId) -> bool {
            !ValidatorCommittee::is_validator_active(&validator)
        }

        fn get_current_epoch() -> u64 {
            ValidatorCommittee::get_current_epoch()
        }

        fn get_total_stake() -> u128 {
            ValidatorCommittee::get_total_stake()
        }

        fn get_validator_info(validator: asf_algorithm::ValidatorId) -> Option<(u128, bool, u32)> {
            ValidatorCommittee::get_validator(&validator).map(|info| {
                (info.stake, info.is_active(), info.reputation_score())
            })
        }

        fn get_ppfa_index(block_number: asf_algorithm::BlockNumber) -> u32 {
            ValidatorCommittee::get_ppfa_index(block_number as u32)
        }

        fn get_certificate_count(block_hash: asf_algorithm::Hash) -> u32 {
            // For Phase 2, return 0 (certificate accumulation not yet active)
            // In Phase 3+, this will query certificate storage
            0
        }

        fn has_bft_finality(block_hash: asf_algorithm::Hash) -> bool {
            // v108: Pure ASF consensus - no GRANDPA
            // BFT finality is determined by ASF certificate accumulation
            // For now, return false until certificate tracking is fully implemented
            false
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // EVM RUNTIME APIs MOVED TO ETH-PBC
    // ═══════════════════════════════════════════════════════════════════════════════
    // Ethereum JSON-RPC compatibility (MetaMask, web3.js, ethers.js) is handled by
    // ETH-PBC, which implements all Frontier EVM runtime APIs.
    //
    // FlareChain focuses on:
    // - Native Substrate extrinsics
    // - ËtwasmVM contract calls
    // - Oracle data feeds
    // - Governance proposals
    // - Cross-chain coordination via XCM
    // ═══════════════════════════════════════════════════════════════════════════════

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
            let weight = Executive::try_runtime_upgrade(checks).unwrap();
            (weight, RuntimeBlockWeights::get().max_block)
        }

        fn execute_block(
            block: Block,
            state_root_check: bool,
            signature_check: bool,
            select: frame_try_runtime::TryStateSelect,
        ) -> Weight {
            Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
        }
    }

}

#[cfg(feature = "runtime-benchmarks")]
frame_benchmarking::define_benchmarks!(
    [frame_system, SystemBench::<Runtime>]
    [pallet_balances, Balances]
    [pallet_timestamp, Timestamp]
    [pallet_accounts, Accounts]
    [pallet_consensus, Consensus]
    [pallet_governance, Governance]
    [pallet_treasury_etrid, EtridTreasury]
);
