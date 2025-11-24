// PrimeSwap Full Deployment Script (DEX + Staking)
// Usage: npx hardhat run scripts/deploy-full.js --network <network>

const hre = require("hardhat");

async function main() {
  console.log("🚀 PrimeSwap Full Stack Deployment Started");
  console.log("=".repeat(60));

  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying contracts with account:", deployer.address);
  console.log("Account balance:", hre.ethers.utils.formatEther(await deployer.getBalance()), "ETH");
  console.log("");

  // ==================== PART 1: DEX DEPLOYMENT ====================
  console.log("📦 PART 1: DEX DEPLOYMENT");
  console.log("-".repeat(60));

  // Step 1: Deploy WETH
  console.log("📦 Deploying WETH...");
  const WETH = await hre.ethers.getContractFactory("WETH");
  const weth = await WETH.deploy();
  await weth.deployed();
  console.log("✅ WETH deployed to:", weth.address);
  console.log("");

  // Step 2: Deploy Factory
  console.log("📦 Deploying PrimeSwapFactory...");
  const Factory = await hre.ethers.getContractFactory("PrimeSwapFactory");
  const factory = await Factory.deploy(deployer.address);
  await factory.deployed();
  console.log("✅ PrimeSwapFactory deployed to:", factory.address);
  console.log("   Fee To Setter:", deployer.address);
  console.log("");

  // Step 3: Deploy Router
  console.log("📦 Deploying PrimeSwapRouter...");
  const Router = await hre.ethers.getContractFactory("PrimeSwapRouter");
  const router = await Router.deploy(factory.address, weth.address);
  await router.deployed();
  console.log("✅ PrimeSwapRouter deployed to:", router.address);
  console.log("");

  // ==================== PART 2: STAKING DEPLOYMENT ====================
  console.log("📦 PART 2: STAKING DEPLOYMENT");
  console.log("-".repeat(60));

  // Step 4: Deploy Reward Token (ETR)
  console.log("📦 Deploying ETR Reward Token...");
  const Token = await hre.ethers.getContractFactory("MockERC20");
  const etrToken = await Token.deploy(
    "Etrid Token",
    "ETR",
    hre.ethers.utils.parseEther("100000000") // 100M total supply
  );
  await etrToken.deployed();
  console.log("✅ ETR Token deployed to:", etrToken.address);
  console.log("   Total Supply:", hre.ethers.utils.formatEther(await etrToken.totalSupply()), "ETR");
  console.log("");

  // Step 5: Deploy MasterChef
  console.log("📦 Deploying MasterChef...");
  const currentBlock = await hre.ethers.provider.getBlockNumber();
  const rewardPerBlock = hre.ethers.utils.parseEther("1"); // 1 ETR per block
  const startBlock = currentBlock + 100; // Start in 100 blocks

  const MasterChef = await hre.ethers.getContractFactory("MasterChef");
  const masterChef = await MasterChef.deploy(
    etrToken.address,
    rewardPerBlock,
    startBlock
  );
  await masterChef.deployed();
  console.log("✅ MasterChef deployed to:", masterChef.address);
  console.log("   Reward Token:", etrToken.address);
  console.log("   Reward Per Block:", hre.ethers.utils.formatEther(rewardPerBlock), "ETR");
  console.log("   Start Block:", startBlock);
  console.log("");

  // Step 6: Transfer reward tokens to MasterChef
  console.log("📦 Funding MasterChef with rewards...");
  const rewardAmount = hre.ethers.utils.parseEther("10000000"); // 10M ETR for rewards
  await etrToken.transfer(masterChef.address, rewardAmount);
  console.log("✅ Transferred", hre.ethers.utils.formatEther(rewardAmount), "ETR to MasterChef");
  console.log("");

  // ==================== PART 3: INITIAL POOL SETUP ====================
  console.log("📦 PART 3: INITIAL POOL SETUP");
  console.log("-".repeat(60));

  // Deploy some test tokens for pools
  console.log("📦 Deploying test tokens...");
  const usdc = await Token.deploy("USD Coin", "USDC", hre.ethers.utils.parseEther("1000000"));
  await usdc.deployed();
  console.log("✅ USDC deployed to:", usdc.address);

  const dai = await Token.deploy("DAI Stablecoin", "DAI", hre.ethers.utils.parseEther("1000000"));
  await dai.deployed();
  console.log("✅ DAI deployed to:", dai.address);
  console.log("");

  // Create initial pairs
  console.log("📦 Creating initial trading pairs...");

  // Create ETR/USDC pair
  console.log("Creating ETR/USDC pair...");
  let tx = await factory.createPair(etrToken.address, usdc.address);
  await tx.wait();
  const etrUsdcPair = await factory.getPair(etrToken.address, usdc.address);
  console.log("✅ ETR/USDC pair created at:", etrUsdcPair);

  // Create ETR/WETH pair
  console.log("Creating ETR/WETH pair...");
  tx = await factory.createPair(etrToken.address, weth.address);
  await tx.wait();
  const etrWethPair = await factory.getPair(etrToken.address, weth.address);
  console.log("✅ ETR/WETH pair created at:", etrWethPair);

  // Create USDC/DAI pair
  console.log("Creating USDC/DAI pair...");
  tx = await factory.createPair(usdc.address, dai.address);
  await tx.wait();
  const usdcDaiPair = await factory.getPair(usdc.address, dai.address);
  console.log("✅ USDC/DAI pair created at:", usdcDaiPair);
  console.log("");

  // Add staking pools to MasterChef
  console.log("📦 Adding staking pools to MasterChef...");

  // Pool 0: ETR/USDC (highest rewards)
  await masterChef.add(1000, etrUsdcPair, false);
  console.log("✅ Pool 0: ETR/USDC LP (1000 allocation points)");

  // Pool 1: ETR/WETH
  await masterChef.add(800, etrWethPair, false);
  console.log("✅ Pool 1: ETR/WETH LP (800 allocation points)");

  // Pool 2: USDC/DAI
  await masterChef.add(500, usdcDaiPair, false);
  console.log("✅ Pool 2: USDC/DAI LP (500 allocation points)");
  console.log("");

  // ==================== SUMMARY ====================
  console.log("=".repeat(60));
  console.log("🎉 PrimeSwap Full Stack Deployment Complete!");
  console.log("=".repeat(60));
  console.log("");
  console.log("📋 Contract Addresses:");
  console.log("-".repeat(60));
  console.log("Core DEX:");
  console.log("  WETH:                 ", weth.address);
  console.log("  PrimeSwapFactory:     ", factory.address);
  console.log("  PrimeSwapRouter:      ", router.address);
  console.log("");
  console.log("Tokens:");
  console.log("  ETR (Reward):         ", etrToken.address);
  console.log("  USDC (Test):          ", usdc.address);
  console.log("  DAI (Test):           ", dai.address);
  console.log("");
  console.log("Staking:");
  console.log("  MasterChef:           ", masterChef.address);
  console.log("");
  console.log("Trading Pairs:");
  console.log("  ETR/USDC:             ", etrUsdcPair);
  console.log("  ETR/WETH:             ", etrWethPair);
  console.log("  USDC/DAI:             ", usdcDaiPair);
  console.log("");
  console.log("Staking Pools:");
  console.log("  Pool 0: ETR/USDC LP   (1000 points)");
  console.log("  Pool 1: ETR/WETH LP   (800 points)");
  console.log("  Pool 2: USDC/DAI LP   (500 points)");
  console.log("");
  console.log("Configuration:");
  console.log("  Total Alloc Points:   ", (await masterChef.totalAllocPoint()).toString());
  console.log("  Reward Per Block:     ", hre.ethers.utils.formatEther(rewardPerBlock), "ETR");
  console.log("  Start Block:          ", startBlock);
  console.log("");

  // Generate config file
  const config = {
    network: hre.network.name,
    timestamp: new Date().toISOString(),
    deployer: deployer.address,
    contracts: {
      weth: weth.address,
      factory: factory.address,
      router: router.address,
      etrToken: etrToken.address,
      masterChef: masterChef.address,
      testTokens: {
        usdc: usdc.address,
        dai: dai.address
      }
    },
    pairs: {
      etrUsdc: etrUsdcPair,
      etrWeth: etrWethPair,
      usdcDai: usdcDaiPair
    },
    stakingPools: [
      { id: 0, pair: "ETR/USDC", lpToken: etrUsdcPair, allocPoints: 1000 },
      { id: 1, pair: "ETR/WETH", lpToken: etrWethPair, allocPoints: 800 },
      { id: 2, pair: "USDC/DAI", lpToken: usdcDaiPair, allocPoints: 500 }
    ],
    parameters: {
      rewardPerBlock: rewardPerBlock.toString(),
      startBlock: startBlock,
      totalAllocPoints: 2300
    }
  };

  const fs = require("fs");
  fs.writeFileSync(
    "./deployments-full.json",
    JSON.stringify(config, null, 2)
  );
  console.log("📄 Full deployment config saved to deployments-full.json");
  console.log("");
  console.log("✅ All done! Your PrimeSwap DEX with staking is ready!");
  console.log("");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
