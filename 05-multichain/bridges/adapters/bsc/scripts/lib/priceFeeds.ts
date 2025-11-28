import { ethers } from "ethers";

/**
 * Price Feed Library
 *
 * Utilities for fetching token prices from various sources
 */

// PancakeSwap V2 Router address on BSC
const PANCAKESWAP_ROUTER = "0x10ED43C718714eb63d5aA57B78B54704E256024E";
const PANCAKESWAP_FACTORY = "0xcA143Ce32Fe78f1f7019d7d551a6402fC5350c73";

// Common token addresses on BSC Mainnet
const WBNB_ADDRESS = "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c";
const BUSD_ADDRESS = "0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56";
const USDT_ADDRESS = "0x55d398326f99059fF775485246999027B3197955";

// ABIs
const PAIR_ABI = [
  "function token0() external view returns (address)",
  "function token1() external view returns (address)",
  "function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)",
  "function totalSupply() external view returns (uint256)",
];

const ERC20_ABI = [
  "function decimals() external view returns (uint8)",
  "function symbol() external view returns (string)",
];

const FACTORY_ABI = [
  "function getPair(address tokenA, address tokenB) external view returns (address pair)",
];

/**
 * Get BNB price in USD from PancakeSwap
 */
export async function getBNBPriceUSD(provider: ethers.Provider): Promise<number> {
  try {
    // Get BNB/BUSD pair
    const factory = new ethers.Contract(PANCAKESWAP_FACTORY, FACTORY_ABI, provider);
    const pairAddress = await factory.getPair(WBNB_ADDRESS, BUSD_ADDRESS);

    if (pairAddress === ethers.ZeroAddress) {
      throw new Error("BNB/BUSD pair not found");
    }

    const pair = new ethers.Contract(pairAddress, PAIR_ABI, provider);
    const [reserve0, reserve1] = await pair.getReserves();
    const token0 = await pair.token0();

    let bnbReserve: bigint;
    let busdReserve: bigint;

    if (token0.toLowerCase() === WBNB_ADDRESS.toLowerCase()) {
      bnbReserve = reserve0;
      busdReserve = reserve1;
    } else {
      bnbReserve = reserve1;
      busdReserve = reserve0;
    }

    // Price = BUSD reserve / BNB reserve
    const price = Number(busdReserve) / Number(bnbReserve);
    return price;
  } catch (error) {
    console.error("Error fetching BNB price:", error);
    // Return fallback price
    return 300; // $300 default
  }
}

/**
 * Get token price in USD
 */
export async function getTokenPriceUSD(
  tokenAddress: string,
  provider: ethers.Provider
): Promise<number> {
  try {
    // Get BNB price first
    const bnbPrice = await getBNBPriceUSD(provider);

    // Get token/BNB pair
    const factory = new ethers.Contract(PANCAKESWAP_FACTORY, FACTORY_ABI, provider);
    const pairAddress = await factory.getPair(tokenAddress, WBNB_ADDRESS);

    if (pairAddress === ethers.ZeroAddress) {
      throw new Error(`Pair not found for token ${tokenAddress}`);
    }

    const pair = new ethers.Contract(pairAddress, PAIR_ABI, provider);
    const [reserve0, reserve1] = await pair.getReserves();
    const token0 = await pair.token0();

    let tokenReserve: bigint;
    let bnbReserve: bigint;

    if (token0.toLowerCase() === tokenAddress.toLowerCase()) {
      tokenReserve = reserve0;
      bnbReserve = reserve1;
    } else {
      tokenReserve = reserve1;
      bnbReserve = reserve0;
    }

    // Price in BNB = BNB reserve / Token reserve
    const priceInBNB = Number(bnbReserve) / Number(tokenReserve);

    // Price in USD = Price in BNB * BNB price
    return priceInBNB * bnbPrice;
  } catch (error) {
    console.error(`Error fetching price for token ${tokenAddress}:`, error);
    throw error;
  }
}

/**
 * Get LP token price in USD
 */
export async function getLPTokenPriceUSD(
  lpTokenAddress: string,
  provider: ethers.Provider
): Promise<number> {
  try {
    const pair = new ethers.Contract(lpTokenAddress, PAIR_ABI, provider);

    const [reserve0, reserve1] = await pair.getReserves();
    const totalSupply = await pair.totalSupply();
    const token0Address = await pair.token0();
    const token1Address = await pair.token1();

    // Get prices of both tokens
    let token0Price: number;
    let token1Price: number;

    // If token is BNB, get BNB price directly
    if (token0Address.toLowerCase() === WBNB_ADDRESS.toLowerCase()) {
      token0Price = await getBNBPriceUSD(provider);
    } else {
      token0Price = await getTokenPriceUSD(token0Address, provider);
    }

    if (token1Address.toLowerCase() === WBNB_ADDRESS.toLowerCase()) {
      token1Price = await getBNBPriceUSD(provider);
    } else {
      token1Price = await getTokenPriceUSD(token1Address, provider);
    }

    // Get decimals for proper calculation
    const token0 = new ethers.Contract(token0Address, ERC20_ABI, provider);
    const token1 = new ethers.Contract(token1Address, ERC20_ABI, provider);

    const decimals0 = await token0.decimals();
    const decimals1 = await token1.decimals();

    // Normalize reserves to 18 decimals
    const reserve0Normalized = Number(reserve0) / 10 ** Number(decimals0);
    const reserve1Normalized = Number(reserve1) / 10 ** Number(decimals1);

    // Total value in USD
    const totalValueUSD =
      reserve0Normalized * token0Price + reserve1Normalized * token1Price;

    // LP token price = total value / total supply
    const totalSupplyNormalized = Number(totalSupply) / 1e18;
    return totalValueUSD / totalSupplyNormalized;
  } catch (error) {
    console.error(`Error fetching LP token price for ${lpTokenAddress}:`, error);
    throw error;
  }
}

/**
 * Calculate TVL for a pool
 */
export async function calculateTVL(
  lpTokenAddress: string,
  stakedAmount: bigint,
  provider: ethers.Provider
): Promise<number> {
  const lpPrice = await getLPTokenPriceUSD(lpTokenAddress, provider);
  const stakedAmountNormalized = Number(stakedAmount) / 1e18;
  return stakedAmountNormalized * lpPrice;
}

/**
 * Calculate APR for a pool
 */
export function calculateAPR(
  yearlyRewards: bigint,
  rewardTokenPrice: number,
  tvl: number
): number {
  if (tvl === 0) {
    return 0;
  }

  const yearlyRewardsNormalized = Number(yearlyRewards) / 1e18;
  const yearlyRewardsUSD = yearlyRewardsNormalized * rewardTokenPrice;

  // APR as percentage
  return (yearlyRewardsUSD / tvl) * 100;
}

/**
 * Fetch all prices for monitoring
 */
export async function fetchAllPrices(
  etrTokenAddress: string,
  lpTokenAddresses: string[],
  provider: ethers.Provider
): Promise<{
  bnbPrice: number;
  etrPrice: number;
  lpPrices: { [address: string]: number };
}> {
  const bnbPrice = await getBNBPriceUSD(provider);
  const etrPrice = await getTokenPriceUSD(etrTokenAddress, provider);

  const lpPrices: { [address: string]: number } = {};

  for (const lpAddress of lpTokenAddresses) {
    try {
      lpPrices[lpAddress] = await getLPTokenPriceUSD(lpAddress, provider);
    } catch (error) {
      console.error(`Failed to get price for LP ${lpAddress}:`, error);
      lpPrices[lpAddress] = 0;
    }
  }

  return {
    bnbPrice,
    etrPrice,
    lpPrices,
  };
}
