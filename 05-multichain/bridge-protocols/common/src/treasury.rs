//! Treasury Integration for Cross-Chain Bridge Fees
//!
//! This module provides a common interface for routing cross-chain bridge fees
//! to the Ëtrid treasury pallet.
//!
//! ## Fee Routing Model
//!
//! All bridge protocols follow the same fee split model:
//! - **10% → Treasury** via `pallet-treasury`
//! - **90% → Validator Pool** for incentivizing bridge operations
//!
//! ## Security Considerations
//!
//! - Bridge fee collection happens during withdrawal operations
//! - Fees are deducted from the user's withdrawal amount
//! - Treasury receives fees via the `receive_cross_chain_fees()` method
//! - All fee transfers use safe arithmetic (saturating operations)
//! - Fee events are emitted for transparency and auditing

use frame_support::dispatch::DispatchResult;

/// Treasury interface for receiving cross-chain bridge fees
///
/// This trait must be implemented by the runtime configuration to connect
/// bridge pallets to `pallet-treasury`.
///
/// # Type Parameters
///
/// * `AccountId` - The account identifier type
/// * `Balance` - The balance type for fee amounts
///
/// # Example Implementation
///
/// ```ignore
/// impl TreasuryInterface<AccountId, Balance> for pallet_treasury::Pallet<Runtime> {
///     fn receive_cross_chain_fees(amount: Balance) -> DispatchResult {
///         // Call pallet-treasury's receive_cross_chain_fees method
///         pallet_treasury::Pallet::<Runtime>::receive_cross_chain_fees(amount)
///     }
/// }
/// ```
pub trait TreasuryInterface<AccountId, Balance> {
    /// Receive cross-chain bridge fees and route to treasury
    ///
    /// This method is called by bridge pallets when fees are collected
    /// from cross-chain operations (deposits/withdrawals).
    ///
    /// # Arguments
    ///
    /// * `amount` - The fee amount to be sent to the treasury (10% of total bridge fee)
    ///
    /// # Returns
    ///
    /// * `DispatchResult` - Ok(()) on success, Error on failure
    ///
    /// # Implementation Notes
    ///
    /// - Should call `pallet_treasury::Pallet::receive_cross_chain_fees()`
    /// - Fees are already denominated in ËTR (conversion happens in bridge)
    /// - Treasury tracks funding sources via FundingSource::CrossChainFees
    fn receive_cross_chain_fees(amount: Balance) -> DispatchResult;
}
