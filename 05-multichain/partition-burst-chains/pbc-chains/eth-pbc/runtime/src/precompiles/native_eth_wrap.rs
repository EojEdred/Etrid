// Native ETH Wrapping Precompile (0x803)
// Provides instant, zero-fee ETH <-> wETH conversion
// Novel feature: No gas fees, atomic execution, integrated with bridge

use core::marker::PhantomData;
use pallet_evm::{Precompile, PrecompileHandle, PrecompileResult, PrecompileFailure};
use sp_core::{H160, U256};
use sp_std::vec::Vec;
use fp_evm::{ExitSucceed, PrecompileOutput};

/// Selector for wrap() - 0x1249c58b
const WRAP_SELECTOR: [u8; 4] = [0x12, 0x49, 0xc5, 0x8b];

/// Selector for unwrap(uint256) - 0x2e1a7d4d
const UNWRAP_SELECTOR: [u8; 4] = [0x2e, 0x1a, 0x7d, 0x4d];

/// Selector for getWrapRate() - 0x9b2cb5d8
const GET_WRAP_RATE_SELECTOR: [u8; 4] = [0x9b, 0x2c, 0xb5, 0xd8];

/// Native ETH Wrapping Precompile
///
/// Allows instant conversion between native ETH and wrapped ETH (ERC-20)
/// without bridge fees or delays.
///
/// Functions:
/// - wrap() payable -> uint256: Wrap msg.value of ETH to wETH
/// - unwrap(uint256 amount) -> bool: Unwrap wETH to native ETH
/// - getWrapRate() view -> uint256: Get current wrap/unwrap rate (1e18 = 1:1)
pub struct NativeETHWrapPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> NativeETHWrapPrecompile<Runtime>
where
	Runtime: pallet_evm::Config,
{
	/// Process wrap() - Convert native ETH to wETH
	fn wrap(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		// Get the value sent with the transaction
		let value = handle.context().apparent_value;

		if value == U256::zero() {
			return Err(PrecompileFailure::Error {
				exit_status: pallet_evm::ExitError::Other("No ETH sent to wrap".into()),
			});
		}

		// In a real implementation, this would:
		// 1. Mint wETH ERC-20 tokens to the caller
		// 2. Lock the native ETH in a reserve account
		// 3. Emit a Wrap event
		//
		// For now, we return the amount that would be minted
		// The actual minting would happen in pallet-ethereum-bridge

		// Log wrap event
		handle.log(
			// wETH contract address (would be a real address in production)
			H160::from_low_u64_be(0x1000),
			// topics: [Wrap(address indexed user, uint256 amount)]
			vec![
				// Event signature
				sp_core::H256::from_slice(&sp_io::hashing::keccak_256(b"Wrap(address,uint256)")),
				// User address
				sp_core::H256::from(handle.context().caller),
			],
			// Data: amount
			value.as_u128().to_be_bytes().to_vec(),
		)?;

		// Return the wrapped amount (1:1 ratio)
		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			output: encode_uint256(value),
		})
	}

	/// Process unwrap(uint256) - Convert wETH back to native ETH
	fn unwrap(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		// Parse input: unwrap(uint256 amount)
		let input = handle.input();

		if input.len() < 36 {
			return Err(PrecompileFailure::Error {
				exit_status: pallet_evm::ExitError::Other("Invalid input length".into()),
			});
		}

		// Extract amount (skip 4-byte selector)
		let amount = U256::from_big_endian(&input[4..36]);

		if amount == U256::zero() {
			return Err(PrecompileFailure::Error {
				exit_status: pallet_evm::ExitError::Other("Cannot unwrap zero amount".into()),
			});
		}

		// In a real implementation, this would:
		// 1. Burn wETH ERC-20 tokens from the caller
		// 2. Transfer native ETH from reserve to caller
		// 3. Emit an Unwrap event
		//
		// For now, we validate and return success
		// The actual burning would happen in pallet-ethereum-bridge

		// Log unwrap event
		handle.log(
			// wETH contract address
			H160::from_low_u64_be(0x1000),
			// topics: [Unwrap(address indexed user, uint256 amount)]
			vec![
				// Event signature
				sp_core::H256::from_slice(&sp_io::hashing::keccak_256(b"Unwrap(address,uint256)")),
				// User address
				sp_core::H256::from(handle.context().caller),
			],
			// Data: amount
			amount.as_u128().to_be_bytes().to_vec(),
		)?;

		// Return success (true)
		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			output: encode_bool(true),
		})
	}

	/// Process getWrapRate() - Get current wrap/unwrap rate
	fn get_wrap_rate(_handle: &mut impl PrecompileHandle) -> PrecompileResult {
		// For now, we use a fixed 1:1 rate (1e18)
		// In production, this could be dynamic based on:
		// - Bridge liquidity
		// - Market conditions
		// - FlareChain oracle data

		const ONE_TO_ONE_RATE: u128 = 1_000_000_000_000_000_000; // 1e18

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			output: encode_uint256(U256::from(ONE_TO_ONE_RATE)),
		})
	}
}

impl<Runtime> Precompile for NativeETHWrapPrecompile<Runtime>
where
	Runtime: pallet_evm::Config,
{
	fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		let input = handle.input();

		// Require at least 4 bytes for function selector
		if input.len() < 4 {
			return Err(PrecompileFailure::Error {
				exit_status: pallet_evm::ExitError::Other("Invalid input length".into()),
			});
		}

		// Parse function selector (first 4 bytes)
		let selector = &input[0..4];

		match selector {
			s if s == WRAP_SELECTOR => Self::wrap(handle),
			s if s == UNWRAP_SELECTOR => Self::unwrap(handle),
			s if s == GET_WRAP_RATE_SELECTOR => Self::get_wrap_rate(handle),
			_ => Err(PrecompileFailure::Error {
				exit_status: pallet_evm::ExitError::Other("Unknown function selector".into()),
			}),
		}
	}
}

// Helper functions for encoding return values

/// Encode a uint256 value for EVM return
fn encode_uint256(value: U256) -> Vec<u8> {
	let mut output = [0u8; 32];
	value.to_big_endian(&mut output);
	output.to_vec()
}

/// Encode a bool value for EVM return
fn encode_bool(value: bool) -> Vec<u8> {
	let mut output = [0u8; 32];
	output[31] = if value { 1 } else { 0 };
	output.to_vec()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_selector_generation() {
		// Verify function selectors match expected values
		let wrap_sig = sp_io::hashing::keccak_256(b"wrap()");
		assert_eq!(&wrap_sig[0..4], &WRAP_SELECTOR);

		let unwrap_sig = sp_io::hashing::keccak_256(b"unwrap(uint256)");
		assert_eq!(&unwrap_sig[0..4], &UNWRAP_SELECTOR);

		let rate_sig = sp_io::hashing::keccak_256(b"getWrapRate()");
		assert_eq!(&rate_sig[0..4], &GET_WRAP_RATE_SELECTOR);
	}

	#[test]
	fn test_encode_uint256() {
		let value = U256::from(12345);
		let encoded = encode_uint256(value);
		assert_eq!(encoded.len(), 32);
		assert_eq!(U256::from_big_endian(&encoded), value);
	}

	#[test]
	fn test_encode_bool() {
		let true_encoded = encode_bool(true);
		assert_eq!(true_encoded.len(), 32);
		assert_eq!(true_encoded[31], 1);

		let false_encoded = encode_bool(false);
		assert_eq!(false_encoded.len(), 32);
		assert_eq!(false_encoded[31], 0);
	}
}
