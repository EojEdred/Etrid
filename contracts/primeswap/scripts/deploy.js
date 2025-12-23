#!/usr/bin/env node
/**
 * PrimeSwap Deployment Script (Hardhat)
 *
 * Deploys PrimeSwap contracts to Primearc Core Chain:
 * - Step 1: PrimeSwapOracle
 * - Step 2: PrimeSwapFactory
 * - Step 3: Display seeding instructions
 *
 * Usage:
 *   npx hardhat run scripts/deploy.js --network primearc
 *
 * Environment:
 *   PRIVATE_KEY - Deployer private key
 */

const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

// Initial prices (8 decimals)
const PRICES = {
  ETR: 400000,           // $0.004
  BTC: 10000000000000,   // $100,000
  ETH: 400000000000,     // $4,000
  XRP: 250000000,        // $2.50
  SOL: 25000000000,      // $250
  BNB: 70000000000,      // $700
  DOGE: 40000000,        // $0.40
  ADA: 100000000,        // $1.00
  LINK: 2500000000,      // $25
  TRX: 30000000,         // $0.30
  XLM: 50000000,         // $0.50
  MATIC: 100000000       // $1.00
};

// Market cap weights (basis points, total = 10000)
const WEIGHTS = {
  BTC: 6766,   // 67.66%
  ETH: 1531,   // 15.31%
  XRP: 499,    // 4.99%
  SOL: 356,    // 3.56%
  BNB: 320,    // 3.20%
  DOGE: 214,   // 2.14%
  ADA: 125,    // 1.25%
  LINK: 71,    // 0.71%
  TRX: 53,     // 0.53%
  XLM: 36,     // 0.36%
  MATIC: 28    // 0.28%
};

async function main() {
  console.log("╔════════════════════════════════════════════════════════════╗");
  console.log("║          PrimeSwap Deployment to Primearc Core             ║");
  console.log("╚════════════════════════════════════════════════════════════╝\n");

  const [deployer] = await hre.ethers.getSigners();
  console.log(`Deployer: ${deployer.address}`);

  const balance = await deployer.getBalance();
  console.log(`Balance: ${hre.ethers.utils.formatEther(balance)} ETR\n`);

  const deployments = {
    network: hre.network.name,
    timestamp: new Date().toISOString(),
    deployer: deployer.address,
    contracts: {}
  };

  // ═══════════════════════════════════════════════════════════════
  // Step 1: Deploy PrimeSwapOracle
  // ═══════════════════════════════════════════════════════════════
  console.log("═══ Step 1: Deploy PrimeSwapOracle ═══");

  const Oracle = await hre.ethers.getContractFactory("PrimeSwapOracle");
  const oracle = await Oracle.deploy();
  await oracle.deployed();

  const oracleAddress = oracle.address;
  console.log(`✓ PrimeSwapOracle deployed at: ${oracleAddress}`);
  deployments.contracts.oracle = oracleAddress;

  // Initialize oracle with ETR price
  console.log("  Initializing ETR price...");
  const tx1 = await oracle.updatePrice(
    "0x0000000000000000000000000000000000000001", // ETR_ADDRESS
    PRICES.ETR
  );
  await tx1.wait();
  console.log(`  ✓ ETR price set to $${PRICES.ETR / 1e8}`);

  // ═══════════════════════════════════════════════════════════════
  // Step 2: Deploy PrimeSwapFactory
  // ═══════════════════════════════════════════════════════════════
  console.log("\n═══ Step 2: Deploy PrimeSwapFactory ═══");

  // Placeholder token addresses (will be updated with real ones)
  const ETR_ADDRESS = process.env.ETR_ADDRESS || "0x0000000000000000000000000000000000000001";
  const EDSC_ADDRESS = process.env.EDSC_ADDRESS || "0x0000000000000000000000000000000000000002";

  const Factory = await hre.ethers.getContractFactory("src/PrimeSwapFactory.sol:PrimeSwapFactory");
  const factory = await Factory.deploy(ETR_ADDRESS, EDSC_ADDRESS, oracleAddress);
  await factory.deployed();

  const factoryAddress = factory.address;
  console.log(`✓ PrimeSwapFactory deployed at: ${factoryAddress}`);
  deployments.contracts.factory = factoryAddress;

  // ═══════════════════════════════════════════════════════════════
  // Step 3: Display Seeding Instructions
  // ═══════════════════════════════════════════════════════════════
  console.log("\n═══ Step 3: Pool Seeding Instructions ═══");

  const TOTAL_ETR = BigInt("1250000000000000000000000000"); // 1.25B ETR
  const totalWeight = Object.values(WEIGHTS).reduce((a, b) => a + b, 0);

  console.log("\nETR Allocations (market cap weighted):");
  console.log("─────────────────────────────────────────");

  for (const [token, weight] of Object.entries(WEIGHTS)) {
    const allocation = (TOTAL_ETR * BigInt(weight)) / BigInt(totalWeight);
    const formatted = (Number(allocation) / 1e18).toLocaleString();
    const price = PRICES[token];
    console.log(`  ETR/w${token.padEnd(5)} ${formatted.padStart(15)} ETR (${(weight / 100).toFixed(2)}%) @ $${price / 1e8}`);
  }

  console.log("\nEDSC Allocations:");
  console.log("─────────────────────────────────────────");
  console.log(`  ETR/EDSC pool:   100,000,000 EDSC`);
  console.log(`  EDSC/USDT pool:   10,000,000 EDSC`);

  // Save deployments
  const deploymentsPath = path.join(__dirname, `../deployments-${hre.network.name}.json`);
  fs.writeFileSync(deploymentsPath, JSON.stringify(deployments, null, 2));
  console.log(`\n✓ Deployments saved to: ${deploymentsPath}`);

  console.log("\n╔════════════════════════════════════════════════════════════╗");
  console.log("║                 Deployment Complete!                       ║");
  console.log("╚════════════════════════════════════════════════════════════╝");

  console.log("\nContract Addresses:");
  console.log(`  Oracle:  ${oracleAddress}`);
  console.log(`  Factory: ${factoryAddress}`);

  console.log("\nNext Steps:");
  console.log("1. Set wrapped token addresses in oracle (updatePrice)");
  console.log("2. Create pools via factory.createVirtualReservePool()");
  console.log("3. Seed pools with ETR from treasury (ca4... account)");
  console.log("4. Create EDSCOraclePool for ETR/EDSC");
  console.log("5. Create EDSCPegPool for EDSC/USDT");

  return deployments;
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Deployment failed:", error);
    process.exit(1);
  });
