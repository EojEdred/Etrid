// Ëtrid Oracle Precompile (0x800)
// Provides Solidity contracts access to FlareChain oracle price feeds

use core::marker::PhantomData;
use pallet_evm::{
    ExitError, ExitSucceed, Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput,
    PrecompileResult,
};
use sp_core::U256;

use super::xcm_bridge::{FlareChainQuery, FlareChainResponse, XcmBridge};

/// Oracle precompile for accessing FlareChain price feeds
///
/// Solidity Interface:
/// ```solidity
/// interface IEtridOracle {
///     function getPriceInETH(bytes32 symbol) external view returns (uint256);
///     function getPrice(bytes32 symbol, bytes32 quoteCurrency) external view returns (uint256);
///     function getLastUpdate(bytes32 symbol) external view returns (uint256);
/// }
/// ```
///
/// Address: 0x0000000000000000000000000000000000000800
pub struct EtridOraclePrecompile<XCM>(PhantomData<XCM>);

impl<XCM> EtridOraclePrecompile<XCM> {
    /// Function selector for getPriceInETH(bytes32)
    /// keccak256("getPriceInETH(bytes32)")[0..4]
    const GET_PRICE_IN_ETH_SELECTOR: [u8; 4] = [0x8a, 0x54, 0xc5, 0x2f];

    /// Function selector for getPrice(bytes32,bytes32)
    /// keccak256("getPrice(bytes32,bytes32)")[0..4]
    const GET_PRICE_SELECTOR: [u8; 4] = [0x41, 0x97, 0x6e, 0x09];

    /// Function selector for getLastUpdate(bytes32)
    /// keccak256("getLastUpdate(bytes32)")[0..4]
    const GET_LAST_UPDATE_SELECTOR: [u8; 4] = [0xb6, 0x33, 0x62, 0x0b];

    /// Parse a bytes32 symbol from input at given offset
    fn parse_symbol(input: &[u8], offset: usize) -> Result<[u8; 32], PrecompileFailure> {
        if input.len() < offset + 32 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Input too short".into()),
            });
        }
        let mut symbol = [0u8; 32];
        symbol.copy_from_slice(&input[offset..offset + 32]);
        Ok(symbol)
    }

    /// Get price in ETH (default quote currency)
    fn get_price_in_eth(symbol: [u8; 32]) -> PrecompileResult
    where
        XCM: XcmBridge,
    {
        // Default quote currency is ETH
        let mut eth_symbol = [0u8; 32];
        eth_symbol[..3].copy_from_slice(b"ETH");

        Self::get_price(symbol, eth_symbol)
    }

    /// Get price with custom quote currency
    fn get_price(symbol: [u8; 32], quote_currency: [u8; 32]) -> PrecompileResult
    where
        XCM: XcmBridge,
    {
        // Query FlareChain oracle via XCM
        let query = FlareChainQuery::OraclePrice {
            symbol,
            quote_currency,
        };

        let response = XCM::query_flarechain(query).map_err(|e| PrecompileFailure::Error {
            exit_status: ExitError::Other(
                sp_std::str::from_utf8(&e)
                    .unwrap_or("XCM query failed")
                    .into(),
            ),
        })?;

        match response {
            FlareChainResponse::OraclePrice(price) => {
                // Convert u128 price to U256 and encode
                let price_u256 = U256::from(price);
                let mut output = [0u8; 32];
                price_u256.to_big_endian(&mut output);

                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: output.to_vec(),
                })
            }
            FlareChainResponse::Error(e) => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other(
                    sp_std::str::from_utf8(&e)
                        .unwrap_or("Oracle error")
                        .into(),
                ),
            }),
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Unexpected response type".into()),
            }),
        }
    }

    /// Get last update timestamp for a symbol
    fn get_last_update(symbol: [u8; 32]) -> PrecompileResult
    where
        XCM: XcmBridge,
    {
        // Query FlareChain oracle via XCM
        let query = FlareChainQuery::OracleLastUpdate { symbol };

        let response = XCM::query_flarechain(query).map_err(|e| PrecompileFailure::Error {
            exit_status: ExitError::Other(
                sp_std::str::from_utf8(&e)
                    .unwrap_or("XCM query failed")
                    .into(),
            ),
        })?;

        match response {
            FlareChainResponse::OracleLastUpdate(timestamp) => {
                // Convert u64 timestamp to U256 and encode
                let timestamp_u256 = U256::from(timestamp);
                let mut output = [0u8; 32];
                timestamp_u256.to_big_endian(&mut output);

                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: output.to_vec(),
                })
            }
            FlareChainResponse::Error(e) => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other(
                    sp_std::str::from_utf8(&e)
                        .unwrap_or("Oracle error")
                        .into(),
                ),
            }),
            _ => Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Unexpected response type".into()),
            }),
        }
    }
}

impl<XCM> Precompile for EtridOraclePrecompile<XCM>
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
            // getPriceInETH(bytes32)
            Self::GET_PRICE_IN_ETH_SELECTOR => {
                let symbol = Self::parse_symbol(input, 4)?;
                Self::get_price_in_eth(symbol)
            }
            // getPrice(bytes32,bytes32)
            Self::GET_PRICE_SELECTOR => {
                let symbol = Self::parse_symbol(input, 4)?;
                let quote = Self::parse_symbol(input, 36)?;
                Self::get_price(symbol, quote)
            }
            // getLastUpdate(bytes32)
            Self::GET_LAST_UPDATE_SELECTOR => {
                let symbol = Self::parse_symbol(input, 4)?;
                Self::get_last_update(symbol)
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
    use crate::precompiles::xcm_bridge::MockXcmBridge;

    type TestOraclePrecompile = EtridOraclePrecompile<MockXcmBridge>;

    #[test]
    fn test_parse_symbol() {
        let mut input = vec![0u8; 36];
        input[4..7].copy_from_slice(b"BTC");

        let symbol = TestOraclePrecompile::parse_symbol(&input, 4).unwrap();
        assert_eq!(&symbol[..3], b"BTC");
    }

    // Note: Full integration tests would require mock PrecompileHandle
    // which is complex to set up. These tests verify the parsing logic.
}
