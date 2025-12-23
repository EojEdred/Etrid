#!/usr/bin/env node
/**
 * PrimeSwap Substrate Deployment Script
 *
 * Deploys PrimeSwap contracts to Primearc via ËtwasmVM pallet extrinsics.
 * Uses your Substrate account (ca4...) directly - no EVM account needed.
 *
 * Usage:
 *   # Set your seed phrase (do this locally, don't share!)
 *   export DEPLOYER_SEED="your twelve word seed phrase here"
 *
 *   # Run deployment
 *   node scripts/deploy-substrate.js --network mainnet
 *
 * Networks:
 *   --network local    -> ws://127.0.0.1:9944
 *   --network mainnet  -> ws://100.96.84.69:9944
 */

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');
const fs = require('fs');
const path = require('path');

// ═══════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════

const NETWORKS = {
  local: {
    endpoint: 'ws://127.0.0.1:9944',
    description: 'Local development node'
  },
  mainnet: {
    endpoint: 'ws://100.96.84.69:9944',
    description: 'Primearc Core mainnet'
  }
};

// Initial prices (8 decimals) - $0.004 ETR
const PRICES = {
  ETR: 400000,
  BTC: 10000000000000,
  ETH: 400000000000,
  XRP: 250000000,
  SOL: 25000000000,
  BNB: 70000000000,
  DOGE: 40000000,
  ADA: 100000000,
  LINK: 2500000000,
  TRX: 30000000,
  XLM: 50000000,
  MATIC: 100000000
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

// ═══════════════════════════════════════════════════════════════
// Helper Functions
// ═══════════════════════════════════════════════════════════════

/**
 * Load compiled contract bytecode from Hardhat artifacts
 */
function loadBytecode(contractName, artifactPath = null) {
  const searchPaths = [
    path.join(__dirname, `../artifacts/src/${contractName}.sol/${contractName}.json`),
    path.join(__dirname, `../artifacts/contracts/${contractName}.sol/${contractName}.json`),
  ];

  if (artifactPath) {
    searchPaths.unshift(artifactPath);
  }

  for (const p of searchPaths) {
    if (fs.existsSync(p)) {
      const artifact = JSON.parse(fs.readFileSync(p, 'utf8'));
      let bytecode = artifact.bytecode;

      // Remove 0x prefix if present
      if (bytecode.startsWith('0x')) {
        bytecode = bytecode.slice(2);
      }

      console.log(`  Loaded ${contractName}: ${bytecode.length / 2} bytes`);
      return bytecode;
    }
  }

  throw new Error(`Artifact not found for ${contractName}`);
}

/**
 * Deploy contract via ËtwasmVM pallet
 */
async function deployContract(api, signer, name, bytecode) {
  console.log(`\n  Deploying ${name}...`);

  const codeBytes = Buffer.from(bytecode, 'hex');

  return new Promise((resolve, reject) => {
    // Check if etwasmVM pallet exists, otherwise try evm pallet
    let tx;
    if (api.tx.etwasmVM) {
      tx = api.tx.etwasmVM.deployContract(codeBytes);
    } else if (api.tx.etwasmVm) {
      tx = api.tx.etwasmVm.deployContract(codeBytes);
    } else if (api.tx.evm) {
      // Frontier EVM pallet - use create
      tx = api.tx.evm.create(
        signer.address,           // source
        codeBytes,                // init code
        0,                        // value
        10000000,                 // gas limit
        null,                     // max fee per gas
        null,                     // max priority fee
        null,                     // nonce
        []                        // access list
      );
    } else {
      reject(new Error('No EVM pallet found (etwasmVm or evm)'));
      return;
    }

    let contractAddress = null;

    tx.signAndSend(signer, { nonce: -1 }, ({ status, events, dispatchError }) => {
      if (dispatchError) {
        if (dispatchError.isModule) {
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          reject(new Error(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
        } else {
          reject(new Error(dispatchError.toString()));
        }
        return;
      }

      if (status.isInBlock || status.isFinalized) {
        // Look for deployment event
        events.forEach(({ event }) => {
          const { section, method, data } = event;

          // ËtwasmVM pallet event (check both casings)
          if ((section === 'etwasmVM' || section === 'etwasmVm') && method === 'ContractDeployed') {
            contractAddress = data[1]?.toString() || data[0]?.toString();
          }

          // Frontier EVM pallet event
          if (section === 'evm' && method === 'Created') {
            contractAddress = data[0]?.toString();
          }

          // Ethereum pallet event
          if (section === 'ethereum' && method === 'Executed') {
            contractAddress = data[1]?.toString();
          }
        });

        if (status.isFinalized) {
          if (contractAddress) {
            console.log(`  Contract deployed at: ${contractAddress}`);
            resolve({ address: contractAddress, name });
          } else {
            // If no specific address found, derive from deployer
            console.log(`  Contract deployed (address in transaction receipt)`);
            resolve({ address: 'pending', name });
          }
        }
      }
    }).catch(reject);
  });
}

/**
 * Wait for transaction finalization
 */
function waitForTx(api, signer, tx) {
  return new Promise((resolve, reject) => {
    tx.signAndSend(signer, { nonce: -1 }, ({ status, dispatchError }) => {
      if (dispatchError) {
        if (dispatchError.isModule) {
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          reject(new Error(`${decoded.section}.${decoded.name}`));
        } else {
          reject(new Error(dispatchError.toString()));
        }
        return;
      }

      if (status.isFinalized) {
        resolve(status.asFinalized.toHex());
      }
    }).catch(reject);
  });
}

// ═══════════════════════════════════════════════════════════════
// Main Deployment
// ═══════════════════════════════════════════════════════════════

async function main() {
  console.log('╔════════════════════════════════════════════════════════════════╗');
  console.log('║     PrimeSwap Deployment via Substrate (ËtwasmVM Pallet)       ║');
  console.log('╚════════════════════════════════════════════════════════════════╝\n');

  // Parse arguments
  const args = process.argv.slice(2);
  const networkArg = args.find(a => a.startsWith('--network='))?.split('=')[1]
                  || args[args.indexOf('--network') + 1]
                  || 'local';

  const network = NETWORKS[networkArg];
  if (!network) {
    console.error(`Unknown network: ${networkArg}`);
    console.error('Available: local, mainnet');
    process.exit(1);
  }

  console.log(`Network: ${networkArg} (${network.description})`);
  console.log(`Endpoint: ${network.endpoint}\n`);

  // Wait for crypto to be ready
  await cryptoWaitReady();

  // Setup keyring and signer
  const keyring = new Keyring({ type: 'sr25519' });
  let signer;

  if (process.env.DEPLOYER_SEED) {
    signer = keyring.addFromMnemonic(process.env.DEPLOYER_SEED);
    console.log(`Deployer: ${signer.address}`);
  } else if (networkArg === 'local') {
    // Use Alice for local testing
    signer = keyring.addFromUri('//Alice');
    console.log(`Deployer: ${signer.address} (//Alice dev account)`);
  } else {
    console.error('\n!! DEPLOYER_SEED environment variable required for mainnet !!');
    console.error('\nSet it like this (keep it secret!):');
    console.error('  export DEPLOYER_SEED="your twelve word seed phrase here"');
    console.error('\nThen run:');
    console.error('  node scripts/deploy-substrate.js --network mainnet\n');
    process.exit(1);
  }

  // Connect to chain
  console.log('\nConnecting to Primearc...');
  const provider = new WsProvider(network.endpoint);
  const api = await ApiPromise.create({ provider });

  const chain = await api.rpc.system.chain();
  const version = await api.rpc.system.version();
  console.log(`Connected to: ${chain} (${version})`);

  // Check balance
  const { data: { free: balance } } = await api.query.system.account(signer.address);
  const balanceETR = balance.toBigInt() / BigInt(10 ** 18);
  console.log(`Balance: ${balanceETR.toLocaleString()} ETR\n`);

  if (balanceETR < 1000n) {
    console.warn('Warning: Low balance. Deployment requires ETR for gas fees.\n');
  }

  // Check available pallets
  console.log('Checking available pallets...');
  const pallets = Object.keys(api.tx);
  const hasEtwasmVM = pallets.includes('etwasmVM');
  const hasEtwasmVm = pallets.includes('etwasmVm');
  const hasEvm = pallets.includes('evm');
  const hasEthereum = pallets.includes('ethereum');

  console.log(`  etwasmVM: ${hasEtwasmVM ? 'Yes' : 'No'}`);
  console.log(`  etwasmVm: ${hasEtwasmVm ? 'Yes' : 'No'}`);
  console.log(`  evm (Frontier): ${hasEvm ? 'Yes' : 'No'}`);
  console.log(`  ethereum: ${hasEthereum ? 'Yes' : 'No'}`);

  if (!hasEtwasmVM && !hasEtwasmVm && !hasEvm) {
    console.error('\nNo EVM pallet found! Cannot deploy Solidity contracts.');
    console.error('Make sure the chain has etwasmVm or evm pallet enabled.');
    await api.disconnect();
    process.exit(1);
  }

  // Store deployments
  const deployments = {
    network: networkArg,
    timestamp: new Date().toISOString(),
    deployer: signer.address,
    chain: chain.toString(),
    contracts: {}
  };

  try {
    // ═══════════════════════════════════════════════════════════════
    // Step 1: Deploy PrimeSwapOracle
    // ═══════════════════════════════════════════════════════════════
    console.log('\n═══ Step 1: Deploy PrimeSwapOracle ═══');

    const oracleBytecode = loadBytecode('PrimeSwapOracle');
    const oracle = await deployContract(api, signer, 'PrimeSwapOracle', oracleBytecode);
    deployments.contracts.oracle = oracle.address;

    // ═══════════════════════════════════════════════════════════════
    // Step 2: Deploy PrimeSwapFactory
    // ═══════════════════════════════════════════════════════════════
    console.log('\n═══ Step 2: Deploy PrimeSwapFactory ═══');

    // For factory, we need to encode constructor arguments
    // Constructor: (address _etr, address _edsc, address _oracle)
    const ETR_ADDRESS = process.env.ETR_ADDRESS || '0x0000000000000000000000000000000000000001';
    const EDSC_ADDRESS = process.env.EDSC_ADDRESS || '0x0000000000000000000000000000000000000002';

    console.log(`  ETR Address: ${ETR_ADDRESS}`);
    console.log(`  EDSC Address: ${EDSC_ADDRESS}`);
    console.log(`  Oracle Address: ${oracle.address}`);

    // Load factory bytecode (constructor args need to be appended)
    let factoryBytecode = loadBytecode('PrimeSwapFactory',
      path.join(__dirname, '../artifacts/src/PrimeSwapFactory.sol/PrimeSwapFactory.json'));

    // Encode constructor arguments (3 addresses, each 32 bytes padded)
    const encodeAddress = (addr) => {
      // Remove 0x and pad to 32 bytes (64 hex chars)
      const clean = addr.replace('0x', '').toLowerCase();
      return clean.padStart(64, '0');
    };

    const constructorArgs = encodeAddress(ETR_ADDRESS)
                          + encodeAddress(EDSC_ADDRESS)
                          + encodeAddress(oracle.address);

    factoryBytecode = factoryBytecode + constructorArgs;

    const factory = await deployContract(api, signer, 'PrimeSwapFactory', factoryBytecode);
    deployments.contracts.factory = factory.address;

    // ═══════════════════════════════════════════════════════════════
    // Step 3: Summary and Next Steps
    // ═══════════════════════════════════════════════════════════════
    console.log('\n═══ Step 3: Deployment Summary ═══');

    console.log('\nContracts Deployed:');
    console.log(`  PrimeSwapOracle:  ${deployments.contracts.oracle}`);
    console.log(`  PrimeSwapFactory: ${deployments.contracts.factory}`);

    // Save deployments
    const deploymentsPath = path.join(__dirname, `../deployments-${networkArg}-substrate.json`);
    fs.writeFileSync(deploymentsPath, JSON.stringify(deployments, null, 2));
    console.log(`\nDeployments saved to: ${deploymentsPath}`);

    // Calculate ETR allocations
    const TOTAL_ETR = BigInt('1250000000000000000000000000');
    const totalWeight = Object.values(WEIGHTS).reduce((a, b) => a + b, 0);

    console.log('\n═══ Pool Seeding Guide ═══');
    console.log('\nETR Allocations (1.25B total, market cap weighted):');
    console.log('─'.repeat(55));

    for (const [token, weight] of Object.entries(WEIGHTS)) {
      const allocation = (TOTAL_ETR * BigInt(weight)) / BigInt(totalWeight);
      const formatted = (Number(allocation) / 1e18).toLocaleString();
      console.log(`  ETR/w${token.padEnd(5)}  ${formatted.padStart(18)} ETR  (${(weight / 100).toFixed(2)}%)`);
    }

    console.log('\nEDSC Allocations:');
    console.log('─'.repeat(55));
    console.log('  ETR/EDSC pool:    100,000,000 EDSC');
    console.log('  EDSC/USDT pool:    10,000,000 EDSC');

    console.log('\n╔════════════════════════════════════════════════════════════════╗');
    console.log('║                   Deployment Complete!                         ║');
    console.log('╚════════════════════════════════════════════════════════════════╝');

    console.log('\nNext Steps:');
    console.log('1. Initialize oracle prices (call updatePrice for each token)');
    console.log('2. Create VirtualReserveAMM pools via Factory');
    console.log('3. Create EDSCOraclePool for ETR/EDSC pair');
    console.log('4. Create EDSCPegPool for EDSC/USDT pair');
    console.log('5. Seed pools with ETR from your treasury account');
    console.log('6. Seed EDSC pools with EDSC tokens\n');

  } catch (error) {
    console.error('\nDeployment failed:', error.message);
    console.error(error.stack);
    process.exit(1);
  } finally {
    await api.disconnect();
  }
}

main().catch(console.error);
