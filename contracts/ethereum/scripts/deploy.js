const hre = require("hardhat");

async function main() {
  console.log("Starting EDSC Ethereum contracts deployment...\n");

  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying contracts with account:", deployer.address);
  console.log("Account balance:", (await hre.ethers.provider.getBalance(deployer.address)).toString(), "\n");

  // Configuration
  const INITIAL_OWNER = process.env.INITIAL_OWNER || deployer.address;
  const MIN_SIGNATURES = parseInt(process.env.INITIAL_MIN_SIGNATURES || "3");
  const TOTAL_ATTESTERS = parseInt(process.env.INITIAL_TOTAL_ATTESTERS || "5");

  console.log("Configuration:");
  console.log("  Owner:", INITIAL_OWNER);
  console.log("  Min Signatures:", MIN_SIGNATURES);
  console.log("  Total Attesters:", TOTAL_ATTESTERS, "\n");

  // 1. Deploy EDSC Token
  console.log("1. Deploying EDSC token...");
  const EDSC = await hre.ethers.getContractFactory("EDSC");
  const edsc = await EDSC.deploy(INITIAL_OWNER);
  await edsc.waitForDeployment();
  const edscAddress = await edsc.getAddress();
  console.log("   EDSC deployed to:", edscAddress, "\n");

  // 2. Deploy AttesterRegistry
  console.log("2. Deploying AttesterRegistry...");
  const AttesterRegistry = await hre.ethers.getContractFactory("AttesterRegistry");
  const attesterRegistry = await AttesterRegistry.deploy(
    INITIAL_OWNER,
    MIN_SIGNATURES,
    TOTAL_ATTESTERS
  );
  await attesterRegistry.waitForDeployment();
  const attesterRegistryAddress = await attesterRegistry.getAddress();
  console.log("   AttesterRegistry deployed to:", attesterRegistryAddress, "\n");

  // 3. Deploy EDSCMessageTransmitter
  console.log("3. Deploying EDSCMessageTransmitter...");
  const EDSCMessageTransmitter = await hre.ethers.getContractFactory("EDSCMessageTransmitter");
  const messageTransmitter = await EDSCMessageTransmitter.deploy(
    INITIAL_OWNER,
    edscAddress,
    attesterRegistryAddress
  );
  await messageTransmitter.waitForDeployment();
  const messageTransmitterAddress = await messageTransmitter.getAddress();
  console.log("   EDSCMessageTransmitter deployed to:", messageTransmitterAddress, "\n");

  // 4. Deploy EDSCTokenMessenger
  console.log("4. Deploying EDSCTokenMessenger...");
  const EDSCTokenMessenger = await hre.ethers.getContractFactory("EDSCTokenMessenger");
  const tokenMessenger = await EDSCTokenMessenger.deploy(
    INITIAL_OWNER,
    edscAddress
  );
  await tokenMessenger.waitForDeployment();
  const tokenMessengerAddress = await tokenMessenger.getAddress();
  console.log("   EDSCTokenMessenger deployed to:", tokenMessengerAddress, "\n");

  // 5. Configure EDSC token
  console.log("5. Configuring EDSC token...");
  if (INITIAL_OWNER === deployer.address) {
    const setTx = await edsc.setMessageTransmitter(messageTransmitterAddress);
    await setTx.wait();
    console.log("   MessageTransmitter authorized to mint/burn\n");
  } else {
    console.log("   ⚠️  Owner is different - manually set MessageTransmitter\n");
  }

  // Print summary
  console.log("=".repeat(60));
  console.log("Deployment Summary");
  console.log("=".repeat(60));
  console.log("EDSC Token:             ", edscAddress);
  console.log("AttesterRegistry:       ", attesterRegistryAddress);
  console.log("MessageTransmitter:     ", messageTransmitterAddress);
  console.log("TokenMessenger:         ", tokenMessengerAddress);
  console.log("=".repeat(60), "\n");

  console.log("Next steps:");
  console.log("1. Register attesters via AttesterRegistry.registerAttester()");
  console.log("2. Configure thresholds if needed");
  console.log("3. Verify contracts on Etherscan");
  console.log("4. Set up off-chain attestation service");
  console.log("5. Set up relayer service\n");

  // Save addresses to file
  const fs = require("fs");
  const addresses = {
    network: hre.network.name,
    timestamp: new Date().toISOString(),
    contracts: {
      EDSC: edscAddress,
      AttesterRegistry: attesterRegistryAddress,
      MessageTransmitter: messageTransmitterAddress,
      TokenMessenger: tokenMessengerAddress,
    },
    config: {
      owner: INITIAL_OWNER,
      minSignatures: MIN_SIGNATURES,
      totalAttesters: TOTAL_ATTESTERS,
    },
  };

  const filename = `deployment-${hre.network.name}-${Date.now()}.json`;
  fs.writeFileSync(filename, JSON.stringify(addresses, null, 2));
  console.log("Deployment info saved to:", filename);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
