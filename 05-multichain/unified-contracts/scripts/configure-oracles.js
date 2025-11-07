/**
 * Configure oracle network for TokenMessenger contracts
 * Sets up 5 oracle addresses for 3-of-5 multisig attestation
 */

const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

// Oracle addresses (replace with actual addresses)
const ORACLE_ADDRESSES = [
  process.env.ORACLE_1 || "0x0000000000000000000000000000000000000001",
  process.env.ORACLE_2 || "0x0000000000000000000000000000000000000002",
  process.env.ORACLE_3 || "0x0000000000000000000000000000000000000003",
  process.env.ORACLE_4 || "0x0000000000000000000000000000000000000004",
  process.env.ORACLE_5 || "0x0000000000000000000000000000000000000005",
];

async function main() {
  console.log("╔════════════════════════════════════════════════════════════════╗");
  console.log("║           Oracle Network Configuration                         ║");
  console.log("╚════════════════════════════════════════════════════════════════╝\n");

  const [deployer] = await hre.ethers.getSigners();
  const network = hre.network.name;
  const chainId = (await hre.ethers.provider.getNetwork()).chainId;

  console.log("📊 Configuration Information:");
  console.log("  Network:", network);
  console.log("  Chain ID:", chainId.toString());
  console.log("  Deployer:", deployer.address);
  console.log("");

  // Load deployment info
  const deploymentPath = path.join(
    __dirname,
    "../deployments",
    `${network}-${chainId.toString()}.json`
  );

  if (!fs.existsSync(deploymentPath)) {
    console.error("❌ Error: Deployment file not found:", deploymentPath);
    console.log("   Please deploy contracts first using: npm run deploy:eth-pbc");
    process.exit(1);
  }

  const deployment = JSON.parse(fs.readFileSync(deploymentPath, "utf8"));
  const tokenMessengerAddress = deployment.contracts.tokenMessenger;

  if (!tokenMessengerAddress) {
    console.error("❌ Error: TokenMessenger address not found in deployment");
    process.exit(1);
  }

  console.log("📝 TokenMessenger:", tokenMessengerAddress);
  console.log("");

  // Get TokenMessenger contract
  const TokenMessenger = await hre.ethers.getContractFactory("TokenMessenger");
  const tokenMessenger = TokenMessenger.attach(tokenMessengerAddress);

  console.log("🔷 Adding Oracle Addresses:\n");

  for (let i = 0; i < ORACLE_ADDRESSES.length; i++) {
    const oracle = ORACLE_ADDRESSES[i];
    console.log(`📝 Adding Oracle ${i + 1}: ${oracle}`);

    try {
      const tx = await tokenMessenger.addOracle(oracle);
      await tx.wait();
      console.log("  ✅ Oracle added successfully");
    } catch (error) {
      console.error(`  ❌ Failed to add oracle: ${error.message}`);
    }
    console.log("");
  }

  // Verify oracle configuration
  console.log("🔍 Verifying Oracle Configuration:\n");
  const oracleCount = await tokenMessenger.oracleCount();
  console.log(`  Total oracles: ${oracleCount.toString()}`);
  console.log("");

  for (let i = 0; i < oracleCount; i++) {
    const oracle = await tokenMessenger.oracles(i);
    const isOracle = await tokenMessenger.isOracle(oracle);
    console.log(`  Oracle ${i + 1}: ${oracle} (active: ${isOracle})`);
  }
  console.log("");

  console.log("✅ Oracle configuration completed!\n");

  console.log("📚 Next Steps:");
  console.log("  1. Distribute private keys to oracle operators");
  console.log("  2. Start oracle nodes on each chain");
  console.log("  3. Monitor oracle health and uptime");
  console.log("  4. Test cross-chain message signing");
  console.log("");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
