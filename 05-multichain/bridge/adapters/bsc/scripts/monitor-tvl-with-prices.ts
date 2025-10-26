import { ethers } from "hardhat";
import {
  getBNBPriceUSD,
  getTokenPriceUSD,
  calculateTVL,
  calculateAPR,
} from "./lib/priceFeeds";

/**
 * Monitor TVL with Price Feeds
 *
 * Enhanced TVL monitoring with real-time price data from PancakeSwap
 *
 * Usage:
 *   npx hardhat run scripts/monitor-tvl-with-prices.ts --network bscTestnet
 *   npx hardhat run scripts/monitor-tvl-with-prices.ts --network bscMainnet
 */

interface PoolData {
  poolId: number;
  lpToken: string;
  lpSymbol: string;
  lpName: string;
  totalStaked: bigint;
  allocPoint: bigint;
  rewardShare: number;
  lpPrice: number;
  tvlUSD: number;
}

async function main() {
  console.log("\n📊 MASTERCHEF TVL MONITOR (WITH PRICE FEEDS)\n");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Get network info
  const network = await ethers.provider.getNetwork();
  const isMainnet = network.chainId === 56n;
  const isTestnet = network.chainId === 97n;

  console.log(`📍 Network: ${isMainnet ? "BSC Mainnet" : isTestnet ? "BSC Testnet" : "Unknown"}`);
  console.log(`   Chain ID: ${network.chainId}`);

  const blockNumber = await ethers.provider.getBlockNumber();
  console.log(`   Block: ${blockNumber}\n`);

  // Get addresses
  const etrAddressKey = isMainnet ? "ETR_TOKEN_ADDRESS_MAINNET" : "ETR_TOKEN_ADDRESS_TESTNET";
  const masterChefAddressKey = isMainnet
    ? "MASTERCHEF_ADDRESS_MAINNET"
    : "MASTERCHEF_ADDRESS_TESTNET";

  const etrAddress = process.env[etrAddressKey];
  const masterChefAddress = process.env[masterChefAddressKey];

  if (!etrAddress || !masterChefAddress) {
    console.log("❌ ERROR: Contract addresses not found in .env\n");
    process.exit(1);
  }

  console.log("📄 Contracts:");
  console.log(`   ÉTR Token:  ${etrAddress}`);
  console.log(`   MasterChef: ${masterChefAddress}\n`);

  // Get MasterChef contract
  const masterChef = await ethers.getContractAt("MasterChef", masterChefAddress);

  // Get basic info
  const poolLength = await masterChef.poolLength();
  const totalAllocPoint = await masterChef.totalAllocPoint();
  const rewardPerBlock = await masterChef.rewardPerBlock();

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📈 MASTERCHEF OVERVIEW\n");
  console.log(`   Total Pools:        ${poolLength}`);
  console.log(`   Total Alloc Points: ${totalAllocPoint}`);
  console.log(`   Reward Rate:        ${ethers.formatEther(rewardPerBlock)} ÉTR/block\n`);

  if (poolLength === 0n) {
    console.log("⚠️  No pools added yet\n");
    process.exit(0);
  }

  // Fetch prices
  console.log("💰 Fetching prices from PancakeSwap...\n");

  let bnbPrice = 0;
  let etrPrice = 0;

  try {
    bnbPrice = await getBNBPriceUSD(ethers.provider);
    console.log(`   BNB Price: $${bnbPrice.toFixed(2)}`);
  } catch (error) {
    console.log(`   ⚠️  Could not fetch BNB price (using fallback)`);
    bnbPrice = 300; // Fallback
  }

  if (isMainnet) {
    try {
      etrPrice = await getTokenPriceUSD(etrAddress, ethers.provider);
      console.log(`   ÉTR Price: $${etrPrice.toFixed(6)}`);
    } catch (error) {
      console.log(`   ⚠️  Could not fetch ÉTR price (pair may not exist yet)`);
      etrPrice = 0.01; // Fallback
    }
  } else {
    console.log(`   ÉTR Price: N/A (testnet)`);
    etrPrice = 0.01; // Testnet default
  }

  console.log();

  // Fetch pool data with prices
  const pools: PoolData[] = [];
  let totalTVL = 0;

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("💰 POOL TVL BREAKDOWN\n");

  for (let i = 0; i < Number(poolLength); i++) {
    const poolInfo = await masterChef.poolInfo(i);

    let lpSymbol = "LP";
    let lpName = "Unknown LP Token";

    try {
      const lpToken = await ethers.getContractAt("IERC20", poolInfo.lpToken);
      lpSymbol = await lpToken.symbol();
      lpName = await lpToken.name();
    } catch (error) {
      // LP token doesn't have symbol/name functions
    }

    const rewardShare =
      totalAllocPoint > 0n ? (Number(poolInfo.allocPoint) / Number(totalAllocPoint)) * 100 : 0;

    // Calculate LP price and TVL
    let lpPrice = 0;
    let tvlUSD = 0;

    if (isMainnet) {
      try {
        tvlUSD = await calculateTVL(poolInfo.lpToken, poolInfo.totalStaked, ethers.provider);
        lpPrice = tvlUSD / (Number(poolInfo.totalStaked) / 1e18);
        totalTVL += tvlUSD;
      } catch (error) {
        console.log(`   ⚠️  Could not calculate TVL for pool ${i} (price feed not available)`);
      }
    }

    const poolData: PoolData = {
      poolId: i,
      lpToken: poolInfo.lpToken,
      lpSymbol,
      lpName,
      totalStaked: poolInfo.totalStaked,
      allocPoint: poolInfo.allocPoint,
      rewardShare,
      lpPrice,
      tvlUSD,
    };

    pools.push(poolData);

    console.log(`Pool ${i}: ${lpSymbol}`);
    console.log(`  Name:           ${lpName}`);
    console.log(`  LP Token:       ${poolInfo.lpToken}`);
    console.log(`  Staked:         ${ethers.formatEther(poolInfo.totalStaked)} LP`);
    console.log(`  Alloc Points:   ${poolInfo.allocPoint} (${rewardShare.toFixed(2)}% of rewards)`);

    if (isMainnet && lpPrice > 0) {
      console.log(`  LP Price:       $${lpPrice.toFixed(4)}`);
      console.log(`  TVL (USD):      $${tvlUSD.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`);

      // Calculate APR
      const blocksPerYear = 28800 * 365;
      const yearlyRewards = rewardPerBlock * BigInt(blocksPerYear);
      const poolYearlyRewards = (yearlyRewards * poolInfo.allocPoint) / totalAllocPoint;

      if (tvlUSD > 0) {
        const apr = calculateAPR(poolYearlyRewards, etrPrice, tvlUSD);
        console.log(`  APR:            ${apr.toFixed(2)}%`);
      }
    } else {
      console.log(`  LP Price:       N/A`);
      console.log(`  TVL (USD):      N/A`);
    }

    console.log(`  Last Update:    Block ${poolInfo.lastRewardBlock}\n`);
  }

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📊 SUMMARY\n");

  const totalStakedLP = pools.reduce((sum, pool) => sum + pool.totalStaked, 0n);
  console.log(`   Total LP Staked: ${ethers.formatEther(totalStakedLP)} LP tokens`);

  if (isMainnet && totalTVL > 0) {
    console.log(
      `   Total TVL:       $${totalTVL.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}\n`
    );
  } else {
    console.log(`   Total TVL:       N/A (${isTestnet ? "testnet" : "price feeds unavailable"})\n`);
  }

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Export data
  const exportData = {
    timestamp: new Date().toISOString(),
    network: isMainnet ? "mainnet" : "testnet",
    chainId: Number(network.chainId),
    blockNumber,
    masterChef: masterChefAddress,
    prices: {
      bnb: bnbPrice,
      etr: etrPrice,
    },
    overview: {
      totalPools: Number(poolLength),
      totalAllocPoint: Number(totalAllocPoint),
      rewardPerBlock: ethers.formatEther(rewardPerBlock),
      totalStakedLP: ethers.formatEther(totalStakedLP),
      totalTVLUSD: totalTVL > 0 ? totalTVL : null,
    },
    pools: pools.map((p) => ({
      poolId: p.poolId,
      lpToken: p.lpToken,
      lpSymbol: p.lpSymbol,
      lpName: p.lpName,
      totalStaked: ethers.formatEther(p.totalStaked),
      allocPoint: Number(p.allocPoint),
      rewardShare: p.rewardShare,
      lpPrice: p.lpPrice > 0 ? p.lpPrice : null,
      tvlUSD: p.tvlUSD > 0 ? p.tvlUSD : null,
    })),
  };

  const fs = require("fs");
  const path = require("path");
  const outputPath = path.join(__dirname, `../tvl-with-prices-${Date.now()}.json`);
  fs.writeFileSync(outputPath, JSON.stringify(exportData, null, 2));

  console.log(`📝 Report saved to: ${path.basename(outputPath)}\n`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Monitor failed:");
    console.error(error);
    process.exit(1);
  });
