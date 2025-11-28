const hre = require("hardhat");

async function main() {
  console.log("🚀 Deploying Ëtrid Ethereum Contracts...\n");

  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying with account:", deployer.address);
  console.log("Account balance:", (await deployer.provider.getBalance(deployer.address)).toString());
  console.log();

  // ═══════════════════════════════════════════════════════════
  // Step 1: Deploy ÉTR Token
  // ═══════════════════════════════════════════════════════════
  console.log("📦 Deploying ÉTR Token...");
  const ETRToken = await hre.ethers.getContractFactory("ETRToken");
  
  // For now, use deployer as admin and bridge (will be replaced)
  const etrToken = await ETRToken.deploy(
    deployer.address,  // admin
    deployer.address   // bridge (temporary, will grant to bridge contract)
  );
  await etrToken.waitForDeployment();
  
  const etrAddress = await etrToken.getAddress();
  console.log("✅ ÉTR Token deployed to:", etrAddress);
  console.log();

  // ═══════════════════════════════════════════════════════════
  // Step 2: Deploy EDSC Token
  // ═══════════════════════════════════════════════════════════
  console.log("📦 Deploying EDSC Token...");
  const EDSCToken = await hre.ethers.getContractFactory("EDSCToken");
  
  const edscToken = await EDSCToken.deploy(
    deployer.address,  // admin
    deployer.address   // bridge (temporary, will grant to bridge contract)
  );
  await edscToken.waitForDeployment();
  
  const edscAddress = await edscToken.getAddress();
  console.log("✅ EDSC Token deployed to:", edscAddress);
  console.log();

  // ═══════════════════════════════════════════════════════════
  // Step 3: Deploy Bridge Contract
  // ═══════════════════════════════════════════════════════════
  console.log("📦 Deploying Ëtrid Bridge...");
  
  // Initial watchtowers (3-of-5 multisig)
  // TODO: Replace with actual watchtower addresses
  const initialWatchtowers = [
    deployer.address,  // Replace with real watchtower 1
    deployer.address,  // Replace with real watchtower 2
    deployer.address,  // Replace with real watchtower 3
  ];
  
  const EtridBridge = await hre.ethers.getContractFactory("EtridBridge");
  const bridge = await EtridBridge.deploy(
    deployer.address,
    etrAddress,
    edscAddress,
    initialWatchtowers
  );
  await bridge.waitForDeployment();
  
  const bridgeAddress = await bridge.getAddress();
  console.log("✅ Ëtrid Bridge deployed to:", bridgeAddress);
  console.log();

  // ═══════════════════════════════════════════════════════════
  // Step 4: Grant Bridge Role to Bridge Contract
  // ═══════════════════════════════════════════════════════════
  console.log("🔑 Granting BRIDGE_ROLE to bridge contract...");
  
  const BRIDGE_ROLE = await etrToken.BRIDGE_ROLE();
  
  // Grant to ETR token
  const tx1 = await etrToken.grantRole(BRIDGE_ROLE, bridgeAddress);
  await tx1.wait();
  console.log("✅ Bridge role granted to ETR token");
  
  // Revoke deployer's bridge role on ETR
  const tx2 = await etrToken.revokeRole(BRIDGE_ROLE, deployer.address);
  await tx2.wait();
  console.log("✅ Deployer's bridge role revoked from ETR token");
  
  // Grant to EDSC token
  const tx3 = await edscToken.grantRole(BRIDGE_ROLE, bridgeAddress);
  await tx3.wait();
  console.log("✅ Bridge role granted to EDSC token");
  
  // Revoke deployer's bridge role on EDSC
  const tx4 = await edscToken.revokeRole(BRIDGE_ROLE, deployer.address);
  await tx4.wait();
  console.log("✅ Deployer's bridge role revoked from EDSC token");
  console.log();

  // ═══════════════════════════════════════════════════════════
  // Deployment Summary
  // ═══════════════════════════════════════════════════════════
  console.log("═════════════════════════════════════════════════════════");
  console.log("✅ Deployment Complete!");
  console.log("═════════════════════════════════════════════════════════");
  console.log();
  console.log("📋 Contract Addresses:");
  console.log("─────────────────────────────────────────────────────────");
  console.log("ÉTR Token:       ", etrAddress);
  console.log("EDSC Token:      ", edscAddress);
  console.log("Ëtrid Bridge:    ", bridgeAddress);
  console.log();
  console.log("🔗 Add to frontend .env:");
  console.log("─────────────────────────────────────────────────────────");
  console.log(`NEXT_PUBLIC_ETR_TOKEN_ADDRESS=${etrAddress}`);
  console.log(`NEXT_PUBLIC_EDSC_TOKEN_ADDRESS=${edscAddress}`);
  console.log(`NEXT_PUBLIC_BRIDGE_ADDRESS=${bridgeAddress}`);
  console.log();
  console.log("⚠️  Next Steps:");
  console.log("─────────────────────────────────────────────────────────");
  console.log("1. Replace dummy watchtower addresses with real ones");
  console.log("2. Verify contracts on Etherscan:");
  console.log(`   npx hardhat verify --network ${hre.network.name} ${etrAddress} "${deployer.address}" "${deployer.address}"`);
  console.log(`   npx hardhat verify --network ${hre.network.name} ${edscAddress} "${deployer.address}" "${deployer.address}"`);
  console.log("3. Create Uniswap V3 pools (use scripts/create-uniswap-pools.js)");
  console.log("4. Add initial liquidity (~$3M)");
  console.log("5. Update Substrate bridge pallets with contract addresses");
  console.log("═════════════════════════════════════════════════════════");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
