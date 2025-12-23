//! Fee calculation and collection for bridge operations
//!
//! This module handles fee calculation, deduction, and collection for cross-chain
//! bridge operations. Fees are calculated based on a configurable percentage of
//! the transfer amount, with a minimum fee floor to prevent dust.

use frame_support::{
	dispatch::DispatchResult,
	traits::{Currency, ExistenceRequirement},
};
use sp_runtime::traits::{CheckedMul, CheckedDiv, CheckedSub, Saturating};
use sp_std::cmp::max;

/// Fee calculation utilities
pub struct FeeCalculator;

impl FeeCalculator {
	/// Calculate bridge fee based on amount and fee rate
	///
	/// # Arguments
	/// * `amount` - The transfer amount (with full precision)
	/// * `fee_rate_bps` - Fee rate in basis points (100 = 1%, 10000 = 100%)
	/// * `min_fee` - Minimum fee amount
	///
	/// # Returns
	/// The calculated fee amount
	///
	/// # Examples
	/// ```ignore
	/// // 1% fee on 10000 tokens = 100 tokens
	/// let fee = FeeCalculator::calculate_fee(10000, 100, 10);
	/// assert_eq!(fee, 100);
	///
	/// // Below minimum: 1% of 500 = 5, but min is 10
	/// let fee = FeeCalculator::calculate_fee(500, 100, 10);
	/// assert_eq!(fee, 10);
	/// ```
	pub fn calculate_fee(amount: u128, fee_rate_bps: u32, min_fee: u128) -> u128 {
		// Calculate fee: amount * fee_rate / 10000
		let fee = amount
			.checked_mul(fee_rate_bps as u128)
			.and_then(|result| result.checked_div(10_000))
			.unwrap_or(0);

		// Apply minimum fee
		max(fee, min_fee)
	}

	/// Calculate the net amount after deducting fees
	///
	/// # Arguments
	/// * `amount` - The gross transfer amount
	/// * `fee` - The calculated fee
	///
	/// # Returns
	/// * `Some(net_amount)` if subtraction doesn't overflow
	/// * `None` if fee exceeds amount
	pub fn net_amount(amount: u128, fee: u128) -> Option<u128> {
		amount.checked_sub(fee)
	}

	/// Validate that fee doesn't exceed amount
	pub fn validate_fee(amount: u128, fee: u128) -> bool {
		fee < amount
	}

	/// Calculate effective fee rate (for display/logging)
	///
	/// Returns the actual fee rate in basis points (100 = 1%)
	pub fn effective_fee_rate(amount: u128, fee: u128) -> u32 {
		if amount == 0 {
			return 0;
		}

		fee
			.checked_mul(10_000)
			.and_then(|result| result.checked_div(amount))
			.and_then(|rate| u32::try_from(rate).ok())
			.unwrap_or(0)
	}
}

/// Fee deduction helper for native currency
///
/// This helper handles the actual transfer of fees from the user
/// to the fee collector account using the pallet's configured currency.
pub struct FeeDeductor<T: frame_system::Config, Currency> {
	_phantom: sp_std::marker::PhantomData<(T, Currency)>,
}

impl<T, C> FeeDeductor<T, C>
where
	T: frame_system::Config,
	C: Currency<T::AccountId>,
{
	/// Deduct fee from sender and transfer to fee collector
	///
	/// # Arguments
	/// * `sender` - The account paying the fee
	/// * `collector` - The fee collector account
	/// * `fee_amount` - The fee amount to transfer
	///
	/// # Returns
	/// * `Ok(())` if fee was successfully transferred
	/// * `Err(_)` if sender has insufficient balance or transfer fails
	pub fn deduct_and_transfer(
		sender: &T::AccountId,
		collector: &T::AccountId,
		fee_amount: C::Balance,
	) -> DispatchResult {
		// Transfer fee from sender to collector
		// Using ExistenceRequirement::KeepAlive to ensure sender account stays alive
		C::transfer(
			sender,
			collector,
			fee_amount,
			ExistenceRequirement::KeepAlive,
		)
	}

	/// Check if account has sufficient balance for fee
	pub fn can_pay_fee(account: &T::AccountId, fee_amount: C::Balance) -> bool {
		C::free_balance(account) >= fee_amount
	}

	/// Get free balance of an account
	pub fn balance_of(account: &T::AccountId) -> C::Balance {
		C::free_balance(account)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_calculate_fee_basic() {
		// 1% of 10000 = 100
		let fee = FeeCalculator::calculate_fee(10_000, 100, 10);
		assert_eq!(fee, 100);

		// 0.5% of 10000 = 50
		let fee = FeeCalculator::calculate_fee(10_000, 50, 10);
		assert_eq!(fee, 50);

		// 10% of 10000 = 1000
		let fee = FeeCalculator::calculate_fee(10_000, 1_000, 10);
		assert_eq!(fee, 1_000);
	}

	#[test]
	fn test_calculate_fee_minimum() {
		// 1% of 500 = 5, but min is 10
		let fee = FeeCalculator::calculate_fee(500, 100, 10);
		assert_eq!(fee, 10);

		// 1% of 100 = 1, but min is 50
		let fee = FeeCalculator::calculate_fee(100, 100, 50);
		assert_eq!(fee, 50);
	}

	#[test]
	fn test_calculate_fee_zero_rate() {
		// 0% fee rate
		let fee = FeeCalculator::calculate_fee(10_000, 0, 10);
		assert_eq!(fee, 10); // Should return minimum
	}

	#[test]
	fn test_calculate_fee_large_amounts() {
		// Test with large amounts (1 billion tokens)
		let amount = 1_000_000_000_000_000_000u128; // 1B with 18 decimals
		let fee = FeeCalculator::calculate_fee(amount, 100, 1000); // 1%
		let expected = 10_000_000_000_000_000u128; // 10M with 18 decimals
		assert_eq!(fee, expected);
	}

	#[test]
	fn test_net_amount() {
		// Normal case
		let net = FeeCalculator::net_amount(10_000, 100);
		assert_eq!(net, Some(9_900));

		// Fee equals amount (should fail - must be less than)
		let net = FeeCalculator::net_amount(100, 100);
		assert_eq!(net, Some(0));

		// Fee exceeds amount
		let net = FeeCalculator::net_amount(100, 150);
		assert_eq!(net, None);
	}

	#[test]
	fn test_validate_fee() {
		assert!(FeeCalculator::validate_fee(10_000, 100));
		assert!(FeeCalculator::validate_fee(10_000, 9_999));
		assert!(!FeeCalculator::validate_fee(100, 100)); // Equal not valid
		assert!(!FeeCalculator::validate_fee(100, 150)); // Exceeds not valid
	}

	#[test]
	fn test_effective_fee_rate() {
		// 100 fee on 10000 amount = 1% = 100 bps
		let rate = FeeCalculator::effective_fee_rate(10_000, 100);
		assert_eq!(rate, 100);

		// 50 fee on 10000 amount = 0.5% = 50 bps
		let rate = FeeCalculator::effective_fee_rate(10_000, 50);
		assert_eq!(rate, 50);

		// 1000 fee on 10000 amount = 10% = 1000 bps
		let rate = FeeCalculator::effective_fee_rate(10_000, 1_000);
		assert_eq!(rate, 1_000);

		// Zero amount
		let rate = FeeCalculator::effective_fee_rate(0, 100);
		assert_eq!(rate, 0);
	}

	#[test]
	fn test_fee_precision() {
		// Test precision with fractional percentages
		// 0.01% (1 bps) of 1,000,000 = 100
		let fee = FeeCalculator::calculate_fee(1_000_000, 1, 1);
		assert_eq!(fee, 100);

		// 0.25% (25 bps) of 1,000,000 = 2,500
		let fee = FeeCalculator::calculate_fee(1_000_000, 25, 1);
		assert_eq!(fee, 2_500);
	}
}
