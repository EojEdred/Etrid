const hre = require("hardhat");
const fs = require("fs");

/**
 * Verify all deployed contracts on Etherscan
 * Run after deployment with: npx hardhat run scripts/verify-all.js --network sepolia
 */
async function main() {
  console.log("🔍 Verifying EDSC contracts on Etherscan...\n");

  // Load deployment file
  const files = fs.readdirSync(".").filter(f => f.startsWith(`deployment-${hre.network.name}`));
  if (files.length === 0) {
    console.error("❌ No deployment file found for network:", hre.network.name);
    process.exit(1);
  }

  const latestFile = files.sort().reverse()[0];
  console.log("Using deployment file:", latestFile);
  const deployment = JSON.parse(fs.readFileSync(latestFile, "utf8"));

  const { EDSC, AttesterRegistry, MessageTransmitter, TokenMessenger } = deployment.contracts;
  const { owner, minSignatures, totalAttesters } = deployment.config;

  console.log("\nContract Addresses:");
  console.log("  EDSC:", EDSC);
  console.log("  AttesterRegistry:", AttesterRegistry);
  console.log("  MessageTransmitter:", MessageTransmitter);
  console.log("  TokenMessenger:", TokenMessenger);
  console.log("");

  // Verify EDSC
  console.log("1. Verifying EDSC token...");
  try {
    await hre.run("verify:verify", {
      address: EDSC,
      constructorArguments: [owner],
    });
    console.log("   ✓ EDSC verified\n");
  } catch (error) {
    if (error.message.includes("Already Verified")) {
      console.log("   ✓ EDSC already verified\n");
    } else {
      console.error("   ✗ EDSC verification failed:", error.message);
    }
  }

  // Verify AttesterRegistry
  console.log("2. Verifying AttesterRegistry...");
  try {
    await hre.run("verify:verify", {
      address: AttesterRegistry,
      constructorArguments: [minSignatures, totalAttesters],
    });
    console.log("   ✓ AttesterRegistry verified\n");
  } catch (error) {
    if (error.message.includes("Already Verified")) {
      console.log("   ✓ AttesterRegistry already verified\n");
    } else {
      console.error("   ✗ AttesterRegistry verification failed:", error.message);
    }
  }

  // Verify MessageTransmitter
  console.log("3. Verifying EDSCMessageTransmitter...");
  try {
    await hre.run("verify:verify", {
      address: MessageTransmitter,
      constructorArguments: [AttesterRegistry],
    });
    console.log("   ✓ MessageTransmitter verified\n");
  } catch (error) {
    if (error.message.includes("Already Verified")) {
      console.log("   ✓ MessageTransmitter already verified\n");
    } else {
      console.error("   ✗ MessageTransmitter verification failed:", error.message);
    }
  }

  // Verify TokenMessenger
  console.log("4. Verifying EDSCTokenMessenger...");
  try {
    await hre.run("verify:verify", {
      address: TokenMessenger,
      constructorArguments: [EDSC],
    });
    console.log("   ✓ TokenMessenger verified\n");
  } catch (error) {
    if (error.message.includes("Already Verified")) {
      console.log("   ✓ TokenMessenger already verified\n");
    } else {
      console.error("   ✗ TokenMessenger verification failed:", error.message);
    }
  }

  console.log("=" .repeat(60));
  console.log("✅ Verification complete!");
  console.log("=" .repeat(60));
  console.log("\nView on Etherscan:");
  console.log(`  EDSC: https://sepolia.etherscan.io/address/${EDSC}#code`);
  console.log(`  AttesterRegistry: https://sepolia.etherscan.io/address/${AttesterRegistry}#code`);
  console.log(`  MessageTransmitter: https://sepolia.etherscan.io/address/${MessageTransmitter}#code`);
  console.log(`  TokenMessenger: https://sepolia.etherscan.io/address/${TokenMessenger}#code`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
