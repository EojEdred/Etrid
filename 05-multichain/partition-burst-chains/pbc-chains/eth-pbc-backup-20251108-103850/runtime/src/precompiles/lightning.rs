// Lightning Bloc Channels Precompile for ETH-PBC
// Address: 0x0000000000000000000000000000000000000808 (0x808)
//
// Provides Solidity interface to Lightning payment channels:
// - open_channel(address counterparty, uint256 capacity)
// - update_channel(bytes32 channelId, uint256 newBalanceA, uint256 newBalanceB, uint64 nonce)
// - close_channel(bytes32 channelId)
// - create_htlc(bytes32 channelId, uint256 amount, bytes32 hashLock, uint256 timeLock)
// - claim_htlc(bytes32 htlcId, bytes preimage)
// - get_channel_info(bytes32 channelId) returns (uint8 state, uint256 balanceA, uint256 balanceB, uint64 nonce)

use core::marker::PhantomData;
use fp_evm::{
	ExitError, ExitSucceed, PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileResult,
};
use pallet_evm::Precompile;
use sp_core::{H160, H256, U256};
use sp_std::vec::Vec;

/// Lightning Channels Precompile
pub struct EtridLightningPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> EtridLightningPrecompile<Runtime> {
	/// Selector for open_channel(address,uint256)
	/// Keccak256("open_channel(address,uint256)")[0..4] = 0x8c3d7f71
	const SELECTOR_OPEN_CHANNEL: [u8; 4] = [0x8c, 0x3d, 0x7f, 0x71];

	/// Selector for update_channel(bytes32,uint256,uint256,uint64)
	/// Keccak256("update_channel(bytes32,uint256,uint256,uint64)")[0..4] = 0xf2d7c4e1
	const SELECTOR_UPDATE_CHANNEL: [u8; 4] = [0xf2, 0xd7, 0xc4, 0xe1];

	/// Selector for close_channel(bytes32)
	/// Keccak256("close_channel(bytes32)")[0..4] = 0xa4f9edbf
	const SELECTOR_CLOSE_CHANNEL: [u8; 4] = [0xa4, 0xf9, 0xed, 0xbf];

	/// Selector for create_htlc(bytes32,uint256,bytes32,uint256)
	/// Keccak256("create_htlc(bytes32,uint256,bytes32,uint256)")[0..4] = 0x8b3f6f2a
	const SELECTOR_CREATE_HTLC: [u8; 4] = [0x8b, 0x3f, 0x6f, 0x2a];

	/// Selector for claim_htlc(bytes32,bytes)
	/// Keccak256("claim_htlc(bytes32,bytes)")[0..4] = 0x9e5c1d85
	const SELECTOR_CLAIM_HTLC: [u8; 4] = [0x9e, 0x5c, 0x1d, 0x85];

	/// Selector for get_channel_info(bytes32)
	/// Keccak256("get_channel_info(bytes32)")[0..4] = 0x5e9f2f5f
	const SELECTOR_GET_CHANNEL_INFO: [u8; 4] = [0x5e, 0x9f, 0x2f, 0x5f];

	/// Parse address from EVM bytes
	fn parse_address(data: &[u8]) -> Result<H160, PrecompileFailure> {
		if data.len() < 32 {
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Invalid address data".into()),
			});
		}
		// Address is in the last 20 bytes of the 32-byte word
		Ok(H160::from_slice(&data[12..32]))
	}

	/// Parse U256 from EVM bytes
	fn parse_u256(data: &[u8]) -> Result<U256, PrecompileFailure> {
		if data.len() < 32 {
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Invalid U256 data".into()),
			});
		}
		Ok(U256::from_big_endian(&data[0..32]))
	}

	/// Parse bytes32 from EVM bytes
	fn parse_bytes32(data: &[u8]) -> Result<[u8; 32], PrecompileFailure> {
		if data.len() < 32 {
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Invalid bytes32 data".into()),
			});
		}
		let mut result = [0u8; 32];
		result.copy_from_slice(&data[0..32]);
		Ok(result)
	}

	/// Parse uint64 from EVM bytes (in last 8 bytes of 32-byte word)
	fn parse_u64(data: &[u8]) -> Result<u64, PrecompileFailure> {
		if data.len() < 32 {
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Invalid u64 data".into()),
			});
		}
		let mut bytes = [0u8; 8];
		bytes.copy_from_slice(&data[24..32]);
		Ok(u64::from_be_bytes(bytes))
	}

	/// Encode success response
	fn encode_success(data: Vec<u8>) -> PrecompileResult {
		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			output: data,
		})
	}

	/// Encode empty success
	fn encode_empty_success() -> PrecompileResult {
		Self::encode_success(Vec::new())
	}

	/// Handle open_channel(address counterparty, uint256 capacity)
	fn handle_open_channel(input: &[u8]) -> PrecompileResult {
		if input.len() < 68 {
			// 4 bytes selector + 32 bytes address + 32 bytes capacity
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Invalid input length for open_channel".into()),
			});
		}

		let counterparty = Self::parse_address(&input[4..36])?;
		let capacity = Self::parse_u256(&input[36..68])?;

		// TODO: Call pallet_lightning_channels::Pallet::<Runtime>::open_channel
		// For now, return success with mock channel_id
		let mut output = Vec::with_capacity(32);
		output.extend_from_slice(&[0u8; 32]); // Mock channel_id
		Self::encode_success(output)
	}

	/// Handle update_channel(bytes32 channelId, uint256 newBalanceA, uint256 newBalanceB, uint64 nonce)
	fn handle_update_channel(input: &[u8]) -> PrecompileResult {
		if input.len() < 132 {
			// 4 + 32 + 32 + 32 + 32
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Invalid input length for update_channel".into()),
			});
		}

		let channel_id = Self::parse_bytes32(&input[4..36])?;
		let new_balance_a = Self::parse_u256(&input[36..68])?;
		let new_balance_b = Self::parse_u256(&input[68..100])?;
		let nonce = Self::parse_u64(&input[100..132])?;

		// TODO: Call pallet_lightning_channels::Pallet::<Runtime>::update_channel
		Self::encode_empty_success()
	}

	/// Handle close_channel(bytes32 channelId)
	fn handle_close_channel(input: &[u8]) -> PrecompileResult {
		if input.len() < 36 {
			// 4 + 32
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Invalid input length for close_channel".into()),
			});
		}

		let channel_id = Self::parse_bytes32(&input[4..36])?;

		// TODO: Call pallet_lightning_channels::Pallet::<Runtime>::close_channel
		Self::encode_empty_success()
	}

	/// Handle create_htlc(bytes32 channelId, uint256 amount, bytes32 hashLock, uint256 timeLock)
	fn handle_create_htlc(input: &[u8]) -> PrecompileResult {
		if input.len() < 132 {
			// 4 + 32 + 32 + 32 + 32
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Invalid input length for create_htlc".into()),
			});
		}

		let channel_id = Self::parse_bytes32(&input[4..36])?;
		let amount = Self::parse_u256(&input[36..68])?;
		let hash_lock = Self::parse_bytes32(&input[68..100])?;
		let time_lock = Self::parse_u256(&input[100..132])?;

		// TODO: Call pallet_lightning_channels::Pallet::<Runtime>::create_htlc
		let mut output = Vec::with_capacity(32);
		output.extend_from_slice(&[0u8; 32]); // Mock htlc_id
		Self::encode_success(output)
	}

	/// Handle claim_htlc(bytes32 htlcId, bytes preimage)
	fn handle_claim_htlc(input: &[u8]) -> PrecompileResult {
		if input.len() < 100 {
			// 4 + 32 + 32 (offset) + 32 (length) + preimage
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Invalid input length for claim_htlc".into()),
			});
		}

		let htlc_id = Self::parse_bytes32(&input[4..36])?;
		// Parse dynamic bytes for preimage
		let preimage_offset = Self::parse_u256(&input[36..68])?;
		// TODO: Parse preimage from offset
		// TODO: Call pallet_lightning_channels::Pallet::<Runtime>::claim_htlc

		Self::encode_empty_success()
	}

	/// Handle get_channel_info(bytes32 channelId) -> (uint8 state, uint256 balanceA, uint256 balanceB, uint64 nonce)
	fn handle_get_channel_info(input: &[u8]) -> PrecompileResult {
		if input.len() < 36 {
			// 4 + 32
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Invalid input length for get_channel_info".into()),
			});
		}

		let channel_id = Self::parse_bytes32(&input[4..36])?;

		// TODO: Call pallet_lightning_channels::Pallet::<Runtime>::channels(channel_id)
		// Mock response for now
		let mut output = Vec::with_capacity(128);
		// state (uint8) - padded to 32 bytes
		output.extend_from_slice(&[0u8; 31]);
		output.push(1); // State: Open
		// balanceA (uint256)
		output.extend_from_slice(&[0u8; 32]);
		// balanceB (uint256)
		output.extend_from_slice(&[0u8; 32]);
		// nonce (uint64) - padded to 32 bytes
		output.extend_from_slice(&[0u8; 32]);

		Self::encode_success(output)
	}
}

impl<Runtime> Precompile for EtridLightningPrecompile<Runtime> {
	fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		let input = handle.input();

		if input.len() < 4 {
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Input too short".into()),
			});
		}

		let selector = &input[0..4];

		match selector {
			s if s == Self::SELECTOR_OPEN_CHANNEL => Self::handle_open_channel(input),
			s if s == Self::SELECTOR_UPDATE_CHANNEL => Self::handle_update_channel(input),
			s if s == Self::SELECTOR_CLOSE_CHANNEL => Self::handle_close_channel(input),
			s if s == Self::SELECTOR_CREATE_HTLC => Self::handle_create_htlc(input),
			s if s == Self::SELECTOR_CLAIM_HTLC => Self::handle_claim_htlc(input),
			s if s == Self::SELECTOR_GET_CHANNEL_INFO => Self::handle_get_channel_info(input),
			_ => Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Unknown selector".into()),
			}),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_selectors() {
		// Ensure selectors are unique
		let selectors = [
			EtridLightningPrecompile::<()>::SELECTOR_OPEN_CHANNEL,
			EtridLightningPrecompile::<()>::SELECTOR_UPDATE_CHANNEL,
			EtridLightningPrecompile::<()>::SELECTOR_CLOSE_CHANNEL,
			EtridLightningPrecompile::<()>::SELECTOR_CREATE_HTLC,
			EtridLightningPrecompile::<()>::SELECTOR_CLAIM_HTLC,
			EtridLightningPrecompile::<()>::SELECTOR_GET_CHANNEL_INFO,
		];

		for (i, sel1) in selectors.iter().enumerate() {
			for (j, sel2) in selectors.iter().enumerate() {
				if i != j {
					assert_ne!(sel1, sel2, "Selectors {} and {} are not unique", i, j);
				}
			}
		}
	}
}
