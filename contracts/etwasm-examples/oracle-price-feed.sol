// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title OraclePriceFeed
 * @notice Example contract demonstrating Ëtrid Oracle Precompile (0x800)
 * @dev Uses XCM to query FlareChain oracle for real-time price feeds
 */

// Interface for Ëtrid Oracle Precompile
interface IEtridOracle {
    function getPriceInETH(bytes32 symbol) external view returns (uint256 price);
    function getPrice(bytes32 symbol, bytes32 quoteCurrency) external view returns (uint256 price);
    function getLastUpdate(bytes32 symbol) external view returns (uint256 timestamp);
}

contract OraclePriceFeed {
    // Oracle precompile address
    IEtridOracle private constant ORACLE = IEtridOracle(0x0000000000000000000000000000000000000800);

    event PriceQueried(bytes32 indexed symbol, uint256 price, uint256 timestamp);
    event PriceComparison(bytes32 indexed symbol1, bytes32 indexed symbol2, uint256 ratio);

    /**
     * @notice Get price of an asset in ETH
     * @param symbol Asset symbol (e.g., "BTC", "SOL", "USDT")
     * @return price Price in ETH (18 decimals)
     */
    function getAssetPriceInETH(string memory symbol) public view returns (uint256 price) {
        bytes32 symbolBytes = stringToBytes32(symbol);
        price = ORACLE.getPriceInETH(symbolBytes);
        return price;
    }

    /**
     * @notice Get price of an asset in a specific quote currency
     * @param symbol Asset symbol
     * @param quote Quote currency symbol (e.g., "USD", "EUR")
     * @return price Price in quote currency (18 decimals)
     */
    function getAssetPrice(string memory symbol, string memory quote) public view returns (uint256 price) {
        bytes32 symbolBytes = stringToBytes32(symbol);
        bytes32 quoteBytes = stringToBytes32(quote);
        price = ORACLE.getPrice(symbolBytes, quoteBytes);
        return price;
    }

    /**
     * @notice Get last update timestamp for an asset
     * @param symbol Asset symbol
     * @return timestamp Unix timestamp of last price update
     */
    function getLastPriceUpdate(string memory symbol) public view returns (uint256 timestamp) {
        bytes32 symbolBytes = stringToBytes32(symbol);
        timestamp = ORACLE.getLastUpdate(symbolBytes);
        return timestamp;
    }

    /**
     * @notice Calculate price ratio between two assets
     * @param symbol1 First asset symbol
     * @param symbol2 Second asset symbol
     * @return ratio Price ratio (symbol1/symbol2) with 18 decimals
     */
    function getPriceRatio(string memory symbol1, string memory symbol2) public view returns (uint256 ratio) {
        uint256 price1 = getAssetPriceInETH(symbol1);
        uint256 price2 = getAssetPriceInETH(symbol2);
        require(price2 > 0, "Division by zero");

        ratio = (price1 * 1e18) / price2;
        emit PriceComparison(stringToBytes32(symbol1), stringToBytes32(symbol2), ratio);
        return ratio;
    }

    /**
     * @notice Query and emit price event
     * @param symbol Asset symbol to query
     */
    function queryAndEmitPrice(string memory symbol) external {
        bytes32 symbolBytes = stringToBytes32(symbol);
        uint256 price = ORACLE.getPriceInETH(symbolBytes);
        uint256 timestamp = ORACLE.getLastUpdate(symbolBytes);

        emit PriceQueried(symbolBytes, price, timestamp);
    }

    /**
     * @notice Check if price data is stale
     * @param symbol Asset symbol
     * @param maxAge Maximum age in seconds
     * @return isStale True if price is older than maxAge
     */
    function isPriceStale(string memory symbol, uint256 maxAge) public view returns (bool isStale) {
        uint256 lastUpdate = getLastPriceUpdate(symbol);
        isStale = (block.timestamp - lastUpdate) > maxAge;
        return isStale;
    }

    // Helper function to convert string to bytes32
    function stringToBytes32(string memory source) internal pure returns (bytes32 result) {
        bytes memory tempEmptyStringTest = bytes(source);
        if (tempEmptyStringTest.length == 0) {
            return 0x0;
        }

        assembly {
            result := mload(add(source, 32))
        }
    }
}

/**
 * @title SimpleSwap
 * @notice Example DEX using Oracle precompile for pricing
 * @dev Demonstrates real-world use case of XCM-enabled oracle
 */
contract SimpleSwap {
    IEtridOracle private constant ORACLE = IEtridOracle(0x0000000000000000000000000000000000000800);

    event Swap(
        address indexed user,
        bytes32 indexed tokenIn,
        bytes32 indexed tokenOut,
        uint256 amountIn,
        uint256 amountOut
    );

    /**
     * @notice Calculate swap output amount using oracle prices
     * @param tokenIn Input token symbol
     * @param tokenOut Output token symbol
     * @param amountIn Input amount (18 decimals)
     * @return amountOut Output amount (18 decimals)
     */
    function calculateSwapOutput(
        string memory tokenIn,
        string memory tokenOut,
        uint256 amountIn
    ) public view returns (uint256 amountOut) {
        bytes32 tokenInBytes = stringToBytes32(tokenIn);
        bytes32 tokenOutBytes = stringToBytes32(tokenOut);

        uint256 priceIn = ORACLE.getPriceInETH(tokenInBytes);
        uint256 priceOut = ORACLE.getPriceInETH(tokenOutBytes);

        require(priceOut > 0, "Invalid output price");

        // Calculate output: (amountIn * priceIn) / priceOut
        amountOut = (amountIn * priceIn) / priceOut;

        return amountOut;
    }

    /**
     * @notice Simulate swap (no actual token transfer)
     * @param tokenIn Input token symbol
     * @param tokenOut Output token symbol
     * @param amountIn Input amount
     */
    function swap(
        string memory tokenIn,
        string memory tokenOut,
        uint256 amountIn
    ) external returns (uint256 amountOut) {
        bytes32 tokenInBytes = stringToBytes32(tokenIn);
        bytes32 tokenOutBytes = stringToBytes32(tokenOut);

        amountOut = calculateSwapOutput(tokenIn, tokenOut, amountIn);

        emit Swap(msg.sender, tokenInBytes, tokenOutBytes, amountIn, amountOut);

        return amountOut;
    }

    // Helper function
    function stringToBytes32(string memory source) internal pure returns (bytes32 result) {
        bytes memory tempEmptyStringTest = bytes(source);
        if (tempEmptyStringTest.length == 0) {
            return 0x0;
        }
        assembly {
            result := mload(add(source, 32))
        }
    }
}
