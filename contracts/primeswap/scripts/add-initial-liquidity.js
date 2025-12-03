// Add Initial Liquidity Script for PrimeSwap
// Usage: npx hardhat run scripts/add-initial-liquidity.js --network <network>
//
// This script adds initial liquidity to PrimeSwap pools
// Customize the amounts below based on your budget

const hre = require("hardhat");

async function main() {
  console.log("💧 Adding Initial Liquidity to PrimeSwap Pools");
  console.log("=".repeat(60));

  const [deployer] = await hre.ethers.getSigners();
  console.log("Using account:", deployer.address);
  console.log("Account balance:", hre.ethers.utils.formatEther(await deployer.getBalance()), "ETH");
  console.log("");

  // ==================== CONFIGURATION ====================
  // Load deployment addresses from deployments-full.json
  const fs = require("fs");
  let config;

  try {
    config = JSON.parse(fs.readFileSync("./deployments-full.json", "utf8"));
    console.log("📄 Loaded deployment config from deployments-full.json");
  } catch (error) {
    console.error("❌ Error: deployments-full.json not found. Run deploy-full.js first!");
    process.exit(1);
  }

  const ROUTER_ADDRESS = config.contracts.router;
  const ETR_TOKEN_ADDRESS = config.contracts.etrToken;
  const USDC_TOKEN_ADDRESS = config.contracts.testTokens.usdc;
  const WETH_ADDRESS = config.contracts.weth;

  console.log("Contract Addresses:");
  console.log("  Router:", ROUTER_ADDRESS);
  console.log("  ETR Token:", ETR_TOKEN_ADDRESS);
  console.log("  USDC Token:", USDC_TOKEN_ADDRESS);
  console.log("  WETH:", WETH_ADDRESS);
  console.log("");

  // ==================== LIQUIDITY AMOUNTS ====================
  // CUSTOMIZE THESE BASED ON YOUR BUDGET
  // For $750 budget, we'll split across two main pools

  // Pool 1: ETR/USDC (Main trading pair) - $400 worth
  const ETR_FOR_USDC_POOL = hre.ethers.utils.parseEther("100000");  // 100k ETR
  const USDC_FOR_ETR_POOL = hre.ethers.utils.parseEther("400");     // $400 USDC

  // Pool 2: ETR/WETH (ETH liquidity) - $350 worth
  const ETR_FOR_WETH_POOL = hre.ethers.utils.parseEther("87500");   // 87.5k ETR
  const WETH_FOR_POOL = hre.ethers.utils.parseEther("0.1");         // 0.1 ETH (~$350)

  // This implies: 1 ETR = $0.004 or 250,000 ETR = $1000

  console.log("💰 Liquidity Amounts (Total Budget: ~$750):");
  console.log("-".repeat(60));
  console.log("Pool 1 - ETR/USDC:");
  console.log("  ETR:  ", hre.ethers.utils.formatEther(ETR_FOR_USDC_POOL), "ETR");
  console.log("  USDC: ", hre.ethers.utils.formatEther(USDC_FOR_ETR_POOL), "USDC (~$400)");
  console.log("");
  console.log("Pool 2 - ETR/WETH:");
  console.log("  ETR:  ", hre.ethers.utils.formatEther(ETR_FOR_WETH_POOL), "ETR");
  console.log("  WETH: ", hre.ethers.utils.formatEther(WETH_FOR_POOL), "ETH (~$350)");
  console.log("");

  // ==================== GET CONTRACTS ====================
  const router = await hre.ethers.getContractAt("PrimeSwapRouter", ROUTER_ADDRESS);
  const etrToken = await hre.ethers.getContractAt("MockERC20", ETR_TOKEN_ADDRESS);
  const usdc = await hre.ethers.getContractAt("MockERC20", USDC_TOKEN_ADDRESS);
  const weth = await hre.ethers.getContractAt("WETH", WETH_ADDRESS);

  const deadline = Math.floor(Date.now() / 1000) + 60 * 20; // 20 minutes from now

  // ==================== POOL 1: ETR/USDC ====================
  console.log("📊 Adding liquidity to ETR/USDC pool...");
  console.log("-".repeat(60));

  // Approve tokens
  console.log("Approving ETR...");
  let tx = await etrToken.approve(router.address, ETR_FOR_USDC_POOL);
  await tx.wait();
  console.log("✅ ETR approved");

  console.log("Approving USDC...");
  tx = await usdc.approve(router.address, USDC_FOR_ETR_POOL);
  await tx.wait();
  console.log("✅ USDC approved");

  // Add liquidity
  console.log("Adding liquidity...");
  tx = await router.addLiquidity(
    etrToken.address,
    usdc.address,
    ETR_FOR_USDC_POOL,
    USDC_FOR_ETR_POOL,
    0, // accept any amount of ETR (first liquidity)
    0, // accept any amount of USDC
    deployer.address,
    deadline
  );
  const receipt1 = await tx.wait();
  console.log("✅ Liquidity added to ETR/USDC pool!");
  console.log("   Transaction:", receipt1.transactionHash);
  console.log("");

  // ==================== POOL 2: ETR/WETH ====================
  console.log("📊 Adding liquidity to ETR/WETH pool...");
  console.log("-".repeat(60));

  // Approve ETR
  console.log("Approving ETR...");
  tx = await etrToken.approve(router.address, ETR_FOR_WETH_POOL);
  await tx.wait();
  console.log("✅ ETR approved");

  // Wrap ETH to WETH
  console.log("Wrapping ETH to WETH...");
  tx = await weth.deposit({ value: WETH_FOR_POOL });
  await tx.wait();
  console.log("✅ ETH wrapped to WETH");

  // Approve WETH
  console.log("Approving WETH...");
  tx = await weth.approve(router.address, WETH_FOR_POOL);
  await tx.wait();
  console.log("✅ WETH approved");

  // Add liquidity
  console.log("Adding liquidity...");
  tx = await router.addLiquidity(
    etrToken.address,
    weth.address,
    ETR_FOR_WETH_POOL,
    WETH_FOR_POOL,
    0, // accept any amount of ETR
    0, // accept any amount of WETH
    deployer.address,
    deadline
  );
  const receipt2 = await tx.wait();
  console.log("✅ Liquidity added to ETR/WETH pool!");
  console.log("   Transaction:", receipt2.transactionHash);
  console.log("");

  // ==================== VERIFY POOLS ====================
  console.log("🔍 Verifying pool reserves...");
  console.log("-".repeat(60));

  const factory = await hre.ethers.getContractAt(
    "PrimeSwapFactory",
    config.contracts.factory
  );

  // Get ETR/USDC pair
  const etrUsdcPairAddress = await factory.getPair(etrToken.address, usdc.address);
  const etrUsdcPair = await hre.ethers.getContractAt("PrimeSwapPair", etrUsdcPairAddress);
  const reserves1 = await etrUsdcPair.getReserves();
  console.log("ETR/USDC Pool:");
  console.log("  Reserve0:", hre.ethers.utils.formatEther(reserves1[0]));
  console.log("  Reserve1:", hre.ethers.utils.formatEther(reserves1[1]));
  console.log("  Pair Address:", etrUsdcPairAddress);
  console.log("");

  // Get ETR/WETH pair
  const etrWethPairAddress = await factory.getPair(etrToken.address, weth.address);
  const etrWethPair = await hre.ethers.getContractAt("PrimeSwapPair", etrWethPairAddress);
  const reserves2 = await etrWethPair.getReserves();
  console.log("ETR/WETH Pool:");
  console.log("  Reserve0:", hre.ethers.utils.formatEther(reserves2[0]));
  console.log("  Reserve1:", hre.ethers.utils.formatEther(reserves2[1]));
  console.log("  Pair Address:", etrWethPairAddress);
  console.log("");

  // ==================== SUMMARY ====================
  console.log("=".repeat(60));
  console.log("🎉 Initial Liquidity Successfully Added!");
  console.log("=".repeat(60));
  console.log("");
  console.log("📊 Summary:");
  console.log("-".repeat(60));
  console.log("Total Budget Used: ~$750");
  console.log("");
  console.log("Pool 1 - ETR/USDC:");
  console.log("  Liquidity: $400");
  console.log("  Pair Address:", etrUsdcPairAddress);
  console.log("");
  console.log("Pool 2 - ETR/WETH:");
  console.log("  Liquidity: $350");
  console.log("  Pair Address:", etrWethPairAddress);
  console.log("");
  console.log("🔗 Next Steps:");
  console.log("-".repeat(60));
  console.log("1. Test swaps to ensure pools work correctly");
  console.log("2. Monitor pool reserves and trading activity");
  console.log("3. Add more liquidity as trading volume increases");
  console.log("4. Set up staking pools in MasterChef");
  console.log("5. Configure frontend to display these pools");
  console.log("");
  console.log("✅ Your DEX is now live with initial liquidity!");
  console.log("=".repeat(60));
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
