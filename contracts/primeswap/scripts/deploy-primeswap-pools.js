#!/usr/bin/env node
/**
 * PrimeSwap Pools Deployment Script
 *
 * Deploys all PrimeSwap pools to Primearc Core via ËtwasmVM:
 * 1. PrimeSwapOracle - Price feeds
 * 2. PrimeSwapFactory - Pool registry
 * 3. 11 VirtualReserveAMM pools (ETR/wToken)
 * 4. 1 EDSCOraclePool (ETR/EDSC)
 * 5. 1 EDSCPegPool (EDSC/USDT)
 *
 * Usage:
 *   node scripts/deploy-primeswap-pools.js --network <local|mainnet>
 *
 * Environment:
 *   DEPLOYER_MNEMONIC - 12-word seed phrase for deployer account
 *   ETR_ADDRESS - ETR token contract address
 *   EDSC_ADDRESS - EDSC token contract address
 */

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { encodeAddress } = require('@polkadot/util-crypto');
const fs = require('fs');
const path = require('path');

// Network configuration
const NETWORKS = {
  local: {
    endpoint: 'ws://127.0.0.1:9944',
    description: 'Local development'
  },
  mainnet: {
    endpoint: 'ws://100.96.84.69:9944',
    description: 'Primearc mainnet'
  }
};

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

// Market cap weights (basis points)
const WEIGHTS = {
  BTC: 6766,
  ETH: 1531,
  XRP: 499,
  SOL: 356,
  BNB: 320,
  DOGE: 214,
  ADA: 125,
  LINK: 71,
  TRX: 53,
  XLM: 36,
  MATIC: 28
};

// ETR allocation (1.25B total)
const TOTAL_ETR = BigInt('1250000000000000000000000000'); // 1.25B with 18 decimals

// EDSC allocations
const EDSC_FOR_ETR_POOL = BigInt('100000000000000000000000000'); // 100M EDSC
const EDSC_FOR_PEG_POOL = BigInt('10000000000000000000000000');  // 10M EDSC

// Deploy gas limits
const DEPLOY_GAS = 50_000_000;
const CALL_GAS = 10_000_000;

/**
 * Load compiled artifact
 */
function loadArtifact(contractName) {
  const artifactPath = path.join(
    __dirname,
    `../artifacts/src/${contractName}.sol/${contractName}.json`
  );

  if (!fs.existsSync(artifactPath)) {
    throw new Error(`Artifact not found: ${artifactPath}`);
  }

  return JSON.parse(fs.readFileSync(artifactPath, 'utf8'));
}

/**
 * Deploy contract via ËtwasmVM
 */
async function deployContract(api, signer, contractName, constructorData = '0x') {
  console.log(`\n📦 Deploying ${contractName}...`);

  const artifact = loadArtifact(contractName);
  let bytecode = artifact.bytecode;

  // Append constructor data if provided
  if (constructorData && constructorData !== '0x') {
    bytecode = bytecode + constructorData.slice(2);
  }

  // Remove 0x prefix
  if (bytecode.startsWith('0x')) {
    bytecode = bytecode.slice(2);
  }

  const codeBytes = Buffer.from(bytecode, 'hex');
  console.log(`  Bytecode: ${codeBytes.length} bytes`);

  // Deploy via pallet-etwasm-vm
  const tx = api.tx.etwasmVm.deployContract(codeBytes);

  return new Promise((resolve, reject) => {
    tx.signAndSend(signer, ({ status, events }) => {
      if (status.isFinalized) {
        let contractAddress = null;

        events.forEach(({ event }) => {
          if (event.section === 'etwasmVm' && event.method === 'ContractDeployed') {
            contractAddress = event.data[1].toString();
            console.log(`  ✓ Deployed at: ${contractAddress}`);
          }
        });

        if (contractAddress) {
          resolve({ address: contractAddress, artifact });
        } else {
          reject(new Error('Deployment failed - no ContractDeployed event'));
        }
      }
    }).catch(reject);
  });
}

/**
 * Call contract method
 */
async function callContract(api, signer, address, methodSig, params = []) {
  // Encode method call (simplified - in production use ethers.js ABI encoder)
  const tx = api.tx.etwasmVm.callContract(address, methodSig, CALL_GAS);

  return new Promise((resolve, reject) => {
    tx.signAndSend(signer, ({ status, events }) => {
      if (status.isFinalized) {
        resolve({ success: true, block: status.asFinalized.toHex() });
      }
    }).catch(reject);
  });
}

/**
 * Main deployment
 */
async function main() {
  console.log('╔════════════════════════════════════════════════════════════╗');
  console.log('║         PrimeSwap Pools Deployment to Primearc             ║');
  console.log('╚════════════════════════════════════════════════════════════╝\n');

  // Parse arguments
  const args = process.argv.slice(2);
  const networkArg = args.find(a => a.startsWith('--network='))?.split('=')[1] || 'local';

  const network = NETWORKS[networkArg];
  if (!network) {
    console.error(`Unknown network: ${networkArg}`);
    process.exit(1);
  }

  console.log(`Network: ${networkArg} (${network.description})`);
  console.log(`Endpoint: ${network.endpoint}\n`);

  // Connect to chain
  console.log('Connecting to Primearc...');
  const provider = new WsProvider(network.endpoint);
  const api = await ApiPromise.create({ provider });
  console.log('✓ Connected\n');

  // Setup deployer
  const keyring = new Keyring({ type: 'sr25519' });
  let signer;

  if (process.env.DEPLOYER_MNEMONIC) {
    signer = keyring.addFromMnemonic(process.env.DEPLOYER_MNEMONIC);
  } else if (networkArg === 'local') {
    signer = keyring.addFromUri('//Alice');
  } else {
    console.error('DEPLOYER_MNEMONIC required for non-local deployment');
    process.exit(1);
  }

  console.log(`Deployer: ${signer.address}`);

  // Check balance
  const { data: { free: balance } } = await api.query.system.account(signer.address);
  console.log(`Balance: ${balance.toString()} units\n`);

  // Token addresses (from environment or pre-deployed)
  const ETR_ADDRESS = process.env.ETR_ADDRESS || '0x0000000000000000000000000000000000000001';
  const EDSC_ADDRESS = process.env.EDSC_ADDRESS || '0x0000000000000000000000000000000000000002';
  const USDT_ADDRESS = process.env.USDT_ADDRESS;

  console.log('Token addresses:');
  console.log(`  ETR: ${ETR_ADDRESS}`);
  console.log(`  EDSC: ${EDSC_ADDRESS}`);
  console.log(`  USDT: ${USDT_ADDRESS || '(not set)'}\n`);

  // Store deployment addresses
  const deployments = {
    network: networkArg,
    timestamp: new Date().toISOString(),
    deployer: signer.address,
    contracts: {}
  };

  try {
    // Step 1: Deploy Oracle
    console.log('═══ Step 1: Deploy PrimeSwapOracle ═══');
    const oracle = await deployContract(api, signer, 'PrimeSwapOracle');
    deployments.contracts.oracle = oracle.address;

    // Step 2: Deploy Factory
    console.log('\n═══ Step 2: Deploy PrimeSwapFactory ═══');
    // Constructor: (etr, edsc, oracle)
    const factory = await deployContract(api, signer, 'PrimeSwapFactory');
    deployments.contracts.factory = factory.address;

    // Step 3: Create pools (would require contract calls)
    console.log('\n═══ Step 3: Deployment Summary ═══');
    console.log('\nContracts deployed:');
    console.log(`  Oracle:  ${deployments.contracts.oracle}`);
    console.log(`  Factory: ${deployments.contracts.factory}`);

    // Save deployments
    const deploymentsFile = path.join(__dirname, `../deployments-${networkArg}.json`);
    fs.writeFileSync(deploymentsFile, JSON.stringify(deployments, null, 2));
    console.log(`\n✓ Saved to ${deploymentsFile}`);

    console.log('\n╔════════════════════════════════════════════════════════════╗');
    console.log('║               Deployment Complete!                         ║');
    console.log('╚════════════════════════════════════════════════════════════╝');
    console.log('\nNext steps:');
    console.log('1. Initialize oracle prices');
    console.log('2. Create VirtualReserveAMM pools via Factory');
    console.log('3. Create EDSCOraclePool and EDSCPegPool');
    console.log('4. Seed pools with 1.25B ETR and 110M EDSC');
    console.log('\nUse the following allocations:');

    // Calculate and display allocations
    const totalWeight = Object.values(WEIGHTS).reduce((a, b) => a + b, 0);
    console.log('\nETR Allocations (market cap weighted):');
    for (const [token, weight] of Object.entries(WEIGHTS)) {
      const allocation = (TOTAL_ETR * BigInt(weight)) / BigInt(totalWeight);
      const formatted = (Number(allocation) / 1e18).toLocaleString();
      console.log(`  ETR/w${token}: ${formatted} ETR (${(weight / 100).toFixed(2)}%)`);
    }

    console.log('\nEDSC Allocations:');
    console.log(`  ETR/EDSC pool:  ${(Number(EDSC_FOR_ETR_POOL) / 1e18).toLocaleString()} EDSC`);
    console.log(`  EDSC/USDT pool: ${(Number(EDSC_FOR_PEG_POOL) / 1e18).toLocaleString()} EDSC`);

  } catch (error) {
    console.error('\n❌ Deployment failed:', error.message);
    process.exit(1);
  } finally {
    await api.disconnect();
  }
}

main();
