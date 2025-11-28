const hre = require("hardhat");
const fs = require("fs");

/**
 * Check deployment status and configuration
 * Run with: npx hardhat run scripts/check-deployment.js --network sepolia
 */
async function main() {
  console.log("🔎 Checking EDSC Bridge Deployment\n");
  console.log("Network:", hre.network.name);
  console.log("=" .repeat(60), "\n");

  // Load deployment
  const files = fs.readdirSync(".").filter(f => f.startsWith(`deployment-${hre.network.name}`));
  if (files.length === 0) {
    console.error("❌ No deployment file found");
    process.exit(1);
  }

  const latestFile = files.sort().reverse()[0];
  const deployment = JSON.parse(fs.readFileSync(latestFile, "utf8"));

  const edscAddress = deployment.contracts.EDSC;
  const registryAddress = deployment.contracts.AttesterRegistry;
  const transmitterAddress = deployment.contracts.MessageTransmitter;
  const messengerAddress = deployment.contracts.TokenMessenger;

  // Get contracts
  const EDSC = await hre.ethers.getContractFactory("EDSC");
  const edsc = EDSC.attach(edscAddress);

  const AttesterRegistry = await hre.ethers.getContractFactory("AttesterRegistry");
  const registry = AttesterRegistry.attach(registryAddress);

  const TokenMessenger = await hre.ethers.getContractFactory("EDSCTokenMessenger");
  const messenger = TokenMessenger.attach(messengerAddress);

  // Check EDSC Token
  console.log("📄 EDSC Token");
  console.log("  Address:", edscAddress);

  const name = await edsc.name();
  const symbol = await edsc.symbol();
  const totalSupply = await edsc.totalSupply();
  const owner = await edsc.owner();
  const messageTransmitter = await edsc.messageTransmitter();
  const paused = await edsc.paused();

  console.log("  Name:", name);
  console.log("  Symbol:", symbol);
  console.log("  Total Supply:", hre.ethers.formatUnits(totalSupply, 18), "EDSC");
  console.log("  Owner:", owner);
  console.log("  Message Transmitter:", messageTransmitter);
  console.log("  Paused:", paused ? "❌ YES" : "✅ NO");

  if (messageTransmitter === hre.ethers.ZeroAddress) {
    console.log("  ⚠️  WARNING: MessageTransmitter not set!");
  }
  console.log("");

  // Check AttesterRegistry
  console.log("🔐 Attester Registry");
  console.log("  Address:", registryAddress);

  const minSignatures = await registry.minSignatures();
  const totalAttesters = await registry.totalAttesters();
  const attesterCount = await registry.attesterCount();

  console.log("  Min Signatures Required:", minSignatures.toString());
  console.log("  Total Attesters Configured:", totalAttesters.toString());
  console.log("  Attesters Registered:", attesterCount.toString());

  if (attesterCount < minSignatures) {
    console.log("  ❌ WARNING: Not enough attesters registered!");
  } else {
    console.log("  ✅ Threshold met");
  }

  console.log("\n  Registered Attesters:");
  for (let i = 0; i < attesterCount; i++) {
    const attesterAddr = await registry.attesters(i);
    const attester = await registry.attesterDetails(attesterAddr);
    console.log(`    [${i}] ${attesterAddr} - ${attester.isActive ? "Active" : "Inactive"}`);
  }
  console.log("");

  // Check TokenMessenger
  console.log("💸 Token Messenger");
  console.log("  Address:", messengerAddress);

  const localDomain = await messenger.LOCAL_DOMAIN();
  const etridDomain = await messenger.ETRID_DOMAIN();
  const maxBurnAmount = await messenger.maxBurnAmount();
  const dailyBurnLimit = await messenger.dailyBurnLimit();
  const nonce = await messenger.nonce();
  const totalMessagesSent = await messenger.totalMessagesSent();
  const totalEDSCBurned = await messenger.totalEDSCBurned();
  const messengerPaused = await messenger.paused();

  console.log("  Local Domain (Ethereum):", localDomain.toString());
  console.log("  Ëtrid Domain:", etridDomain.toString());
  console.log("  Max Burn Per TX:", hre.ethers.formatUnits(maxBurnAmount, 18), "EDSC");
  console.log("  Daily Burn Limit:", hre.ethers.formatUnits(dailyBurnLimit, 18), "EDSC");
  console.log("  Current Nonce:", nonce.toString());
  console.log("  Total Messages Sent:", totalMessagesSent.toString());
  console.log("  Total EDSC Burned:", hre.ethers.formatUnits(totalEDSCBurned, 18), "EDSC");
  console.log("  Paused:", messengerPaused ? "❌ YES" : "✅ NO");
  console.log("");

  // Network Status
  console.log("🌐 Network Status");
  const [deployer] = await hre.ethers.getSigners();
  const deployerBalance = await hre.ethers.provider.getBalance(deployer.address);
  const blockNumber = await hre.ethers.provider.getBlockNumber();

  console.log("  Deployer:", deployer.address);
  console.log("  Deployer Balance:", hre.ethers.formatEther(deployerBalance), "ETH");
  console.log("  Current Block:", blockNumber);
  console.log("");

  // Configuration Summary
  console.log("=" .repeat(60));
  console.log("📊 Configuration Summary");
  console.log("=" .repeat(60));

  const checks = {
    "Contracts Deployed": true,
    "EDSC Not Paused": !paused,
    "MessageTransmitter Set": messageTransmitter !== hre.ethers.ZeroAddress,
    "MessageTransmitter Authorized": messageTransmitter === messengerAddress,
    "Enough Attesters": attesterCount >= minSignatures,
    "TokenMessenger Not Paused": !messengerPaused,
    "Deployer Has ETH": deployerBalance > 0n,
  };

  let allGood = true;
  for (const [check, status] of Object.entries(checks)) {
    console.log(`  ${status ? "✅" : "❌"} ${check}`);
    if (!status) allGood = false;
  }

  console.log("");
  if (allGood) {
    console.log("🎉 All checks passed! Bridge is ready for operation.");
  } else {
    console.log("⚠️  Some checks failed. Please review configuration.");
  }

  console.log("\nEtherscan Links:");
  console.log(`  https://sepolia.etherscan.io/address/${edscAddress}`);
  console.log(`  https://sepolia.etherscan.io/address/${messengerAddress}`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
