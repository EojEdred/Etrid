/**
 * PrimeSwap Pool Seeding Script
 *
 * This script:
 * 1. Deploys PrimeSwapFactory
 * 2. Registers all 11 PBC wrapped tokens
 * 3. Creates VirtualReserveAMM pools for each token
 * 4. Creates EDSCOraclePool (ETR/EDSC)
 * 5. Creates EDSCPegPool (EDSC/USDT)
 * 6. Seeds pools with 1.25B ETR (market cap weighted)
 * 7. Seeds EDSC pool with 100M EDSC
 *
 * Market Cap Weighted Distribution:
 * BTC:   67.66% = 845,750,000 ETR
 * ETH:   15.31% = 191,375,000 ETR
 * XRP:    4.99% =  62,375,000 ETR
 * SOL:    3.56% =  44,500,000 ETR
 * BNB:    3.20% =  40,000,000 ETR
 * DOGE:   2.14% =  26,750,000 ETR
 * ADA:    1.25% =  15,625,000 ETR
 * LINK:   0.71% =   8,875,000 ETR
 * TRX:    0.53% =   6,625,000 ETR
 * XLM:    0.36% =   4,500,000 ETR
 * MATIC:  0.28% =   3,625,000 ETR
 * ─────────────────────────────
 * Total: 100.00% = 1,250,000,000 ETR
 */

const { ethers } = require("hardhat");

// Configuration
const config = {
  // Total ETR to distribute across pools
  totalETR: ethers.parseEther("1250000000"), // 1.25B ETR

  // Initial ETR price in USD (8 decimals)
  initialETRPrice: 400000, // $0.004

  // EDSC to seed in ETR/EDSC pool
  initialEDSC: ethers.parseEther("100000000"), // 100M EDSC

  // EDSC/USDT pool - single-sided EDSC (no USDT needed)
  initialEDSCForPeg: ethers.parseEther("10000000"), // 10M EDSC
  stableSwapThreshold: ethers.parseUnits("1000000", 6), // 1M USDT to activate StableSwap

  // Virtual reserve phase out thresholds
  btcPhaseOutThreshold: ethers.parseUnits("100", 8), // 100 BTC
  ethPhaseOutThreshold: ethers.parseEther("5000"), // 5000 ETH
  defaultPhaseOutThreshold: ethers.parseEther("1000000"), // $1M equivalent

  // AMM transition threshold for ETR/EDSC pool
  ammTransitionThreshold: ethers.parseEther("100000000"), // 100M ETR
};

// PBC Tokens with market cap weights (basis points, 10000 = 100%)
const pbcTokens = [
  { symbol: "wBTC", weight: 6766, priceUSD: 10000000000000 }, // $100,000 (8 decimals)
  { symbol: "wETH", weight: 1531, priceUSD: 400000000000 },   // $4,000
  { symbol: "wXRP", weight: 499, priceUSD: 250000000 },       // $2.50
  { symbol: "wSOL", weight: 356, priceUSD: 25000000000 },     // $250
  { symbol: "wBNB", weight: 320, priceUSD: 70000000000 },     // $700
  { symbol: "wDOGE", weight: 214, priceUSD: 40000000 },       // $0.40
  { symbol: "wADA", weight: 125, priceUSD: 100000000 },       // $1.00
  { symbol: "wLINK", weight: 71, priceUSD: 2500000000 },      // $25
  { symbol: "wTRX", weight: 53, priceUSD: 30000000 },         // $0.30
  { symbol: "wXLM", weight: 36, priceUSD: 50000000 },         // $0.50
  { symbol: "wMATIC", weight: 28, priceUSD: 100000000 },      // $1.00
];

async function main() {
  console.log("╔════════════════════════════════════════════════════════════╗");
  console.log("║           PrimeSwap Pool Seeding Script                    ║");
  console.log("║      Deploying 13 pools with 1.25B ETR + 100M EDSC         ║");
  console.log("╚════════════════════════════════════════════════════════════╝");
  console.log("");

  const [deployer] = await ethers.getSigners();
  console.log("Deployer:", deployer.address);

  // Get contract instances (assumes tokens already deployed)
  const etrAddress = process.env.ETR_ADDRESS;
  const edscAddress = process.env.EDSC_ADDRESS;
  const usdtAddress = process.env.USDT_ADDRESS;
  const oracleAddress = process.env.ORACLE_ADDRESS;

  if (!etrAddress || !edscAddress) {
    console.error("ETR_ADDRESS and EDSC_ADDRESS must be set");
    process.exit(1);
  }

  const ETR = await ethers.getContractAt("IERC20", etrAddress);
  const EDSC = await ethers.getContractAt("IERC20", edscAddress);

  console.log("\n=== Step 1: Deploy PrimeSwapFactory ===");

  const PrimeSwapFactory = await ethers.getContractFactory("PrimeSwapFactory");
  const factory = await PrimeSwapFactory.deploy(
    etrAddress,
    edscAddress,
    oracleAddress || deployer.address
  );
  await factory.waitForDeployment();
  console.log("Factory deployed:", await factory.getAddress());

  console.log("\n=== Step 2: Register PBC Tokens ===");

  // In production, these addresses come from PBC bridge deployments
  // For now, use placeholder addresses (or deploy mock tokens)
  const tokenAddresses = {};
  for (const token of pbcTokens) {
    // Deploy mock token for testing (in production, use real addresses)
    const MockERC20 = await ethers.getContractFactory("MockERC20");
    const mockToken = await MockERC20.deploy(token.symbol, token.symbol, 18);
    await mockToken.waitForDeployment();
    tokenAddresses[token.symbol] = await mockToken.getAddress();

    // Register with factory
    await factory.registerPBCToken(
      token.symbol,
      tokenAddresses[token.symbol],
      token.weight
    );
    console.log(`Registered ${token.symbol}: ${tokenAddresses[token.symbol]} (weight: ${token.weight})`);
  }

  console.log("\n=== Step 3: Create VirtualReserveAMM Pools ===");

  for (const token of pbcTokens) {
    const threshold = token.symbol === "wBTC"
      ? config.btcPhaseOutThreshold
      : token.symbol === "wETH"
        ? config.ethPhaseOutThreshold
        : config.defaultPhaseOutThreshold;

    await factory.createVirtualReservePool(
      tokenAddresses[token.symbol],
      threshold
    );
    const poolAddress = await factory.virtualReservePools(tokenAddresses[token.symbol]);
    console.log(`ETR/${token.symbol} pool: ${poolAddress}`);
  }

  console.log("\n=== Step 4: Create EDSCOraclePool (ETR/EDSC) ===");

  await factory.createEDSCOraclePool(config.ammTransitionThreshold);
  const edscPoolAddress = await factory.edscOraclePool();
  console.log(`ETR/EDSC pool: ${edscPoolAddress}`);

  console.log("\n=== Step 5: Create EDSCPegPool (EDSC/USDT) - Single-sided ===");

  if (usdtAddress) {
    await factory.createEDSCPegPool(usdtAddress, 6, config.stableSwapThreshold);
    const pegPoolAddress = await factory.edscPegPool();
    console.log(`EDSC/USDT peg pool: ${pegPoolAddress}`);
    console.log(`  (Single-sided: EDSC seeded, users add USDT)`);
  } else {
    console.log("USDT_ADDRESS not set, skipping peg pool");
  }

  console.log("\n=== Step 6: Calculate ETR Allocations ===");

  const [tokens, allocations] = await factory.calculateAllocations(config.totalETR);
  console.log("\nAllocation breakdown:");
  for (let i = 0; i < tokens.length; i++) {
    const token = pbcTokens[i];
    const allocation = ethers.formatEther(allocations[i]);
    console.log(`  ${token.symbol.padEnd(8)} ${Number(allocation).toLocaleString().padStart(20)} ETR (${(token.weight / 100).toFixed(2)}%)`);
  }

  console.log("\n=== Step 7: Approve ETR for Pool Seeding ===");

  // Approve factory to spend ETR
  const totalApproval = config.totalETR;
  await ETR.approve(await factory.getAddress(), totalApproval);
  console.log(`Approved ${ethers.formatEther(totalApproval)} ETR for factory`);

  console.log("\n=== Step 8: Initialize VirtualReserveAMM Pools ===");

  for (let i = 0; i < pbcTokens.length; i++) {
    const token = pbcTokens[i];
    const poolAddress = await factory.virtualReservePools(tokenAddresses[token.symbol]);
    const pool = await ethers.getContractAt("VirtualReserveAMM", poolAddress);

    // Approve ETR for this pool
    await ETR.approve(poolAddress, allocations[i]);

    // Initialize pool with ETR and oracle prices
    await pool.initializePool(
      allocations[i],
      config.initialETRPrice,
      token.priceUSD
    );

    console.log(`Initialized ETR/${token.symbol}: ${ethers.formatEther(allocations[i])} ETR`);
  }

  console.log("\n=== Step 9: Initialize EDSCOraclePool ===");

  // Approve EDSC for ETR/EDSC pool
  await EDSC.approve(edscPoolAddress, config.initialEDSC);

  const edscPool = await ethers.getContractAt("EDSCOraclePool", edscPoolAddress);
  await edscPool.initializePool(config.initialEDSC, config.initialETRPrice);
  console.log(`Initialized ETR/EDSC: ${ethers.formatEther(config.initialEDSC)} EDSC`);

  console.log("\n=== Step 10: Initialize EDSCPegPool (Single-sided EDSC) ===");

  if (usdtAddress) {
    const pegPoolAddress = await factory.edscPegPool();

    // Approve EDSC only (single-sided)
    await EDSC.approve(pegPoolAddress, config.initialEDSCForPeg);

    const pegPool = await ethers.getContractAt("EDSCPegPool", pegPoolAddress);
    await pegPool.initializePool(config.initialEDSCForPeg);
    console.log(`Initialized EDSC/USDT: ${ethers.formatEther(config.initialEDSCForPeg)} EDSC (single-sided)`);
    console.log(`  Users can swap USDT → EDSC at 1:1`);
    console.log(`  USDT liquidity builds as users swap`);
    console.log(`  StableSwap activates at ${ethers.formatUnits(config.stableSwapThreshold, 6)} USDT`);
  }

  console.log("\n╔════════════════════════════════════════════════════════════╗");
  console.log("║              Pool Seeding Complete!                        ║");
  console.log("╚════════════════════════════════════════════════════════════╝");
  console.log("");
  console.log("Summary:");
  console.log("  Factory:      ", await factory.getAddress());
  console.log("  Total Pools:  ", (await factory.getAllPools()).length);
  console.log("  ETR Seeded:   ", ethers.formatEther(config.totalETR), "ETR");
  console.log("  EDSC Seeded:  ", ethers.formatEther(config.initialEDSC), "EDSC");
  console.log("");
  console.log("Pool Addresses:");
  const allPools = await factory.getAllPools();
  for (let i = 0; i < allPools.length; i++) {
    console.log(`  ${i + 1}. ${allPools[i]}`);
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
