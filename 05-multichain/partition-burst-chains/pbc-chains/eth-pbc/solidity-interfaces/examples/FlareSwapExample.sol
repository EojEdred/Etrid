// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "../IEtridOracle.sol";

/**
 * @title FlareSwap Example
 * @notice Demonstrates using Ëtrid Oracle precompile in a DEX
 * @dev This is a simplified example showing oracle integration
 */
contract FlareSwapExample {
    IEtridOracle private constant ORACLE =
        IEtridOracle(0x0000000000000000000000000000000000000800);

    event Swap(
        address indexed user,
        bytes32 indexed tokenIn,
        bytes32 indexed tokenOut,
        uint256 amountIn,
        uint256 amountOut
    );

    /**
     * @notice Swap tokens using real-time oracle prices
     * @param tokenIn Symbol of input token (e.g., "BTC")
     * @param tokenOut Symbol of output token (e.g., "ETH")
     * @param amountIn Amount of input token (in wei)
     * @return amountOut Amount of output token (in wei)
     */
    function swap(
        bytes32 tokenIn,
        bytes32 tokenOut,
        uint256 amountIn
    ) external returns (uint256 amountOut) {
        // Get real-time prices from FlareChain oracle
        uint256 priceIn = ORACLE.getPriceInETH(tokenIn);   // Price of tokenIn in ETH
        uint256 priceOut = ORACLE.getPriceInETH(tokenOut); // Price of tokenOut in ETH

        // Calculate output amount: (amountIn * priceIn) / priceOut
        // Note: Prices are scaled by 1e18, so we need to divide once
        amountOut = (amountIn * priceIn) / priceOut;

        // In production, you would:
        // 1. Transfer tokens from user
        // 2. Apply fees
        // 3. Update liquidity pools
        // 4. Transfer tokens to user

        emit Swap(msg.sender, tokenIn, tokenOut, amountIn, amountOut);

        return amountOut;
    }

    /**
     * @notice Get current exchange rate between two tokens
     * @param tokenA First token symbol
     * @param tokenB Second token symbol
     * @return rate Exchange rate (tokenA per tokenB), scaled by 1e18
     */
    function getExchangeRate(bytes32 tokenA, bytes32 tokenB)
        external
        view
        returns (uint256 rate)
    {
        uint256 priceA = ORACLE.getPriceInETH(tokenA);
        uint256 priceB = ORACLE.getPriceInETH(tokenB);

        // Rate = priceA / priceB (how many tokenB per tokenA)
        rate = (priceA * 1e18) / priceB;

        return rate;
    }

    /**
     * @notice Check if oracle data is fresh (updated within last hour)
     * @param token Token symbol to check
     * @return isFresh True if data is fresh, false if stale
     */
    function isOracleFresh(bytes32 token) external view returns (bool isFresh) {
        uint256 lastUpdate = ORACLE.getLastUpdate(token);
        uint256 age = block.timestamp - lastUpdate;

        // Consider fresh if updated within last hour (3600 seconds)
        isFresh = age < 3600;

        return isFresh;
    }
}
