import {
  getDatabase,
  getLatestMetrics,
  getLatestPools,
  getTVLHistory,
  getPoolTVLHistory,
  getAPRHistory,
  getRecentEvents,
  getDatabaseStats,
  closeDatabase,
} from "./lib/database";

/**
 * Query Historical Data
 *
 * Interactive script to query and analyze historical metrics
 *
 * Usage:
 *   npx ts-node scripts/query-history.ts
 */

const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

function question(query: string): Promise<string> {
  return new Promise((resolve) => rl.question(query, resolve));
}

async function main() {
  console.log("\n📊 HISTORICAL DATA QUERY TOOL\n");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  const db = getDatabase();

  // Show database stats
  const stats = getDatabaseStats();
  console.log("📈 Database Statistics:\n");
  console.log(`   Metrics Snapshots: ${stats.metrics_snapshots.toLocaleString()}`);
  console.log(`   Pool Snapshots:    ${stats.pool_snapshots.toLocaleString()}`);
  console.log(`   Events:            ${stats.events.toLocaleString()}`);
  console.log(`   Health Checks:     ${stats.health_checks.toLocaleString()}`);
  console.log(`   Alerts:            ${stats.alerts.toLocaleString()}`);
  console.log(`   Backups:           ${stats.backups.toLocaleString()}\n`);

  if (stats.metrics_snapshots === 0) {
    console.log("⚠️  No data in database yet.");
    console.log("   Run: npm run collect-metrics:mainnet\n");
    rl.close();
    closeDatabase();
    return;
  }

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📋 QUERY OPTIONS\n");
  console.log("1. Latest metrics");
  console.log("2. TVL history");
  console.log("3. Pool TVL history");
  console.log("4. APR history");
  console.log("5. Recent events");
  console.log("6. Export data\n");

  const choice = await question("Select option (1-6): ");

  console.log();

  switch (choice) {
    case "1":
      await showLatestMetrics();
      break;
    case "2":
      await showTVLHistory();
      break;
    case "3":
      await showPoolTVLHistory();
      break;
    case "4":
      await showAPRHistory();
      break;
    case "5":
      await showRecentEvents();
      break;
    case "6":
      await exportData();
      break;
    default:
      console.log("Invalid option\n");
  }

  rl.close();
  closeDatabase();
}

async function showLatestMetrics() {
  const network = await question("Network (mainnet/testnet): ");
  console.log();

  const metrics = getLatestMetrics(network);

  if (!metrics) {
    console.log(`No metrics found for ${network}\n`);
    return;
  }

  console.log("📊 LATEST METRICS\n");
  console.log(`   Timestamp:        ${new Date(metrics.timestamp).toLocaleString()}`);
  console.log(`   Block:            ${metrics.block_number.toLocaleString()}`);
  console.log(`   Total Pools:      ${metrics.total_pools}`);
  console.log(`   Reward/Block:     ${parseFloat(metrics.reward_per_block).toFixed(2)} ÉTR`);
  console.log(`   MasterChef Balance: ${parseFloat(metrics.masterchef_balance).toLocaleString()} ÉTR`);
  console.log(`   Days Remaining:   ${metrics.days_remaining}`);
  console.log(`   Paused:           ${metrics.is_paused ? "Yes" : "No"}`);

  if (metrics.bnb_price) {
    console.log(`   BNB Price:        $${metrics.bnb_price.toFixed(2)}`);
  }

  if (metrics.etr_price) {
    console.log(`   ÉTR Price:        $${metrics.etr_price.toFixed(6)}`);
  }

  if (metrics.total_tvl_usd) {
    console.log(`   Total TVL:        $${metrics.total_tvl_usd.toLocaleString()}`);
  }

  console.log();

  const pools = getLatestPools(network);
  console.log(`📋 POOLS (${pools.length}):\n`);

  for (const pool of pools) {
    console.log(`   Pool ${pool.pool_id}: ${pool.lp_symbol}`);
    console.log(`     Staked: ${parseFloat(pool.total_staked).toLocaleString()} LP`);
    console.log(`     Share:  ${pool.reward_share.toFixed(1)}%`);

    if (pool.tvl_usd) {
      console.log(`     TVL:    $${pool.tvl_usd.toLocaleString()}`);
    }

    if (pool.apr_percent) {
      console.log(`     APR:    ${pool.apr_percent.toFixed(2)}%`);
    }

    console.log();
  }
}

async function showTVLHistory() {
  const network = await question("Network (mainnet/testnet): ");
  const daysStr = await question("Days of history (default: 30): ");
  const days = daysStr ? parseInt(daysStr) : 30;

  console.log();

  const history = getTVLHistory(network, days);

  if (history.length === 0) {
    console.log(`No TVL history found for ${network}\n`);
    return;
  }

  console.log(`📈 TVL HISTORY (Last ${days} days)\n`);

  // Show summary
  const tvls = history.map((h) => h.total_tvl_usd);
  const min = Math.min(...tvls);
  const max = Math.max(...tvls);
  const avg = tvls.reduce((sum, tvl) => sum + tvl, 0) / tvls.length;
  const latest = tvls[tvls.length - 1];

  console.log("   Summary:");
  console.log(`     Current: $${latest.toLocaleString()}`);
  console.log(`     Average: $${avg.toLocaleString()}`);
  console.log(`     Min:     $${min.toLocaleString()}`);
  console.log(`     Max:     $${max.toLocaleString()}`);
  console.log(`     Points:  ${history.length}\n`);

  // Show recent data points
  console.log("   Recent Data Points:\n");
  history.slice(-10).forEach((point) => {
    const date = new Date(point.timestamp);
    console.log(`     ${date.toLocaleString()}: $${point.total_tvl_usd.toLocaleString()}`);
  });

  console.log();
}

async function showPoolTVLHistory() {
  const network = await question("Network (mainnet/testnet): ");
  const poolIdStr = await question("Pool ID: ");
  const poolId = parseInt(poolIdStr);
  const daysStr = await question("Days of history (default: 30): ");
  const days = daysStr ? parseInt(daysStr) : 30;

  console.log();

  const history = getPoolTVLHistory(network, poolId, days);

  if (history.length === 0) {
    console.log(`No TVL history found for pool ${poolId} on ${network}\n`);
    return;
  }

  console.log(`📈 POOL ${poolId} TVL HISTORY (Last ${days} days)\n`);

  // Show summary
  const tvls = history.map((h) => h.tvl_usd);
  const min = Math.min(...tvls);
  const max = Math.max(...tvls);
  const avg = tvls.reduce((sum, tvl) => sum + tvl, 0) / tvls.length;
  const latest = tvls[tvls.length - 1];

  console.log("   Summary:");
  console.log(`     Current: $${latest.toLocaleString()}`);
  console.log(`     Average: $${avg.toLocaleString()}`);
  console.log(`     Min:     $${min.toLocaleString()}`);
  console.log(`     Max:     $${max.toLocaleString()}`);
  console.log(`     Points:  ${history.length}\n`);

  // Show recent data points
  console.log("   Recent Data Points:\n");
  history.slice(-10).forEach((point) => {
    const date = new Date(point.timestamp);
    const apr = point.apr_percent ? ` (APR: ${point.apr_percent.toFixed(2)}%)` : "";
    console.log(`     ${date.toLocaleString()}: $${point.tvl_usd.toLocaleString()}${apr}`);
  });

  console.log();
}

async function showAPRHistory() {
  const network = await question("Network (mainnet/testnet): ");
  const poolIdStr = await question("Pool ID: ");
  const poolId = parseInt(poolIdStr);
  const daysStr = await question("Days of history (default: 30): ");
  const days = daysStr ? parseInt(daysStr) : 30;

  console.log();

  const history = getAPRHistory(network, poolId, days);

  if (history.length === 0) {
    console.log(`No APR history found for pool ${poolId} on ${network}\n`);
    return;
  }

  console.log(`📊 POOL ${poolId} APR HISTORY (Last ${days} days)\n`);

  // Show summary
  const aprs = history.map((h) => h.apr_percent);
  const min = Math.min(...aprs);
  const max = Math.max(...aprs);
  const avg = aprs.reduce((sum, apr) => sum + apr, 0) / aprs.length;
  const latest = aprs[aprs.length - 1];

  console.log("   Summary:");
  console.log(`     Current: ${latest.toFixed(2)}%`);
  console.log(`     Average: ${avg.toFixed(2)}%`);
  console.log(`     Min:     ${min.toFixed(2)}%`);
  console.log(`     Max:     ${max.toFixed(2)}%`);
  console.log(`     Points:  ${history.length}\n`);

  // Show recent data points
  console.log("   Recent Data Points:\n");
  history.slice(-10).forEach((point) => {
    const date = new Date(point.timestamp);
    console.log(`     ${date.toLocaleString()}: ${point.apr_percent.toFixed(2)}%`);
  });

  console.log();
}

async function showRecentEvents() {
  const network = await question("Network (mainnet/testnet): ");
  const limitStr = await question("Number of events (default: 50): ");
  const limit = limitStr ? parseInt(limitStr) : 50;

  console.log();

  const events = getRecentEvents(network, limit);

  if (events.length === 0) {
    console.log(`No events found for ${network}\n`);
    return;
  }

  console.log(`📋 RECENT EVENTS (Last ${events.length})\n`);

  events.forEach((event) => {
    const date = new Date(event.timestamp);
    console.log(`   ${date.toLocaleString()} - ${event.event_type}`);

    if (event.pool_id !== null) {
      console.log(`     Pool: ${event.pool_id}`);
    }

    if (event.user_address) {
      console.log(`     User: ${event.user_address}`);
    }

    if (event.amount) {
      console.log(`     Amount: ${event.amount}`);
    }

    if (event.tx_hash) {
      console.log(`     Tx: ${event.tx_hash}`);
    }

    console.log();
  });
}

async function exportData() {
  console.log("📤 EXPORT DATA\n");
  console.log("Feature coming soon...\n");
  console.log("You can query the database directly:");
  console.log("   sqlite3 database/masterchef.db\n");
}

main().catch((error) => {
  console.error("\n❌ Query failed:");
  console.error(error);
  rl.close();
  closeDatabase();
  process.exit(1);
});
