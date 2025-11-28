import { ethers } from "hardhat";

/**
 * Generate a new wallet for testnet/mainnet deployment
 *
 * Usage:
 *   npm run generate-wallet
 *
 * IMPORTANT: Save the private key securely!
 * Add it to .env as DEPLOYER_PRIVATE_KEY
 */
async function main() {
  console.log("🔐 Generating new Ethereum wallet...\n");

  // Generate random wallet
  const wallet = ethers.Wallet.createRandom();

  console.log("✅ New Wallet Generated!");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
  console.log(`Address:     ${wallet.address}`);
  console.log(`Private Key: ${wallet.privateKey}`);
  console.log(`Mnemonic:    ${wallet.mnemonic?.phrase}`);
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  console.log("⚠️  SECURITY WARNINGS:");
  console.log("1. NEVER share your private key or mnemonic with anyone!");
  console.log("2. NEVER commit .env file to version control!");
  console.log("3. Back up your mnemonic in a secure location (paper wallet)");
  console.log("4. This wallet is for TESTNET ONLY initially\n");

  console.log("📝 Next Steps:");
  console.log("1. Copy the private key above");
  console.log("2. Create .env file: cp .env.example .env");
  console.log("3. Add to .env: DEPLOYER_PRIVATE_KEY=<your_private_key>");
  console.log("4. Get testnet BNB from faucet:");
  console.log("   https://testnet.bnbchain.org/faucet-smart");
  console.log(`   Paste this address: ${wallet.address}`);
  console.log("5. Check balance: npm run check-balance\n");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
