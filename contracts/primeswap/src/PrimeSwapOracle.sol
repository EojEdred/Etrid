// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title PrimeSwapOracle
 * @dev Simple price oracle for PrimeSwap pools
 *
 * Provides prices for:
 * - ETR (native token)
 * - All PBC wrapped tokens (wBTC, wETH, etc.)
 *
 * Price format: USD with 8 decimals (e.g., 400000 = $0.004)
 *
 * In production, this would be replaced with Chainlink or
 * a decentralized oracle network. For initial deployment,
 * the Foundation manages price feeds.
 */
contract PrimeSwapOracle is Ownable {

    // Price storage: token address => price in USD (8 decimals)
    mapping(address => uint256) public prices;

    // Last update timestamps
    mapping(address => uint256) public lastUpdated;

    // Authorized price feeders
    mapping(address => bool) public authorizedFeeders;

    // Staleness threshold (default: 1 hour)
    uint256 public stalenessThreshold = 3600;

    // Special address for ETR native token price
    address public constant ETR_ADDRESS = address(1);

    // Events
    event PriceUpdated(address indexed token, uint256 price, uint256 timestamp);
    event FeederAuthorized(address indexed feeder);
    event FeederRevoked(address indexed feeder);
    event StalenessThresholdUpdated(uint256 newThreshold);

    constructor() Ownable(msg.sender) {
        // Owner is automatically an authorized feeder
        authorizedFeeders[msg.sender] = true;
    }

    modifier onlyFeeder() {
        require(authorizedFeeders[msg.sender], "Not authorized feeder");
        _;
    }

    /**
     * @dev Update price for a single token
     * @param token Token address (use ETR_ADDRESS for native ETR)
     * @param price Price in USD with 8 decimals
     */
    function updatePrice(address token, uint256 price) external onlyFeeder {
        require(price > 0, "Invalid price");
        prices[token] = price;
        lastUpdated[token] = block.timestamp;
        emit PriceUpdated(token, price, block.timestamp);
    }

    /**
     * @dev Batch update prices
     * @param tokens Array of token addresses
     * @param newPrices Array of prices (8 decimals)
     */
    function updatePrices(
        address[] calldata tokens,
        uint256[] calldata newPrices
    ) external onlyFeeder {
        require(tokens.length == newPrices.length, "Array length mismatch");

        for (uint256 i = 0; i < tokens.length; i++) {
            require(newPrices[i] > 0, "Invalid price");
            prices[tokens[i]] = newPrices[i];
            lastUpdated[tokens[i]] = block.timestamp;
            emit PriceUpdated(tokens[i], newPrices[i], block.timestamp);
        }
    }

    /**
     * @dev Get price for a token
     * @param token Token address
     * @return price Price in USD (8 decimals)
     */
    function getPrice(address token) external view returns (uint256 price) {
        price = prices[token];
        require(price > 0, "Price not set");
        require(
            block.timestamp - lastUpdated[token] <= stalenessThreshold,
            "Price is stale"
        );
    }

    /**
     * @dev Get price without staleness check (for initialization)
     */
    function getPriceUnsafe(address token) external view returns (uint256) {
        return prices[token];
    }

    /**
     * @dev Get ETR price specifically
     */
    function getETRPrice() external view returns (uint256) {
        uint256 price = prices[ETR_ADDRESS];
        require(price > 0, "ETR price not set");
        return price;
    }

    /**
     * @dev Check if price is fresh
     */
    function isPriceFresh(address token) external view returns (bool) {
        if (prices[token] == 0) return false;
        return block.timestamp - lastUpdated[token] <= stalenessThreshold;
    }

    /**
     * @dev Authorize a new price feeder
     */
    function authorizeFedeer(address feeder) external onlyOwner {
        authorizedFeeders[feeder] = true;
        emit FeederAuthorized(feeder);
    }

    /**
     * @dev Revoke feeder authorization
     */
    function revokeFeeder(address feeder) external onlyOwner {
        authorizedFeeders[feeder] = false;
        emit FeederRevoked(feeder);
    }

    /**
     * @dev Update staleness threshold
     */
    function setStalenessThreshold(uint256 newThreshold) external onlyOwner {
        require(newThreshold > 0, "Invalid threshold");
        stalenessThreshold = newThreshold;
        emit StalenessThresholdUpdated(newThreshold);
    }

    /**
     * @dev Initialize prices for all PBC tokens (convenience function)
     * Called at deployment to set initial prices
     */
    function initializePrices(
        uint256 etrPrice,      // $0.004 = 400000
        uint256 btcPrice,      // $100000 = 10000000000000
        uint256 ethPrice,      // $4000 = 400000000000
        uint256 xrpPrice,      // $2.50 = 250000000
        uint256 solPrice,      // $250 = 25000000000
        uint256 bnbPrice,      // $700 = 70000000000
        uint256 dogePrice,     // $0.40 = 40000000
        uint256 adaPrice,      // $1.00 = 100000000
        uint256 linkPrice,     // $25 = 2500000000
        uint256 trxPrice,      // $0.30 = 30000000
        uint256 xlmPrice,      // $0.50 = 50000000
        uint256 maticPrice     // $1.00 = 100000000
    ) external onlyOwner {
        prices[ETR_ADDRESS] = etrPrice;
        lastUpdated[ETR_ADDRESS] = block.timestamp;

        // Note: Token addresses would be set after wrapped tokens are deployed
        // This just sets up the oracle with initial values
        emit PriceUpdated(ETR_ADDRESS, etrPrice, block.timestamp);
    }
}
