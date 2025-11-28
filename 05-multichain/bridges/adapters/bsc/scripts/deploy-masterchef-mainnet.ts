import { ethers } from "hardhat";
import * as readline from "readline";

/**
 * Deploy MasterChef LP Rewards Contract to BSC Mainnet
 *
 * Usage:
 *   npm run deploy:masterchef:mainnet
 *
 * Prerequisites:
 *   1. ÉTR token deployed on mainnet
 *   2. Wallet funded with REAL BNB (~$10-15 for gas)
 *   3. Successfully tested MasterChef on testnet
 *   4. 20M ÉTR ready to transfer to contract
 *
 * ⚠️ WARNING: THIS COSTS REAL MONEY!
 */

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

function question(query: string): Promise<string> {
  return new Promise((resolve) => {
    rl.question(query, resolve);
  });
}

async function main() {
  console.log("\n⚠️  MASTERCHEF MAINNET DEPLOYMENT ⚠️\n");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Get deployer
  const [deployer] = await ethers.getSigners();
  const deployerAddress = await deployer.getAddress();

  console.log("📍 Deployment Details:");
  console.log(`  Deployer: ${deployerAddress}`);

  // Check balance
  const balance = await ethers.provider.getBalance(deployerAddress);
  const balanceInBNB = ethers.formatEther(balance);
  console.log(`  Balance:  ${balanceInBNB} BNB`);

  // Estimate cost
  const gasPrice = (await ethers.provider.getFeeData()).gasPrice!;
  const estimatedGas = 3000000n; // ~3M gas for MasterChef
  const estimatedCost = gasPrice * estimatedGas;
  const estimatedCostBNB = ethers.formatEther(estimatedCost);

  console.log(`  Gas Price: ${ethers.formatUnits(gasPrice, "gwei")} gwei`);
  console.log(`  Est. Cost: ~${estimatedCostBNB} BNB (~$${(parseFloat(estimatedCostBNB) * 600).toFixed(2)} USD @ $600/BNB)`);

  if (balance < estimatedCost * 2n) {
    console.log("\n❌ Insufficient balance!");
    console.log(`   Required: ~${ethers.formatEther(estimatedCost * 2n)} BNB (2x buffer)`);
    rl.close();
    process.exit(1);
  }

  // Get network info
  const network = await ethers.provider.getNetwork();
  console.log(`  Network:  BSC Mainnet (Chain ID: ${network.chainId})`);

  if (network.chainId !== 56n) {
    console.log("\n❌ ERROR: Not connected to BSC Mainnet!");
    rl.close();
    process.exit(1);
  }

  console.log();

  // MasterChef parameters
  const ETR_TOKEN_ADDRESS = process.env.ETR_TOKEN_ADDRESS_MAINNET;

  if (!ETR_TOKEN_ADDRESS) {
    console.log("❌ ETR_TOKEN_ADDRESS_MAINNET not set in .env");
    console.log("Deploy ÉTR token first: npm run deploy:mainnet\n");
    rl.close();
    process.exit(1);
  }

  // Configuration based on LP_REWARDS_CONTRACT_SPEC.md
  // Month 1: 2.89 ÉTR per block
  const REWARD_PER_BLOCK = ethers.parseEther("2.89");

  // Start block (can be immediate or scheduled for specific date)
  // For Nov 5, 10:00 AM UTC launch, calculate blocks ahead
  const currentBlock = await ethers.provider.getBlockNumber();
  const START_BLOCK = currentBlock + 100; // ~5 minutes after deployment

  console.log("📝 MasterChef Configuration:");
  console.log(`  Reward Token: ${ETR_TOKEN_ADDRESS}`);
  console.log(`  Reward/Block: ${ethers.formatEther(REWARD_PER_BLOCK)} ÉTR (Month 1)`);
  console.log(`  Start Block:  ${START_BLOCK}`);
  console.log(`  Current Block: ${currentBlock}`);
  console.log();

  // Calculate expected rewards
  const BLOCKS_PER_DAY = 28800; // BSC: ~3 second blocks
  const dailyRewards = REWARD_PER_BLOCK * BigInt(BLOCKS_PER_DAY);
  const monthlyRewards = dailyRewards * 30n;

  console.log("📊 Expected Emission (Month 1):");
  console.log(`  Per Day:   ${ethers.formatEther(dailyRewards)} ÉTR (~83,333)`);
  console.log(`  Per Month: ${ethers.formatEther(monthlyRewards)} ÉTR (~2,500,000)`);
  console.log();

  // Safety warnings
  console.log("⚠️  FINAL WARNINGS:");
  console.log("  1. This will cost REAL MONEY (~$10-15 in gas)");
  console.log("  2. You must transfer 20M ÉTR to this contract after deployment");
  console.log("  3. You must add LP pools after deployment");
  console.log("  4. You must transfer ownership to multi-sig ASAP");
  console.log("  5. This deployment is PERMANENT and IRREVERSIBLE");
  console.log();

  const confirm1 = await question("Have you tested MasterChef on BSC testnet? (yes/no): ");
  if (confirm1.toLowerCase() !== "yes") {
    console.log("\n❌ Please test on testnet first!");
    console.log("   Run: npm run deploy:masterchef:testnet\n");
    rl.close();
    process.exit(0);
  }

  const confirm2 = await question("\nDo you have 20M ÉTR ready to transfer? (yes/no): ");
  if (confirm2.toLowerCase() !== "yes") {
    console.log("\n❌ You need 20M ÉTR to fund the rewards program!\n");
    rl.close();
    process.exit(0);
  }

  const confirm3 = await question(`\nType "DEPLOY MASTERCHEF" to continue: `);
  if (confirm3 !== "DEPLOY MASTERCHEF") {
    console.log("\n❌ Deployment cancelled\n");
    rl.close();
    process.exit(0);
  }

  console.log();
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Deploy MasterChef
  console.log("⏳ Deploying MasterChef to BSC Mainnet...");
  console.log("   This may take 1-2 minutes...");
  console.log();

  const MasterChef = await ethers.getContractFactory("MasterChef");
  const masterChef = await MasterChef.deploy(
    ETR_TOKEN_ADDRESS,
    REWARD_PER_BLOCK,
    START_BLOCK
  );

  console.log("  Transaction submitted, waiting for confirmations...");
  await masterChef.waitForDeployment();

  const masterChefAddress = await masterChef.getAddress();
  const txHash = masterChef.deploymentTransaction()?.hash;

  console.log(`  ✅ MasterChef deployed!`);
  console.log();

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("🎉 MASTERCHEF MAINNET DEPLOYMENT SUCCESSFUL!\n");
  console.log("📋 Contract Details:");
  console.log(`  Address: ${masterChefAddress}`);
  console.log(`  TX Hash: ${txHash}`);
  console.log(`  Explorer: https://bscscan.com/address/${masterChefAddress}`);
  console.log();

  console.log("⚠️  CRITICAL: Save this address immediately!");
  console.log(`  MASTERCHEF_ADDRESS_MAINNET=${masterChefAddress}`);
  console.log();

  console.log("📝 IMMEDIATE Next Steps (DO NOW - IN ORDER):");
  console.log();
  console.log("1. Save contract address to .env:");
  console.log(`   echo "MASTERCHEF_ADDRESS_MAINNET=${masterChefAddress}" >> .env`);
  console.log();
  console.log("2. Transfer 20M ÉTR to MasterChef (CRITICAL!):");
  console.log(`   - Go to: https://bscscan.com/address/${ETR_TOKEN_ADDRESS}#writeContract`);
  console.log(`   - Connect wallet`);
  console.log(`   - Call transfer()`);
  console.log(`   - to: ${masterChefAddress}`);
  console.log(`   - amount: 20000000000000000000000000 (20M ÉTR with 18 decimals)`);
  console.log();
  console.log("3. Verify contract on BscScan:");
  console.log(`   npx hardhat verify --network bscMainnet ${masterChefAddress} \\`);
  console.log(`     "${ETR_TOKEN_ADDRESS}" \\`);
  console.log(`     "${REWARD_PER_BLOCK}" \\`);
  console.log(`     "${START_BLOCK}"`);
  console.log();
  console.log("4. Add ÉTR/BNB LP pool:");
  console.log(`   - First ensure ÉTR/BNB pool exists on PancakeSwap`);
  console.log(`   - Go to MasterChef Write Contract`);
  console.log(`   - Call add(allocPoint: 1000, lpToken: <LP_TOKEN_ADDRESS>, withUpdate: false)`);
  console.log();
  console.log("5. Transfer ownership to multi-sig:");
  console.log(`   - Call transferOwnership(<MULTISIG_ADDRESS>)`);
  console.log(`   - Multi-sig must accept ownership`);
  console.log();
  console.log("6. Announce to community:");
  console.log(`   - Blog post (use template)`);
  console.log(`   - Twitter, Discord, Telegram`);
  console.log();

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Save deployment info
  const fs = require("fs");
  const deploymentInfo = {
    network: "bscMainnet",
    chainId: Number(network.chainId),
    contractAddress: masterChefAddress,
    rewardToken: ETR_TOKEN_ADDRESS,
    rewardPerBlock: REWARD_PER_BLOCK.toString(),
    startBlock: START_BLOCK,
    deployer: deployerAddress,
    timestamp: new Date().toISOString(),
    txHash: txHash,
  };

  fs.writeFileSync(
    "masterchef-deployment-mainnet.json",
    JSON.stringify(deploymentInfo, null, 2)
  );

  console.log("💾 Deployment info saved to: masterchef-deployment-mainnet.json\n");
  console.log("🎉 MasterChef is now live on BSC Mainnet!\n");
  console.log("⚠️  Remember: Transfer 20M ÉTR immediately!\n");

  rl.close();
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Deployment failed:");
    console.error(error);
    rl.close();
    process.exit(1);
  });
