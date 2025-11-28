import { ethers } from "hardhat";
import * as readline from "readline";

/**
 * Deploy ÉTR token to BSC Mainnet
 *
 * Usage:
 *   npm run deploy:mainnet
 *
 * Prerequisites:
 *   1. Wallet funded with REAL BNB (~$20 worth for gas)
 *   2. .env file configured with DEPLOYER_PRIVATE_KEY
 *   3. Successfully tested on testnet first!
 *
 * ⚠️ WARNING: THIS COSTS REAL MONEY!
 */

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

function question(query: string): Promise<string> {
  return new Promise((resolve) => {
    rl.question(query, resolve);
  });
}

async function main() {
  console.log("\n⚠️  BSC MAINNET DEPLOYMENT ⚠️\n");
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

  // Estimate cost
  const gasPrice = (await ethers.provider.getFeeData()).gasPrice!;
  const estimatedGas = 2000000n; // ~2M gas
  const estimatedCost = gasPrice * estimatedGas;
  const estimatedCostBNB = ethers.formatEther(estimatedCost);

  console.log(`  Gas Price: ${ethers.formatUnits(gasPrice, "gwei")} gwei`);
  console.log(`  Est. Cost: ~${estimatedCostBNB} BNB (~$${(parseFloat(estimatedCostBNB) * 600).toFixed(2)} USD @ $600/BNB)`);

  // Safety check
  if (balance < estimatedCost * 2n) {
    console.log("\n❌ Insufficient balance!");
    console.log(`   Required: ~${ethers.formatEther(estimatedCost * 2n)} BNB (2x buffer)`);
    console.log(`   You have: ${balanceInBNB} BNB`);
    console.log("\n   Buy BNB on Binance or another exchange and send to:");
    console.log(`   ${deployerAddress}\n`);
    rl.close();
    process.exit(1);
  }

  // Get network info
  const network = await ethers.provider.getNetwork();
  console.log(`  Network:  BSC Mainnet (Chain ID: ${network.chainId})`);

  if (network.chainId !== 56n) {
    console.log("\n❌ ERROR: Not connected to BSC Mainnet!");
    console.log(`   Expected Chain ID: 56`);
    console.log(`   Current Chain ID: ${network.chainId}`);
    console.log("\n   Check your hardhat.config.ts and --network flag\n");
    rl.close();
    process.exit(1);
  }

  console.log();

  // Token parameters
  const TOKEN_NAME = "Etrid Coin";
  const TOKEN_SYMBOL = "ÉTR";

  console.log("📝 Token Configuration:");
  console.log(`  Name:     ${TOKEN_NAME}`);
  console.log(`  Symbol:   ${TOKEN_SYMBOL}`);
  console.log(`  Decimals: 18`);
  console.log(`  Supply:   0 (minted via bridge only)`);
  console.log();

  // Final confirmation
  console.log("⚠️  FINAL WARNINGS:");
  console.log("  1. This will cost REAL MONEY (~$5-20 in gas)");
  console.log("  2. This deployment is PERMANENT and IRREVERSIBLE");
  console.log("  3. You should have tested on testnet first");
  console.log("  4. Contract ownership will be YOUR wallet initially");
  console.log("  5. You must transfer ownership to multi-sig ASAP after deployment");
  console.log();

  const confirm1 = await question("Have you tested this on BSC testnet? (yes/no): ");
  if (confirm1.toLowerCase() !== "yes") {
    console.log("\n❌ Please test on testnet first!");
    console.log("   Run: npm run deploy:testnet\n");
    rl.close();
    process.exit(0);
  }

  const confirm2 = await question(`\nType "DEPLOY TO MAINNET" to continue: `);
  if (confirm2 !== "DEPLOY TO MAINNET") {
    console.log("\n❌ Deployment cancelled\n");
    rl.close();
    process.exit(0);
  }

  console.log();
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Deploy token contract
  console.log("⏳ Deploying contract to BSC Mainnet...");
  console.log("   This may take 1-2 minutes...");
  console.log();

  const EtridToken = await ethers.getContractFactory("EtridToken");
  const etr = await EtridToken.deploy(TOKEN_NAME, TOKEN_SYMBOL);

  console.log("  Transaction submitted, waiting for confirmations...");
  await etr.waitForDeployment();

  const etrAddress = await etr.getAddress();
  const txHash = etr.deploymentTransaction()?.hash;

  console.log(`  ✅ Contract deployed!`);
  console.log();

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("🎉 MAINNET DEPLOYMENT SUCCESSFUL!\n");
  console.log("📋 Contract Details:");
  console.log(`  Address: ${etrAddress}`);
  console.log(`  TX Hash: ${txHash}`);
  console.log(`  Explorer: https://bscscan.com/address/${etrAddress}`);
  console.log(`  TX: https://bscscan.com/tx/${txHash}`);
  console.log();

  console.log("⚠️  CRITICAL: Save this address immediately!");
  console.log(`  ETR_TOKEN_ADDRESS_MAINNET=${etrAddress}`);
  console.log();

  console.log("📝 IMMEDIATE Next Steps (DO NOW):");
  console.log();
  console.log("1. Save contract address to .env:");
  console.log(`   echo "ETR_TOKEN_ADDRESS_MAINNET=${etrAddress}" >> .env`);
  console.log();
  console.log("2. Verify contract on BscScan (IMPORTANT for trust):");
  console.log(`   npx hardhat verify --network bscMainnet ${etrAddress} "${TOKEN_NAME}" "${TOKEN_SYMBOL}"`);
  console.log();
  console.log("3. Transfer ownership to multi-sig wallet:");
  console.log(`   - Go to: https://bscscan.com/address/${etrAddress}#writeContract`);
  console.log(`   - Call grantRole() for each role to multi-sig address`);
  console.log(`   - Call revokeRole() to remove your address from all roles`);
  console.log();
  console.log("4. Add ÉTR to CoinGecko/CoinMarketCap:");
  console.log(`   - CoinGecko: https://www.coingecko.com/en/coins/new`);
  console.log(`   - CoinMarketCap: https://coinmarketcap.com/request/`);
  console.log();
  console.log("5. Create PancakeSwap liquidity pool:");
  console.log(`   - Visit: https://pancakeswap.finance/add`);
  console.log(`   - Add ÉTR/BNB pair`);
  console.log(`   - Start with small amount to test!`);
  console.log();

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Save deployment info
  const fs = require("fs");
  const deploymentInfo = {
    network: "bscMainnet",
    chainId: Number(network.chainId),
    contractAddress: etrAddress,
    deployer: deployerAddress,
    timestamp: new Date().toISOString(),
    txHash: txHash,
    gasUsed: etr.deploymentTransaction()?.gasLimit?.toString(),
    gasPrice: gasPrice.toString(),
  };

  fs.writeFileSync(
    "deployment-mainnet.json",
    JSON.stringify(deploymentInfo, null, 2)
  );

  console.log("💾 Deployment info saved to: deployment-mainnet.json\n");
  console.log("🎉 Congratulations! ÉTR is now live on BSC Mainnet!\n");

  rl.close();
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Deployment failed:");
    console.error(error);
    rl.close();
    process.exit(1);
  });
