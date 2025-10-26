const hre = require("hardhat");

/**
 * Creates Uniswap V3 pools for ÉTR.e and EDSC.e
 * 
 * Pools to create:
 * 1. WETH/ÉTR.e (0.3% fee tier)
 * 2. USDC/EDSC.e (0.05% fee tier - tightest for stablecoins)
 * 
 * Initial Liquidity Requirements:
 * - WETH/ÉTR.e: 100 ETH + 1M ÉTR (~$400k)
 * - USDC/EDSC.e: 500k USDC + 500k EDSC (~$1M)
 * Total: ~$3M
 */

// Uniswap V3 addresses (Ethereum Mainnet)
const UNISWAP_V3_FACTORY = "0x1F98431c8aD98523631AE4a59f267346ea31F984";
const UNISWAP_V3_POSITION_MANAGER = "0xC36442b4a4522E871399CD717aBDD847Ab11FE88";
const WETH_ADDRESS = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
const USDC_ADDRESS = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";

// Uniswap V3 fee tiers (in hundredths of a bip, e.g., 3000 = 0.3%)
const FEE_TIER_LOW = 500;     // 0.05% (stablecoins)
const FEE_TIER_MEDIUM = 3000; // 0.3% (standard)

async function main() {
  console.log("🦄 Creating Uniswap V3 Pools for Ëtrid Tokens...\n");

  const [deployer] = await hre.ethers.getSigners();
  console.log("Using account:", deployer.address);
  console.log("Account balance:", hre.ethers.formatEther(await deployer.provider.getBalance(deployer.address)), "ETH");
  console.log();

  // Load deployed contract addresses from environment
  const ETR_TOKEN_ADDRESS = process.env.ETR_TOKEN_ADDRESS;
  const EDSC_TOKEN_ADDRESS = process.env.EDSC_TOKEN_ADDRESS;

  if (!ETR_TOKEN_ADDRESS || !EDSC_TOKEN_ADDRESS) {
    console.error("❌ Error: ETR_TOKEN_ADDRESS and EDSC_TOKEN_ADDRESS must be set in .env");
    process.exit(1);
  }

  console.log("Token Addresses:");
  console.log("ÉTR Token:  ", ETR_TOKEN_ADDRESS);
  console.log("EDSC Token: ", EDSC_TOKEN_ADDRESS);
  console.log();

  // Get contract instances
  const factory = await hre.ethers.getContractAt(
    "IUniswapV3Factory",
    UNISWAP_V3_FACTORY
  );

  // ═══════════════════════════════════════════════════════════
  // Pool 1: WETH/ÉTR.e (0.3% fee)
  // ═══════════════════════════════════════════════════════════
  console.log("📊 Creating WETH/ÉTR.e pool (0.3% fee)...");
  
  // Token order matters: tokens must be sorted by address
  const [token0_weth_etr, token1_weth_etr] = WETH_ADDRESS < ETR_TOKEN_ADDRESS
    ? [WETH_ADDRESS, ETR_TOKEN_ADDRESS]
    : [ETR_TOKEN_ADDRESS, WETH_ADDRESS];

  // Initial price: 1 ÉTR = 0.0001 ETH (~$0.40 if ETH = $4000)
  // sqrtPriceX96 = sqrt(price) * 2^96
  const initialPriceWethEtr = WETH_ADDRESS < ETR_TOKEN_ADDRESS
    ? "250541448375047931186413799" // sqrt(0.0001) * 2^96
    : "3984280606858234405641759956"; // sqrt(10000) * 2^96

  const tx1 = await factory.createAndInitializePoolIfNecessary(
    token0_weth_etr,
    token1_weth_etr,
    FEE_TIER_MEDIUM,
    initialPriceWethEtr
  );
  await tx1.wait();

  const poolAddressWethEtr = await factory.getPool(
    token0_weth_etr,
    token1_weth_etr,
    FEE_TIER_MEDIUM
  );

  console.log("✅ WETH/ÉTR.e pool created at:", poolAddressWethEtr);
  console.log();

  // ═══════════════════════════════════════════════════════════
  // Pool 2: USDC/EDSC.e (0.05% fee)
  // ═══════════════════════════════════════════════════════════
  console.log("📊 Creating USDC/EDSC.e pool (0.05% fee)...");

  // Token order
  const [token0_usdc_edsc, token1_usdc_edsc] = USDC_ADDRESS < EDSC_TOKEN_ADDRESS
    ? [USDC_ADDRESS, EDSC_TOKEN_ADDRESS]
    : [EDSC_TOKEN_ADDRESS, USDC_ADDRESS];

  // Initial price: 1 EDSC = 1 USDC (1:1 peg)
  // sqrtPriceX96 = sqrt(1) * 2^96 = 2^96
  const initialPriceUsdcEdsc = "79228162514264337593543950336"; // 2^96

  const tx2 = await factory.createAndInitializePoolIfNecessary(
    token0_usdc_edsc,
    token1_usdc_edsc,
    FEE_TIER_LOW,
    initialPriceUsdcEdsc
  );
  await tx2.wait();

  const poolAddressUsdcEdsc = await factory.getPool(
    token0_usdc_edsc,
    token1_usdc_edsc,
    FEE_TIER_LOW
  );

  console.log("✅ USDC/EDSC.e pool created at:", poolAddressUsdcEdsc);
  console.log();

  // ═══════════════════════════════════════════════════════════
  // Summary
  // ═══════════════════════════════════════════════════════════
  console.log("═════════════════════════════════════════════════════════");
  console.log("✅ Uniswap Pools Created!");
  console.log("═════════════════════════════════════════════════════════");
  console.log();
  console.log("📋 Pool Addresses:");
  console.log("─────────────────────────────────────────────────────────");
  console.log("WETH/ÉTR.e:  ", poolAddressWethEtr);
  console.log("USDC/EDSC.e: ", poolAddressUsdcEdsc);
  console.log();
  console.log("🔗 Uniswap Interface Links:");
  console.log("─────────────────────────────────────────────────────────");
  console.log(`WETH/ÉTR.e:  https://app.uniswap.org/pools/${poolAddressWethEtr}`);
  console.log(`USDC/EDSC.e: https://app.uniswap.org/pools/${poolAddressUsdcEdsc}`);
  console.log();
  console.log("⚠️  Next Steps:");
  console.log("─────────────────────────────────────────────────────────");
  console.log("1. Add liquidity to pools using Uniswap interface:");
  console.log("   - WETH/ÉTR.e: 100 ETH + 1M ÉTR (~$400k)");
  console.log("   - USDC/EDSC.e: 500k USDC + 500k EDSC (~$1M)");
  console.log("2. Set price ranges (concentrated liquidity):");
  console.log("   - ÉTR: ±20% range around initial price");
  console.log("   - EDSC: ±1% range (tight stablecoin peg)");
  console.log("3. Monitor pool health via Dune Analytics");
  console.log("4. List on Uniswap token lists for discoverability");
  console.log("═════════════════════════════════════════════════════════");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
