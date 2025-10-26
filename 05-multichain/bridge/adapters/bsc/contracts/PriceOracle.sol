// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./interfaces/IPancakePair.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title PriceOracle
 * @dev Price oracle for ÉTR token using PancakeSwap pairs
 *
 * Features:
 * - Get ÉTR price from PancakeSwap ÉTR/BNB pair
 * - Get LP token price
 * - Calculate TVL
 * - Support multiple price feeds
 */
contract PriceOracle is Ownable {
    // Price feeds
    mapping(address => address) public priceFeedPairs; // token => PancakeSwap pair

    // BNB price feed (can be updated by oracle)
    uint256 public bnbPriceUSD; // BNB price in USD (18 decimals)

    // Events
    event PriceFeedSet(address indexed token, address indexed pair);
    event BNBPriceUpdated(uint256 newPrice);

    constructor() Ownable(msg.sender) {
        // Default BNB price: $300 (adjust as needed)
        bnbPriceUSD = 300 * 1e18;
    }

    /**
     * @dev Set price feed pair for a token
     * @param token Token address
     * @param pair PancakeSwap pair address
     */
    function setPriceFeed(address token, address pair) external onlyOwner {
        require(token != address(0), "PriceOracle: zero address");
        require(pair != address(0), "PriceOracle: zero pair");

        priceFeedPairs[token] = pair;
        emit PriceFeedSet(token, pair);
    }

    /**
     * @dev Update BNB price in USD
     * @param newPrice New BNB price (18 decimals)
     */
    function updateBNBPrice(uint256 newPrice) external onlyOwner {
        require(newPrice > 0, "PriceOracle: invalid price");
        bnbPriceUSD = newPrice;
        emit BNBPriceUpdated(newPrice);
    }

    /**
     * @dev Get token price in USD
     * @param token Token address
     * @return price Token price in USD (18 decimals)
     */
    function getTokenPriceUSD(address token) public view returns (uint256) {
        address pair = priceFeedPairs[token];
        require(pair != address(0), "PriceOracle: no price feed");

        IPancakePair pancakePair = IPancakePair(pair);
        (uint112 reserve0, uint112 reserve1, ) = pancakePair.getReserves();

        address token0 = pancakePair.token0();
        address token1 = pancakePair.token1();

        // Determine which reserve is the token and which is BNB
        uint256 tokenReserve;
        uint256 bnbReserve;

        if (token0 == token) {
            tokenReserve = uint256(reserve0);
            bnbReserve = uint256(reserve1);
        } else if (token1 == token) {
            tokenReserve = uint256(reserve1);
            bnbReserve = uint256(reserve0);
        } else {
            revert("PriceOracle: token not in pair");
        }

        require(tokenReserve > 0, "PriceOracle: zero token reserve");

        // Price = (BNB reserve / Token reserve) * BNB price
        // Result in 18 decimals
        return (bnbReserve * bnbPriceUSD) / tokenReserve;
    }

    /**
     * @dev Get LP token price in USD
     * @param lpToken LP token address (PancakeSwap pair)
     * @return price LP token price in USD (18 decimals)
     */
    function getLPTokenPriceUSD(address lpToken) public view returns (uint256) {
        IPancakePair pair = IPancakePair(lpToken);

        (uint112 reserve0, uint112 reserve1, ) = pair.getReserves();
        uint256 totalSupply = pair.totalSupply();

        require(totalSupply > 0, "PriceOracle: zero LP supply");

        address token0 = pair.token0();
        address token1 = pair.token1();

        // Get prices of both tokens
        uint256 token0PriceUSD = getTokenPriceUSD(token0);
        uint256 token1PriceUSD = getTokenPriceUSD(token1);

        // Total value = (reserve0 * price0) + (reserve1 * price1)
        uint256 totalValueUSD = (uint256(reserve0) * token0PriceUSD +
                                 uint256(reserve1) * token1PriceUSD) / 1e18;

        // LP price = total value / total supply
        return (totalValueUSD * 1e18) / totalSupply;
    }

    /**
     * @dev Calculate TVL for a pool
     * @param lpToken LP token address
     * @param stakedAmount Amount of LP tokens staked
     * @return tvl TVL in USD (18 decimals)
     */
    function calculateTVL(address lpToken, uint256 stakedAmount)
        external
        view
        returns (uint256)
    {
        uint256 lpPrice = getLPTokenPriceUSD(lpToken);
        return (stakedAmount * lpPrice) / 1e18;
    }

    /**
     * @dev Calculate APR for a pool
     * @param yearlyRewards Yearly reward amount (in reward token)
     * @param rewardTokenPrice Price of reward token in USD
     * @param tvl TVL of the pool in USD
     * @return apr APR in basis points (10000 = 100%)
     */
    function calculateAPR(
        uint256 yearlyRewards,
        uint256 rewardTokenPrice,
        uint256 tvl
    ) external pure returns (uint256) {
        require(tvl > 0, "PriceOracle: zero TVL");

        // Yearly rewards value in USD
        uint256 yearlyRewardsUSD = (yearlyRewards * rewardTokenPrice) / 1e18;

        // APR = (yearlyRewardsUSD / tvl) * 10000
        return (yearlyRewardsUSD * 10000) / tvl;
    }
}
