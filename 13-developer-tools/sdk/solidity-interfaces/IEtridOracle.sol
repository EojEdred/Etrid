// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title IEtridOracle
 * @dev Interface for Etrid Oracle Precompile (0x800)
 *
 * Provides access to FlareChain price feeds from ETH PBC contracts.
 * No gas fees for oracle queries - data is provided via XCM from FlareChain.
 *
 * Address: 0x0000000000000000000000000000000000000800
 *
 * @notice All prices are scaled by 1e18 (e.g., $50,000 = 50000e18)
 */
interface IEtridOracle {
    /**
     * @notice Get price of an asset in a specific quote currency
     * @param symbol Asset symbol (e.g., bytes32("BTC"), bytes32("ETH"))
     * @param quoteCurrency Quote currency (e.g., bytes32("USD"), bytes32("ETH"))
     * @return Price scaled by 1e18
     *
     * @dev Example:
     *   bytes32 btc = bytes32("BTC");
     *   bytes32 usd = bytes32("USD");
     *   uint256 price = oracle.getPrice(btc, usd);
     *   // price = 50000e18 means $50,000
     */
    function getPrice(bytes32 symbol, bytes32 quoteCurrency)
        external
        view
        returns (uint256);

    /**
     * @notice Get price of an asset in ETH (convenience function)
     * @param symbol Asset symbol
     * @return Price in ETH scaled by 1e18
     *
     * @dev Example:
     *   bytes32 btc = bytes32("BTC");
     *   uint256 priceInEth = oracle.getPriceInETH(btc);
     *   // priceInEth = 16.5e18 means 1 BTC = 16.5 ETH
     */
    function getPriceInETH(bytes32 symbol) external view returns (uint256);

    /**
     * @notice Get the last update timestamp for a price feed
     * @param symbol Asset symbol
     * @return Unix timestamp of last oracle update
     *
     * @dev Useful for checking staleness of price data
     *   uint256 lastUpdate = oracle.getLastUpdate(bytes32("BTC"));
     *   require(block.timestamp - lastUpdate < 300, "Price too stale");
     */
    function getLastUpdate(bytes32 symbol) external view returns (uint256);
}
