// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

/**
 * @title IEtridOracle
 * @notice Interface for accessing FlareChain oracle price feeds from ETH-PBC
 * @dev Precompile address: 0x0000000000000000000000000000000000000800
 *
 * This precompile allows Solidity contracts on ETH-PBC to query real-time
 * price data from the FlareChain oracle network via XCM messaging.
 *
 * All prices are returned scaled by 1e18 for precision.
 *
 * Example Usage:
 * ```solidity
 * IEtridOracle oracle = IEtridOracle(0x0000000000000000000000000000000000000800);
 * uint256 btcPriceInETH = oracle.getPriceInETH("BTC");
 * uint256 btcPriceInUSD = oracle.getPrice("BTC", "USD");
 * ```
 */
interface IEtridOracle {
    /**
     * @notice Get price of symbol quoted in ETH
     * @param symbol The asset symbol to query (e.g., "BTC", "SOL", "XRP")
     * @return price The price in ETH, scaled by 1e18
     *
     * Example:
     * - symbol: "BTC"
     * - returns: 16_666_666_666_666_666_666 (16.67 ETH per BTC if BTC=$50k, ETH=$3k)
     */
    function getPriceInETH(bytes32 symbol) external view returns (uint256 price);

    /**
     * @notice Get price of symbol quoted in custom currency
     * @param symbol The asset symbol to query (e.g., "BTC", "ETH", "SOL")
     * @param quoteCurrency The quote currency (e.g., "USD", "EUR", "ETH")
     * @return price The price in quote currency, scaled by 1e18
     *
     * Example:
     * - symbol: "BTC"
     * - quoteCurrency: "USD"
     * - returns: 50_000_000_000_000_000_000_000 ($50,000 per BTC)
     */
    function getPrice(bytes32 symbol, bytes32 quoteCurrency) external view returns (uint256 price);

    /**
     * @notice Get the timestamp of the last oracle update for a symbol
     * @param symbol The asset symbol to query
     * @return timestamp Unix timestamp of last update
     *
     * Example:
     * - symbol: "BTC"
     * - returns: 1700000000 (Unix timestamp)
     */
    function getLastUpdate(bytes32 symbol) external view returns (uint256 timestamp);
}
