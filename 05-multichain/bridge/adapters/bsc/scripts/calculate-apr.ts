import { ethers } from "hardhat";
import * as readline from "readline";

/**
 * Calculate APR for MasterChef Pools
 *
 * Calculates real-time APR based on:
 * - Current emission rate
 * - Pool allocation
 * - TVL (requires price input)
 *
 * Usage:
 *   npx hardhat run scripts/calculate-apr.ts --network bscTestnet
 *   npx hardhat run scripts/calculate-apr.ts --network bscMainnet
 */

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

function question(query: string): Promise<string> {
  return new Promise((resolve) => rl.question(query, resolve));
}

interface PoolAPR {
  poolId: number;
  lpSymbol: string;
  lpName: string;
  allocPoint: bigint;
  rewardShare: number;
  dailyRewards: bigint;
  monthlyRewards: bigint;
  yearlyRewards: bigint;
  tvlUSD?: number;
  aprPercent?: number;
}

async function main() {
  console.log("\n📈 MASTERCHEF APR CALCULATOR\n");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Get network info
  const network = await ethers.provider.getNetwork();
  const isMainnet = network.chainId === 56n;
  const isTestnet = network.chainId === 97n;

  console.log(`📍 Network: ${isMainnet ? "BSC Mainnet" : isTestnet ? "BSC Testnet" : "Unknown"}`);
  console.log(`   Chain ID: ${network.chainId}`);

  const blockNumber = await ethers.provider.getBlockNumber();
  console.log(`   Block: ${blockNumber}\n`);

  // Get MasterChef address
  const masterChefAddressKey = isMainnet ? "MASTERCHEF_ADDRESS_MAINNET" : "MASTERCHEF_ADDRESS_TESTNET";
  const masterChefAddress = process.env[masterChefAddressKey];

  if (!masterChefAddress) {
    console.log(`❌ ERROR: ${masterChefAddressKey} not found in .env\n`);
    rl.close();
    process.exit(1);
  }

  console.log(`📄 MasterChef: ${masterChefAddress}\n`);

  // Get MasterChef contract
  const masterChef = await ethers.getContractAt("MasterChef", masterChefAddress);

  // Get basic info
  const poolLength = await masterChef.poolLength();
  const totalAllocPoint = await masterChef.totalAllocPoint();
  const rewardPerBlock = await masterChef.rewardPerBlock();

  const blocksPerDay = 28800; // BSC: 3 second blocks
  const blocksPerMonth = blocksPerDay * 30;
  const blocksPerYear = blocksPerDay * 365;

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📊 EMISSION INFO\n");
  console.log(`   Reward Rate:     ${ethers.formatEther(rewardPerBlock)} ÉTR/block`);
  console.log(`   Blocks per Day:  ${blocksPerDay.toLocaleString()}`);
  console.log(`   Total Pools:     ${poolLength}`);
  console.log(`   Total Alloc:     ${totalAllocPoint}\n`);

  const totalDailyRewards = rewardPerBlock * BigInt(blocksPerDay);
  const totalMonthlyRewards = rewardPerBlock * BigInt(blocksPerMonth);
  const totalYearlyRewards = rewardPerBlock * BigInt(blocksPerYear);

  console.log("💰 TOTAL EMISSIONS:\n");
  console.log(`   Daily:    ${ethers.formatEther(totalDailyRewards).toLocaleString()} ÉTR`);
  console.log(`   Monthly:  ${ethers.formatEther(totalMonthlyRewards).toLocaleString()} ÉTR`);
  console.log(`   Yearly:   ${ethers.formatEther(totalYearlyRewards).toLocaleString()} ÉTR\n`);

  if (poolLength === 0n) {
    console.log("⚠️  No pools added yet\n");
    rl.close();
    process.exit(0);
  }

  // Fetch all pool data
  const pools: PoolAPR[] = [];

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📋 POOL REWARDS BREAKDOWN\n");

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

    const rewardShare = totalAllocPoint > 0n
      ? (Number(poolInfo.allocPoint) / Number(totalAllocPoint))
      : 0;

    const poolDailyRewards = totalDailyRewards * BigInt(Math.floor(rewardShare * 1000000)) / 1000000n;
    const poolMonthlyRewards = totalMonthlyRewards * BigInt(Math.floor(rewardShare * 1000000)) / 1000000n;
    const poolYearlyRewards = totalYearlyRewards * BigInt(Math.floor(rewardShare * 1000000)) / 1000000n;

    const poolData: PoolAPR = {
      poolId: i,
      lpSymbol,
      lpName,
      allocPoint: poolInfo.allocPoint,
      rewardShare: rewardShare * 100,
      dailyRewards: poolDailyRewards,
      monthlyRewards: poolMonthlyRewards,
      yearlyRewards: poolYearlyRewards,
    };

    pools.push(poolData);

    console.log(`Pool ${i}: ${lpSymbol}`);
    console.log(`  Reward Share:   ${(rewardShare * 100).toFixed(2)}%`);
    console.log(`  Daily Rewards:  ${ethers.formatEther(poolDailyRewards).toLocaleString()} ÉTR`);
    console.log(`  Monthly:        ${ethers.formatEther(poolMonthlyRewards).toLocaleString()} ÉTR`);
    console.log(`  Yearly:         ${ethers.formatEther(poolYearlyRewards).toLocaleString()} ÉTR\n`);
  }

  // Ask for price data to calculate APR
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("💵 PRICE DATA (for APR calculation)\n");
  console.log("To calculate APR, we need:\n");
  console.log("  1. ÉTR token price (USD)");
  console.log("  2. Pool TVL (USD) for each pool\n");

  const calculateAPR = await question("Do you have this data? (yes/no): ");

  if (calculateAPR.toLowerCase() === "yes") {
    console.log();
    const etrPriceStr = await question("Enter ÉTR price in USD (e.g., 0.01): $");
    const etrPrice = parseFloat(etrPriceStr);

    if (isNaN(etrPrice) || etrPrice <= 0) {
      console.log("\n❌ Invalid ÉTR price\n");
      rl.close();
      process.exit(1);
    }

    console.log("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    console.log("📊 APR CALCULATIONS\n");

    for (let i = 0; i < pools.length; i++) {
      const pool = pools[i];

      console.log(`Pool ${pool.poolId}: ${pool.lpSymbol}`);

      const tvlStr = await question(`  Enter TVL in USD (e.g., 50000): $`);
      const tvl = parseFloat(tvlStr.replace(/,/g, ""));

      if (isNaN(tvl) || tvl <= 0) {
        console.log(`  ⚠️  Invalid TVL - skipping APR calculation\n`);
        continue;
      }

      pool.tvlUSD = tvl;

      // APR = (Yearly Rewards × ÉTR Price) / TVL × 100
      const yearlyRewardsFloat = parseFloat(ethers.formatEther(pool.yearlyRewards));
      const yearlyRewardsUSD = yearlyRewardsFloat * etrPrice;
      const apr = (yearlyRewardsUSD / tvl) * 100;

      pool.aprPercent = apr;

      console.log(`  TVL:            $${tvl.toLocaleString()}`);
      console.log(`  Yearly Rewards: ${yearlyRewardsFloat.toLocaleString()} ÉTR ($${yearlyRewardsUSD.toLocaleString()})`);
      console.log(`  APR:            ${apr.toFixed(2)}%\n`);
    }

    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    console.log("📈 APR SUMMARY\n");

    for (const pool of pools) {
      if (pool.aprPercent) {
        console.log(`  ${pool.lpSymbol}: ${pool.aprPercent.toFixed(2)}% APR`);
      }
    }
    console.log();

  } else {
    console.log("\n💡 TIP: To calculate APR manually:\n");
    console.log("   APR = (Yearly ÉTR Rewards × ÉTR Price) / Pool TVL × 100\n");
    console.log("   Example:");
    console.log("   - Yearly Rewards: 10,000,000 ÉTR");
    console.log("   - ÉTR Price: $0.01");
    console.log("   - TVL: $50,000");
    console.log("   - APR = (10,000,000 × $0.01) / $50,000 × 100 = 200%\n");
  }

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Export data
  const exportData = {
    timestamp: new Date().toISOString(),
    network: isMainnet ? "mainnet" : "testnet",
    chainId: Number(network.chainId),
    blockNumber,
    masterChef: masterChefAddress,
    emissions: {
      rewardPerBlock: ethers.formatEther(rewardPerBlock),
      dailyTotal: ethers.formatEther(totalDailyRewards),
      monthlyTotal: ethers.formatEther(totalMonthlyRewards),
      yearlyTotal: ethers.formatEther(totalYearlyRewards),
    },
    pools: pools.map(p => ({
      poolId: p.poolId,
      lpSymbol: p.lpSymbol,
      lpName: p.lpName,
      allocPoint: Number(p.allocPoint),
      rewardShare: p.rewardShare,
      dailyRewards: ethers.formatEther(p.dailyRewards),
      monthlyRewards: ethers.formatEther(p.monthlyRewards),
      yearlyRewards: ethers.formatEther(p.yearlyRewards),
      tvlUSD: p.tvlUSD || null,
      aprPercent: p.aprPercent || null,
    })),
  };

  const fs = require("fs");
  const path = require("path");
  const outputPath = path.join(__dirname, `../apr-report-${Date.now()}.json`);
  fs.writeFileSync(outputPath, JSON.stringify(exportData, null, 2));

  console.log(`📝 Report saved to: ${path.basename(outputPath)}\n`);

  rl.close();
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Calculator failed:");
    console.error(error);
    rl.close();
    process.exit(1);
  });
