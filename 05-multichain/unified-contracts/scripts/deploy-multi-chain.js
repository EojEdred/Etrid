/**
 * Multi-chain deployment orchestrator
 * Deploys contracts to multiple chains in sequence
 */

const { exec } = require("child_process");
const util = require("util");
const execPromise = util.promisify(exec);

// Chain configurations
const TESTNETS = [
  { name: "ethPBC", displayName: "ETH PBC Testnet" },
  { name: "sepolia", displayName: "Ethereum Sepolia" },
  { name: "bscTestnet", displayName: "BNB Testnet" },
  { name: "polygonMumbai", displayName: "Polygon Mumbai" },
  { name: "arbitrumSepolia", displayName: "Arbitrum Sepolia" },
  { name: "baseSepolia", displayName: "Base Sepolia" },
];

const MAINNETS = [
  { name: "ethPBC", displayName: "ETH PBC Mainnet" },
  { name: "ethereum", displayName: "Ethereum Mainnet" },
  { name: "bsc", displayName: "BNB Chain" },
  { name: "polygon", displayName: "Polygon" },
  { name: "arbitrum", displayName: "Arbitrum" },
  { name: "base", displayName: "Base" },
  { name: "optimism", displayName: "Optimism" },
];

async function deployToChain(chain) {
  console.log("\n╔════════════════════════════════════════════════════════════════╗");
  console.log(`║  Deploying to: ${chain.displayName.padEnd(48)} ║`);
  console.log("╚════════════════════════════════════════════════════════════════╝\n");

  try {
    const { stdout, stderr } = await execPromise(
      `npx hardhat run scripts/deploy-all.js --network ${chain.name}`
    );

    console.log(stdout);
    if (stderr) console.error(stderr);

    console.log(`✅ ${chain.displayName} deployment completed!\n`);
    return { chain: chain.name, success: true };
  } catch (error) {
    console.error(`❌ ${chain.displayName} deployment failed:`, error.message);
    return { chain: chain.name, success: false, error: error.message };
  }
}

async function main() {
  console.log("╔════════════════════════════════════════════════════════════════╗");
  console.log("║        Ëtrid Multi-Chain Deployment Orchestrator               ║");
  console.log("╚════════════════════════════════════════════════════════════════╝\n");

  // Parse command line arguments
  const args = process.argv.slice(2);
  const deployTestnets = args.includes("--testnets");
  const deployMainnets = args.includes("--mainnets");

  if (!deployTestnets && !deployMainnets) {
    console.error("❌ Error: Please specify --testnets or --mainnets\n");
    console.log("Usage:");
    console.log("  node scripts/deploy-multi-chain.js --testnets");
    console.log("  node scripts/deploy-multi-chain.js --mainnets");
    console.log("");
    process.exit(1);
  }

  const chains = deployTestnets ? TESTNETS : MAINNETS;
  const mode = deployTestnets ? "TESTNET" : "MAINNET";

  console.log(`🚀 Starting ${mode} deployment to ${chains.length} chains...\n`);

  // Confirm before mainnet deployment
  if (deployMainnets) {
    console.log("⚠️  WARNING: You are about to deploy to MAINNET!");
    console.log("   This will use real funds and deploy production contracts.");
    console.log("   Make sure you have:");
    console.log("   1. Sufficient funds on all chains");
    console.log("   2. Verified all contract code");
    console.log("   3. Performed security audits");
    console.log("   4. Tested on testnets first\n");
    console.log("   Press Ctrl+C to cancel or wait 10 seconds to continue...\n");

    // Wait 10 seconds
    await new Promise((resolve) => setTimeout(resolve, 10000));
  }

  const results = [];
  const startTime = Date.now();

  // Deploy sequentially to each chain
  for (const chain of chains) {
    const result = await deployToChain(chain);
    results.push(result);

    // Wait 5 seconds between deployments
    if (chains.indexOf(chain) < chains.length - 1) {
      console.log("⏳ Waiting 5 seconds before next deployment...\n");
      await new Promise((resolve) => setTimeout(resolve, 5000));
    }
  }

  const endTime = Date.now();
  const duration = ((endTime - startTime) / 1000).toFixed(2);

  // Summary
  console.log("\n╔════════════════════════════════════════════════════════════════╗");
  console.log("║                    Deployment Summary                          ║");
  console.log("╚════════════════════════════════════════════════════════════════╝\n");

  console.log(`📊 Total chains: ${chains.length}`);
  console.log(`✅ Successful: ${results.filter((r) => r.success).length}`);
  console.log(`❌ Failed: ${results.filter((r) => !r.success).length}`);
  console.log(`⏱️  Total time: ${duration}s\n`);

  console.log("📋 Detailed Results:");
  results.forEach((result) => {
    const status = result.success ? "✅" : "❌";
    console.log(`  ${status} ${result.chain}`);
    if (!result.success) {
      console.log(`     Error: ${result.error}`);
    }
  });
  console.log("");

  if (results.every((r) => r.success)) {
    console.log("🎉 All deployments completed successfully!\n");

    console.log("📚 Next Steps:");
    console.log("  1. Configure oracle network on each chain");
    console.log("  2. Link TokenMessengers across chains");
    console.log("  3. Verify contracts on block explorers");
    console.log("  4. Update frontend configuration");
    console.log("  5. Perform integration testing");
    console.log("");
  } else {
    console.log("⚠️  Some deployments failed. Please review the errors above.\n");
    process.exit(1);
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
