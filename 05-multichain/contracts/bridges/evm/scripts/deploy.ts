import { ethers, network } from "hardhat";
import * as fs from "fs";
import * as path from "path";

interface NetworkConfig {
  name: string;
  chainId: number;
  domainId: number;
  attesterRegistry: string;
  feeRecipient: string;
}

const NETWORKS: Record<string, NetworkConfig> = {
  ethereum: {
    name: "Ethereum Mainnet",
    chainId: 1,
    domainId: 100,
    attesterRegistry: "", // To be filled after attestation deployment
    feeRecipient: "", // To be filled
  },
  arbitrum: {
    name: "Arbitrum One",
    chainId: 42161,
    domainId: 101,
    attesterRegistry: "",
    feeRecipient: "",
  },
  polygon: {
    name: "Polygon",
    chainId: 137,
    domainId: 102,
    attesterRegistry: "",
    feeRecipient: "",
  },
  avalanche: {
    name: "Avalanche C-Chain",
    chainId: 43114,
    domainId: 103,
    attesterRegistry: "",
    feeRecipient: "",
  },
  base: {
    name: "Base",
    chainId: 8453,
    domainId: 104,
    attesterRegistry: "0x...", // Deployed
    feeRecipient: "",
  },
  bsc: {
    name: "BNB Chain",
    chainId: 56,
    domainId: 105,
    attesterRegistry: "0x...", // Deployed
    feeRecipient: "",
  },
  optimism: {
    name: "Optimism",
    chainId: 10,
    domainId: 106,
    attesterRegistry: "",
    feeRecipient: "",
  },
};

async function main() {
  const networkName = network.name;
  const config = NETWORKS[networkName];

  if (!config) {
    throw new Error(`Unknown network: ${networkName}`);
  }

  console.log(`\n========================================`);
  console.log(`Deploying ETRID Bridge to ${config.name}`);
  console.log(`Chain ID: ${config.chainId}`);
  console.log(`Domain ID: ${config.domainId}`);
  console.log(`========================================\n`);

  const [deployer] = await ethers.getSigners();
  console.log("Deploying with account:", deployer.address);
  console.log("Account balance:", ethers.formatEther(await ethers.provider.getBalance(deployer.address)));

  // Get fee recipient (default to deployer)
  const feeRecipient = config.feeRecipient || deployer.address;

  // Get attester registry (must be deployed first)
  if (!config.attesterRegistry) {
    console.log("\nWARNING: AttesterRegistry not set for this network.");
    console.log("Please deploy AttesterRegistry first and update config.\n");
    // For now, we'll deploy with a placeholder and update later
  }

  // Deploy EtridBridge
  console.log("\n1. Deploying EtridBridge...");
  const EtridBridge = await ethers.getContractFactory("EtridBridge");
  const bridge = await EtridBridge.deploy(
    config.domainId,
    config.attesterRegistry || deployer.address, // Placeholder if not set
    feeRecipient
  );
  await bridge.waitForDeployment();
  const bridgeAddress = await bridge.getAddress();
  console.log(`   EtridBridge deployed to: ${bridgeAddress}`);

  // Deploy WrappedTokenFactory
  console.log("\n2. Deploying WrappedTokenFactory...");
  const WrappedTokenFactory = await ethers.getContractFactory("WrappedTokenFactory");
  const factory = await WrappedTokenFactory.deploy(bridgeAddress);
  await factory.waitForDeployment();
  const factoryAddress = await factory.getAddress();
  console.log(`   WrappedTokenFactory deployed to: ${factoryAddress}`);

  // Save deployment info
  const deployment = {
    network: networkName,
    chainId: config.chainId,
    domainId: config.domainId,
    deployer: deployer.address,
    timestamp: new Date().toISOString(),
    contracts: {
      EtridBridge: bridgeAddress,
      WrappedTokenFactory: factoryAddress,
      AttesterRegistry: config.attesterRegistry || "NOT_SET",
    },
    feeRecipient: feeRecipient,
  };

  // Write deployment file
  const deploymentsDir = path.join(__dirname, "../deployments");
  if (!fs.existsSync(deploymentsDir)) {
    fs.mkdirSync(deploymentsDir, { recursive: true });
  }

  const deploymentFile = path.join(deploymentsDir, `${networkName}.json`);
  fs.writeFileSync(deploymentFile, JSON.stringify(deployment, null, 2));
  console.log(`\nDeployment saved to: ${deploymentFile}`);

  console.log("\n========================================");
  console.log("DEPLOYMENT SUMMARY");
  console.log("========================================");
  console.log(`Network:              ${config.name}`);
  console.log(`EtridBridge:          ${bridgeAddress}`);
  console.log(`WrappedTokenFactory:  ${factoryAddress}`);
  console.log(`AttesterRegistry:     ${config.attesterRegistry || "NOT_SET"}`);
  console.log(`Fee Recipient:        ${feeRecipient}`);
  console.log("========================================\n");

  // Verification commands
  console.log("To verify contracts on explorer:");
  console.log(`npx hardhat verify --network ${networkName} ${bridgeAddress} ${config.domainId} "${config.attesterRegistry || deployer.address}" "${feeRecipient}"`);
  console.log(`npx hardhat verify --network ${networkName} ${factoryAddress} "${bridgeAddress}"`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
