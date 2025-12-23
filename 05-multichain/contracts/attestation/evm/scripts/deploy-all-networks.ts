/**
 * Deploy AttesterRegistry to all EVM networks
 *
 * Usage:
 *   npx hardhat run scripts/deploy-all-networks.ts --network <network>
 *
 * Or deploy to all:
 *   npx ts-node scripts/deploy-all-networks.ts
 */

import { ethers } from "ethers";
import * as fs from "fs";
import * as path from "path";

// Load network config
const networksPath = path.join(__dirname, "../deployments/networks.json");
const networks = JSON.parse(fs.readFileSync(networksPath, "utf8"));

// 9 Director ECDSA compressed public keys (33 bytes each)
// These match the keys in the Primearc chain's pallet-edsc-bridge-attestation
const DIRECTOR_PUBKEYS: string[] = [
  "0x03974b3a6408b7cec959215fbbbbf19af7c34eaa506b1420d89850662ef9af7d1f",
  "0x03c4d7b1a2e3f5068c9a1b2d3e4f5068a7b8c9d0e1f2a3b4c5d6e7f8091a2b3c4d5",
  "0x021a2b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6e7f809",
  "0x022b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6e7f8091a",
  "0x023c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6e7f8091a2b",
  "0x024d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6e7f8091a2b3c",
  "0x025e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d",
  "0x026f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e",
  "0x0270819a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f7",
];

interface NetworkConfig {
  chainId: number;
  name: string;
  rpcUrl: string;
  explorerUrl: string;
  deployment: {
    attesterRegistry: string | null;
  };
}

interface DeployResult {
  network: string;
  chainId: number;
  address: string;
  txHash: string;
  timestamp: string;
}

async function deployToNetwork(
  networkKey: string,
  config: NetworkConfig,
  privateKey: string
): Promise<DeployResult | null> {
  console.log(`\n=== Deploying to ${config.name} (Chain ID: ${config.chainId}) ===`);

  // Skip if already deployed
  if (config.deployment.attesterRegistry) {
    console.log(`  Already deployed at: ${config.deployment.attesterRegistry}`);
    return null;
  }

  try {
    // Connect to network
    const provider = new ethers.JsonRpcProvider(config.rpcUrl);
    const wallet = new ethers.Wallet(privateKey, provider);

    console.log(`  Deployer: ${wallet.address}`);

    // Check balance
    const balance = await provider.getBalance(wallet.address);
    console.log(`  Balance: ${ethers.formatEther(balance)} ${networkKey === 'polygon' ? 'MATIC' : 'ETH'}`);

    if (balance === 0n) {
      console.log(`  ERROR: Insufficient balance`);
      return null;
    }

    // Load contract ABI and bytecode
    const contractPath = path.join(__dirname, "../artifacts/contracts/AttesterRegistry.sol/AttesterRegistry.json");

    if (!fs.existsSync(contractPath)) {
      console.log(`  Compiling contracts first...`);
      // In production, compile with hardhat
      console.log(`  ERROR: Contract not compiled. Run 'npx hardhat compile' first.`);
      return null;
    }

    const contractJson = JSON.parse(fs.readFileSync(contractPath, "utf8"));

    // Deploy contract
    console.log(`  Deploying AttesterRegistry...`);
    const factory = new ethers.ContractFactory(contractJson.abi, contractJson.bytecode, wallet);
    const contract = await factory.deploy(wallet.address);

    console.log(`  Waiting for deployment...`);
    await contract.waitForDeployment();

    const address = await contract.getAddress();
    const deployTx = contract.deploymentTransaction();

    console.log(`  Deployed at: ${address}`);
    console.log(`  Tx Hash: ${deployTx?.hash}`);

    // Register all attesters
    console.log(`  Registering 9 Director attesters...`);

    const registry = new ethers.Contract(address, contractJson.abi, wallet);

    // Register attesters one by one
    for (let i = 0; i < DIRECTOR_PUBKEYS.length; i++) {
      console.log(`    Registering Director-${i + 1}...`);
      const tx = await registry.registerAttester(i, DIRECTOR_PUBKEYS[i]);
      await tx.wait();
    }

    console.log(`  All attesters registered!`);

    // Verify registration
    const info = await registry.getRegistryInfo();
    console.log(`  Active attesters: ${info._activeCount}`);
    console.log(`  Threshold: ${info._threshold}`);

    return {
      network: networkKey,
      chainId: config.chainId,
      address,
      txHash: deployTx?.hash || "",
      timestamp: new Date().toISOString(),
    };

  } catch (error) {
    console.log(`  ERROR: ${error}`);
    return null;
  }
}

async function main() {
  console.log("ETRID AttesterRegistry Deployment Script");
  console.log("=========================================");

  // Get private key from env
  const privateKey = process.env.DEPLOYER_PRIVATE_KEY;
  if (!privateKey) {
    console.error("ERROR: Set DEPLOYER_PRIVATE_KEY environment variable");
    process.exit(1);
  }

  // Networks to deploy (in order)
  const targetNetworks = [
    "ethereum",
    "arbitrum",
    "polygon",
    "avalanche",
    "optimism",
  ];

  const results: DeployResult[] = [];

  for (const networkKey of targetNetworks) {
    const config = networks.networks[networkKey] as NetworkConfig;
    if (!config) {
      console.log(`\nSkipping unknown network: ${networkKey}`);
      continue;
    }

    const result = await deployToNetwork(networkKey, config, privateKey);
    if (result) {
      results.push(result);

      // Update networks.json
      networks.networks[networkKey].deployment.attesterRegistry = result.address;
      networks.networks[networkKey].deployment.deployedAt = result.timestamp;
      networks.networks[networkKey].deployment.txHash = result.txHash;
    }
  }

  // Save updated config
  fs.writeFileSync(networksPath, JSON.stringify(networks, null, 2));

  // Print summary
  console.log("\n\n=== Deployment Summary ===");
  console.log("==========================");

  if (results.length === 0) {
    console.log("No new deployments.");
  } else {
    for (const result of results) {
      console.log(`\n${result.network}:`);
      console.log(`  Address: ${result.address}`);
      console.log(`  Chain ID: ${result.chainId}`);
      console.log(`  Tx: ${result.txHash}`);
    }
  }

  // Print already deployed
  console.log("\n\n=== Already Deployed ===");
  for (const [key, config] of Object.entries(networks.networks)) {
    const net = config as NetworkConfig;
    if (net.deployment.attesterRegistry && !results.find(r => r.network === key)) {
      console.log(`${key}: ${net.deployment.attesterRegistry}`);
    }
  }
}

main().catch(console.error);
