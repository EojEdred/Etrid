import { ethers } from "hardhat";

/**
 * Monitor Total Value Locked (TVL)
 *
 * Tracks TVL across all MasterChef pools in real-time
 *
 * Usage:
 *   npx hardhat run scripts/monitor-tvl.ts --network bscTestnet
 *   npx hardhat run scripts/monitor-tvl.ts --network bscMainnet
 */

interface PoolTVL {
  poolId: number;
  lpToken: string;
  lpSymbol: string;
  lpName: string;
  totalStaked: bigint;
  allocPoint: bigint;
  rewardShare: number;
  lpPrice?: number;
  tvlUSD?: number;
}

async function getPriceFromPancakeSwap(tokenAddress: string): Promise<number | undefined> {
  try {
    // This would normally call PancakeSwap Router or price oracle
    // For now, return undefined (price must be provided manually or via API)
    return undefined;
  } catch (error) {
    return undefined;
  }
}

async function main() {
  console.log("\n📊 MASTERCHEF TVL MONITOR\n");
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
    process.exit(1);
  }

  console.log(`📄 MasterChef: ${masterChefAddress}\n`);

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

  // Fetch all pool data
  const pools: PoolTVL[] = [];
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

    const rewardShare = totalAllocPoint > 0n
      ? (Number(poolInfo.allocPoint) / Number(totalAllocPoint)) * 100
      : 0;

    const poolData: PoolTVL = {
      poolId: i,
      lpToken: poolInfo.lpToken,
      lpSymbol,
      lpName,
      totalStaked: poolInfo.totalStaked,
      allocPoint: poolInfo.allocPoint,
      rewardShare,
    };

    pools.push(poolData);

    console.log(`Pool ${i}: ${lpSymbol}`);
    console.log(`  Name:           ${lpName}`);
    console.log(`  LP Token:       ${poolInfo.lpToken}`);
    console.log(`  Staked:         ${ethers.formatEther(poolInfo.totalStaked)} LP`);
    console.log(`  Alloc Points:   ${poolInfo.allocPoint} (${rewardShare.toFixed(2)}% of rewards)`);
    console.log(`  Last Update:    Block ${poolInfo.lastRewardBlock}`);

    // Placeholder for TVL calculation (requires LP token price)
    console.log(`  TVL (USD):      N/A (requires LP price feed)\n`);
  }

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📊 SUMMARY\n");

  const totalStakedLP = pools.reduce((sum, pool) => sum + pool.totalStaked, 0n);
  console.log(`   Total LP Staked: ${ethers.formatEther(totalStakedLP)} LP tokens\n`);

  if (totalTVL > 0) {
    console.log(`   Total TVL:       $${totalTVL.toLocaleString()}\n`);
  } else {
    console.log(`   Total TVL:       N/A (requires price feed integration)\n`);
  }

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("💡 NOTES:\n");
  console.log("   • LP token prices need to be fetched from PancakeSwap or DEX");
  console.log("   • For accurate TVL, integrate with price oracle (e.g., Chainlink)");
  console.log("   • TVL = (LP tokens staked × LP token price)");
  console.log("   • LP token price = (Reserve0 + Reserve1) / Total LP Supply\n");

  console.log("🔧 TO GET ACCURATE TVL:\n");
  console.log("   1. Fetch LP token reserves from PancakeSwap pair contract");
  console.log("   2. Get token prices (ÉTR, BNB, etc.) from price feed");
  console.log("   3. Calculate: LP price = (reserve0 × price0 + reserve1 × price1) / totalSupply");
  console.log("   4. Calculate: TVL = staked LP × LP price\n");

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Export data
  const exportData = {
    timestamp: new Date().toISOString(),
    network: isMainnet ? "mainnet" : "testnet",
    chainId: Number(network.chainId),
    blockNumber,
    masterChef: masterChefAddress,
    overview: {
      totalPools: Number(poolLength),
      totalAllocPoint: Number(totalAllocPoint),
      rewardPerBlock: ethers.formatEther(rewardPerBlock),
      totalStakedLP: ethers.formatEther(totalStakedLP),
      totalTVLUSD: totalTVL > 0 ? totalTVL : null,
    },
    pools: pools.map(p => ({
      poolId: p.poolId,
      lpToken: p.lpToken,
      lpSymbol: p.lpSymbol,
      lpName: p.lpName,
      totalStaked: ethers.formatEther(p.totalStaked),
      allocPoint: Number(p.allocPoint),
      rewardShare: p.rewardShare,
      tvlUSD: p.tvlUSD || null,
    })),
  };

  const fs = require("fs");
  const path = require("path");
  const outputPath = path.join(__dirname, `../tvl-report-${Date.now()}.json`);
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
