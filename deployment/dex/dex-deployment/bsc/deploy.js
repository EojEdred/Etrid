const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

async function main() {
  console.log("╔════════════════════════════════════════════════════════════╗");
  console.log("║     ËTRID DEX DEPLOYMENT - BSC (BEP-20) TOKEN            ║");
  console.log("╚════════════════════════════════════════════════════════════╝\n");

  const network = hre.network.name;
  console.log(`Network: ${network}`);
  console.log(`ChainId: ${hre.network.config.chainId}\n`);

  // Get deployer
  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying with account:", deployer.address);

  const balance = await hre.ethers.provider.getBalance(deployer.address);
  console.log("Account balance:", hre.ethers.formatEther(balance), "BNB\n");

  if (parseFloat(hre.ethers.formatEther(balance)) < 0.01) {
    console.error("⚠️  WARNING: Insufficient BNB balance. Need at least 0.01 BNB for deployment.");
    console.error("   Get BNB at: https://www.binance.com");
    process.exit(1);
  }

  // Deploy EtridBSC token
  console.log("Deploying EtridBSC token...");
  const EtridBSC = await hre.ethers.getContractFactory("EtridBSC");
  const token = await EtridBSC.deploy();

  await token.waitForDeployment();
  const tokenAddress = await token.getAddress();

  console.log("✅ EtridBSC token deployed to:", tokenAddress);

  // Get token info
  const name = await token.name();
  const symbol = await token.symbol();
  const decimals = await token.decimals();
  const totalSupply = await token.totalSupply();

  console.log("\n📊 Token Information:");
  console.log("   Name:", name);
  console.log("   Symbol:", symbol);
  console.log("   Decimals:", decimals.toString());
  console.log("   Total Supply:", hre.ethers.formatEther(totalSupply), "ÉTR");
  console.log("   Max Supply:", "1,000,000,000 ÉTR");

  // Save deployment info
  const deploymentInfo = {
    network: network,
    chainId: hre.network.config.chainId,
    tokenAddress: tokenAddress,
    deployer: deployer.address,
    deployedAt: new Date().toISOString(),
    name: name,
    symbol: symbol,
    decimals: decimals.toString(),
    totalSupply: hre.ethers.formatEther(totalSupply),
    blockNumber: await hre.ethers.provider.getBlockNumber(),
    transactionHash: token.deploymentTransaction().hash
  };

  const deployDir = path.join(__dirname, "deployments");
  if (!fs.existsSync(deployDir)) {
    fs.mkdirSync(deployDir, { recursive: true });
  }

  const deployFile = path.join(deployDir, `${network}-${Date.now()}.json`);
  fs.writeFileSync(deployFile, JSON.stringify(deploymentInfo, null, 2));

  console.log("\n💾 Deployment info saved to:", deployFile);

  // Verification instructions
  if (network !== "hardhat" && network !== "localhost") {
    console.log("\n📝 Next Steps:");
    console.log("\n1. Verify contract on BSCScan:");
    console.log(`   npx hardhat verify --network ${network} ${tokenAddress}`);

    console.log("\n2. Create PancakeSwap pool:");
    console.log(`   - Go to: https://pancakeswap.finance/liquidity`);
    console.log(`   - Token 0: ${tokenAddress} (ÉTR)`);
    console.log(`   - Token 1: 0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c (WBNB)`);
    console.log(`   - Fee tier: 0.25%`);
    console.log(`   - Add liquidity: 50,000 ÉTR + ~$17 BNB (if splitting budget)`);
    console.log(`   - NOTE: Recommend deploying Polygon first ($50 budget)`);

    console.log("\n3. Submit to CoinGecko:");
    console.log(`   https://www.coingecko.com/en/coins/new`);

    console.log("\n4. Submit to CoinMarketCap:");
    console.log(`   https://coinmarketcap.com/request/`);

    console.log("\n5. Update etrid.org with token address");

    // Wait for confirmations before verification
    console.log("\n⏳ Waiting for 5 block confirmations...");
    await token.deploymentTransaction().wait(5);
    console.log("✅ Confirmed!");

    // Auto-verify if API key is set
    if (process.env.BSCSCAN_API_KEY) {
      console.log("\n🔍 Auto-verifying on BSCScan...");
      try {
        await hre.run("verify:verify", {
          address: tokenAddress,
          constructorArguments: [],
        });
        console.log("✅ Contract verified on BSCScan!");
      } catch (error) {
        console.error("❌ Verification failed:", error.message);
        console.log("   Verify manually with the command above");
      }
    }
  }

  console.log("\n═══════════════════════════════════════════════════════════");
  console.log("🎉 DEPLOYMENT COMPLETE!");
  console.log("═══════════════════════════════════════════════════════════\n");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
