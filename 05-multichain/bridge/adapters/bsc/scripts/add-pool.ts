import { ethers } from "hardhat";
import * as readline from "readline";

/**
 * Add LP Pool to MasterChef
 *
 * Helper script to add a new liquidity pool to MasterChef
 *
 * Usage:
 *   npx hardhat run scripts/add-pool.ts --network bscTestnet
 *   npx hardhat run scripts/add-pool.ts --network bscMainnet
 */

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

function question(query: string): Promise<string> {
  return new Promise((resolve) => rl.question(query, resolve));
}

async function main() {
  console.log("\n🔧 ADD LP POOL TO MASTERCHEF\n");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Get network info
  const network = await ethers.provider.getNetwork();
  const isMainnet = network.chainId === 56n;
  const isTestnet = network.chainId === 97n;

  console.log(`📍 Network: ${isMainnet ? "BSC Mainnet" : isTestnet ? "BSC Testnet" : "Unknown"}`);
  console.log(`   Chain ID: ${network.chainId}\n`);

  if (!isMainnet && !isTestnet) {
    console.log("❌ ERROR: Not connected to BSC network!");
    rl.close();
    process.exit(1);
  }

  // Get MasterChef address
  const masterChefAddressKey = isMainnet ? "MASTERCHEF_ADDRESS_MAINNET" : "MASTERCHEF_ADDRESS_TESTNET";
  const masterChefAddress = process.env[masterChefAddressKey];

  if (!masterChefAddress) {
    console.log(`❌ ERROR: ${masterChefAddressKey} not found in .env`);
    rl.close();
    process.exit(1);
  }

  console.log(`📄 MasterChef: ${masterChefAddress}\n`);

  // Get deployer
  const [deployer] = await ethers.getSigners();
  const deployerAddress = await deployer.getAddress();
  console.log(`👤 Signer: ${deployerAddress}\n`);

  // Get MasterChef contract
  const masterChef = await ethers.getContractAt("MasterChef", masterChefAddress);

  // Check ownership
  const owner = await masterChef.owner();
  if (owner.toLowerCase() !== deployerAddress.toLowerCase()) {
    console.log(`⚠️  WARNING: You are not the owner of MasterChef!`);
    console.log(`   Owner: ${owner}`);
    console.log(`   You: ${deployerAddress}\n`);

    const proceed = await question("Continue anyway? (yes/no): ");
    if (proceed.toLowerCase() !== "yes") {
      console.log("\n❌ Cancelled\n");
      rl.close();
      process.exit(0);
    }
  }

  // Show current state
  const poolLength = await masterChef.poolLength();
  const totalAllocPoint = await masterChef.totalAllocPoint();

  console.log("📊 Current State:");
  console.log(`   Pools: ${poolLength}`);
  console.log(`   Total Alloc Points: ${totalAllocPoint}\n`);

  if (poolLength > 0n) {
    console.log("📋 Existing Pools:");
    for (let i = 0; i < Number(poolLength); i++) {
      const poolInfo = await masterChef.poolInfo(i);
      console.log(`   Pool ${i}:`);
      console.log(`     LP Token: ${poolInfo.lpToken}`);
      console.log(`     Alloc Points: ${poolInfo.allocPoint}`);
      console.log(`     Total Staked: ${ethers.formatEther(poolInfo.totalStaked)} LP`);
    }
    console.log();
  }

  // Get user input
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📝 Pool Configuration\n");

  const lpTokenAddress = await question("LP Token Address: ");

  if (!ethers.isAddress(lpTokenAddress)) {
    console.log("\n❌ ERROR: Invalid LP token address\n");
    rl.close();
    process.exit(1);
  }

  const allocPointStr = await question("Allocation Points (e.g., 1000 for 100%): ");
  const allocPoint = parseInt(allocPointStr);

  if (isNaN(allocPoint) || allocPoint <= 0) {
    console.log("\n❌ ERROR: Invalid allocation points\n");
    rl.close();
    process.exit(1);
  }

  const withUpdate = await question("Update existing pools first? (yes/no): ");
  const shouldUpdate = withUpdate.toLowerCase() === "yes";

  console.log("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📋 Summary:\n");
  console.log(`   LP Token: ${lpTokenAddress}`);
  console.log(`   Allocation Points: ${allocPoint}`);
  console.log(`   Update Existing Pools: ${shouldUpdate}\n`);

  // Calculate new distribution
  const newTotalAllocPoint = totalAllocPoint + BigInt(allocPoint);
  const percentage = (BigInt(allocPoint) * 10000n) / newTotalAllocPoint;
  console.log(`   This pool will get: ${Number(percentage) / 100}% of rewards`);
  console.log(`   New total alloc points: ${newTotalAllocPoint}\n`);

  // Verify LP token
  try {
    const lpToken = await ethers.getContractAt("IERC20", lpTokenAddress);
    const symbol = await lpToken.symbol();
    const name = await lpToken.name();
    console.log(`✅ LP Token verified: ${name} (${symbol})\n`);
  } catch (error) {
    console.log(`⚠️  WARNING: Could not verify LP token contract`);
    console.log(`   Error: ${error}\n`);

    const proceed = await question("Continue anyway? (yes/no): ");
    if (proceed.toLowerCase() !== "yes") {
      console.log("\n❌ Cancelled\n");
      rl.close();
      process.exit(0);
    }
  }

  // Final confirmation
  if (isMainnet) {
    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    console.log("⚠️  MAINNET DEPLOYMENT - CANNOT BE UNDONE!\n");

    const confirm = await question('Type "ADD POOL" to continue: ');
    if (confirm !== "ADD POOL") {
      console.log("\n❌ Cancelled\n");
      rl.close();
      process.exit(0);
    }
  } else {
    const confirm = await question('\nProceed? (yes/no): ');
    if (confirm.toLowerCase() !== "yes") {
      console.log("\n❌ Cancelled\n");
      rl.close();
      process.exit(0);
    }
  }

  console.log("\n🚀 Adding pool...\n");

  try {
    const tx = await masterChef.add(allocPoint, lpTokenAddress, shouldUpdate);
    console.log(`📤 Transaction submitted: ${tx.hash}`);
    console.log("⏳ Waiting for confirmation...\n");

    const receipt = await tx.wait();
    console.log(`✅ Pool added successfully!`);
    console.log(`   Block: ${receipt?.blockNumber}`);
    console.log(`   Gas used: ${receipt?.gasUsed.toString()}\n`);

    // Show updated state
    const newPoolLength = await masterChef.poolLength();
    const newPoolId = Number(newPoolLength) - 1;

    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    console.log("✅ SUCCESS!\n");
    console.log(`📋 New Pool ID: ${newPoolId}`);
    console.log(`   LP Token: ${lpTokenAddress}`);
    console.log(`   Allocation Points: ${allocPoint}`);
    console.log(`   Share of Rewards: ${Number(percentage) / 100}%\n`);

    console.log("📝 Next Steps:");
    console.log(`   1. Announce new pool to community`);
    console.log(`   2. Verify LP token address in docs`);
    console.log(`   3. Monitor deposits and TVL\n`);

  } catch (error: any) {
    console.log("\n❌ Transaction failed!");
    console.log(`   Error: ${error.message}\n`);

    if (error.message.includes("LP token already added")) {
      console.log("💡 This LP token is already added to MasterChef");
    } else if (error.message.includes("Ownable")) {
      console.log("💡 You don't have permission to add pools");
    }

    rl.close();
    process.exit(1);
  }

  rl.close();
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Script failed:");
    console.error(error);
    rl.close();
    process.exit(1);
  });
