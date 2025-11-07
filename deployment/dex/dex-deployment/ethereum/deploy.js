const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

/**
 * ËTRID Ethereum Deployment Script
 *
 * Deploys ÉTR token to Ethereum mainnet for Uniswap V3 integration
 *
 * Requirements:
 * - PRIVATE_KEY in .env (Foundation multisig signer)
 * - ETHERSCAN_API_KEY in .env (for contract verification)
 * - At least 0.1 ETH for deployment gas (~$300 @ $3000/ETH)
 *
 * Deployment Steps:
 * 1. Deploy EtridETH contract
 * 2. Verify on Etherscan
 * 3. Save deployment addresses
 * 4. Generate next steps guide
 */

async function main() {
  console.log("╔════════════════════════════════════════════════════════════╗");
  console.log("║     ËTRID DEX DEPLOYMENT - ETHEREUM (ERC-20) TOKEN        ║");
  console.log("╚════════════════════════════════════════════════════════════╝\n");

  const network = hre.network.name;
  console.log(`📡 Network: ${network}`);
  console.log(`⛓️  Chain ID: ${hre.network.config.chainId}\n`);

  // Get deployer account
  const [deployer] = await hre.ethers.getSigners();
  const deployerAddress = await deployer.getAddress();
  const balance = await hre.ethers.provider.getBalance(deployerAddress);

  console.log("👤 Deployer:", deployerAddress);
  console.log("💰 Balance:", hre.ethers.formatEther(balance), "ETH\n");

  // Check balance
  const minBalance = hre.ethers.parseEther("0.05");
  if (balance < minBalance) {
    console.error("❌ ERROR: Insufficient ETH balance");
    console.error("   Minimum required: 0.05 ETH");
    console.error("   Current balance:", hre.ethers.formatEther(balance), "ETH");
    process.exit(1);
  }

  // Confirmation for mainnet
  if (network === "mainnet") {
    console.log("⚠️  WARNING: Deploying to ETHEREUM MAINNET");
    console.log("   This will cost real ETH (~0.05 ETH = ~$150)");
    console.log("   Make sure you have reviewed the contract!\n");
  }

  console.log("🚀 Deploying EtridETH contract...\n");

  // Deploy token contract
  const EtridETH = await hre.ethers.getContractFactory("EtridETH");

  console.log("📝 Contract: EtridETH (ÉTR on Ethereum)");
  console.log("   Initial Supply: 100,000 ÉTR (Bootstrap amount)");
  console.log("   Decimals: 18");
  console.log("   Owner: Foundation Multisig\n");

  // Deploy with deployer as initial owner
  // NOTE: Transfer ownership to Foundation multisig after deployment
  const token = await EtridETH.deploy(deployerAddress);
  await token.waitForDeployment();

  const tokenAddress = await token.getAddress();
  console.log("✅ EtridETH deployed successfully!");
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

  // Auto-verify on Etherscan if on mainnet/testnet
  if (network !== "hardhat" && network !== "localhost") {
    console.log("🔍 Verifying contract on Etherscan...");
    console.log("   (This may take a minute...)\n");

    try {
      await hre.run("verify:verify", {
        address: tokenAddress,
        constructorArguments: [deployerAddress],
      });
      console.log("✅ Contract verified on Etherscan!");
      console.log("   View at: https://etherscan.io/address/" + tokenAddress);
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

  console.log("2️⃣  Create Uniswap V3 pool:");
  console.log("   - Go to: https://app.uniswap.org/pools");
  console.log("   - Click 'Create Pool'");
  console.log("   - Token 0: ÉTR (" + tokenAddress + ")");
  console.log("   - Token 1: WETH (0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2)");
  console.log("   - Fee tier: 0.30% (3000)");
  console.log("   - Initial price: Set based on BSC/Solana price\n");

  console.log("3️⃣  Add liquidity:");
  console.log("   - Amount: 25,000,000 ÉTR + ~666 ETH (~$2M)");
  console.log("   - Price range: Full range or concentrated");
  console.log("   - Recipient: Foundation multisig\n");

  console.log("4️⃣  Submit to tracking sites:");
  console.log("   - CoinGecko: https://www.coingecko.com/en/coins/new");
  console.log("   - CoinMarketCap: https://coinmarketcap.com/request/");
  console.log("   - Include contract address: " + tokenAddress + "\n");

  console.log("5️⃣  Configure bridge (if not already done):");
  console.log("   - Call: token.setBridgeContract(<BRIDGE_ADDRESS>)");
  console.log("   - This allows cross-chain transfers\n");

  console.log("6️⃣  Announce deployment:");
  console.log("   - Twitter: @EtridProtocol");
  console.log("   - Discord: #announcements");
  console.log("   - Website: Update etrid.org with Ethereum address\n");

  console.log("📊 Important Addresses:");
  console.log("   ÉTR Token:", tokenAddress);
  console.log("   WETH:", "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
  console.log("   Uniswap V3 Factory:", "0x1F98431c8aD98523631AE4a59f267346ea31F984");
  console.log("   Uniswap V3 Router:", "0xE592427A0AEce92De3Edee1F18E0157C05861564");
  console.log("   Uniswap Position Manager:", "0xC36442b4a4522E871399CD717aBDD847Ab11FE88");
  console.log("");

  console.log("🔐 Security Reminders:");
  console.log("   - Store deployment info securely");
  console.log("   - Backup all transaction hashes");
  console.log("   - Test small swaps before announcing");
  console.log("   - Monitor liquidity 24/7 for first week");
  console.log("   - Foundation multisig controls all operations");
  console.log("");

  console.log("✅ Ethereum deployment complete! Ready for Uniswap V3 🦄");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
