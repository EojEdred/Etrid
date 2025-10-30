#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

// Make the WASM binary available
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));


use sp_api::impl_runtime_apis;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{
        AccountIdLookup, AccountIdConversion, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, One, Verify,
    },
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, FixedU128, MultiSignature, Perbill,
};
use sp_arithmetic::Permill;
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// Substrate and Polkadot dependencies
use frame_support::{
    construct_runtime, derive_impl,
    dispatch::DispatchClass,
    parameter_types,
    traits::{ConstBool, ConstU128, ConstU16, ConstU32, ConstU64, ConstU8},
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
            pub grandpa: Grandpa,
        }
    }
}

// To learn more about runtime versioning, see:
// https://docs.substrate.io/main-docs/build/upgrade#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("etrid"),
    impl_name: create_runtime_str!("etrid"),
    authoring_version: 1,
    spec_version: 103,
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

impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxNominators = ConstU32<0>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 3000; // 3 seconds (half of block time)
}

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
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
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

parameter_types! {
    pub FeeMultiplier: Multiplier = Multiplier::one();
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = FungibleAdapter<Balances, ()>;
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

parameter_types! {
    pub const SpendPeriod: BlockNumber = 7 * 24 * 600; // 7 days (assuming 6 second blocks)
    pub const Burn: Permill = Permill::from_percent(0); // No burn, all rejected funds stay in treasury
    pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
    pub const MaxApprovals: u32 = 100; // Maximum proposals that can be approved in one spend period
    pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
}

impl pallet_treasury::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type PalletId = TreasuryPalletId;
    type Currency = Balances;
    type RejectOrigin = frame_system::EnsureRoot<AccountId>; // Sudo or governance can reject
    type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>; // Disable direct spend, use proposals
    type SpendPeriod = SpendPeriod;
    type Burn = Burn;
    type BurnDestination = (); // No burn destination since Burn = 0%
    type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
    type SpendFunds = (); // No automatic spend mechanism
    type MaxApprovals = MaxApprovals;
    type AssetKind = ();
    type Beneficiary = AccountId;
    type BeneficiaryLookup = AccountIdLookup<AccountId, ()>;
    type Paymaster = frame_support::traits::tokens::pay::PayFromAccount<Balances, TreasuryAccount>;
    type BalanceConverter = frame_support::traits::tokens::UnityAssetBalanceConversion;
    type PayoutPeriod = ConstU32<0>; // Instant payout
    type BlockNumberProvider = System;
}

impl pallet_insecure_randomness_collective_flip::Config for Runtime {}

// ========================================
// ËTRID CUSTOM PALLETS CONFIGURATION
// ========================================

/// Configure the pallet-accounts (ETR/ETD token system)
impl pallet_accounts::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u64;
    type GovernanceOrigin = frame_system::EnsureRoot<AccountId>;
}

/// Configure the pallet-etrid-staking (peer roles staking system)
impl pallet_etrid_staking::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type UnbondPeriod = ConstU32<28800>; // ~2 days at 6 second blocks
    type MaxUnbondingEntries = ConstU32<32>; // Max unbonding entries per account
}

/// Configure the pallet-etwasm-vm (smart contract execution)
impl pallet_etwasm_vm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxCodeSize = ConstU32<1024>;
    type DefaultGasLimit = ConstU64<10_000_000>; // 10 million gas default
    type MaxGasLimit = ConstU64<100_000_000>; // 100 million gas max
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
    type RuntimeEvent = RuntimeEvent;
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
    // Bridge authority accounts (use well-known test accounts for now)
    pub BitcoinBridgeAuthority: AccountId = AccountId::from([1u8; 32]);
    pub CardanoBridgeAuthority: AccountId = AccountId::from([2u8; 32]);
    // Doge bridge fee as Perbill (0.1% = 1_000_000 parts per billion)
    pub const DogeBridgeFee: Perbill = Perbill::from_parts(1_000_000);
}

/// Configure Bitcoin Bridge
impl pallet_bitcoin_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = ConstU32<6>;
    type MinDepositAmount = ConstU64<1_000_000>; // 0.01 BTC in satoshis
    type MaxDepositAmount = ConstU64<100_000_000_000>; // 1000 BTC in satoshis
    type BridgeAuthority = BitcoinBridgeAuthority;
}

/// Configure Ethereum Bridge
impl eth_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = ConstU32<12>;
    type BridgeFeeRate = ConstU32<10>; // 0.1%
    type MaxGasLimit = ConstU64<10_000_000>;
    type MaxDepositsPerAccount = ConstU32<100>;
    type MaxWithdrawalsPerAccount = ConstU32<100>;
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
    type Currency = Balances;
    type MinConfirmations = ConstU32<3>;
    type BridgeFeeRate = ConstU32<10>; // 0.1%
    type MaxFeeDrops = ConstU64<1_000_000>; // 1 XRP
    type MaxDepositsPerAccount = ConstU32<100>;
    type MaxWithdrawalsPerAccount = ConstU32<100>;
}

/// Configure Solana Bridge
impl sol_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = ConstU32<32>;
    type BridgeFeeRate = ConstU32<10>; // 0.1%
    type MaxPriorityFee = ConstU64<10_000>;
    type MaxComputeUnits = ConstU32<1_400_000>;
    type MaxDepositsPerAccount = ConstU32<100>;
    type MaxWithdrawalsPerAccount = ConstU32<100>;
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
    type Currency = Balances;
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
    type Currency = Balances;
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

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub struct Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Grandpa: pallet_grandpa,
        Balances: pallet_balances,
        Vesting: pallet_vesting,
        Multisig: pallet_multisig,
        Treasury: pallet_treasury,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,
        RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip,

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

        // Oracle Network
        OracleNetwork: pallet_oracle_network,

        // OpenDID pallets (Component 02)
        DidRegistry: pallet_did_registry,
        AIDID: pallet_aidid,
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

    impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> sp_consensus_grandpa::SetId {
            Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: sp_consensus_grandpa::SetId,
            _authority_id: GrandpaId,
        ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
            None
        }
    }

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
            ]
        }
    }

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

    // ═══════════════════════════════════════════════════════════════════════════════
    #[cfg(feature = "runtime-benchmarks")]
    frame_benchmarking::define_benchmarks!(
        [frame_system, SystemBench::<Runtime>]
        [pallet_balances, Balances]
        [pallet_timestamp, Timestamp]
        [pallet_accounts, PalletAccounts]
        [pallet_consensus, PalletConsensus]
        [pallet_governance, PalletGovernance]
    );

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use frame_system_benchmarking::Pallet as SystemBench;
            use baseline::Pallet as BaselineBench;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);

            let storage_info = AllPalletsWithSystem::storage_info();

            (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch};
            use sp_storage::TrackedStorageKey;
            use frame_system_benchmarking::Pallet as SystemBench;
            use baseline::Pallet as BaselineBench;

            impl frame_system_benchmarking::Config for Runtime {}
            impl baseline::Config for Runtime {}

            use frame_support::traits::WhitelistedStorageKeys;
            let whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);
            add_benchmarks!(params, batches);

            Ok(batches)
        }
    }
}
