import { ethers } from "hardhat";

/**
 * Check wallet balance on BSC testnet or mainnet
 *
 * Usage:
 *   npm run check-balance (uses testnet by default)
 *   npx hardhat run scripts/check-balance.ts --network bscMainnet
 */
async function main() {
  // Get network info
  const network = await ethers.provider.getNetwork();
  const networkName = network.chainId === 97n ? "BSC Testnet" :
                      network.chainId === 56n ? "BSC Mainnet" :
                      "Unknown Network";

  console.log(`\n🔍 Checking balance on ${networkName} (Chain ID: ${network.chainId})`);
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Get signers (wallets) from config
  const signers = await ethers.getSigners();

  if (signers.length === 0) {
    console.log("❌ No wallet configured!");
    console.log("\n📝 Setup Steps:");
    console.log("1. Generate wallet: npm run generate-wallet");
    console.log("2. Add private key to .env file");
    console.log("3. Run this script again\n");
    return;
  }

  // Check each signer's balance
  for (let i = 0; i < signers.length; i++) {
    const signer = signers[i];
    const address = await signer.getAddress();
    const balance = await ethers.provider.getBalance(address);
    const balanceInBNB = ethers.formatEther(balance);

    console.log(`Wallet ${i + 1}:`);
    console.log(`  Address: ${address}`);
    console.log(`  Balance: ${balanceInBNB} BNB`);

    // Gas cost estimate
    const gasPrice = (await ethers.provider.getFeeData()).gasPrice;
    const estimatedDeploymentGas = 2000000n; // ~2M gas for token deployment
    const estimatedCost = gasPrice! * estimatedDeploymentGas;
    const estimatedCostInBNB = ethers.formatEther(estimatedCost);

    console.log(`  Gas Price: ${ethers.formatUnits(gasPrice!, "gwei")} gwei`);
    console.log(`  Est. Deployment Cost: ~${estimatedCostInBNB} BNB`);

    // Check if sufficient balance
    if (balance < estimatedCost) {
      console.log(`  ⚠️  Insufficient balance for deployment!`);
      if (network.chainId === 97n) {
        console.log(`  Get testnet BNB: https://testnet.bnbchain.org/faucet-smart`);
      } else {
        console.log(`  ⚠️  You need to buy BNB on mainnet!`);
      }
    } else {
      console.log(`  ✅ Sufficient balance for deployment`);
    }
    console.log();
  }

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
