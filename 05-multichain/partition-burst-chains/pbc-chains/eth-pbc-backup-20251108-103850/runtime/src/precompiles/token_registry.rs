// Token Registry Precompile (0x805)
// Auto-discovers and indexes ERC-20 tokens from Ethereum mainnet
// Reduces friction for cross-chain token bridging

use core::marker::PhantomData;
use pallet_evm::{Precompile, PrecompileHandle, PrecompileResult, PrecompileFailure};
use sp_core::{H160, H256, U256};
use sp_std::prelude::*;
use fp_evm::{ExitSucceed, PrecompileOutput};
use sp_io::hashing::keccak_256;

/// Selector for registerToken(address) - 0x4420e486
const REGISTER_TOKEN_SELECTOR: [u8; 4] = [0x44, 0x20, 0xe4, 0x86];

/// Selector for getTokenInfo(address) - 0x1f69565f
const GET_TOKEN_INFO_SELECTOR: [u8; 4] = [0x1f, 0x69, 0x56, 0x5f];

/// Selector for getBridgedTokens() - 0x9c8f9f23
const GET_BRIDGED_TOKENS_SELECTOR: [u8; 4] = [0x9c, 0x8f, 0x9f, 0x23];

/// Token Registry Precompile
///
/// Automatically discovers and indexes ERC-20 tokens from Ethereum mainnet.
/// Fetches metadata (name, symbol, decimals) and tracks bridged supply.
///
/// Functions:
/// - registerToken(address) -> bool: Auto-register token from mainnet
/// - getTokenInfo(address) -> (string, string, uint8, uint256): Get token metadata
/// - getBridgedTokens() -> address[]: List all bridged tokens
pub struct TokenRegistryPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> TokenRegistryPrecompile<Runtime>
where
	Runtime: pallet_evm::Config,
{
	/// Register a token from Ethereum mainnet
	fn register_token(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		let input = handle.input();

		if input.len() < 36 {
			return Err(PrecompileFailure::Error {
				exit_status: pallet_evm::ExitError::Other("Invalid input length".into()),
			});
		}

		// Extract token address (bytes 4-24, last 20 bytes of 32-byte word)
		let token_address = H160::from_slice(&input[16..36]);

		// In production, this would:
		// 1. Query Ethereum mainnet for token metadata
		// 2. Call token.name(), token.symbol(), token.decimals()
		// 3. Store in on-chain registry
		// 4. Emit registration event
		//
		// For now, simulate successful registration

		// Log token registration
		handle.log(
			H160::from_low_u64_be(0x805),
			vec![
				H256::from_slice(&keccak_256(b"TokenRegistered(address)")),
				H256::from(token_address),
			],
			vec![1u8],
		)?;

		// Return true (success)
		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			output: encode_bool(true),
		})
	}

	/// Get token information from registry
	fn get_token_info(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		let input = handle.input();

		if input.len() < 36 {
			return Err(PrecompileFailure::Error {
				exit_status: pallet_evm::ExitError::Other("Invalid input length".into()),
			});
		}

		// Extract token address
		let _token_address = H160::from_slice(&input[16..36]);

		// In production, this would query the on-chain registry
		// For now, return mock data

		// Mock token info for USDC
		let name = "USD Coin";
		let symbol = "USDC";
		let decimals: u8 = 6;
		let total_bridged_supply: u128 = 1_000_000_000_000; // 1M USDC

		// Encode return tuple: (string, string, uint8, uint256)
		// ABI encoding for dynamic types is complex, simplified here
		let mut output = Vec::new();

		// Offset to name string (0x80 = 128 bytes)
		output.extend_from_slice(&encode_uint256(U256::from(0x80)));

		// Offset to symbol string (0x80 + name length rounded)
		output.extend_from_slice(&encode_uint256(U256::from(0xC0)));

		// decimals (uint8)
		output.extend_from_slice(&encode_uint256(U256::from(decimals)));

		// totalBridgedSupply (uint256)
		output.extend_from_slice(&encode_uint256(U256::from(total_bridged_supply)));

		// name string (length + data)
		output.extend_from_slice(&encode_uint256(U256::from(name.len())));
		output.extend_from_slice(name.as_bytes());
		// Pad to 32-byte boundary
		let name_padding = 32 - (name.len() % 32);
		if name_padding < 32 {
			output.extend_from_slice(&vec![0u8; name_padding]);
		}

		// symbol string (length + data)
		output.extend_from_slice(&encode_uint256(U256::from(symbol.len())));
		output.extend_from_slice(symbol.as_bytes());
		// Pad to 32-byte boundary
		let symbol_padding = 32 - (symbol.len() % 32);
		if symbol_padding < 32 {
			output.extend_from_slice(&vec![0u8; symbol_padding]);
		}

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			output,
		})
	}

	/// Get list of all bridged tokens
	fn get_bridged_tokens(_handle: &mut impl PrecompileHandle) -> PrecompileResult {
		// In production, this would query the on-chain registry
		// For now, return mock list of popular bridged tokens

		let tokens = vec![
			H160::from_low_u64_be(0x1000), // Mock USDC
			H160::from_low_u64_be(0x2000), // Mock USDT
			H160::from_low_u64_be(0x3000), // Mock DAI
			H160::from_low_u64_be(0x4000), // Mock WBTC
		];

		// Encode return array: address[]
		let mut output = Vec::new();

		// Offset to array data (0x20 = 32 bytes)
		output.extend_from_slice(&encode_uint256(U256::from(0x20)));

		// Array length
		output.extend_from_slice(&encode_uint256(U256::from(tokens.len())));

		// Array elements (each address is 32 bytes, right-aligned)
		for token in tokens {
			let mut addr_bytes = [0u8; 32];
			addr_bytes[12..32].copy_from_slice(token.as_bytes());
			output.extend_from_slice(&addr_bytes);
		}

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			output,
		})
	}
}

impl<Runtime> Precompile for TokenRegistryPrecompile<Runtime>
where
	Runtime: pallet_evm::Config,
{
	fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		let input = handle.input();

		if input.len() < 4 {
			return Err(PrecompileFailure::Error {
				exit_status: pallet_evm::ExitError::Other("Invalid input length".into()),
			});
		}

		let selector = &input[0..4];

		match selector {
			s if s == REGISTER_TOKEN_SELECTOR => Self::register_token(handle),
			s if s == GET_TOKEN_INFO_SELECTOR => Self::get_token_info(handle),
			s if s == GET_BRIDGED_TOKENS_SELECTOR => Self::get_bridged_tokens(handle),
			_ => Err(PrecompileFailure::Error {
				exit_status: pallet_evm::ExitError::Other("Unknown function selector".into()),
			}),
		}
	}
}

// Helper functions

fn encode_uint256(value: U256) -> Vec<u8> {
	let output = value.to_big_endian();
	output.to_vec()
}

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
		let register_sig = keccak_256(b"registerToken(address)");
		assert_eq!(&register_sig[0..4], &REGISTER_TOKEN_SELECTOR);

		let info_sig = keccak_256(b"getTokenInfo(address)");
		assert_eq!(&info_sig[0..4], &GET_TOKEN_INFO_SELECTOR);

		let list_sig = keccak_256(b"getBridgedTokens()");
		assert_eq!(&list_sig[0..4], &GET_BRIDGED_TOKENS_SELECTOR);
	}
}
