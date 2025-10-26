import { ethers } from "hardhat";
import * as readline from "readline";

/**
 * Fund MasterChef with ÉTR Rewards
 *
 * Helper script to transfer 20M ÉTR to MasterChef for rewards distribution
 *
 * Usage:
 *   npx hardhat run scripts/fund-masterchef.ts --network bscTestnet
 *   npx hardhat run scripts/fund-masterchef.ts --network bscMainnet
 */

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

function question(query: string): Promise<string> {
  return new Promise((resolve) => rl.question(query, resolve));
}

const REWARD_POOL_AMOUNT = ethers.parseEther("20000000"); // 20M ÉTR

async function main() {
  console.log("\n💰 FUND MASTERCHEF WITH ÉTR REWARDS\n");
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

  // Get contract addresses
  const etrAddressKey = isMainnet ? "ETR_TOKEN_ADDRESS_MAINNET" : "ETR_TOKEN_ADDRESS_TESTNET";
  const masterChefAddressKey = isMainnet ? "MASTERCHEF_ADDRESS_MAINNET" : "MASTERCHEF_ADDRESS_TESTNET";

  const etrAddress = process.env[etrAddressKey];
  const masterChefAddress = process.env[masterChefAddressKey];

  if (!etrAddress) {
    console.log(`❌ ERROR: ${etrAddressKey} not found in .env`);
    rl.close();
    process.exit(1);
  }

  if (!masterChefAddress) {
    console.log(`❌ ERROR: ${masterChefAddressKey} not found in .env`);
    rl.close();
    process.exit(1);
  }

  console.log("📄 Contract Addresses:");
  console.log(`   ÉTR Token:  ${etrAddress}`);
  console.log(`   MasterChef: ${masterChefAddress}\n`);

  // Get deployer
  const [deployer] = await ethers.getSigners();
  const deployerAddress = await deployer.getAddress();
  console.log(`👤 Sender: ${deployerAddress}\n`);

  // Get contracts
  const etr = await ethers.getContractAt("EtridToken", etrAddress);
  const masterChef = await ethers.getContractAt("MasterChef", masterChefAddress);

  // Check balances
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("💰 Current Balances:\n");

  const deployerBalance = await etr.balanceOf(deployerAddress);
  const masterChefBalance = await etr.balanceOf(masterChefAddress);

  console.log(`   Your ÉTR Balance:        ${ethers.formatEther(deployerBalance)} ÉTR`);
  console.log(`   MasterChef ÉTR Balance:  ${ethers.formatEther(masterChefBalance)} ÉTR\n`);

  // Get custom amount or use default
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📝 Transfer Amount\n");
  console.log(`   Default: 20,000,000 ÉTR (recommended for 6-month program)\n`);

  const useDefault = await question("Use default amount? (yes/no): ");
  let transferAmount = REWARD_POOL_AMOUNT;

  if (useDefault.toLowerCase() !== "yes") {
    const customAmount = await question("Enter amount in ÉTR (e.g., 1000000): ");
    const customFloat = parseFloat(customAmount.replace(/,/g, ""));

    if (isNaN(customFloat) || customFloat <= 0) {
      console.log("\n❌ ERROR: Invalid amount\n");
      rl.close();
      process.exit(1);
    }

    transferAmount = ethers.parseEther(customFloat.toString());
  }

  // Verify sufficient balance
  if (deployerBalance < transferAmount) {
    console.log("\n❌ ERROR: Insufficient ÉTR balance!");
    console.log(`   Need:  ${ethers.formatEther(transferAmount)} ÉTR`);
    console.log(`   Have:  ${ethers.formatEther(deployerBalance)} ÉTR`);
    console.log(`   Short: ${ethers.formatEther(transferAmount - deployerBalance)} ÉTR\n`);
    rl.close();
    process.exit(1);
  }

  const newMasterChefBalance = masterChefBalance + transferAmount;

  console.log("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📋 Summary:\n");
  console.log(`   Transfer Amount:  ${ethers.formatEther(transferAmount)} ÉTR`);
  console.log(`   From:             ${deployerAddress}`);
  console.log(`   To:               ${masterChefAddress}\n`);

  console.log("💰 Balance Changes:");
  console.log(`   Your Balance:     ${ethers.formatEther(deployerBalance)} → ${ethers.formatEther(deployerBalance - transferAmount)} ÉTR`);
  console.log(`   MasterChef:       ${ethers.formatEther(masterChefBalance)} → ${ethers.formatEther(newMasterChefBalance)} ÉTR\n`);

  // Calculate program duration
  const rewardPerBlock = await masterChef.rewardPerBlock();
  const blocksPerDay = 28800; // BSC: 3 second blocks
  const dailyRewards = rewardPerBlock * BigInt(blocksPerDay);
  const daysSupported = Number(transferAmount / dailyRewards);

  console.log("📊 Program Duration (at current rate):");
  console.log(`   Reward Rate:  ${ethers.formatEther(rewardPerBlock)} ÉTR/block`);
  console.log(`   Daily:        ${ethers.formatEther(dailyRewards)} ÉTR/day`);
  console.log(`   Duration:     ~${daysSupported.toFixed(0)} days with this funding\n`);

  if (transferAmount >= REWARD_POOL_AMOUNT) {
    console.log("✅ This is sufficient for the 6-month LP rewards program\n");
  } else {
    const shortfall = REWARD_POOL_AMOUNT - transferAmount;
    console.log(`⚠️  Note: Full 6-month program requires 20M ÉTR`);
    console.log(`   You're transferring ${ethers.formatEther(transferAmount)} ÉTR`);
    console.log(`   Shortfall: ${ethers.formatEther(shortfall)} ÉTR\n`);
  }

  // Final confirmation
  if (isMainnet) {
    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    console.log("⚠️  MAINNET TRANSFER - CANNOT BE UNDONE!\n");
    console.log("⚠️  Make sure MasterChef address is correct!\n");

    const confirm1 = await question('Type "FUND MASTERCHEF" to continue: ');
    if (confirm1 !== "FUND MASTERCHEF") {
      console.log("\n❌ Cancelled\n");
      rl.close();
      process.exit(0);
    }

    const confirm2 = await question(`\nType MasterChef address again: `);
    if (confirm2.toLowerCase() !== masterChefAddress.toLowerCase()) {
      console.log("\n❌ Address mismatch - Cancelled\n");
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

  console.log("\n🚀 Transferring ÉTR...\n");

  try {
    const tx = await etr.transfer(masterChefAddress, transferAmount);
    console.log(`📤 Transaction submitted: ${tx.hash}`);
    console.log("⏳ Waiting for confirmation...\n");

    const receipt = await tx.wait();
    console.log(`✅ Transfer successful!`);
    console.log(`   Block: ${receipt?.blockNumber}`);
    console.log(`   Gas used: ${receipt?.gasUsed.toString()}\n`);

    // Verify final balances
    const finalDeployerBalance = await etr.balanceOf(deployerAddress);
    const finalMasterChefBalance = await etr.balanceOf(masterChefAddress);

    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    console.log("✅ SUCCESS!\n");

    console.log("💰 Final Balances:");
    console.log(`   Your Balance:     ${ethers.formatEther(finalDeployerBalance)} ÉTR`);
    console.log(`   MasterChef:       ${ethers.formatEther(finalMasterChefBalance)} ÉTR\n`);

    console.log("📝 Next Steps:");
    console.log(`   1. Verify balance on BscScan`);
    console.log(`   2. Add LP pools to MasterChef (if not done)`);
    console.log(`   3. Test deposit/harvest with small amount`);
    console.log(`   4. Transfer ownership to multi-sig (if not done)`);
    console.log(`   5. Announce LP rewards program to community\n`);

    console.log("💡 REMINDER:");
    console.log(`   • MasterChef now has ${ethers.formatEther(finalMasterChefBalance)} ÉTR for rewards`);
    console.log(`   • At ${ethers.formatEther(rewardPerBlock)} ÉTR/block, this will last ~${daysSupported.toFixed(0)} days`);
    console.log(`   • Monitor the balance regularly to ensure rewards don't run out\n`);

  } catch (error: any) {
    console.log("\n❌ Transaction failed!");
    console.log(`   Error: ${error.message}\n`);

    if (error.message.includes("ERC20: transfer amount exceeds balance")) {
      console.log("💡 Insufficient ÉTR balance");
    } else if (error.message.includes("Pausable: paused")) {
      console.log("💡 ÉTR token is paused - unpause it first");
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
