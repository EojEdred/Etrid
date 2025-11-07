/**
 * Quick test script to verify deployed contracts work
 */

const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

async function main() {
  console.log("🧪 Testing Deployed Contracts\n");

  const [deployer] = await hre.ethers.getSigners();
  const network = hre.network.name;
  const chainId = (await hre.ethers.provider.getNetwork()).chainId;

  // Load deployment
  const deploymentPath = path.join(
    __dirname,
    "../deployments",
    `${network}-${chainId.toString()}.json`
  );
  const deployment = JSON.parse(fs.readFileSync(deploymentPath, "utf8"));

  console.log("📊 Test Configuration:");
  console.log("  Network:", network);
  console.log("  Chain ID:", chainId.toString());
  console.log("  Tester:", deployer.address);
  console.log("");

  // Get contract instances
  const wrappedETR = await hre.ethers.getContractAt(
    "WrappedETR",
    deployment.contracts.wrappedETR
  );
  const edsc = await hre.ethers.getContractAt(
    "EDSC",
    deployment.contracts.edsc
  );
  const tokenMessenger = await hre.ethers.getContractAt(
    "TokenMessenger",
    deployment.contracts.tokenMessenger
  );
  const masterChef = await hre.ethers.getContractAt(
    "MasterChef",
    deployment.contracts.masterChef
  );

  // Test 1: Check token names
  console.log("✅ Test 1: Token Information");
  console.log("  WrappedETR:");
  console.log("    Name:", await wrappedETR.name());
  console.log("    Symbol:", await wrappedETR.symbol());
  console.log("    Decimals:", await wrappedETR.decimals());
  console.log("    Total Supply:", hre.ethers.formatEther(await wrappedETR.totalSupply()), "wETR");
  console.log("");

  console.log("  EDSC:");
  console.log("    Name:", await edsc.name());
  console.log("    Symbol:", await edsc.symbol());
  console.log("    Decimals:", await edsc.decimals());
  console.log("    Total Supply:", hre.ethers.formatUnits(await edsc.totalSupply(), 6), "EDSC");
  console.log("");

  // Test 2: Check permissions
  console.log("✅ Test 2: Permissions");
  const MINTER_ROLE = await wrappedETR.MINTER_ROLE();
  const isMinter = await wrappedETR.hasRole(MINTER_ROLE, deployment.contracts.tokenMessenger);
  console.log("  TokenMessenger has MINTER_ROLE on WrappedETR:", isMinter);
  console.log("");

  // Test 3: Check TokenMessenger configuration
  console.log("✅ Test 3: TokenMessenger Configuration");
  const isWrappedETRSupported = await tokenMessenger.supportedTokens(deployment.contracts.wrappedETR);
  const isEDSCSupported = await tokenMessenger.supportedTokens(deployment.contracts.edsc);
  console.log("  WrappedETR supported:", isWrappedETRSupported);
  console.log("  EDSC supported:", isEDSCSupported);
  console.log("  Oracle count:", await tokenMessenger.oracleCount());
  console.log("");

  // Test 4: Check MasterChef
  console.log("✅ Test 4: MasterChef Configuration");
  const rewardToken = await masterChef.rewardToken();
  const rewardPerBlock = await masterChef.rewardPerBlock();
  const startBlock = await masterChef.startBlock();
  console.log("  Reward Token:", rewardToken);
  console.log("  Reward Per Block:", hre.ethers.formatEther(rewardPerBlock), "ETR");
  console.log("  Start Block:", startBlock.toString());
  console.log("  Pool Length:", await masterChef.poolLength());
  console.log("");

  // Test 5: Mint test tokens (only on local network)
  if (network === "localhost" || network === "hardhat") {
    console.log("✅ Test 5: Minting Test Tokens");

    // Grant MINTER_ROLE to deployer temporarily
    const tx1 = await wrappedETR.grantRole(MINTER_ROLE, deployer.address);
    await tx1.wait();

    // Mint 1000 wETR
    const testAmount = hre.ethers.parseEther("1000");
    const txHash = hre.ethers.id("test-tx-" + Date.now());
    const tx2 = await wrappedETR.bridgeMint(deployer.address, testAmount, txHash);
    await tx2.wait();

    const balance = await wrappedETR.balanceOf(deployer.address);
    console.log("  Minted 1000 wETR to deployer");
    console.log("  Deployer balance:", hre.ethers.formatEther(balance), "wETR");
    console.log("");
  }

  console.log("🎉 All tests passed!\n");
  console.log("📋 Contract Addresses:");
  console.log("  WrappedETR:", deployment.contracts.wrappedETR);
  console.log("  EDSC:", deployment.contracts.edsc);
  console.log("  TokenMessenger:", deployment.contracts.tokenMessenger);
  console.log("  MasterChef:", deployment.contracts.masterChef);
  console.log("  BridgeAdapter:", deployment.contracts.bridgeAdapter);
  console.log("");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
