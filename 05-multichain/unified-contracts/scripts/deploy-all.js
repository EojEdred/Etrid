/**
 * Master deployment script for all Ëtrid contracts
 * Deploys in the correct order with proper configuration
 */

const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

// Domain mappings
const DOMAINS = {
  ethPBC: 2,
  ethereum: 3,
  bsc: 4,
  polygon: 5,
  arbitrum: 6,
  base: 7,
};

async function main() {
  console.log("╔════════════════════════════════════════════════════════════════╗");
  console.log("║        Ëtrid Unified Contract Deployment System                ║");
  console.log("╚════════════════════════════════════════════════════════════════╝\n");

  const [deployer] = await hre.ethers.getSigners();
  const network = hre.network.name;
  const chainId = (await hre.ethers.provider.getNetwork()).chainId;

  console.log("📊 Deployment Information:");
  console.log("  Network:", network);
  console.log("  Chain ID:", chainId.toString());
  console.log("  Deployer:", deployer.address);
  console.log("  Balance:", hre.ethers.formatEther(await hre.ethers.provider.getBalance(deployer.address)), "ETH");
  console.log("  Domain ID:", DOMAINS[network] || 0);
  console.log("");

  // Deployment results
  const deployment = {
    network,
    chainId: chainId.toString(),
    deployer: deployer.address,
    timestamp: new Date().toISOString(),
    contracts: {},
  };

  try {
    // ========================================
    // Phase 1: Token Contracts
    // ========================================
    console.log("🔷 Phase 1: Deploying Token Contracts\n");

    // Deploy WrappedETR
    console.log("📝 Deploying WrappedETR...");
    const WrappedETR = await hre.ethers.getContractFactory("WrappedETR");
    const wrappedETR = await WrappedETR.deploy(deployer.address);
    await wrappedETR.waitForDeployment();
    const wrappedETRAddress = await wrappedETR.getAddress();
    deployment.contracts.wrappedETR = wrappedETRAddress;
    console.log("  ✅ WrappedETR deployed to:", wrappedETRAddress);
    console.log("");

    // Deploy EDSC
    console.log("📝 Deploying EDSC...");
    const EDSC = await hre.ethers.getContractFactory("EDSC");
    const edsc = await EDSC.deploy(deployer.address);
    await edsc.waitForDeployment();
    const edscAddress = await edsc.getAddress();
    deployment.contracts.edsc = edscAddress;
    console.log("  ✅ EDSC deployed to:", edscAddress);
    console.log("");

    // ========================================
    // Phase 2: Bridge Infrastructure
    // ========================================
    console.log("🌉 Phase 2: Deploying Bridge Infrastructure\n");

    // Deploy TokenMessenger
    console.log("📝 Deploying TokenMessenger...");
    const localDomain = DOMAINS[network] || 2;
    const TokenMessenger = await hre.ethers.getContractFactory("TokenMessenger");
    const tokenMessenger = await TokenMessenger.deploy(localDomain, deployer.address);
    await tokenMessenger.waitForDeployment();
    const tokenMessengerAddress = await tokenMessenger.getAddress();
    deployment.contracts.tokenMessenger = tokenMessengerAddress;
    console.log("  ✅ TokenMessenger deployed to:", tokenMessengerAddress);
    console.log("");

    // ========================================
    // Phase 3: Configuration
    // ========================================
    console.log("⚙️  Phase 3: Configuring Contracts\n");

    // Grant MINTER_ROLE and BURNER_ROLE to TokenMessenger
    console.log("📝 Configuring WrappedETR permissions...");
    const MINTER_ROLE = await wrappedETR.MINTER_ROLE();
    const BURNER_ROLE = await wrappedETR.BURNER_ROLE();

    let tx = await wrappedETR.grantRole(MINTER_ROLE, tokenMessengerAddress);
    await tx.wait();
    console.log("  ✅ Granted MINTER_ROLE to TokenMessenger");

    tx = await wrappedETR.grantRole(BURNER_ROLE, tokenMessengerAddress);
    await tx.wait();
    console.log("  ✅ Granted BURNER_ROLE to TokenMessenger");
    console.log("");

    // Configure EDSC permissions
    console.log("📝 Configuring EDSC permissions...");
    const EDSC_MINTER_ROLE = await edsc.MINTER_ROLE();
    const EDSC_BURNER_ROLE = await edsc.BURNER_ROLE();

    tx = await edsc.grantRole(EDSC_MINTER_ROLE, tokenMessengerAddress);
    await tx.wait();
    console.log("  ✅ Granted MINTER_ROLE to TokenMessenger");

    tx = await edsc.grantRole(EDSC_BURNER_ROLE, tokenMessengerAddress);
    await tx.wait();
    console.log("  ✅ Granted BURNER_ROLE to TokenMessenger");
    console.log("");

    // Add supported tokens to TokenMessenger
    console.log("📝 Configuring TokenMessenger...");
    tx = await tokenMessenger.addSupportedToken(wrappedETRAddress);
    await tx.wait();
    console.log("  ✅ Added WrappedETR as supported token");

    tx = await tokenMessenger.addSupportedToken(edscAddress);
    await tx.wait();
    console.log("  ✅ Added EDSC as supported token");
    console.log("");

    // ========================================
    // Phase 4: DeFi Contracts (ETH PBC only)
    // ========================================
    if (network === "ethPBC" || network === "localhost") {
      console.log("💰 Phase 4: Deploying DeFi Contracts\n");

      // Deploy MasterChef
      console.log("📝 Deploying MasterChef...");
      const rewardPerBlock = hre.ethers.parseEther("1"); // 1 ETR per block
      const startBlock = await hre.ethers.provider.getBlockNumber() + 10;

      const MasterChef = await hre.ethers.getContractFactory("MasterChef");
      const masterChef = await MasterChef.deploy(
        wrappedETRAddress,
        rewardPerBlock,
        startBlock
      );
      await masterChef.waitForDeployment();
      const masterChefAddress = await masterChef.getAddress();
      deployment.contracts.masterChef = masterChefAddress;
      console.log("  ✅ MasterChef deployed to:", masterChefAddress);
      console.log("     Reward per block:", hre.ethers.formatEther(rewardPerBlock), "ETR");
      console.log("     Start block:", startBlock);
      console.log("");

      // Deploy Bridge Adapter
      console.log("📝 Deploying ETHPBCBridgeAdapter...");
      const ETHPBCBridgeAdapter = await hre.ethers.getContractFactory("ETHPBCBridgeAdapter");
      const bridgeAdapter = await ETHPBCBridgeAdapter.deploy(
        edscAddress,
        wrappedETRAddress,
        tokenMessengerAddress,
        masterChefAddress
      );
      await bridgeAdapter.waitForDeployment();
      const bridgeAdapterAddress = await bridgeAdapter.getAddress();
      deployment.contracts.bridgeAdapter = bridgeAdapterAddress;
      console.log("  ✅ ETHPBCBridgeAdapter deployed to:", bridgeAdapterAddress);
      console.log("");
    }

    // ========================================
    // Phase 5: Save Deployment Info
    // ========================================
    console.log("💾 Saving deployment information...\n");

    const deploymentDir = path.join(__dirname, "../deployments");
    if (!fs.existsSync(deploymentDir)) {
      fs.mkdirSync(deploymentDir, { recursive: true });
    }

    const deploymentPath = path.join(
      deploymentDir,
      `${network}-${chainId.toString()}.json`
    );
    fs.writeFileSync(deploymentPath, JSON.stringify(deployment, null, 2));
    console.log("  ✅ Deployment info saved to:", deploymentPath);
    console.log("");

    // ========================================
    // Summary
    // ========================================
    console.log("╔════════════════════════════════════════════════════════════════╗");
    console.log("║                    Deployment Summary                          ║");
    console.log("╚════════════════════════════════════════════════════════════════╝\n");

    console.log("📋 Deployed Contracts:");
    for (const [name, address] of Object.entries(deployment.contracts)) {
      console.log(`  ${name}: ${address}`);
    }
    console.log("");

    console.log("🎉 Deployment completed successfully!\n");

    console.log("📚 Next Steps:");
    console.log("  1. Verify contracts on block explorer:");
    console.log(`     npx hardhat verify --network ${network} ${wrappedETRAddress} ${deployer.address}`);
    console.log("  2. Add oracle addresses to TokenMessenger");
    console.log("  3. Test cross-chain transfers");
    console.log("  4. Update frontend configuration");
    console.log("");

  } catch (error) {
    console.error("❌ Deployment failed:", error);
    process.exit(1);
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
