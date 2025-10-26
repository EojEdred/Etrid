import { ethers } from "hardhat";

/**
 * Deploy ÉTR token to BSC Testnet
 *
 * Usage:
 *   npm run deploy:testnet
 *
 * Prerequisites:
 *   1. Wallet funded with testnet BNB (check with: npm run check-balance)
 *   2. .env file configured with DEPLOYER_PRIVATE_KEY
 */
async function main() {
  console.log("\n🚀 Deploying ÉTR Token to BSC Testnet...\n");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Get deployer wallet
  const [deployer] = await ethers.getSigners();
  const deployerAddress = await deployer.getAddress();

  console.log("📍 Deployment Details:");
  console.log(`  Deployer: ${deployerAddress}`);

  // Check balance
  const balance = await ethers.provider.getBalance(deployerAddress);
  const balanceInBNB = ethers.formatEther(balance);
  console.log(`  Balance:  ${balanceInBNB} BNB`);

  if (balance < ethers.parseEther("0.01")) {
    console.log("\n❌ Insufficient balance!");
    console.log("Get testnet BNB: https://testnet.bnbchain.org/faucet-smart\n");
    return;
  }

  // Get network info
  const network = await ethers.provider.getNetwork();
  console.log(`  Network:  BSC Testnet (Chain ID: ${network.chainId})`);
  console.log();

  // Token parameters
  const TOKEN_NAME = "Etrid Coin (BSC Testnet)";
  const TOKEN_SYMBOL = "ÉTR";

  console.log("📝 Token Configuration:");
  console.log(`  Name:     ${TOKEN_NAME}`);
  console.log(`  Symbol:   ${TOKEN_SYMBOL}`);
  console.log(`  Decimals: 18`);
  console.log(`  Supply:   0 (minted via bridge only)`);
  console.log();

  // Deploy token contract
  console.log("⏳ Deploying contract...");
  const EtridToken = await ethers.getContractFactory("EtridToken");
  const etr = await EtridToken.deploy(TOKEN_NAME, TOKEN_SYMBOL);

  console.log("  Transaction submitted, waiting for confirmation...");
  await etr.waitForDeployment();

  const etrAddress = await etr.getAddress();
  console.log(`  ✅ Contract deployed!`);
  console.log();

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("🎉 DEPLOYMENT SUCCESSFUL!\n");
  console.log("📋 Contract Details:");
  console.log(`  Address: ${etrAddress}`);
  console.log(`  Explorer: https://testnet.bscscan.com/address/${etrAddress}`);
  console.log();

  console.log("📝 Next Steps:");
  console.log("1. Save contract address to .env:");
  console.log(`   ETR_TOKEN_ADDRESS_TESTNET=${etrAddress}`);
  console.log();
  console.log("2. Verify contract on BscScan:");
  console.log(`   npx hardhat verify --network bscTestnet ${etrAddress} "${TOKEN_NAME}" "${TOKEN_SYMBOL}"`);
  console.log();
  console.log("3. Add token to MetaMask:");
  console.log(`   - Token Address: ${etrAddress}`);
  console.log(`   - Symbol: ${TOKEN_SYMBOL}`);
  console.log(`   - Decimals: 18`);
  console.log();
  console.log("4. Test minting (if you have MINTER_ROLE):");
  console.log(`   - Call bridgeMint() function on BscScan`);
  console.log(`   - Mint to your address for testing`);
  console.log();

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Save deployment info to file
  const fs = require("fs");
  const deploymentInfo = {
    network: "bscTestnet",
    chainId: Number(network.chainId),
    contractAddress: etrAddress,
    deployer: deployerAddress,
    timestamp: new Date().toISOString(),
    txHash: etr.deploymentTransaction()?.hash,
  };

  fs.writeFileSync(
    "deployment-testnet.json",
    JSON.stringify(deploymentInfo, null, 2)
  );
  console.log("💾 Deployment info saved to: deployment-testnet.json\n");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Deployment failed:");
    console.error(error);
    process.exit(1);
  });
