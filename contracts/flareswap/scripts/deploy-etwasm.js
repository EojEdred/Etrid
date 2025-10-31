#!/usr/bin/env node
/**
 * FlareSwap Deployment Script for ËtwasmVM
 *
 * This script deploys FlareSwap DEX contracts to the Ëtrid blockchain
 * via the ËtwasmVM pallet (pallet-etwasm-vm).
 *
 * Usage:
 *   node scripts/deploy-etwasm.js --network <local|testnet|mainnet>
 */

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const fs = require('fs');
const path = require('path');

// Configuration
const NETWORKS = {
  local: {
    endpoint: 'ws://127.0.0.1:9944',
    description: 'Local development node'
  },
  testnet: {
    endpoint: 'wss://testnet.etrid.org',
    description: 'Ëtrid testnet'
  },
  mainnet: {
    endpoint: 'wss://rpc.etrid.org',
    description: 'Ëtrid mainnet'
  }
};

// Gas limits
const DEFAULT_GAS_LIMIT = 10_000_000; // 10M VMw
const DEPLOY_GAS_LIMIT = 20_000_000; // 20M VMw for deployment

/**
 * Load compiled contract artifact
 */
function loadArtifact(contractName) {
  const artifactPath = path.join(
    __dirname,
    '../artifacts/src',
    getContractPath(contractName),
    `${contractName}.json`
  );

  console.log(`Loading artifact: ${artifactPath}`);

  if (!fs.existsSync(artifactPath)) {
    throw new Error(`Artifact not found: ${contractName} at ${artifactPath}`);
  }

  const artifact = JSON.parse(fs.readFileSync(artifactPath, 'utf8'));
  return {
    abi: artifact.abi,
    bytecode: artifact.bytecode,
    deployedBytecode: artifact.deployedBytecode
  };
}

/**
 * Get contract path in artifacts
 */
function getContractPath(contractName) {
  const paths = {
    'WETH': 'periphery/WETH.sol',
    'FlareSwapFactory': 'core/FlareSwapFactory.sol',
    'FlareSwapRouter': 'periphery/FlareSwapRouter.sol',
    'FlareSwapPair': 'core/FlareSwapPair.sol',
    'FlareSwapERC20': 'core/FlareSwapERC20.sol'
  };

  return paths[contractName] || `${contractName}.sol`;
}

/**
 * Deploy contract to ËtwasmVM
 */
async function deployContract(api, signer, contractName, constructorArgs = []) {
  console.log(`\n📦 Deploying ${contractName}...`);

  const artifact = loadArtifact(contractName);

  // Encode constructor if needed
  let bytecode = artifact.bytecode;
  if (constructorArgs.length > 0) {
    // TODO: Encode constructor arguments
    console.log(`  Constructor args:`, constructorArgs);
    // This would need to encode the constructor args according to ABI
  }

  // Remove '0x' prefix if present
  if (bytecode.startsWith('0x')) {
    bytecode = bytecode.slice(2);
  }

  // Convert hex string to bytes
  const codeBytes = Buffer.from(bytecode, 'hex');

  console.log(`  Bytecode size: ${codeBytes.length} bytes`);
  console.log(`  Deploying to ËtwasmVM...`);

  // Create deployment transaction
  const tx = api.tx.etwasmVm.deployContract(codeBytes);

  return new Promise((resolve, reject) => {
    tx.signAndSend(signer, ({ status, events }) => {
      if (status.isInBlock) {
        console.log(`  ✓ Included in block: ${status.asInBlock}`);
      }

      if (status.isFinalized) {
        console.log(`  ✓ Finalized in block: ${status.asFinalized}`);

        // Find the ContractDeployed event
        let contractAddress = null;
        events.forEach(({ event }) => {
          if (event.section === 'etwasmVm' && event.method === 'ContractDeployed') {
            const [deployer, address, codeHash] = event.data;
            contractAddress = address.toString();
            console.log(`  ✓ Contract deployed at: ${contractAddress}`);
            console.log(`  ✓ Code hash: ${codeHash.toHex()}`);
          }
        });

        if (contractAddress) {
          resolve({
            address: contractAddress,
            artifact,
            tx: status.asFinalized.toHex()
          });
        } else {
          reject(new Error('ContractDeployed event not found'));
        }
      }
    }).catch(reject);
  });
}

/**
 * Call contract method
 */
async function callContract(api, signer, contractAddress, inputData, gasLimit = DEFAULT_GAS_LIMIT) {
  console.log(`📞 Calling contract ${contractAddress}...`);

  const tx = api.tx.etwasmVm.callContract(
    contractAddress,
    inputData,
    gasLimit
  );

  return new Promise((resolve, reject) => {
    tx.signAndSend(signer, ({ status, events }) => {
      if (status.isFinalized) {
        console.log(`  ✓ Call finalized`);

        events.forEach(({ event }) => {
          if (event.section === 'etwasmVm' && event.method === 'ContractExecuted') {
            const [contract, gasUsed, success] = event.data;
            console.log(`  ✓ Gas used: ${gasUsed.toString()}`);
            console.log(`  ✓ Success: ${success.toString()}`);
          }
        });

        resolve();
      }
    }).catch(reject);
  });
}

/**
 * Main deployment function
 */
async function main() {
  // Parse command line arguments
  const args = process.argv.slice(2);
  const networkArg = args.find(arg => arg.startsWith('--network='));
  const networkName = networkArg ? networkArg.split('=')[1] : 'local';

  if (!NETWORKS[networkName]) {
    console.error(`❌ Unknown network: ${networkName}`);
    console.log(`Available networks: ${Object.keys(NETWORKS).join(', ')}`);
    process.exit(1);
  }

  const network = NETWORKS[networkName];

  console.log(`\n╔════════════════════════════════════════════╗`);
  console.log(`║  FlareSwap Deployment to ËtwasmVM         ║`);
  console.log(`╚════════════════════════════════════════════╝\n`);
  console.log(`Network: ${network.description}`);
  console.log(`Endpoint: ${network.endpoint}\n`);

  // Connect to the blockchain
  console.log(`🔗 Connecting to Ëtrid blockchain...`);
  const provider = new WsProvider(network.endpoint);
  const api = await ApiPromise.create({ provider });

  console.log(`✓ Connected to chain: ${(await api.rpc.system.chain()).toString()}`);
  console.log(`✓ Node version: ${(await api.rpc.system.version()).toString()}\n`);

  // Setup deployer account
  const keyring = new Keyring({ type: 'sr25519' });

  // Use //Alice for local development, otherwise load from env
  let deployer;
  if (networkName === 'local') {
    deployer = keyring.addFromUri('//Alice');
    console.log(`👤 Using Alice account for deployment`);
  } else {
    // Load from environment variable or keystore
    const mnemonic = process.env.DEPLOYER_MNEMONIC;
    if (!mnemonic) {
      console.error(`❌ DEPLOYER_MNEMONIC environment variable not set`);
      console.log(`Set it with: export DEPLOYER_MNEMONIC="your twelve word mnemonic phrase here"`);
      process.exit(1);
    }
    deployer = keyring.addFromMnemonic(mnemonic);
  }

  console.log(`✓ Deployer address: ${deployer.address}\n`);

  // Check deployer balance
  const { data: balance } = await api.query.system.account(deployer.address);
  console.log(`💰 Deployer balance: ${balance.free.toHuman()}\n`);

  // Deployment sequence
  const deployments = {};

  try {
    // Step 1: Deploy WETH
    console.log(`\n═══ Step 1: Deploy WETH ═══`);
    deployments.weth = await deployContract(api, deployer, 'WETH');

    // Step 2: Deploy FlareSwapFactory
    console.log(`\n═══ Step 2: Deploy FlareSwapFactory ═══`);
    deployments.factory = await deployContract(api, deployer, 'FlareSwapFactory', [
      deployer.address // feeToSetter
    ]);

    // Step 3: Deploy FlareSwapRouter
    console.log(`\n═══ Step 3: Deploy FlareSwapRouter ═══`);
    deployments.router = await deployContract(api, deployer, 'FlareSwapRouter', [
      deployments.factory.address,
      deployments.weth.address
    ]);

    // Save deployment info
    const deploymentInfo = {
      network: networkName,
      chainName: (await api.rpc.system.chain()).toString(),
      deployer: deployer.address,
      timestamp: new Date().toISOString(),
      contracts: {
        weth: {
          address: deployments.weth.address,
          tx: deployments.weth.tx
        },
        factory: {
          address: deployments.factory.address,
          tx: deployments.factory.tx
        },
        router: {
          address: deployments.router.address,
          tx: deployments.router.tx
        }
      }
    };

    const outputPath = path.join(__dirname, `../deployments-etwasm-${networkName}.json`);
    fs.writeFileSync(outputPath, JSON.stringify(deploymentInfo, null, 2));

    console.log(`\n╔════════════════════════════════════════════╗`);
    console.log(`║  🎉 DEPLOYMENT SUCCESSFUL!                ║`);
    console.log(`╚════════════════════════════════════════════╝\n`);

    console.log(`Contract Addresses:`);
    console.log(`  WETH:     ${deployments.weth.address}`);
    console.log(`  Factory:  ${deployments.factory.address}`);
    console.log(`  Router:   ${deployments.router.address}\n`);

    console.log(`Deployment info saved to: ${outputPath}\n`);

    console.log(`Next steps:`);
    console.log(`  1. Verify contracts are deployed correctly`);
    console.log(`  2. Create initial trading pairs`);
    console.log(`  3. Add initial liquidity`);
    console.log(`  4. Test swaps\n`);

  } catch (error) {
    console.error(`\n❌ Deployment failed:`, error.message);
    console.error(error);
    process.exit(1);
  } finally {
    await api.disconnect();
  }
}

// Run deployment
main().catch(console.error);
