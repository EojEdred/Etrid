import { ethers } from "hardhat";
import * as readline from "readline";

/**
 * Update Emission Rate
 *
 * Helper script to update MasterChef emission rate (for monthly APR adjustments)
 *
 * Usage:
 *   npx hardhat run scripts/update-emission.ts --network bscTestnet
 *   npx hardhat run scripts/update-emission.ts --network bscMainnet
 */

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

function question(query: string): Promise<string> {
  return new Promise((resolve) => rl.question(query, resolve));
}

// Emission schedule for reference
const EMISSION_SCHEDULE = [
  { month: 1, rate: "2.89", apr: "150%", blocks: 777600, total: "2.25M" },
  { month: 2, rate: "4.05", apr: "120%", blocks: 777600, total: "3.15M" },
  { month: 3, rate: "4.63", apr: "90%", blocks: 777600, total: "3.60M" },
  { month: 4, rate: "4.63", apr: "70%", blocks: 777600, total: "3.60M" },
  { month: 5, rate: "4.05", apr: "50%", blocks: 777600, total: "3.15M" },
  { month: 6, rate: "2.89", apr: "35%", blocks: 777600, total: "2.25M" },
];

async function main() {
  console.log("\n⚡ UPDATE MASTERCHEF EMISSION RATE\n");
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

  // Show current emission rate
  const currentRate = await masterChef.rewardPerBlock();
  const currentRateFormatted = ethers.formatEther(currentRate);

  console.log("📊 Current Emission Rate:");
  console.log(`   ${currentRateFormatted} ÉTR per block\n`);

  // Show emission schedule
  console.log("📅 Emission Schedule (for reference):\n");
  EMISSION_SCHEDULE.forEach((schedule) => {
    const marker = schedule.rate === currentRateFormatted ? " ← CURRENT" : "";
    console.log(
      `   Month ${schedule.month}: ${schedule.rate} ÉTR/block (${schedule.apr} APR)${marker}`
    );
  });
  console.log();

  // Get user input
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📝 New Emission Rate\n");

  const newRateStr = await question("New rate (ÉTR per block, e.g., 4.05): ");
  const newRateFloat = parseFloat(newRateStr);

  if (isNaN(newRateFloat) || newRateFloat <= 0) {
    console.log("\n❌ ERROR: Invalid emission rate\n");
    rl.close();
    process.exit(1);
  }

  const newRate = ethers.parseEther(newRateStr);

  // Calculate change
  const change = ((newRateFloat - parseFloat(currentRateFormatted)) / parseFloat(currentRateFormatted)) * 100;
  const changeDirection = change > 0 ? "increase" : "decrease";
  const changeAbs = Math.abs(change);

  console.log("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📋 Summary:\n");
  console.log(`   Current Rate: ${currentRateFormatted} ÉTR/block`);
  console.log(`   New Rate:     ${newRateStr} ÉTR/block`);
  console.log(`   Change:       ${changeAbs.toFixed(2)}% ${changeDirection}\n`);

  // Estimate daily/monthly emissions
  const blocksPerDay = 28800; // BSC: 3 second blocks
  const blocksPerMonth = blocksPerDay * 30;

  const dailyEmissions = newRateFloat * blocksPerDay;
  const monthlyEmissions = newRateFloat * blocksPerMonth;

  console.log("📊 Projected Emissions:");
  console.log(`   Daily:   ${dailyEmissions.toLocaleString()} ÉTR`);
  console.log(`   Monthly: ${monthlyEmissions.toLocaleString()} ÉTR`);
  console.log(`   (${blocksPerMonth.toLocaleString()} blocks)\n`);

  // Get reason for update
  const reason = await question("Reason for update (optional): ");

  // Final confirmation
  if (isMainnet) {
    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    console.log("⚠️  MAINNET UPDATE - THIS WILL AFFECT ALL REWARDS!\n");
    console.log("⚠️  Make sure you've announced this to the community!\n");

    const confirm = await question('Type "UPDATE EMISSION" to continue: ');
    if (confirm !== "UPDATE EMISSION") {
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

  console.log("\n🚀 Updating emission rate...\n");

  try {
    const tx = await masterChef.updateRewardPerBlock(newRate);
    console.log(`📤 Transaction submitted: ${tx.hash}`);
    console.log("⏳ Waiting for confirmation...\n");

    const receipt = await tx.wait();
    console.log(`✅ Emission rate updated successfully!`);
    console.log(`   Block: ${receipt?.blockNumber}`);
    console.log(`   Gas used: ${receipt?.gasUsed.toString()}\n`);

    // Verify update
    const updatedRate = await masterChef.rewardPerBlock();
    const updatedRateFormatted = ethers.formatEther(updatedRate);

    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    console.log("✅ SUCCESS!\n");
    console.log(`📊 New Emission Rate: ${updatedRateFormatted} ÉTR/block\n`);

    if (reason) {
      console.log(`📝 Reason: ${reason}\n`);
    }

    console.log("📝 Next Steps:");
    console.log(`   1. Announce change to community`);
    console.log(`   2. Update website/docs with new APR`);
    console.log(`   3. Monitor rewards distribution`);
    console.log(`   4. Update analytics dashboards\n`);

    console.log("💡 TIP: APR will automatically adjust based on:");
    console.log(`   - New emission rate: ${updatedRateFormatted} ÉTR/block`);
    console.log(`   - Current TVL in pools`);
    console.log(`   - ÉTR token price\n`);

  } catch (error: any) {
    console.log("\n❌ Transaction failed!");
    console.log(`   Error: ${error.message}\n`);

    if (error.message.includes("Ownable")) {
      console.log("💡 You don't have permission to update emission rate");
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
