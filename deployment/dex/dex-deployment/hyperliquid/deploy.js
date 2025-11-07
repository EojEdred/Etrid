const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

/**
 * ËTRID Polygon Deployment Script
 *
 * Deploys ÉTR token to Polygon PoS for QuickSwap V3 integration
 *
 * Requirements:
 * - PRIVATE_KEY in .env (Foundation multisig signer)
 * - POLYGONSCAN_API_KEY in .env (for contract verification)
 * - At least 10 MATIC for deployment gas (~$10 @ $1/MATIC)
 *
 * Deployment Steps:
 * 1. Deploy EtridPoly contract
 * 2. Verify on PolygonScan
 * 3. Save deployment addresses
 * 4. Generate next steps guide
 */

async function main() {
  console.log("╔════════════════════════════════════════════════════════════╗");
  console.log("║     ËTRID DEX DEPLOYMENT - POLYGON (ERC-20) TOKEN         ║");
  console.log("╚════════════════════════════════════════════════════════════╝\n");

  const network = hre.network.name;
  console.log(`📡 Network: ${network}`);
  console.log(`⛓️  Chain ID: ${hre.network.config.chainId}\n`);

  // Get deployer account
  const [deployer] = await hre.ethers.getSigners();
  const deployerAddress = await deployer.getAddress();
  const balance = await hre.ethers.provider.getBalance(deployerAddress);

  console.log("👤 Deployer:", deployerAddress);
  console.log("💰 Balance:", hre.ethers.formatEther(balance), "MATIC\n");

  // Check balance
  const minBalance = hre.ethers.parseEther("1");
  if (balance < minBalance) {
    console.error("❌ ERROR: Insufficient MATIC balance");
    console.error("   Minimum required: 1 MATIC");
    console.error("   Current balance:", hre.ethers.formatEther(balance), "MATIC");
    process.exit(1);
  }

  // Confirmation for mainnet
  if (network === "polygon") {
    console.log("⚠️  WARNING: Deploying to POLYGON MAINNET");
    console.log("   This will cost real MATIC (~10 MATIC = ~$10)");
    console.log("   Make sure you have reviewed the contract!\n");
  }

  console.log("🚀 Deploying EtridPoly contract...\n");

  // Deploy token contract
  const EtridPoly = await hre.ethers.getContractFactory("EtridPoly");

  console.log("📝 Contract: EtridPoly (ÉTR on Polygon)");
  console.log("   Initial Supply: 100,000 ÉTR (Bootstrap amount)");
  console.log("   Decimals: 18");
  console.log("   Owner: Foundation Multisig\n");

  // Deploy with deployer as initial owner
  // NOTE: Transfer ownership to Foundation multisig after deployment
  const token = await EtridPoly.deploy(deployerAddress);
  await token.waitForDeployment();

  const tokenAddress = await token.getAddress();
  console.log("✅ EtridPoly deployed successfully!");
  console.log("   Contract Address:", tokenAddress);
  console.log("   Transaction Hash:", token.deploymentTransaction().hash);
  console.log("");

  // Get contract details
  const name = await token.name();
  const symbol = await token.symbol();
  const decimals = await token.decimals();
  const totalSupply = await token.totalSupply();
  const maxSupply = await token.MAX_SUPPLY();

  console.log("📊 Contract Details:");
  console.log("   Name:", name);
  console.log("   Symbol:", symbol);
  console.log("   Decimals:", decimals.toString());
  console.log("   Initial Supply:", hre.ethers.formatEther(totalSupply), "ÉTR");
  console.log("   Max Supply:", hre.ethers.formatEther(maxSupply), "ÉTR");
  console.log("");

  // Save deployment info
  const deploymentInfo = {
    network: network,
    chainId: hre.network.config.chainId,
    contractAddress: tokenAddress,
    deployerAddress: deployerAddress,
    deploymentHash: token.deploymentTransaction().hash,
    timestamp: new Date().toISOString(),
    contractDetails: {
      name: name,
      symbol: symbol,
      decimals: decimals.toString(),
      initialSupply: hre.ethers.formatEther(totalSupply),
      maxSupply: hre.ethers.formatEther(maxSupply)
    }
  };

  const deploymentsDir = path.join(__dirname, "deployments");
  if (!fs.existsSync(deploymentsDir)) {
    fs.mkdirSync(deploymentsDir);
  }

  const deploymentFile = path.join(deploymentsDir, `${network}-deployment.json`);
  fs.writeFileSync(deploymentFile, JSON.stringify(deploymentInfo, null, 2));
  console.log("💾 Deployment info saved to:", deploymentFile);
  console.log("");

  // Auto-verify on PolygonScan if on mainnet/testnet
  if (network !== "hardhat" && network !== "localhost") {
    console.log("🔍 Verifying contract on PolygonScan...");
    console.log("   (This may take a minute...)\n");

    try {
      await hre.run("verify:verify", {
        address: tokenAddress,
        constructorArguments: [deployerAddress],
      });
      console.log("✅ Contract verified on PolygonScan!");
      console.log("   View at: https://polygonscan.com/address/" + tokenAddress);
    } catch (error) {
      console.log("⚠️  Verification failed (you can verify manually later)");
      console.log("   Error:", error.message);
      console.log("\n📝 Manual verification command:");
      console.log(`   npx hardhat verify --network ${network} ${tokenAddress} ${deployerAddress}`);
    }
    console.log("");
  }

  // Next steps
  console.log("╔════════════════════════════════════════════════════════════╗");
  console.log("║                    DEPLOYMENT COMPLETE! ✅                 ║");
  console.log("╚════════════════════════════════════════════════════════════╝\n");

  console.log("📋 NEXT STEPS:\n");

  console.log("1️⃣  Transfer ownership to Foundation multisig:");
  console.log("   - Current owner:", deployerAddress);
  console.log("   - Transfer to: <FOUNDATION_MULTISIG_ADDRESS>");
  console.log("   - Use: token.transferOwnership(multisigAddress)\n");

  console.log("2️⃣  Create QuickSwap V3 pool:");
  console.log("   - Go to: https://quickswap.exchange/#/pools");
  console.log("   - Click 'Create Pool'");
  console.log("   - Token 0: ÉTR (" + tokenAddress + ")");
  console.log("   - Token 1: WMATIC (0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270)");
  console.log("   - Fee tier: 0.30% (3000)");
  console.log("   - Initial price: Set based on BSC/Ethereum price\n");

  console.log("3️⃣  Add liquidity:");
  console.log("   - Amount: 50,000 ÉTR + ~34 MATIC (~$34)");
  console.log("   - Price range: Full range or concentrated");
  console.log("   - Recipient: Foundation multisig");
  console.log("   - NOTE: This is BOOTSTRAP liquidity ($50 budget)");
  console.log("   - Expect HIGH slippage until more liquidity added\n");

  console.log("4️⃣  Submit to tracking sites:");
  console.log("   - Update CoinGecko with Polygon address");
  console.log("   - Update CoinMarketCap with Polygon address");
  console.log("   - Contract: " + tokenAddress + "\n");

  console.log("5️⃣  Configure bridges:");
  console.log("   - Polygon PoS Bridge: token.setPolygonBridge(<POLYGON_BRIDGE>)");
  console.log("   - Cross-chain Bridge: token.setCrossChainBridge(<BRIDGE_ADDRESS>)");
  console.log("   - This allows Ethereum ↔ Polygon + other chains\n");

  console.log("6️⃣  Announce deployment:");
  console.log("   - Twitter: @EtridProtocol");
  console.log("   - Discord: #announcements");
  console.log("   - Website: Update etrid.org with Polygon address\n");

  console.log("📊 Important Addresses:");
  console.log("   ÉTR Token:", tokenAddress);
  console.log("   WMATIC:", "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270");
  console.log("   QuickSwap V3 Factory:", "0x411b0fAcC3489691f28ad58c47006AF5E3Ab3A28");
  console.log("   QuickSwap Router:", "0xf5b509bB0909a69B1c207E495f687a596C168E12");
  console.log("   Polygon PoS Bridge:", "0xA0c68C638235ee32657e8f720a23ceC1bFc77C77");
  console.log("");

  console.log("🔐 Security Reminders:");
  console.log("   - Store deployment info securely");
  console.log("   - Backup all transaction hashes");
  console.log("   - Test small swaps before announcing");
  console.log("   - Monitor liquidity 24/7 for first week");
  console.log("   - Foundation multisig controls all operations");
  console.log("   - Polygon has low fees - great for testing!");
  console.log("");

  console.log("✅ Polygon deployment complete! Ready for QuickSwap ⚡");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
