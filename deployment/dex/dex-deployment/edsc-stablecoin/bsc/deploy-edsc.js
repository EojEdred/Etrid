const hre = require("hardhat");

async function main() {
  console.log("\n═══════════════════════════════════════════════════════════");
  console.log("  EDSC (Ëtrid Dollar Stablecoin) Deployment - Base L2");
  console.log("═══════════════════════════════════════════════════════════\n");

  const [deployer] = await hre.ethers.getSigners();

  console.log("Deploying with account:", deployer.address);
  console.log("Account balance:", hre.ethers.formatEther(await hre.ethers.provider.getBalance(deployer.address)), "ETH\n");

  // Get foundation multisig from env (fallback to deployer for testing)
  const foundationMultisig = process.env.FOUNDATION_MULTISIG || deployer.address;

  // Get reserve vault from env (fallback to deployer for testing)
  // IMPORTANT: For mainnet, use proper multisig vault address!
  const reserveVault = process.env.RESERVE_VAULT || deployer.address;

  console.log("📝 Contract Configuration:");
  console.log("  Name: Etrid Dollar Stablecoin");
  console.log("  Symbol: EDSC");
  console.log("  Decimals: 18");
  console.log("  Total Supply: 1,000,000,000 EDSC");
  console.log("  Owner:", foundationMultisig);
  console.log("  Reserve Vault:", reserveVault);
  console.log("  Peg: $1.00 USD");
  console.log("  Backing: Treasury-backed (organic from purchases)\n");

  console.log("🚀 Deploying EdscuBSC contract...");
  const EdscuBSC = await hre.ethers.getContractFactory("EdscuBSC");
  const edsc = await EdscuBSC.deploy(foundationMultisig, reserveVault);
  await edsc.waitForDeployment();

  const edscAddress = await edsc.getAddress();
  console.log("✅ EdscuBSC deployed to:", edscAddress);

  // Get deployment info
  const totalSupply = await edsc.totalSupply();
  const maxSupply = await edsc.MAX_SUPPLY();
  const isPaused = await edsc.paused();
  const owner = await edsc.owner();

  console.log("\n📊 Deployment Summary:");
  console.log("════════════════════════════════════════════════════════════");
  console.log("  Contract Address:", edscAddress);
  console.log("  Total Supply:", hre.ethers.formatEther(totalSupply), "EDSC");
  console.log("  Max Supply:", hre.ethers.formatEther(maxSupply), "EDSC");
  console.log("  Owner:", owner);
  console.log("  Paused:", isPaused);
  console.log("  Target Peg:", "$1.00 USD");
  console.log("════════════════════════════════════════════════════════════\n");

  // Save deployment info
  const deployment = {
    network: hre.network.name,
    chainId: (await hre.ethers.provider.getNetwork()).chainId.toString(),
    token: "EDSC",
    type: "Stablecoin",
    contract: "EdscuBSC",
    address: edscAddress,
    deployer: deployer.address,
    owner: owner,
    totalSupply: hre.ethers.formatEther(totalSupply),
    maxSupply: hre.ethers.formatEther(maxSupply),
    decimals: 18,
    targetPeg: "$1.00 USD",
    deployedAt: new Date().toISOString(),
    transactionHash: edsc.deploymentTransaction().hash
  };

  const fs = require('fs');
  const filename = `deployment-edsc-${hre.network.name}-${Date.now()}.json`;
  fs.writeFileSync(filename, JSON.stringify(deployment, null, 2));
  console.log("💾 Deployment info saved to:", filename);

  console.log("\n📋 Next Steps:");
  console.log("════════════════════════════════════════════════════════════");
  console.log("1. Verify contract on BaseScan:");
  console.log(`   npx hardhat verify --network ${hre.network.name} ${edscAddress} ${foundationMultisig}`);
  console.log("\n2. Create EDSC/USDC stable pool:");
  console.log("   - Visit: https://app.uniswap.org/#/add");
  console.log("   - Or Curve (if available on Base)");
  console.log("   - Pair: EDSC/USDC");
  console.log("   - Range: $0.99 - $1.01 (tight range for stables)");
  console.log("\n3. Set bridge contracts (after bridge deployment):");
  console.log(`   edsc.setBaseBridge(BRIDGE_ADDRESS)`);
  console.log(`   edsc.setFlarechainBridge(BRIDGE_ADDRESS)`);
  console.log("\n4. Transfer ownership to multisig (if not already set):");
  console.log(`   edsc.transferOwnership(MULTISIG_ADDRESS)`);
  console.log("\n5. Lock equivalent EDSC on FlareChain:");
  console.log("   - Lock 100,000 EDSC on FlareChain reserve");
  console.log("   - Maintain 1:1 backing ratio");
  console.log("════════════════════════════════════════════════════════════\n");

  console.log("✅ EDSC deployment complete on Base!\n");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
