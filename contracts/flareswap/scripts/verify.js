// FlareSwap Contract Verification Script
// Usage: npx hardhat run scripts/verify.js --network <network>

const hre = require("hardhat");
const fs = require("fs");

async function main() {
  console.log("🔍 FlareSwap Contract Verification");
  console.log("=".repeat(50));

  // Load deployment addresses
  const deployments = JSON.parse(fs.readFileSync("./deployments.json", "utf8"));

  console.log("Network:", deployments.network);
  console.log("");

  // Verify WETH
  console.log("Verifying WETH...");
  try {
    await hre.run("verify:verify", {
      address: deployments.weth,
      constructorArguments: []
    });
    console.log("✅ WETH verified");
  } catch (error) {
    console.log("❌ WETH verification failed:", error.message);
  }
  console.log("");

  // Verify Factory
  console.log("Verifying FlareSwapFactory...");
  try {
    await hre.run("verify:verify", {
      address: deployments.factory,
      constructorArguments: [deployments.deployer]
    });
    console.log("✅ FlareSwapFactory verified");
  } catch (error) {
    console.log("❌ FlareSwapFactory verification failed:", error.message);
  }
  console.log("");

  // Verify Router
  console.log("Verifying FlareSwapRouter...");
  try {
    await hre.run("verify:verify", {
      address: deployments.router,
      constructorArguments: [deployments.factory, deployments.weth]
    });
    console.log("✅ FlareSwapRouter verified");
  } catch (error) {
    console.log("❌ FlareSwapRouter verification failed:", error.message);
  }
  console.log("");

  console.log("=".repeat(50));
  console.log("Verification complete!");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
