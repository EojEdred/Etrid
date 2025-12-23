//! Token Operations Implementation
//!
//! This module provides a concrete implementation of the `TokenOperations` trait
//! that integrates with Substrate's native balance system (pallet-balances).
//!
//! The implementation uses the `Currency` trait to perform burn (withdraw) and
//! mint (deposit) operations on native tokens.

use frame_support::{
	dispatch::DispatchResult,
	traits::{Currency, ExistenceRequirement, WithdrawReasons},
};
use sp_runtime::traits::SaturatedConversion;
use sp_std::marker::PhantomData;

/// Balance type for token operations
pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Configuration trait for token operations
///
/// This trait must be implemented by any runtime that uses the token operations.
pub trait Config: frame_system::Config {
	/// The currency type for native balance operations
	type Currency: Currency<Self::AccountId>;
}

/// Token operations implementation using pallet-balances
///
/// This struct provides burn and mint operations by wrapping Substrate's
/// Currency trait. It converts between u128 amounts (used in cross-chain
/// messages) and the runtime's Balance type.
///
/// # Type Parameters
/// * `T` - Runtime configuration that implements `Config` and `pallet_balances::Config`
///
/// # Examples
///
/// ```rust,ignore
/// // In your PBC runtime configuration:
/// impl pallet_token_messenger::Config for Runtime {
///     type RuntimeEvent = RuntimeEvent;
///     type TokenOperations = pallet_token_messenger::token_ops::BalancesTokenOps<Runtime>;
///     // ... other config items
/// }
/// ```
pub struct BalancesTokenOps<T>(PhantomData<T>);

impl<T: Config> crate::TokenOperations<T::AccountId> for BalancesTokenOps<T> {
	/// Burn tokens from an account
	///
	/// This operation withdraws tokens from the account, effectively burning them
	/// from the source chain. The tokens are removed from circulation on this chain
	/// and will be minted on the destination chain.
	///
	/// # Arguments
	/// * `account` - The account to burn tokens from
	/// * `amount` - The amount to burn in smallest units (e.g., 10^18 for 1 token with 18 decimals)
	///
	/// # Returns
	/// * `Ok(())` if burn succeeds
	/// * `Err(_)` if insufficient balance or other error
	///
	/// # Implementation Details
	/// - Converts u128 amount to Balance type
	/// - Uses `withdraw` with `ExistenceRequirement::AllowDeath` (account may be reaped if balance reaches zero)
	/// - Handles overflow by using saturated conversion
	fn burn_tokens(account: &T::AccountId, amount: u128) -> DispatchResult {
		// Convert u128 to Balance type
		let balance_amount: BalanceOf<T> = amount.saturated_into();

		// Withdraw (burn) tokens from the account
		// ExistenceRequirement::AllowDeath: Allow account to be reaped if balance reaches zero
		// WithdrawReasons::all(): Allow withdrawal for any reason
		T::Currency::withdraw(
			account,
			balance_amount,
			WithdrawReasons::all(),
			ExistenceRequirement::AllowDeath,
		)
		.map(|_| ()) // Discard the imbalance, converting result to ()
	}

	/// Mint tokens to an account
	///
	/// This operation deposits tokens to the account, effectively minting them
	/// on the destination chain. These tokens were burned on the source chain
	/// and are being recreated here.
	///
	/// # Arguments
	/// * `account` - The account to mint tokens to
	/// * `amount` - The amount to mint in smallest units
	///
	/// # Returns
	/// * `Ok(())` if mint succeeds
	/// * `Err(_)` if minting fails (e.g., overflow)
	///
	/// # Implementation Details
	/// - Converts u128 amount to Balance type
	/// - Uses `deposit_creating` which creates account if it doesn't exist
	/// - Handles overflow by using saturated conversion
	fn mint_tokens(account: &T::AccountId, amount: u128) -> DispatchResult {
		// Convert u128 to Balance type
		let balance_amount: BalanceOf<T> = amount.saturated_into();

		// Deposit (mint) tokens to the account
		// deposit_creating: Creates the account if it doesn't exist
		T::Currency::deposit_creating(account, balance_amount);

		Ok(())
	}

	/// Get the balance of an account
	///
	/// Returns the free balance (spendable balance) of the account in u128 format
	/// for use in cross-chain messaging.
	///
	/// # Arguments
	/// * `account` - The account to query
	///
	/// # Returns
	/// The account's free balance as u128
	///
	/// # Implementation Details
	/// - Queries free_balance (excludes reserved balance)
	/// - Converts Balance type to u128 using saturated conversion
	fn balance_of(account: &T::AccountId) -> u128 {
		// Get free balance (excludes reserved balance)
		let balance = <T::Currency as Currency<T::AccountId>>::free_balance(account);

		// Convert Balance to u128
		balance.saturated_into()
	}
}

/// Helper struct for easier integration in PBC runtimes
///
/// This is a convenience type that can be used directly in runtime configurations
/// without needing to specify the full generic parameters.
///
/// # Example Usage
///
/// ```rust,ignore
/// impl pallet_token_messenger::Config for Runtime {
///     type RuntimeEvent = RuntimeEvent;
///     type TokenOperations = PbcTokenOperations<Runtime>;
///     type AttestationVerifier = pallet_bridge_attestation::Pallet<Runtime>;
///     type WeightInfo = pallet_token_messenger::weights::SubstrateWeight<Runtime>;
///     type MaxMessageBodySize = ConstU32<512>;
///     type MaxBurnAmount = ConstU128<{ 1_000_000 * EDSC }>;
///     type DailyBurnCap = ConstU128<{ 10_000_000 * EDSC }>;
///     type MinBurnAmount = ConstU128<{ EDSC / 100 }>; // 0.01 tokens
///     type MessageTimeout = ConstU32<14400>; // ~24 hours
///     type BlocksPerDay = ConstU32<14400>; // 14400 blocks * 6s = 24h
///     type LocalDomain = PbcDomainId; // Set to specific PBC domain (e.g., 100 for EDSC)
/// }
/// ```
pub struct PbcTokenOperations<T>(PhantomData<T>);

impl<T> crate::TokenOperations<T::AccountId> for PbcTokenOperations<T>
where
	T: frame_system::Config + pallet_balances::Config,
{
	/// Burn tokens from an account using pallet-balances
	///
	/// # Arguments
	/// * `from` - The account to burn from
	/// * `amount` - Amount to burn (u128 with full precision)
	///
	/// # Returns
	/// * `Ok(())` on success
	/// * `Err(_)` on failure (insufficient balance, etc.)
	fn burn_tokens(from: &T::AccountId, amount: u128) -> DispatchResult {
		// Convert u128 to Balance type
		let balance_amount: <T as pallet_balances::Config>::Balance = amount.saturated_into();

		// Use Currency trait to withdraw (burn)
		<pallet_balances::Pallet<T> as Currency<T::AccountId>>::withdraw(
			from,
			balance_amount,
			WithdrawReasons::all(),
			ExistenceRequirement::AllowDeath,
		)
		.map(|_| ()) // Discard imbalance
	}

	/// Mint tokens to an account using pallet-balances
	///
	/// # Arguments
	/// * `to` - The account to mint to
	/// * `amount` - Amount to mint (u128 with full precision)
	///
	/// # Returns
	/// * `Ok(())` on success (always succeeds unless overflow)
	fn mint_tokens(to: &T::AccountId, amount: u128) -> DispatchResult {
		// Convert u128 to Balance type
		let balance_amount: <T as pallet_balances::Config>::Balance = amount.saturated_into();

		// Use Currency trait to deposit (mint)
		<pallet_balances::Pallet<T> as Currency<T::AccountId>>::deposit_creating(
			to,
			balance_amount,
		);

		Ok(())
	}

	/// Get balance of an account
	///
	/// # Arguments
	/// * `account` - The account to query
	///
	/// # Returns
	/// The free balance as u128
	fn balance_of(account: &T::AccountId) -> u128 {
		// Get free balance from pallet-balances
		let balance = <pallet_balances::Pallet<T> as Currency<T::AccountId>>::free_balance(account);

		// Convert to u128
		balance.saturated_into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use frame_support::{
		assert_ok, parameter_types,
		traits::{ConstU32, ConstU64},
	};
	use sp_core::H256;
	use sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup},
	};

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
	type Block = frame_system::mocking::MockBlock<Test>;

	// Configure a mock runtime for testing
	frame_support::construct_runtime!(
		pub enum Test where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system,
			Balances: pallet_balances,
		}
	);

	parameter_types! {
		pub const BlockHashCount: u64 = 250;
	}

	impl frame_system::Config for Test {
		type BaseCallFilter = frame_support::traits::Everything;
		type BlockWeights = ();
		type BlockLength = ();
		type DbWeight = ();
		type RuntimeOrigin = RuntimeOrigin;
		type RuntimeCall = RuntimeCall;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type RuntimeEvent = RuntimeEvent;
		type BlockHashCount = BlockHashCount;
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = pallet_balances::AccountData<u128>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
		type MaxConsumers = ConstU32<16>;
	}

	parameter_types! {
		pub const ExistentialDeposit: u128 = 1;
	}

	impl pallet_balances::Config for Test {
		type MaxLocks = ();
		type MaxReserves = ();
		type ReserveIdentifier = [u8; 8];
		type Balance = u128;
		type RuntimeEvent = RuntimeEvent;
		type DustRemoval = ();
		type ExistentialDeposit = ExistentialDeposit;
		type AccountStore = System;
		type WeightInfo = ();
		type FreezeIdentifier = ();
		type MaxFreezes = ();
		type RuntimeHoldReason = ();
		type MaxHolds = ();
	}

	#[test]
	fn test_burn_tokens_works() {
		sp_io::TestExternalities::new_empty().execute_with(|| {
			let account = 1u64;
			let amount = 1000u128;

			// Give account some balance
			<pallet_balances::Pallet<Test> as Currency<u64>>::deposit_creating(&account, amount);

			// Burn tokens
			assert_ok!(PbcTokenOperations::<Test>::burn_tokens(&account, 500));

			// Check remaining balance
			assert_eq!(PbcTokenOperations::<Test>::balance_of(&account), 500);
		});
	}

	#[test]
	fn test_mint_tokens_works() {
		sp_io::TestExternalities::new_empty().execute_with(|| {
			let account = 1u64;
			let amount = 1000u128;

			// Mint tokens
			assert_ok!(PbcTokenOperations::<Test>::mint_tokens(&account, amount));

			// Check balance
			assert_eq!(PbcTokenOperations::<Test>::balance_of(&account), amount);
		});
	}

	#[test]
	fn test_balance_of_works() {
		sp_io::TestExternalities::new_empty().execute_with(|| {
			let account = 1u64;
			let amount = 1000u128;

			// Initially zero
			assert_eq!(PbcTokenOperations::<Test>::balance_of(&account), 0);

			// Mint some tokens
			<pallet_balances::Pallet<Test> as Currency<u64>>::deposit_creating(&account, amount);

			// Check balance
			assert_eq!(PbcTokenOperations::<Test>::balance_of(&account), amount);
		});
	}

	#[test]
	fn test_burn_more_than_balance_fails() {
		sp_io::TestExternalities::new_empty().execute_with(|| {
			let account = 1u64;
			let amount = 100u128;

			// Give account some balance
			<pallet_balances::Pallet<Test> as Currency<u64>>::deposit_creating(&account, amount);

			// Try to burn more than balance - should fail
			assert!(PbcTokenOperations::<Test>::burn_tokens(&account, 200).is_err());
		});
	}
}
