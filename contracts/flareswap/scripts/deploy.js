// FlareSwap Deployment Script
// Usage: npx hardhat run scripts/deploy.js --network <network>

const hre = require("hardhat");

async function main() {
  console.log("🚀 FlareSwap Deployment Started");
  console.log("=".repeat(50));

  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying contracts with account:", deployer.address);
  console.log("Account balance:", (await deployer.getBalance()).toString());
  console.log("");

  // Step 1: Deploy WETH
  console.log("📦 Deploying WETH...");
  const WETH = await hre.ethers.getContractFactory("WETH");
  const weth = await WETH.deploy();
  await weth.deployed();
  console.log("✅ WETH deployed to:", weth.address);
  console.log("");

  // Step 2: Deploy Factory
  console.log("📦 Deploying FlareSwapFactory...");
  const Factory = await hre.ethers.getContractFactory("FlareSwapFactory");
  const factory = await Factory.deploy(deployer.address); // deployer is feeToSetter
  await factory.deployed();
  console.log("✅ FlareSwapFactory deployed to:", factory.address);
  console.log("   Fee To Setter:", deployer.address);
  console.log("");

  // Step 3: Deploy Router
  console.log("📦 Deploying FlareSwapRouter...");
  const Router = await hre.ethers.getContractFactory("FlareSwapRouter");
  const router = await Router.deploy(factory.address, weth.address);
  await router.deployed();
  console.log("✅ FlareSwapRouter deployed to:", router.address);
  console.log("");

  // Summary
  console.log("=".repeat(50));
  console.log("🎉 FlareSwap Deployment Complete!");
  console.log("=".repeat(50));
  console.log("");
  console.log("Contract Addresses:");
  console.log("-------------------");
  console.log("WETH:              ", weth.address);
  console.log("FlareSwapFactory:  ", factory.address);
  console.log("FlareSwapRouter:   ", router.address);
  console.log("");
  console.log("Save these addresses for your frontend configuration!");
  console.log("");

  // Generate config file
  const config = {
    network: hre.network.name,
    weth: weth.address,
    factory: factory.address,
    router: router.address,
    deployer: deployer.address,
    deployedAt: new Date().toISOString()
  };

  const fs = require("fs");
  fs.writeFileSync(
    "./deployments.json",
    JSON.stringify(config, null, 2)
  );
  console.log("📄 Deployment config saved to deployments.json");
  console.log("");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
