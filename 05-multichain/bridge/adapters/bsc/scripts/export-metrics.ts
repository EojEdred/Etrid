import { ethers } from "hardhat";

/**
 * Export Metrics for Dashboards
 *
 * Exports comprehensive metrics in formats suitable for:
 * - Grafana dashboards
 * - Analytics tools
 * - Website displays
 * - Historical tracking
 *
 * Outputs:
 * - JSON (for APIs)
 * - CSV (for spreadsheets)
 * - Prometheus format (for monitoring)
 *
 * Usage:
 *   npx hardhat run scripts/export-metrics.ts --network bscTestnet
 *   npx hardhat run scripts/export-metrics.ts --network bscMainnet
 */

interface Metrics {
  timestamp: string;
  blockNumber: number;
  network: string;
  contracts: {
    etrToken: string;
    masterChef: string;
  };
  masterchef: {
    totalPools: number;
    totalAllocPoint: string;
    rewardPerBlock: string;
    paused: boolean;
    owner: string;
  };
  emissions: {
    perBlock: string;
    perDay: string;
    perMonth: string;
    perYear: string;
  };
  balance: {
    masterChefETR: string;
    daysRemaining: number;
  };
  pools: Array<{
    poolId: number;
    lpToken: string;
    lpSymbol: string;
    lpName: string;
    allocPoint: string;
    rewardShare: number;
    totalStaked: string;
    lastRewardBlock: number;
    dailyRewards: string;
    monthlyRewards: string;
  }>;
}

async function exportJSON(metrics: Metrics, outputDir: string) {
  const fs = require("fs");
  const path = require("path");

  const filePath = path.join(outputDir, `metrics-${Date.now()}.json`);
  fs.writeFileSync(filePath, JSON.stringify(metrics, null, 2));

  console.log(`✅ JSON exported: ${path.basename(filePath)}`);
  return filePath;
}

async function exportCSV(metrics: Metrics, outputDir: string) {
  const fs = require("fs");
  const path = require("path");

  // Pool metrics CSV
  const poolCSV = [
    "timestamp,network,poolId,lpToken,lpSymbol,allocPoint,rewardShare,totalStaked,dailyRewards,monthlyRewards",
    ...metrics.pools.map(p =>
      [
        metrics.timestamp,
        metrics.network,
        p.poolId,
        p.lpToken,
        p.lpSymbol,
        p.allocPoint,
        p.rewardShare.toFixed(2),
        p.totalStaked,
        p.dailyRewards,
        p.monthlyRewards,
      ].join(",")
    ),
  ].join("\n");

  const poolFilePath = path.join(outputDir, `pool-metrics-${Date.now()}.csv`);
  fs.writeFileSync(poolFilePath, poolCSV);

  // Overview metrics CSV
  const overviewCSV = [
    "timestamp,network,blockNumber,totalPools,rewardPerBlock,masterChefBalance,daysRemaining",
    [
      metrics.timestamp,
      metrics.network,
      metrics.blockNumber,
      metrics.masterchef.totalPools,
      metrics.emissions.perBlock,
      metrics.balance.masterChefETR,
      metrics.balance.daysRemaining,
    ].join(","),
  ].join("\n");

  const overviewFilePath = path.join(outputDir, `overview-metrics-${Date.now()}.csv`);
  fs.writeFileSync(overviewFilePath, overviewCSV);

  console.log(`✅ CSV exported: ${path.basename(poolFilePath)}, ${path.basename(overviewFilePath)}`);
}

async function exportPrometheus(metrics: Metrics, outputDir: string) {
  const fs = require("fs");
  const path = require("path");

  const lines: string[] = [];

  // MasterChef metrics
  lines.push("# HELP masterchef_total_pools Total number of LP pools");
  lines.push("# TYPE masterchef_total_pools gauge");
  lines.push(`masterchef_total_pools{network="${metrics.network}"} ${metrics.masterchef.totalPools}`);
  lines.push("");

  lines.push("# HELP masterchef_reward_per_block ÉTR rewards per block");
  lines.push("# TYPE masterchef_reward_per_block gauge");
  lines.push(`masterchef_reward_per_block{network="${metrics.network}"} ${metrics.emissions.perBlock}`);
  lines.push("");

  lines.push("# HELP masterchef_balance_etr MasterChef ÉTR balance");
  lines.push("# TYPE masterchef_balance_etr gauge");
  lines.push(`masterchef_balance_etr{network="${metrics.network}"} ${metrics.balance.masterChefETR}`);
  lines.push("");

  lines.push("# HELP masterchef_days_remaining Days of rewards remaining");
  lines.push("# TYPE masterchef_days_remaining gauge");
  lines.push(`masterchef_days_remaining{network="${metrics.network}"} ${metrics.balance.daysRemaining}`);
  lines.push("");

  lines.push("# HELP masterchef_paused Whether MasterChef is paused (1=paused, 0=active)");
  lines.push("# TYPE masterchef_paused gauge");
  lines.push(`masterchef_paused{network="${metrics.network}"} ${metrics.masterchef.paused ? 1 : 0}`);
  lines.push("");

  // Pool metrics
  lines.push("# HELP pool_total_staked Total LP tokens staked in pool");
  lines.push("# TYPE pool_total_staked gauge");
  metrics.pools.forEach(pool => {
    lines.push(
      `pool_total_staked{network="${metrics.network}",pool_id="${pool.poolId}",lp_symbol="${pool.lpSymbol}"} ${pool.totalStaked}`
    );
  });
  lines.push("");

  lines.push("# HELP pool_daily_rewards Daily ÉTR rewards for pool");
  lines.push("# TYPE pool_daily_rewards gauge");
  metrics.pools.forEach(pool => {
    lines.push(
      `pool_daily_rewards{network="${metrics.network}",pool_id="${pool.poolId}",lp_symbol="${pool.lpSymbol}"} ${pool.dailyRewards}`
    );
  });
  lines.push("");

  lines.push("# HELP pool_reward_share Percentage of total rewards");
  lines.push("# TYPE pool_reward_share gauge");
  metrics.pools.forEach(pool => {
    lines.push(
      `pool_reward_share{network="${metrics.network}",pool_id="${pool.poolId}",lp_symbol="${pool.lpSymbol}"} ${pool.rewardShare}`
    );
  });
  lines.push("");

  const filePath = path.join(outputDir, `metrics-${Date.now()}.prom`);
  fs.writeFileSync(filePath, lines.join("\n"));

  console.log(`✅ Prometheus format exported: ${path.basename(filePath)}`);
}

async function main() {
  console.log("\n📊 EXPORT METRICS FOR DASHBOARDS\n");
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
  const masterChefAddressKey = isMainnet ? "MASTERCHEF_ADDRESS_MAINNET" : "MASTERCHEF_ADDRESS_TESTNET";

  const etrAddress = process.env[etrAddressKey];
  const masterChefAddress = process.env[masterChefAddressKey];

  if (!etrAddress || !masterChefAddress) {
    console.log("❌ ERROR: Contract addresses not found in .env\n");
    process.exit(1);
  }

  console.log("📄 Contracts:");
  console.log(`   ÉTR Token:  ${etrAddress}`);
  console.log(`   MasterChef: ${masterChefAddress}\n`);

  // Get contracts
  const etr = await ethers.getContractAt("EtridToken", etrAddress);
  const masterChef = await ethers.getContractAt("MasterChef", masterChefAddress);

  console.log("🔍 Collecting metrics...\n");

  // Collect MasterChef data
  const poolLength = await masterChef.poolLength();
  const totalAllocPoint = await masterChef.totalAllocPoint();
  const rewardPerBlock = await masterChef.rewardPerBlock();
  const paused = await masterChef.paused();
  const owner = await masterChef.owner();

  const blocksPerDay = 28800;
  const blocksPerMonth = blocksPerDay * 30;
  const blocksPerYear = blocksPerDay * 365;

  const dailyRewards = rewardPerBlock * BigInt(blocksPerDay);
  const monthlyRewards = rewardPerBlock * BigInt(blocksPerMonth);
  const yearlyRewards = rewardPerBlock * BigInt(blocksPerYear);

  const masterChefBalance = await etr.balanceOf(masterChefAddress);
  const daysRemaining = dailyRewards > 0n ? Number(masterChefBalance / dailyRewards) : 0;

  // Collect pool data
  const pools: Metrics["pools"] = [];

  for (let i = 0; i < Number(poolLength); i++) {
    const poolInfo = await masterChef.poolInfo(i);

    let lpSymbol = "LP";
    let lpName = "Unknown LP Token";

    try {
      const lpToken = await ethers.getContractAt("IERC20", poolInfo.lpToken);
      lpSymbol = await lpToken.symbol();
      lpName = await lpToken.name();
    } catch (error) {
      // LP token doesn't have symbol/name
    }

    const rewardShare =
      totalAllocPoint > 0n ? (Number(poolInfo.allocPoint) / Number(totalAllocPoint)) * 100 : 0;

    const poolDailyRewards =
      (dailyRewards * poolInfo.allocPoint) / (totalAllocPoint > 0n ? totalAllocPoint : 1n);
    const poolMonthlyRewards =
      (monthlyRewards * poolInfo.allocPoint) / (totalAllocPoint > 0n ? totalAllocPoint : 1n);

    pools.push({
      poolId: i,
      lpToken: poolInfo.lpToken,
      lpSymbol,
      lpName,
      allocPoint: poolInfo.allocPoint.toString(),
      rewardShare,
      totalStaked: ethers.formatEther(poolInfo.totalStaked),
      lastRewardBlock: Number(poolInfo.lastRewardBlock),
      dailyRewards: ethers.formatEther(poolDailyRewards),
      monthlyRewards: ethers.formatEther(poolMonthlyRewards),
    });
  }

  // Build metrics object
  const metrics: Metrics = {
    timestamp: new Date().toISOString(),
    blockNumber,
    network: isMainnet ? "mainnet" : "testnet",
    contracts: {
      etrToken: etrAddress,
      masterChef: masterChefAddress,
    },
    masterchef: {
      totalPools: Number(poolLength),
      totalAllocPoint: totalAllocPoint.toString(),
      rewardPerBlock: ethers.formatEther(rewardPerBlock),
      paused,
      owner,
    },
    emissions: {
      perBlock: ethers.formatEther(rewardPerBlock),
      perDay: ethers.formatEther(dailyRewards),
      perMonth: ethers.formatEther(monthlyRewards),
      perYear: ethers.formatEther(yearlyRewards),
    },
    balance: {
      masterChefETR: ethers.formatEther(masterChefBalance),
      daysRemaining,
    },
    pools,
  };

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📈 METRICS SUMMARY\n");

  console.log("MasterChef:");
  console.log(`  Total Pools:      ${metrics.masterchef.totalPools}`);
  console.log(`  Reward Rate:      ${metrics.emissions.perBlock} ÉTR/block`);
  console.log(`  Daily Emissions:  ${metrics.emissions.perDay} ÉTR`);
  console.log(`  Balance:          ${metrics.balance.masterChefETR} ÉTR`);
  console.log(`  Days Remaining:   ${metrics.balance.daysRemaining} days`);
  console.log(`  Status:           ${metrics.masterchef.paused ? "PAUSED" : "ACTIVE"}\n`);

  console.log("Pools:");
  metrics.pools.forEach(pool => {
    console.log(`  Pool ${pool.poolId} (${pool.lpSymbol}):`);
    console.log(`    Staked:         ${pool.totalStaked} LP`);
    console.log(`    Reward Share:   ${pool.rewardShare.toFixed(2)}%`);
    console.log(`    Daily Rewards:  ${pool.dailyRewards} ÉTR`);
  });

  console.log("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("💾 Exporting metrics...\n");

  const fs = require("fs");
  const path = require("path");
  const outputDir = path.join(__dirname, "..");

  // Ensure output directory exists
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  // Export in all formats
  await exportJSON(metrics, outputDir);
  await exportCSV(metrics, outputDir);
  await exportPrometheus(metrics, outputDir);

  console.log("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("✅ EXPORT COMPLETE!\n");

  console.log("📊 Integration Options:\n");
  console.log("  JSON Format:");
  console.log("    • Use with REST APIs");
  console.log("    • Import into databases");
  console.log("    • Website/dashboard displays\n");

  console.log("  CSV Format:");
  console.log("    • Import into Excel/Google Sheets");
  console.log("    • Create charts and graphs");
  console.log("    • Historical analysis\n");

  console.log("  Prometheus Format:");
  console.log("    • Integrate with Grafana");
  console.log("    • Set up alerts (e.g., low balance)");
  console.log("    • Real-time monitoring dashboards\n");

  console.log("💡 Next Steps:");
  console.log("  1. Set up automated exports (cron job)");
  console.log("  2. Create Grafana dashboard");
  console.log("  3. Set up alerts for critical metrics");
  console.log("  4. Display metrics on website\n");

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Export failed:");
    console.error(error);
    process.exit(1);
  });
