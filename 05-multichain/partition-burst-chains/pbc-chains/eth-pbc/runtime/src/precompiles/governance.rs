// Ëtrid Governance Precompile (0x801)
// Allows Solidity contracts to interact with FlareChain governance

use core::marker::PhantomData;
use pallet_evm::{
    ExitError, ExitSucceed, Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput,
    PrecompileResult,
};
use sp_core::{H160, U256};
use sp_std::vec::Vec;

use super::xcm_bridge::{FlareChainQuery, FlareChainResponse, XcmBridge};

/// Governance precompile for interacting with FlareChain governance
///
/// Solidity Interface:
/// ```solidity
/// interface IEtridGovernance {
///     function submitProposal(string memory title, string memory description) external returns (uint256 proposalId);
///     function voteOnProposal(uint256 proposalId, bool support) external;
///     function getProposalStatus(uint256 proposalId) external view returns (uint8 status);
/// }
/// ```
///
/// Address: 0x0000000000000000000000000000000000000801
///
/// Proposal Status Enum:
/// - 0: Pending
/// - 1: Active
/// - 2: Passed
/// - 3: Failed
pub struct EtridGovernancePrecompile<XCM>(PhantomData<XCM>);

impl<XCM> EtridGovernancePrecompile<XCM> {
    /// Function selector for submitProposal(string,string)
    /// keccak256("submitProposal(string,string)")[0..4]
    const SUBMIT_PROPOSAL_SELECTOR: [u8; 4] = [0xda, 0x35, 0xc6, 0x64];

    /// Function selector for voteOnProposal(uint256,bool)
    /// keccak256("voteOnProposal(uint256,bool)")[0..4]
    const VOTE_ON_PROPOSAL_SELECTOR: [u8; 4] = [0x01, 0x58, 0x11, 0xbf];

    /// Function selector for getProposalStatus(uint256)
    /// keccak256("getProposalStatus(uint256)")[0..4]
    const GET_PROPOSAL_STATUS_SELECTOR: [u8; 4] = [0x7e, 0xcb, 0xe4, 0xe4];

    /// Parse a string from ABI-encoded input
    fn parse_string(input: &[u8], offset_location: usize) -> Result<Vec<u8>, PrecompileFailure> {
        // Read offset to string data
        if input.len() < offset_location + 32 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Input too short for offset".into()),
            });
        }

        let offset = U256::from_big_endian(&input[offset_location..offset_location + 32])
            .as_usize();

        // Read string length
        if input.len() < 4 + offset + 32 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Input too short for length".into()),
            });
        }

        let length =
            U256::from_big_endian(&input[4 + offset..4 + offset + 32]).as_usize();

        // Read string data
        if input.len() < 4 + offset + 32 + length {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Input too short for string data".into()),
            });
        }

        let string_data = input[4 + offset + 32..4 + offset + 32 + length].to_vec();
        Ok(string_data)
    }

    /// Parse a uint256 from input at given offset
    fn parse_uint256(input: &[u8], offset: usize) -> Result<u64, PrecompileFailure> {
        if input.len() < offset + 32 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Input too short".into()),
            });
        }
        let value = U256::from_big_endian(&input[offset..offset + 32]);
        Ok(value.as_u64())
    }

    /// Parse a bool from input at given offset
    fn parse_bool(input: &[u8], offset: usize) -> Result<bool, PrecompileFailure> {
        if input.len() < offset + 32 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Input too short".into()),
            });
        }
        let value = U256::from_big_endian(&input[offset..offset + 32]);
        Ok(!value.is_zero())
    }

    /// Submit a governance proposal to FlareChain
    fn submit_proposal(
        title: Vec<u8>,
        description: Vec<u8>,
        caller: H160,
    ) -> PrecompileResult
    where
        XCM: XcmBridge,
    {
        // Validate inputs
        if title.is_empty() || title.len() > 256 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Invalid title length".into()),
            });
        }
        if description.is_empty() || description.len() > 10000 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Invalid description length".into()),
            });
        }

        // Query FlareChain governance via XCM
        let query = FlareChainQuery::GovernanceProposal {
            title,
            description,
            caller,
        };

        let response = XCM::query_flarechain(query).map_err(|e| PrecompileFailure::Error {
            exit_status: ExitError::Other(
                sp_std::str::from_utf8(&e)
                    .unwrap_or("XCM query failed")
                    .to_string()
                    .into(),
            ),
        })?;

        match response {
            FlareChainResponse::GovernanceProposalId(proposal_id) => {
                // Return proposal ID as uint256
                let id_u256 = U256::from(proposal_id);
                let output = id_u256.to_big_endian();

                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: output.to_vec(),
                })
            }
            FlareChainResponse::Error(e) => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other(
                    sp_std::str::from_utf8(&e)
                        .unwrap_or("Governance error")
                        .to_string()
                        .into(),
                ),
            }),
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Unexpected response type".into()),
            }),
        }
    }

    /// Vote on a governance proposal
    fn vote_on_proposal(
        proposal_id: u64,
        support: bool,
        caller: H160,
    ) -> PrecompileResult
    where
        XCM: XcmBridge,
    {
        // Query FlareChain governance via XCM
        let query = FlareChainQuery::GovernanceVote {
            proposal_id,
            support,
            caller,
        };

        let response = XCM::query_flarechain(query).map_err(|e| PrecompileFailure::Error {
            exit_status: ExitError::Other(
                sp_std::str::from_utf8(&e)
                    .unwrap_or("XCM query failed")
                    .to_string()
                    .into(),
            ),
        })?;

        match response {
            FlareChainResponse::GovernanceVoteConfirmed => {
                // Return empty success
                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: Vec::new(),
                })
            }
            FlareChainResponse::Error(e) => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other(
                    sp_std::str::from_utf8(&e)
                        .unwrap_or("Governance error")
                        .to_string()
                        .into(),
                ),
            }),
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Unexpected response type".into()),
            }),
        }
    }

    /// Get proposal status
    fn get_proposal_status(proposal_id: u64) -> PrecompileResult
    where
        XCM: XcmBridge,
    {
        // Query FlareChain governance via XCM
        let query = FlareChainQuery::GovernanceProposalStatus { proposal_id };

        let response = XCM::query_flarechain(query).map_err(|e| PrecompileFailure::Error {
            exit_status: ExitError::Other(
                sp_std::str::from_utf8(&e)
                    .unwrap_or("XCM query failed")
                    .to_string()
                    .into(),
            ),
        })?;

        match response {
            FlareChainResponse::GovernanceProposalStatus(status) => {
                // Return status as uint8 (encoded as uint256)
                let status_u256 = U256::from(status);
                let output = status_u256.to_big_endian();

                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: output.to_vec(),
                })
            }
            FlareChainResponse::Error(e) => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other(
                    sp_std::str::from_utf8(&e)
                        .unwrap_or("Governance error")
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

impl<XCM> Precompile for EtridGovernancePrecompile<XCM>
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

        // Get caller address
        let caller = handle.context().caller;

        // Parse function selector
        let mut selector = [0u8; 4];
        selector.copy_from_slice(&input[0..4]);

        // Route to appropriate function
        match selector {
            // submitProposal(string,string)
            Self::SUBMIT_PROPOSAL_SELECTOR => {
                let title = Self::parse_string(input, 4)?;
                let description = Self::parse_string(input, 36)?;
                Self::submit_proposal(title, description, caller)
            }
            // voteOnProposal(uint256,bool)
            Self::VOTE_ON_PROPOSAL_SELECTOR => {
                let proposal_id = Self::parse_uint256(input, 4)?;
                let support = Self::parse_bool(input, 36)?;
                Self::vote_on_proposal(proposal_id, support, caller)
            }
            // getProposalStatus(uint256)
            Self::GET_PROPOSAL_STATUS_SELECTOR => {
                let proposal_id = Self::parse_uint256(input, 4)?;
                Self::get_proposal_status(proposal_id)
            }
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Unknown function selector".into()),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uint256() {
        let mut input = vec![0u8; 36];
        input[35] = 42; // uint256 value of 42

        let value = EtridGovernancePrecompile::<()>::parse_uint256(&input, 4).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_parse_bool() {
        let mut input = vec![0u8; 36];
        input[35] = 1; // bool value of true

        let value = EtridGovernancePrecompile::<()>::parse_bool(&input, 4).unwrap();
        assert_eq!(value, true);
    }
}
