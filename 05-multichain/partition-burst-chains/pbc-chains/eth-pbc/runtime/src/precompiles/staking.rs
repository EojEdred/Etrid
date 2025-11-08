// Ëtrid Staking Precompile (0x802)
// Allows Solidity contracts to query FlareChain validator and staking information

use core::marker::PhantomData;
use pallet_evm::{
    ExitError, ExitSucceed, Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput,
    PrecompileResult,
};
use sp_core::U256;

use super::xcm_bridge::{FlareChainQuery, FlareChainResponse, XcmBridge};

/// Staking precompile for querying FlareChain validator/staking data
///
/// Solidity Interface:
/// ```solidity
/// interface IEtridStaking {
///     function getValidatorStake(bytes32 validatorId) external view returns (uint256);
///     function isValidatorActive(bytes32 validatorId) external view returns (bool);
///     function getTotalStaked() external view returns (uint256);
///     function getValidatorCount() external view returns (uint256);
/// }
/// ```
///
/// Address: 0x0000000000000000000000000000000000000802
pub struct EtridStakingPrecompile<XCM>(PhantomData<XCM>);

impl<XCM> EtridStakingPrecompile<XCM> {
    /// Function selector for getValidatorStake(bytes32)
    /// keccak256("getValidatorStake(bytes32)")[0..4]
    const GET_VALIDATOR_STAKE_SELECTOR: [u8; 4] = [0x4d, 0x2a, 0x5b, 0xc8];

    /// Function selector for isValidatorActive(bytes32)
    /// keccak256("isValidatorActive(bytes32)")[0..4]
    const IS_VALIDATOR_ACTIVE_SELECTOR: [u8; 4] = [0x40, 0x50, 0x0d, 0xc4];

    /// Function selector for getTotalStaked()
    /// keccak256("getTotalStaked()")[0..4]
    const GET_TOTAL_STAKED_SELECTOR: [u8; 4] = [0x5e, 0xa8, 0x9e, 0x0e];

    /// Function selector for getValidatorCount()
    /// keccak256("getValidatorCount()")[0..4]
    const GET_VALIDATOR_COUNT_SELECTOR: [u8; 4] = [0x85, 0x3e, 0x30, 0xb4];

    /// Parse a bytes32 validator ID from input at given offset
    fn parse_validator_id(input: &[u8], offset: usize) -> Result<[u8; 32], PrecompileFailure> {
        if input.len() < offset + 32 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Input too short".into()),
            });
        }
        let mut validator_id = [0u8; 32];
        validator_id.copy_from_slice(&input[offset..offset + 32]);
        Ok(validator_id)
    }

    /// Get validator stake amount
    fn get_validator_stake(validator_id: [u8; 32]) -> PrecompileResult
    where
        XCM: XcmBridge,
    {
        // Query FlareChain staking via XCM
        let query = FlareChainQuery::ValidatorStake { validator_id };

        let response = XCM::query_flarechain(query).map_err(|e| PrecompileFailure::Error {
            exit_status: ExitError::Other(
                sp_std::str::from_utf8(&e)
                    .unwrap_or("XCM query failed")
                    .to_string()
                    .into(),
            ),
        })?;

        match response {
            FlareChainResponse::ValidatorStake(stake) => {
                // Convert u128 stake to U256 and encode
                let stake_u256 = U256::from(stake);
                let output = stake_u256.to_big_endian();

                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: output.to_vec(),
                })
            }
            FlareChainResponse::Error(e) => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other(
                    sp_std::str::from_utf8(&e)
                        .unwrap_or("Staking error")
                        .to_string()
                        .into(),
                ),
            }),
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Unexpected response type".into()),
            }),
        }
    }

    /// Check if validator is active
    fn is_validator_active(validator_id: [u8; 32]) -> PrecompileResult
    where
        XCM: XcmBridge,
    {
        // Query FlareChain staking via XCM
        let query = FlareChainQuery::ValidatorActive { validator_id };

        let response = XCM::query_flarechain(query).map_err(|e| PrecompileFailure::Error {
            exit_status: ExitError::Other(
                sp_std::str::from_utf8(&e)
                    .unwrap_or("XCM query failed")
                    .to_string()
                    .into(),
            ),
        })?;

        match response {
            FlareChainResponse::ValidatorActive(is_active) => {
                // Convert bool to U256 (0 or 1) and encode
                let active_u256 = if is_active { U256::one() } else { U256::zero() };
                let output = active_u256.to_big_endian();

                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: output.to_vec(),
                })
            }
            FlareChainResponse::Error(e) => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other(
                    sp_std::str::from_utf8(&e)
                        .unwrap_or("Staking error")
                        .to_string()
                        .into(),
                ),
            }),
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Unexpected response type".into()),
            }),
        }
    }

    /// Get total amount staked across all validators
    fn get_total_staked() -> PrecompileResult
    where
        XCM: XcmBridge,
    {
        // Query FlareChain staking via XCM
        let query = FlareChainQuery::TotalStaked;

        let response = XCM::query_flarechain(query).map_err(|e| PrecompileFailure::Error {
            exit_status: ExitError::Other(
                sp_std::str::from_utf8(&e)
                    .unwrap_or("XCM query failed")
                    .to_string()
                    .into(),
            ),
        })?;

        match response {
            FlareChainResponse::TotalStaked(total) => {
                // Convert u128 total to U256 and encode
                let total_u256 = U256::from(total);
                let output = total_u256.to_big_endian();

                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: output.to_vec(),
                })
            }
            FlareChainResponse::Error(e) => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other(
                    sp_std::str::from_utf8(&e)
                        .unwrap_or("Staking error")
                        .to_string()
                        .into(),
                ),
            }),
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Unexpected response type".into()),
            }),
        }
    }

    /// Get total number of validators
    fn get_validator_count() -> PrecompileResult
    where
        XCM: XcmBridge,
    {
        // Query FlareChain staking via XCM
        let query = FlareChainQuery::ValidatorCount;

        let response = XCM::query_flarechain(query).map_err(|e| PrecompileFailure::Error {
            exit_status: ExitError::Other(
                sp_std::str::from_utf8(&e)
                    .unwrap_or("XCM query failed")
                    .to_string()
                    .into(),
            ),
        })?;

        match response {
            FlareChainResponse::ValidatorCount(count) => {
                // Convert u32 count to U256 and encode
                let count_u256 = U256::from(count);
                let output = count_u256.to_big_endian();

                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: output.to_vec(),
                })
            }
            FlareChainResponse::Error(e) => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other(
                    sp_std::str::from_utf8(&e)
                        .unwrap_or("Staking error")
                        .to_string()
                        .into(),
                ),
            }),
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Unexpected response type".into()),
            }),
        }
    }
}

impl<XCM> Precompile for EtridStakingPrecompile<XCM>
where
    XCM: XcmBridge,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        // Get input data
        let input = handle.input();

        // Require at least 4 bytes for function selector
        if input.len() < 4 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Input too short".into()),
            });
        }

        // Parse function selector
        let mut selector = [0u8; 4];
        selector.copy_from_slice(&input[0..4]);

        // Route to appropriate function
        match selector {
            // getValidatorStake(bytes32)
            Self::GET_VALIDATOR_STAKE_SELECTOR => {
                let validator_id = Self::parse_validator_id(input, 4)?;
                Self::get_validator_stake(validator_id)
            }
            // isValidatorActive(bytes32)
            Self::IS_VALIDATOR_ACTIVE_SELECTOR => {
                let validator_id = Self::parse_validator_id(input, 4)?;
                Self::is_validator_active(validator_id)
            }
            // getTotalStaked()
            Self::GET_TOTAL_STAKED_SELECTOR => Self::get_total_staked(),
            // getValidatorCount()
            Self::GET_VALIDATOR_COUNT_SELECTOR => Self::get_validator_count(),
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Unknown function selector".into()),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::precompiles::xcm_bridge::MockXcmBridge;

    type TestStakingPrecompile = EtridStakingPrecompile<MockXcmBridge>;

    #[test]
    fn test_parse_validator_id() {
        let mut input = vec![0u8; 36];
        input[4..36].copy_from_slice(&[1u8; 32]);

        let validator_id = TestStakingPrecompile::parse_validator_id(&input, 4).unwrap();
        assert_eq!(validator_id, [1u8; 32]);
    }
}
