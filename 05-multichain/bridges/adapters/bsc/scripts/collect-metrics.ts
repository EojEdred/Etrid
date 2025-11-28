import { ethers } from "hardhat";
import {
  getDatabase,
  saveMetricsSnapshot,
  savePoolSnapshot,
  closeDatabase,
  logEvent,
} from "./lib/database";
import { getBNBPriceUSD, getTokenPriceUSD, calculateTVL, calculateAPR } from "./lib/priceFeeds";

/**
 * Collect and Store Metrics to Database
 *
 * Runs periodically (via cron) to collect historical data
 *
 * Usage:
 *   npx hardhat run scripts/collect-metrics.ts --network bscTestnet
 *   npx hardhat run scripts/collect-metrics.ts --network bscMainnet
 */

async function main() {
  console.log("\n📊 COLLECTING METRICS FOR HISTORICAL STORAGE\n");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Get network info
  const network = await ethers.provider.getNetwork();
  const isMainnet = network.chainId === 56n;
  const isTestnet = network.chainId === 97n;
  const networkName = isMainnet ? "mainnet" : "testnet";

  console.log(`📍 Network: ${isMainnet ? "BSC Mainnet" : isTestnet ? "BSC Testnet" : "Unknown"}`);

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

  // Get contracts
  const etr = await ethers.getContractAt("EtridToken", etrAddress);
  const masterChef = await ethers.getContractAt("MasterChef", masterChefAddress);

  console.log("🔍 Fetching on-chain data...\n");

  // Get MasterChef state
  const poolLength = await masterChef.poolLength();
  const totalAllocPoint = await masterChef.totalAllocPoint();
  const rewardPerBlock = await masterChef.rewardPerBlock();
  const isPaused = await masterChef.paused();
  const masterChefBalance = await etr.balanceOf(masterChefAddress);

  // Calculate days remaining
  const blocksPerDay = 28800;
  const dailyRewards = rewardPerBlock * BigInt(blocksPerDay);
  const daysRemaining = dailyRewards > 0n ? Number(masterChefBalance / dailyRewards) : 0;

  // Get prices (if mainnet)
  let bnbPrice: number | undefined;
  let etrPrice: number | undefined;

  if (isMainnet) {
    try {
      bnbPrice = await getBNBPriceUSD(ethers.provider);
      console.log(`   BNB Price: $${bnbPrice.toFixed(2)}`);
    } catch (error) {
      console.log(`   ⚠️  Could not fetch BNB price`);
    }

    try {
      etrPrice = await getTokenPriceUSD(etrAddress, ethers.provider);
      console.log(`   ÉTR Price: $${etrPrice.toFixed(6)}`);
    } catch (error) {
      console.log(`   ⚠️  Could not fetch ÉTR price`);
    }
  }

  console.log();

  // Collect pool data
  const pools: any[] = [];
  let totalStakedLP = 0n;
  let totalTVL = 0;

  for (let i = 0; i < Number(poolLength); i++) {
    const poolInfo = await masterChef.poolInfo(i);

    let lpSymbol = "LP";
    let lpName = "Unknown LP Token";

    try {
      const lpToken = await ethers.getContractAt("IERC20", poolInfo.lpToken);
      lpSymbol = await lpToken.symbol();
      lpName = await lpToken.name();
    } catch (error) {
      // Skip if can't read symbol/name
    }

    const rewardShare =
      totalAllocPoint > 0n ? (Number(poolInfo.allocPoint) / Number(totalAllocPoint)) * 100 : 0;

    // Calculate rewards
    const blocksPerYear = 28800 * 365;
    const yearlyRewards = rewardPerBlock * BigInt(blocksPerYear);
    const poolYearlyRewards = (yearlyRewards * poolInfo.allocPoint) / totalAllocPoint;
    const poolDailyRewards = (dailyRewards * poolInfo.allocPoint) / totalAllocPoint;
    const poolMonthlyRewards = poolDailyRewards * 30n;

    totalStakedLP += poolInfo.totalStaked;

    // Calculate TVL and APR (if mainnet and prices available)
    let lpPrice: number | undefined;
    let tvlUSD: number | undefined;
    let apr: number | undefined;

    if (isMainnet && etrPrice) {
      try {
        tvlUSD = await calculateTVL(poolInfo.lpToken, poolInfo.totalStaked, ethers.provider);
        lpPrice = tvlUSD / (Number(poolInfo.totalStaked) / 1e18);

        if (tvlUSD > 0) {
          apr = calculateAPR(poolYearlyRewards, etrPrice, tvlUSD);
        }

        totalTVL += tvlUSD;
      } catch (error) {
        // Price feed not available
      }
    }

    pools.push({
      pool_id: i,
      lp_token: poolInfo.lpToken,
      lp_symbol: lpSymbol,
      lp_name: lpName,
      total_staked: ethers.formatEther(poolInfo.totalStaked),
      alloc_point: poolInfo.allocPoint.toString(),
      reward_share: rewardShare,
      lp_price: lpPrice,
      tvl_usd: tvlUSD,
      apr_percent: apr,
      daily_rewards: ethers.formatEther(poolDailyRewards),
      monthly_rewards: ethers.formatEther(poolMonthlyRewards),
    });
  }

  console.log("💾 Saving to database...\n");

  // Initialize database
  const db = getDatabase();
  const timestamp = new Date().toISOString();

  try {
    // Save metrics snapshot
    const snapshotId = saveMetricsSnapshot({
      timestamp,
      network: networkName,
      block_number: blockNumber,
      total_pools: Number(poolLength),
      reward_per_block: ethers.formatEther(rewardPerBlock),
      total_alloc_point: totalAllocPoint.toString(),
      masterchef_balance: ethers.formatEther(masterChefBalance),
      days_remaining: daysRemaining,
      is_paused: isPaused,
      bnb_price: bnbPrice,
      etr_price: etrPrice,
      total_tvl_usd: totalTVL > 0 ? totalTVL : undefined,
      total_staked_lp: ethers.formatEther(totalStakedLP),
    });

    console.log(`   ✅ Saved metrics snapshot (ID: ${snapshotId})`);

    // Save pool snapshots
    for (const pool of pools) {
      savePoolSnapshot({
        snapshot_id: snapshotId,
        timestamp,
        network: networkName,
        ...pool,
      });
    }

    console.log(`   ✅ Saved ${pools.length} pool snapshot(s)`);

    // Log collection event
    logEvent({
      network: networkName,
      event_type: "metrics_collected",
      block_number: blockNumber,
      details: {
        snapshot_id: snapshotId,
        pools: pools.length,
        tvl: totalTVL,
      },
    });

    console.log(`   ✅ Logged collection event\n`);

    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    console.log("✅ METRICS COLLECTION COMPLETE\n");

    console.log("📊 Summary:");
    console.log(`   Snapshot ID: ${snapshotId}`);
    console.log(`   Pools: ${pools.length}`);
    console.log(`   Total TVL: ${totalTVL > 0 ? `$${totalTVL.toLocaleString()}` : "N/A"}`);
    console.log(`   Total Staked: ${ethers.formatEther(totalStakedLP)} LP`);
    console.log(`   Days Remaining: ${daysRemaining} days\n`);
  } catch (error) {
    console.error("\n❌ Error saving to database:");
    console.error(error);
    process.exit(1);
  } finally {
    closeDatabase();
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Collection failed:");
    console.error(error);
    process.exit(1);
  });
