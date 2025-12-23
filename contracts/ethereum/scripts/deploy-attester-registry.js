/**
 * Deploy AttesterRegistry Contract and Register 9 Director Attesters
 *
 * Usage:
 *   npx hardhat run scripts/deploy-attester-registry.js --network base
 *   npx hardhat run scripts/deploy-attester-registry.js --network bsc
 */

const { ethers } = require("hardhat");

// 9 Director Attester Addresses (ECDSA/EVM addresses from attester-keys-9directors.json)
const ATTESTERS = [
  { id: 1, name: "Director-1", address: "0xA12d48dB2589cfe7ff11a595b80013CffFf5eE3d" },
  { id: 2, name: "Director-2", address: "0x18a6034995CC0c6Db7fC9Ee53E535f5b1984f83e" },
  { id: 3, name: "Director-3", address: "0x574B03172d7e637e2aA645eA9789Fe1E36DdBE33" },
  { id: 4, name: "Director-4", address: "0x698AEdAd3550e716eDA5C923950caC3aA163883F" },
  { id: 5, name: "Director-5", address: "0xa27f49Bf5a5daa961fECF86526bDa0FD315bE988" },
  { id: 6, name: "Director-6", address: "0x6250F01Ca6fcCeB81a1c7E5c2f8A114511188934" },
  { id: 7, name: "Director-7", address: "0x56824F247Bbb54c353025306E860E8edA8877c7b" },
  { id: 8, name: "Director-8", address: "0x64810209643c663D0505806e66Fe5dc0C5cEdB37" },
  { id: 9, name: "Director-9", address: "0x3D9f108A558f9DDDc3c0881d6eafF7292d64dF92" },
];

// Configuration
const CONFIG = {
  threshold: 5,        // 5-of-9
  totalAttesters: 9,   // Total attesters
};

async function main() {
  const [deployer] = await ethers.getSigners();
  const network = await ethers.provider.getNetwork();

  console.log("═══════════════════════════════════════════════════════════");
  console.log("   ËTRID AttesterRegistry Deployment");
  console.log("═══════════════════════════════════════════════════════════");
  console.log(`Network: ${network.name} (chainId: ${network.chainId})`);
  console.log(`Deployer: ${deployer.address}`);

  const balance = await ethers.provider.getBalance(deployer.address);
  console.log(`Balance: ${ethers.formatEther(balance)} ETH/BNB\n`);

  // Check if we have enough balance
  if (balance < ethers.parseEther("0.001")) {
    console.log("WARNING: Very low balance. Deployment may fail.");
  }

  // Deploy AttesterRegistry
  console.log("Deploying AttesterRegistry...");
  const AttesterRegistry = await ethers.getContractFactory("AttesterRegistry");
  const registry = await AttesterRegistry.deploy(
    deployer.address,  // owner
    CONFIG.threshold,  // minSignatures (5)
    CONFIG.totalAttesters  // totalAttesters (9)
  );

  await registry.waitForDeployment();
  const registryAddress = await registry.getAddress();

  console.log(`✓ AttesterRegistry deployed: ${registryAddress}\n`);

  // Register attesters
  console.log("Registering attesters...");

  for (const attester of ATTESTERS) {
    process.stdout.write(`  ${attester.name} (${attester.address.slice(0, 10)}...): `);

    try {
      const tx = await registry.registerAttester(attester.address);
      await tx.wait();
      console.log("✓ Registered");
    } catch (error) {
      if (error.message.includes("AttesterAlreadyRegistered")) {
        console.log("Already registered");
      } else {
        console.log(`✗ Failed: ${error.message.slice(0, 50)}`);
      }
    }
  }

  // Verify registration
  console.log("\nVerifying registrations...");
  const count = await registry.getAttesterCount();
  const enabledCount = await registry.enabledAttesterCount();
  const [minSig, totalAtt] = await registry.getThreshold(0);

  console.log(`  Total attesters: ${count}`);
  console.log(`  Enabled attesters: ${enabledCount}`);
  console.log(`  Threshold: ${minSig}-of-${totalAtt}`);

  // Summary
  console.log("\n═══════════════════════════════════════════════════════════");
  console.log("   Deployment Summary");
  console.log("═══════════════════════════════════════════════════════════");
  console.log(`Network: ${network.name} (${network.chainId})`);
  console.log(`AttesterRegistry: ${registryAddress}`);
  console.log(`Owner: ${deployer.address}`);
  console.log(`Threshold: ${CONFIG.threshold}-of-${CONFIG.totalAttesters}`);
  console.log(`Registered: ${count} attesters`);

  // Save deployment info
  const fs = require("fs");
  const deploymentInfo = {
    network: network.name,
    chainId: Number(network.chainId),
    attesterRegistry: registryAddress,
    owner: deployer.address,
    threshold: {
      minSignatures: CONFIG.threshold,
      totalAttesters: CONFIG.totalAttesters,
    },
    attesters: ATTESTERS,
    deployedAt: new Date().toISOString(),
  };

  const filename = `deployment-attester-registry-${network.chainId}-${Date.now()}.json`;
  fs.writeFileSync(filename, JSON.stringify(deploymentInfo, null, 2));
  console.log(`\nDeployment info saved to: ${filename}`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("Deployment failed:", error);
    process.exit(1);
  });
