import { ethers } from "hardhat";

/**
 * Deploy MasterChef LP Rewards Contract to BSC Testnet
 *
 * Usage:
 *   npm run deploy:masterchef:testnet
 *
 * Prerequisites:
 *   1. ÉTR token already deployed on testnet
 *   2. Wallet funded with testnet BNB
 *   3. .env configured with ETR_TOKEN_ADDRESS_TESTNET
 */
async function main() {
  console.log("\n🚀 Deploying MasterChef to BSC Testnet...\n");
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

  if (balance < ethers.parseEther("0.01")) {
    console.log("\n❌ Insufficient balance!");
    console.log("Get testnet BNB: https://testnet.bnbchain.org/faucet-smart\n");
    return;
  }

  // Get network info
  const network = await ethers.provider.getNetwork();
  console.log(`  Network:  BSC Testnet (Chain ID: ${network.chainId})`);
  console.log();

  // MasterChef parameters
  const ETR_TOKEN_ADDRESS = process.env.ETR_TOKEN_ADDRESS_TESTNET;

  if (!ETR_TOKEN_ADDRESS) {
    console.log("❌ ETR_TOKEN_ADDRESS_TESTNET not set in .env");
    console.log("Deploy ÉTR token first: npm run deploy:testnet\n");
    return;
  }

  // Configuration based on LP_REWARDS_CONTRACT_SPEC.md
  const REWARD_PER_BLOCK = ethers.parseEther("2.89"); // Month 1: 2.89 ÉTR per block
  const START_BLOCK = (await ethers.provider.getBlockNumber()) + 100; // Start in ~5 minutes

  console.log("📝 MasterChef Configuration:");
  console.log(`  Reward Token: ${ETR_TOKEN_ADDRESS}`);
  console.log(`  Reward/Block: ${ethers.formatEther(REWARD_PER_BLOCK)} ÉTR`);
  console.log(`  Start Block:  ${START_BLOCK}`);
  console.log(`  Current Block: ${await ethers.provider.getBlockNumber()}`);
  console.log();

  // Calculate expected rewards
  const BLOCKS_PER_DAY = 28800; // BSC: ~3 second blocks
  const dailyRewards = REWARD_PER_BLOCK * BigInt(BLOCKS_PER_DAY);
  const monthlyRewards = dailyRewards * 30n;

  console.log("📊 Expected Emission:");
  console.log(`  Per Day:   ${ethers.formatEther(dailyRewards)} ÉTR`);
  console.log(`  Per Month: ${ethers.formatEther(monthlyRewards)} ÉTR`);
  console.log();

  // Deploy MasterChef
  console.log("⏳ Deploying MasterChef contract...");
  const MasterChef = await ethers.getContractFactory("MasterChef");
  const masterChef = await MasterChef.deploy(
    ETR_TOKEN_ADDRESS,
    REWARD_PER_BLOCK,
    START_BLOCK
  );

  console.log("  Transaction submitted, waiting for confirmation...");
  await masterChef.waitForDeployment();

  const masterChefAddress = await masterChef.getAddress();
  const txHash = masterChef.deploymentTransaction()?.hash;

  console.log(`  ✅ MasterChef deployed!`);
  console.log();

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("🎉 MASTERCHEF DEPLOYMENT SUCCESSFUL!\n");
  console.log("📋 Contract Details:");
  console.log(`  Address: ${masterChefAddress}`);
  console.log(`  TX Hash: ${txHash}`);
  console.log(`  Explorer: https://testnet.bscscan.com/address/${masterChefAddress}`);
  console.log();

  console.log("⚠️  IMPORTANT: Save this address!");
  console.log(`  MASTERCHEF_ADDRESS_TESTNET=${masterChefAddress}`);
  console.log();

  console.log("📝 Next Steps:");
  console.log();
  console.log("1. Save contract address to .env:");
  console.log(`   echo "MASTERCHEF_ADDRESS_TESTNET=${masterChefAddress}" >> .env`);
  console.log();
  console.log("2. Transfer test ÉTR to MasterChef:");
  console.log(`   # You need to send ÉTR tokens to MasterChef for rewards`);
  console.log(`   # For testing, send 1,000 ÉTR:`);
  console.log(`   # Go to BscScan → ÉTR Contract → Write Contract → transfer()`);
  console.log(`   # to: ${masterChefAddress}`);
  console.log(`   # amount: 1000000000000000000000 (1000 ÉTR)`);
  console.log();
  console.log("3. Add LP pool (ÉTR/BNB):");
  console.log(`   # First, create ÉTR/BNB pool on PancakeSwap Testnet`);
  console.log(`   # Then run:`);
  console.log(`   npx hardhat run scripts/add-pool.ts --network bscTestnet`);
  console.log();
  console.log("4. Test deposit/harvest/withdraw:");
  console.log(`   # Use BscScan Write Contract interface`);
  console.log(`   # Or create test script`);
  console.log();
  console.log("5. Verify contract:");
  console.log(`   npx hardhat verify --network bscTestnet ${masterChefAddress} \\`);
  console.log(`     "${ETR_TOKEN_ADDRESS}" \\`);
  console.log(`     "${REWARD_PER_BLOCK}" \\`);
  console.log(`     "${START_BLOCK}"`);
  console.log();

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Save deployment info
  const fs = require("fs");
  const deploymentInfo = {
    network: "bscTestnet",
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
    "masterchef-deployment-testnet.json",
    JSON.stringify(deploymentInfo, null, 2)
  );

  console.log("💾 Deployment info saved to: masterchef-deployment-testnet.json\n");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Deployment failed:");
    console.error(error);
    process.exit(1);
  });
