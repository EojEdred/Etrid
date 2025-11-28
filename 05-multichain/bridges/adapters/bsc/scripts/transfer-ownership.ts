import { ethers } from "hardhat";
import * as readline from "readline";

/**
 * Transfer Ownership to Multi-Sig
 *
 * Helper script to transfer contract ownership to multi-sig wallet
 *
 * CRITICAL: This is irreversible! Only use after thorough testing.
 *
 * Usage:
 *   npx hardhat run scripts/transfer-ownership.ts --network bscTestnet
 *   npx hardhat run scripts/transfer-ownership.ts --network bscMainnet
 */

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

function question(query: string): Promise<string> {
  return new Promise((resolve) => rl.question(query, resolve));
}

async function main() {
  console.log("\n🔐 TRANSFER OWNERSHIP TO MULTI-SIG\n");
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

  // Get deployer
  const [deployer] = await ethers.getSigners();
  const deployerAddress = await deployer.getAddress();
  console.log(`👤 Current Owner: ${deployerAddress}\n`);

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📝 Which contract do you want to transfer?\n");
  console.log("   1. ÉTR Token");
  console.log("   2. MasterChef");
  console.log("   3. Both\n");

  const choice = await question("Enter choice (1/2/3): ");

  let contracts: Array<{ name: string; address: string; contract: any }> = [];

  // Get contract addresses
  const etrAddressKey = isMainnet ? "ETR_TOKEN_ADDRESS_MAINNET" : "ETR_TOKEN_ADDRESS_TESTNET";
  const masterChefAddressKey = isMainnet ? "MASTERCHEF_ADDRESS_MAINNET" : "MASTERCHEF_ADDRESS_TESTNET";

  if (choice === "1" || choice === "3") {
    const etrAddress = process.env[etrAddressKey];
    if (!etrAddress) {
      console.log(`\n❌ ERROR: ${etrAddressKey} not found in .env\n`);
      rl.close();
      process.exit(1);
    }
    const etr = await ethers.getContractAt("EtridToken", etrAddress);
    contracts.push({ name: "ÉTR Token", address: etrAddress, contract: etr });
  }

  if (choice === "2" || choice === "3") {
    const masterChefAddress = process.env[masterChefAddressKey];
    if (!masterChefAddress) {
      console.log(`\n❌ ERROR: ${masterChefAddressKey} not found in .env\n`);
      rl.close();
      process.exit(1);
    }
    const masterChef = await ethers.getContractAt("MasterChef", masterChefAddress);
    contracts.push({ name: "MasterChef", address: masterChefAddress, contract: masterChef });
  }

  if (contracts.length === 0) {
    console.log("\n❌ ERROR: Invalid choice\n");
    rl.close();
    process.exit(1);
  }

  console.log("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📋 Contracts to Transfer:\n");

  // Verify current ownership
  for (const { name, address, contract } of contracts) {
    const currentOwner = await contract.owner();
    console.log(`   ${name}:`);
    console.log(`     Address: ${address}`);
    console.log(`     Current Owner: ${currentOwner}`);

    if (currentOwner.toLowerCase() !== deployerAddress.toLowerCase()) {
      console.log(`     ⚠️  WARNING: You are not the current owner!\n`);
      rl.close();
      process.exit(1);
    }
  }

  console.log();

  // Get new owner address
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  const newOwnerAddress = await question("Enter multi-sig wallet address: ");

  if (!ethers.isAddress(newOwnerAddress)) {
    console.log("\n❌ ERROR: Invalid multi-sig address\n");
    rl.close();
    process.exit(1);
  }

  if (newOwnerAddress.toLowerCase() === deployerAddress.toLowerCase()) {
    console.log("\n❌ ERROR: New owner is the same as current owner\n");
    rl.close();
    process.exit(1);
  }

  // Verify multi-sig has code (is a contract)
  const code = await ethers.provider.getCode(newOwnerAddress);
  const isContract = code !== "0x";

  console.log("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📋 Summary:\n");
  console.log(`   New Owner: ${newOwnerAddress}`);
  console.log(`   Type: ${isContract ? "Smart Contract (Multi-Sig)" : "EOA (Externally Owned Account)"}\n`);

  if (!isContract && isMainnet) {
    console.log("⚠️  WARNING: New owner is NOT a smart contract!");
    console.log("   For mainnet, you should use a multi-sig contract like Gnosis Safe.\n");

    const proceed = await question("Continue anyway? (yes/no): ");
    if (proceed.toLowerCase() !== "yes") {
      console.log("\n❌ Cancelled\n");
      rl.close();
      process.exit(0);
    }
  }

  console.log("📋 Contracts that will be transferred:\n");
  for (const { name, address } of contracts) {
    console.log(`   ✓ ${name} (${address})`);
  }
  console.log();

  console.log("⚠️  CRITICAL WARNINGS:\n");
  console.log("   • This is IRREVERSIBLE - you will lose ownership!");
  console.log("   • Make sure the multi-sig is correctly configured");
  console.log("   • Verify you have access to the multi-sig signers");
  console.log("   • Test on testnet first if possible");
  console.log("   • Do NOT use unless you're absolutely certain!\n");

  // Final confirmation
  if (isMainnet) {
    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    console.log("⚠️  MAINNET - THIS CANNOT BE UNDONE!\n");

    const confirm1 = await question('Type "I UNDERSTAND THIS IS IRREVERSIBLE" to continue: ');
    if (confirm1 !== "I UNDERSTAND THIS IS IRREVERSIBLE") {
      console.log("\n❌ Cancelled\n");
      rl.close();
      process.exit(0);
    }

    const confirm2 = await question(`\nType the new owner address again: `);
    if (confirm2.toLowerCase() !== newOwnerAddress.toLowerCase()) {
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

  console.log("\n🚀 Transferring ownership...\n");

  // Transfer ownership
  const results: Array<{ name: string; success: boolean; txHash?: string; error?: string }> = [];

  for (const { name, address, contract } of contracts) {
    try {
      console.log(`📤 Transferring ${name}...`);
      const tx = await contract.transferOwnership(newOwnerAddress);
      console.log(`   Transaction: ${tx.hash}`);
      console.log(`   Waiting for confirmation...`);

      const receipt = await tx.wait();
      console.log(`   ✅ Confirmed in block ${receipt?.blockNumber}\n`);

      results.push({
        name,
        success: true,
        txHash: tx.hash,
      });
    } catch (error: any) {
      console.log(`   ❌ Failed: ${error.message}\n`);
      results.push({
        name,
        success: false,
        error: error.message,
      });
    }
  }

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📊 TRANSFER RESULTS\n");

  let allSucceeded = true;
  for (const result of results) {
    if (result.success) {
      console.log(`✅ ${result.name}`);
      console.log(`   Transaction: ${result.txHash}`);
      console.log(`   New Owner: ${newOwnerAddress}\n`);
    } else {
      console.log(`❌ ${result.name}`);
      console.log(`   Error: ${result.error}\n`);
      allSucceeded = false;
    }
  }

  if (allSucceeded) {
    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    console.log("✅ ALL TRANSFERS SUCCESSFUL!\n");

    console.log("📝 Next Steps:");
    console.log(`   1. Verify ownership on BscScan`);
    console.log(`   2. Test multi-sig access with a non-critical operation`);
    console.log(`   3. Document multi-sig signer addresses`);
    console.log(`   4. Update your .env and documentation`);
    console.log(`   5. Announce ownership transfer (if public)\n`);

    console.log("⚠️  IMPORTANT:");
    console.log(`   • You can NO LONGER manage these contracts from ${deployerAddress}`);
    console.log(`   • All future operations must go through: ${newOwnerAddress}`);
    console.log(`   • Make sure all signers have access to the multi-sig!\n`);
  } else {
    console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    console.log("⚠️  SOME TRANSFERS FAILED\n");
    console.log("Review the errors above and try again for failed contracts.\n");
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
