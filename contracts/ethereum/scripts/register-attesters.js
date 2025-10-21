const hre = require("hardhat");

async function main() {
  console.log("Registering attesters for EDSC bridge...\n");

  // Read deployment file
  const fs = require("fs");
  const files = fs.readdirSync(".").filter(f => f.startsWith("deployment-localhost"));
  const latestFile = files.sort().reverse()[0];

  if (!latestFile) {
    throw new Error("No deployment file found. Deploy contracts first.");
  }

  const deployment = JSON.parse(fs.readFileSync(latestFile, "utf8"));
  const attesterRegistryAddress = deployment.contracts.AttesterRegistry;

  console.log("Using deployment:", latestFile);
  console.log("AttesterRegistry:", attesterRegistryAddress, "\n");

  const AttesterRegistry = await hre.ethers.getContractFactory("AttesterRegistry");
  const registry = AttesterRegistry.attach(attesterRegistryAddress);

  // Get the deployer
  const [deployer] = await hre.ethers.getSigners();
  console.log("Registering with account:", deployer.address);

  // Test attester addresses (using Hardhat's default accounts)
  // These correspond to the private keys in Hardhat
  const attesters = [
    "0x70997970C51812dc3A010C7d01b50e0d17dc79C8", // Account #1
    "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC", // Account #2
    "0x90F79bf6EB2c4f870365E785982E1f101E93b906", // Account #3
  ];

  console.log("Registering", attesters.length, "attesters...\n");

  for (let i = 0; i < attesters.length; i++) {
    console.log(`[${i + 1}/${attesters.length}] Registering:`, attesters[i]);

    try {
      const tx = await registry.registerAttester(attesters[i]);
      await tx.wait();
      console.log("  ✓ Registered successfully\n");
    } catch (error) {
      console.log("  ✗ Failed:", error.message, "\n");
    }
  }

  // Verify registration
  console.log("Verification:");
  const attesterCount = await registry.getAttesterCount();
  console.log("  Total attesters registered:", attesterCount.toString());
  console.log("  Required signatures: 3 (configured at deployment)");

  console.log("\n✅ Attester registration complete!");
  console.log("\nNext steps:");
  console.log("1. Start FlareChain node");
  console.log("2. Configure attestation service with these attester keys");
  console.log("3. Configure relayer service");
  console.log("4. Test cross-chain transfer");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
