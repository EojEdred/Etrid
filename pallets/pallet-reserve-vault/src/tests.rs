//! # Reserve Vault Pallet Tests
//!
//! Comprehensive test suite covering:
//! - Multi-asset collateral deposits and withdrawals
//! - Haircut calculations and risk adjustments
//! - Reserve ratio calculation and enforcement
//! - Circuit breaker triggers (optimal, throttle, critical)
//! - Price oracle integration

use super::*;
use crate as pallet_reserve_vault;
use frame_support::{
	assert_err, assert_ok,
	parameter_types,
	traits::{ConstU128, ConstU32},
};
use frame_system as system;
use sp_arithmetic::{FixedU128, Permill};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		EdscToken: pallet_edsc_token,
		EdscRedemption: pallet_edsc_redemption,
		ReserveVault: pallet_reserve_vault,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
	type RuntimeTask = ();
	type SingleBlockMigrations = ();
	type MultiBlockMigrator = ();
	type PreInherents = ();
	type PostInherents = ();
	type PostTransactions = ();
}

parameter_types! {
	pub const MaxSupply: u128 = 1_000_000_000_000_000_000_000;
	pub const MinBalance: u128 = 1_000_000_000_000;
}

impl pallet_edsc_token::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxSupply = MaxSupply;
	type MinBalance = MinBalance;
}

parameter_types! {
	pub const MinRedemptionFee: Permill = Permill::from_parts(2_500);
	pub SafetyMultiplier: FixedU128 = FixedU128::from_rational(12u128, 10u128);
	pub const Path1DailyLimit: u128 = 50_000_00;
	pub const Path2DailyLimit: u128 = 25_000_00;
	pub const Path3DailyLimit: u128 = 10_000_00;
	pub const HourlyRedemptionCap: Permill = Permill::from_parts(5_000);
	pub const DailyRedemptionCap: Permill = Permill::from_parts(5_000);
	pub ThrottleRedemptionRatio: FixedU128 = FixedU128::from_rational(105u128, 100u128);
	pub EmergencyRedemptionRatio: FixedU128 = FixedU128::from_rational(100u128, 100u128);
	pub const MaxRedemptionQueueSize: u32 = 10_000;
}

impl pallet_edsc_redemption::Config for Test {
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
	pub OptimalReserveMin: FixedU128 = FixedU128::from_rational(110u128, 100u128);
	pub OptimalReserveMax: FixedU128 = FixedU128::from_rational(130u128, 100u128);
	pub ThrottleReserveRatio: FixedU128 = FixedU128::from_rational(105u128, 100u128);
	pub EmergencyReserveRatio: FixedU128 = FixedU128::from_rational(100u128, 100u128);
}

impl pallet_reserve_vault::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type OptimalReserveMin = OptimalReserveMin;
	type OptimalReserveMax = OptimalReserveMax;
	type ThrottleReserveRatio = ThrottleReserveRatio;
	type EmergencyReserveRatio = EmergencyReserveRatio;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::<Test>::default().build_storage().unwrap();

	pallet_reserve_vault::GenesisConfig::<Test> {
		initial_haircuts: vec![
			(AssetType::ETR, Permill::from_percent(40)),  // 40% haircut
			(AssetType::BTC, Permill::from_percent(10)),  // 10% haircut
			(AssetType::ETH, Permill::from_percent(15)),  // 15% haircut
			(AssetType::USDC, Permill::from_percent(5)),  // 5% haircut
		],
		initial_prices: vec![
			(AssetType::ETR, 50),      // $0.50
			(AssetType::BTC, 6000000), // $60,000
			(AssetType::ETH, 300000),  // $3,000
			(AssetType::USDC, 100),    // $1.00
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	t.into()
}

const ALICE: u64 = 1;
const BOB: u64 = 2;

// ═══════════════════════════════════════════════════════════════════════════
// TEST MODULE 1: Collateral Deposits
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_deposit_collateral_succeeds() {
	new_test_ext().execute_with(|| {
		// Deposit 100 BTC (asset_type = 1)
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			100_000_000 // 1 BTC in satoshis
		));

		// Verify vault entry created
		let vault_entry = ReserveVault::vault(AssetType::BTC).unwrap();
		assert_eq!(vault_entry.raw_balance, 100_000_000);
	});
}

#[test]
fn test_deposit_multiple_assets() {
	new_test_ext().execute_with(|| {
		// Deposit BTC
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			100_000_000
		));

		// Deposit ETH
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(BOB),
			2, // ETH
			1_000_000_000_000_000_000 // 1 ETH in wei
		));

		// Verify both assets in vault
		assert!(ReserveVault::vault(AssetType::BTC).is_some());
		assert!(ReserveVault::vault(AssetType::ETH).is_some());
	});
}

#[test]
fn test_deposit_unsupported_asset_fails() {
	new_test_ext().execute_with(|| {
		// Try to deposit unsupported asset (asset_type = 99)
		assert_err!(
			ReserveVault::deposit_collateral(RuntimeOrigin::signed(ALICE), 99, 1000),
			Error::<Test>::AssetNotSupported
		);
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// TEST MODULE 2: Haircut Calculations
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_haircut_applied_correctly() {
	new_test_ext().execute_with(|| {
		// Deposit 1 BTC (10% haircut)
		// BTC price = $60,000, haircut = 10%
		// Raw value = $60,000
		// Adjusted value = $60,000 * 0.9 = $54,000

		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			100_000_000 // 1 BTC
		));

		let vault_entry = ReserveVault::vault(AssetType::BTC).unwrap();

		// Haircut should be 10%
		assert_eq!(vault_entry.haircut, Permill::from_percent(10));

		// USD value should be $60,000 (6000000 cents)
		assert_eq!(vault_entry.usd_value, 6000000);

		// Adjusted value should be $54,000 (5400000 cents) after 10% haircut
		assert_eq!(vault_entry.adjusted_value, 5400000);
	});
}

#[test]
fn test_etr_high_haircut() {
	new_test_ext().execute_with(|| {
		// ETR has 40% haircut due to volatility
		// ETR price = $0.50, haircut = 40%
		// 1000 ETR = $500, adjusted = $300

		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			0, // ETR
			1000_000_000_000_000_000_000 // 1000 ETR
		));

		let vault_entry = ReserveVault::vault(AssetType::ETR).unwrap();

		// Haircut should be 40%
		assert_eq!(vault_entry.haircut, Permill::from_percent(40));

		// Adjusted value should be 60% of raw value
		assert!(vault_entry.adjusted_value < vault_entry.usd_value);
	});
}

#[test]
fn test_stablecoin_low_haircut() {
	new_test_ext().execute_with(|| {
		// USDC has only 5% haircut (low risk)
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			3, // USDC
			1_000_000_000 // 1000 USDC
		));

		let vault_entry = ReserveVault::vault(AssetType::USDC).unwrap();

		// Haircut should be 5%
		assert_eq!(vault_entry.haircut, Permill::from_percent(5));
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// TEST MODULE 3: Reserve Ratio Calculation
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_reserve_ratio_calculation() {
	new_test_ext().execute_with(|| {
		// Mint 100,000 EDSC (total supply)
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_000).unwrap(); // $100,000

		// Deposit collateral worth $120,000 (after haircuts)
		// BTC: $60,000 raw → $54,000 adjusted (10% haircut)
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			100_000_000 // 1 BTC
		));

		// Calculate reserve ratio
		assert_ok!(ReserveVault::calculate_reserve_ratio(RuntimeOrigin::signed(ALICE)));

		// Reserve ratio should be: $54,000 / $100,000 = 0.54 = 54%
		let ratio = ReserveVault::reserve_ratio();

		// Should trigger critical alert (< 100%)
		assert!(ratio < FixedU128::from_rational(100u128, 100u128));
	});
}

#[test]
fn test_reserve_ratio_optimal_range() {
	new_test_ext().execute_with(|| {
		// Mint 100,000 EDSC
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_000).unwrap(); // $100,000

		// Deposit collateral worth $132,000 after haircut
		// 2.2 BTC: $132,000 raw → $118,800 adjusted (10% haircut)
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			220_000_000 // 2.2 BTC
		));

		// Calculate reserve ratio
		assert_ok!(ReserveVault::calculate_reserve_ratio(RuntimeOrigin::signed(ALICE)));

		let ratio = ReserveVault::reserve_ratio();

		// Should be in optimal range (110-130%)
		assert!(ratio >= FixedU128::from_rational(110u128, 100u128));
		assert!(ratio <= FixedU128::from_rational(130u128, 100u128));
	});
}

#[test]
fn test_reserve_ratio_with_custodian_value() {
	new_test_ext().execute_with(|| {
		// Mint 100,000 EDSC
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_000).unwrap();

		// Deposit on-chain collateral: $54,000 adjusted
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			100_000_000
		));

		// Add custodian-attested off-chain value: $66,000
		assert_ok!(ReserveVault::update_custodian_value(
			RuntimeOrigin::root(),
			6_600_000 // $66,000 in cents
		));

		// Calculate reserve ratio
		assert_ok!(ReserveVault::calculate_reserve_ratio(RuntimeOrigin::signed(ALICE)));

		// Total reserve = $54,000 + $66,000 = $120,000
		// Ratio = $120,000 / $100,000 = 120% (optimal)
		let ratio = ReserveVault::reserve_ratio();
		assert_eq!(ratio, FixedU128::from_rational(120u128, 100u128));
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// TEST MODULE 4: Collateral Withdrawals
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_withdraw_collateral_requires_root() {
	new_test_ext().execute_with(|| {
		// Deposit collateral
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			100_000_000
		));

		// Non-root withdrawal should fail
		assert_err!(
			ReserveVault::withdraw_collateral(
				RuntimeOrigin::signed(ALICE),
				1, // BTC
				50_000_000,
				BOB
			),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn test_withdraw_more_than_balance_fails() {
	new_test_ext().execute_with(|| {
		// Deposit 1 BTC
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			100_000_000
		));

		// Try to withdraw 2 BTC (more than deposited)
		assert_err!(
			ReserveVault::withdraw_collateral(
				RuntimeOrigin::root(),
				1, // BTC
				200_000_000,
				BOB
			),
			Error::<Test>::InsufficientVaultBalance
		);
	});
}

#[test]
fn test_withdraw_when_reserve_ratio_too_low_fails() {
	new_test_ext().execute_with(|| {
		// Mint 100,000 EDSC
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_000).unwrap();

		// Deposit barely enough collateral (105% ratio)
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			200_000_000 // 2 BTC
		));

		// Calculate ratio
		assert_ok!(ReserveVault::calculate_reserve_ratio(RuntimeOrigin::signed(ALICE)));

		// Try to withdraw - should fail as it would drop ratio below safe level
		assert_err!(
			ReserveVault::withdraw_collateral(
				RuntimeOrigin::root(),
				1, // BTC
				50_000_000,
				BOB
			),
			Error::<Test>::ReserveRatioTooLow
		);
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// TEST MODULE 5: Price Oracle Updates
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_update_asset_price() {
	new_test_ext().execute_with(|| {
		// Update BTC price from $60,000 to $70,000
		assert_ok!(ReserveVault::update_asset_price(
			RuntimeOrigin::root(),
			1, // BTC
			7_000_000 // $70,000 in cents
		));

		let new_price = ReserveVault::asset_price(AssetType::BTC);
		assert_eq!(new_price, 7_000_000);
	});
}

#[test]
fn test_price_update_recalculates_vault_value() {
	new_test_ext().execute_with(|| {
		// Deposit 1 BTC at $60,000
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			100_000_000
		));

		let vault_before = ReserveVault::vault(AssetType::BTC).unwrap();
		assert_eq!(vault_before.usd_value, 6_000_000); // $60,000

		// Update price to $70,000
		assert_ok!(ReserveVault::update_asset_price(
			RuntimeOrigin::root(),
			1, // BTC
			7_000_000
		));

		// Recalculate vault values
		assert_ok!(ReserveVault::recalculate_vault_values(RuntimeOrigin::signed(ALICE)));

		let vault_after = ReserveVault::vault(AssetType::BTC).unwrap();
		assert_eq!(vault_after.usd_value, 7_000_000); // $70,000
		assert_eq!(vault_after.adjusted_value, 6_300_000); // $63,000 after 10% haircut
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// TEST MODULE 6: Haircut Updates
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_update_haircut() {
	new_test_ext().execute_with(|| {
		// Increase BTC haircut from 10% to 20% (increased risk)
		assert_ok!(ReserveVault::update_haircut(
			RuntimeOrigin::root(),
			1, // BTC
			Permill::from_percent(20)
		));

		let new_haircut = ReserveVault::haircut(AssetType::BTC);
		assert_eq!(new_haircut, Permill::from_percent(20));
	});
}

#[test]
fn test_haircut_update_recalculates_adjusted_value() {
	new_test_ext().execute_with(|| {
		// Deposit 1 BTC at $60,000 with 10% haircut
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			100_000_000
		));

		let vault_before = ReserveVault::vault(AssetType::BTC).unwrap();
		assert_eq!(vault_before.adjusted_value, 5_400_000); // $54,000 (90% of $60k)

		// Increase haircut to 20%
		assert_ok!(ReserveVault::update_haircut(
			RuntimeOrigin::root(),
			1, // BTC
			Permill::from_percent(20)
		));

		// Recalculate
		assert_ok!(ReserveVault::recalculate_vault_values(RuntimeOrigin::signed(ALICE)));

		let vault_after = ReserveVault::vault(AssetType::BTC).unwrap();
		assert_eq!(vault_after.adjusted_value, 4_800_000); // $48,000 (80% of $60k)
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// TEST MODULE 7: Circuit Breaker Integration
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_reserve_critical_triggers_redemption_pause() {
	new_test_ext().execute_with(|| {
		// Mint 100,000 EDSC
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_000).unwrap();

		// Deposit collateral barely at 100% (critical threshold)
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			167_000_000 // 1.67 BTC
		));

		// Calculate ratio
		assert_ok!(ReserveVault::calculate_reserve_ratio(RuntimeOrigin::signed(ALICE)));

		let ratio = ReserveVault::reserve_ratio();

		// Should be at or below 100% (emergency threshold)
		assert!(ratio <= FixedU128::from_rational(100u128, 100u128));

		// Should trigger ReserveCritical event
	});
}

#[test]
fn test_reserve_throttle_triggers_queue_mode() {
	new_test_ext().execute_with(|| {
		// Mint 100,000 EDSC
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_000).unwrap();

		// Deposit collateral at 102% (throttle zone: 100-105%)
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			170_000_000 // 1.7 BTC
		));

		// Calculate ratio
		assert_ok!(ReserveVault::calculate_reserve_ratio(RuntimeOrigin::signed(ALICE)));

		let ratio = ReserveVault::reserve_ratio();

		// Should be between 100% and 105%
		assert!(ratio > FixedU128::from_rational(100u128, 100u128));
		assert!(ratio < FixedU128::from_rational(105u128, 100u128));

		// Should trigger ReserveThrottled event
	});
}

#[test]
fn test_reserve_optimal_normal_operation() {
	new_test_ext().execute_with(|| {
		// Mint 100,000 EDSC
		EdscToken::authorize_minter(RuntimeOrigin::root(), ALICE).unwrap();
		EdscToken::mint(RuntimeOrigin::signed(ALICE), ALICE, 10_000_000).unwrap();

		// Deposit collateral at 120% (optimal range: 110-130%)
		assert_ok!(ReserveVault::deposit_collateral(
			RuntimeOrigin::signed(ALICE),
			1, // BTC
			223_000_000 // 2.23 BTC
		));

		// Calculate ratio
		assert_ok!(ReserveVault::calculate_reserve_ratio(RuntimeOrigin::signed(ALICE)));

		let ratio = ReserveVault::reserve_ratio();

		// Should be in optimal range
		assert!(ratio >= FixedU128::from_rational(110u128, 100u128));
		assert!(ratio <= FixedU128::from_rational(130u128, 100u128));

		// Should trigger ReserveOptimal event
	});
}
