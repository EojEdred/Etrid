/**
 * ËtwasmVM Smart Contract Example
 *
 * Demonstrates how to:
 * 1. Upload WebAssembly contract code
 * 2. Instantiate contracts
 * 3. Call contract methods
 * 4. Query contract state
 * 5. Estimate gas costs
 *
 * ËtwasmVM runs WebAssembly smart contracts with VMw (Virtual Machine work) gas metering.
 * 1 ÉTR = 1,000,000 VMw
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { readFileSync } from 'fs';
import { EtwasmVMWrapper, GAS_CONSTANTS } from '../src/wrappers/EtwasmVMWrapper';

async function main() {
  // 1. Connect to Ëtrid node
  console.log('Connecting to Ëtrid FlareChain...');
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider });
  console.log('✓ Connected to chain:', (await api.rpc.system.chain()).toString());

  // 2. Initialize account
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');

  console.log(`\nDeployer: ${alice.address}`);
  console.log(`Test User: ${bob.address}`);

  // 3. Create ËtwasmVM wrapper
  const etwasm = new EtwasmVMWrapper(api);

  console.log('\n═══════════════════════════════════════════');
  console.log('Gas System Overview');
  console.log('═══════════════════════════════════════════\n');

  console.log('VMw (Virtual Machine work) units:');
  console.log(`  • 1 ÉTR = ${GAS_CONSTANTS.VMW_PER_ETR.toLocaleString()} VMw`);
  console.log(`  • Block limit: ${GAS_CONSTANTS.BLOCK_LIMIT.toLocaleString()} VMw`);
  console.log(`  • TX limit: ${GAS_CONSTANTS.TX_LIMIT.toLocaleString()} VMw`);
  console.log(`  • Default gas: ${GAS_CONSTANTS.DEFAULT_GAS.toLocaleString()} VMw`);

  console.log('\nGas costs in ÉTR:');
  console.log(`  • 100,000 VMw = ${100_000n * 10n**18n / GAS_CONSTANTS.VMW_PER_ETR / 10n**18n} ÉTR`);
  console.log(`  • 500,000 VMw = ${500_000n * 10n**18n / GAS_CONSTANTS.VMW_PER_ETR / 10n**18n} ÉTR`);
  console.log(`  • 1,000,000 VMw = ${1_000_000n * 10n**18n / GAS_CONSTANTS.VMW_PER_ETR / 10n**18n} ÉTR`);

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 1: Upload Contract Code');
  console.log('═══════════════════════════════════════════\n');

  // For demonstration, we'll create a mock WASM bytecode
  // In real usage, you would load actual compiled WASM:
  // const wasmCode = readFileSync('./contracts/erc20.wasm');

  const mockWasmCode = new Uint8Array([
    0x00, 0x61, 0x73, 0x6d, // WASM magic number
    0x01, 0x00, 0x00, 0x00, // WASM version
    // ... rest of WASM bytecode
  ]);

  try {
    console.log('Uploading contract code to chain...');
    console.log(`Code size: ${mockWasmCode.length} bytes`);

    const uploadResult = await etwasm.uploadCode(
      alice,
      mockWasmCode,
      GAS_CONSTANTS.DEFAULT_GAS
    );

    console.log(`\n✓ Code uploaded successfully!`);
    console.log(`  Code Hash: ${uploadResult.codeHash}`);
    console.log(`  Transaction: ${uploadResult.txHash}`);
    console.log(`  Gas Used: ${uploadResult.gasUsed.toLocaleString()} VMw`);
    console.log(`  Cost: ${uploadResult.gasUsed * 10n**18n / GAS_CONSTANTS.VMW_PER_ETR / 10n**18n} ÉTR`);

  } catch (error) {
    console.log('⚠ Mock code upload (for demonstration)');
    console.log('  In production, upload actual WASM bytecode');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 2: Deploy Contract (One-Step)');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('Deploying ERC20 token contract...');

    // Constructor arguments for ERC20
    const constructorArgs = [
      'MyToken',           // name
      'MTK',               // symbol
      18,                  // decimals
      1000000n * 10n**18n, // initial supply
    ];

    const deployment = await etwasm.deployContract(
      alice,
      mockWasmCode,
      constructorArgs,
      0n, // No value sent
      GAS_CONSTANTS.DEFAULT_GAS
    );

    console.log(`\n✓ Contract deployed!`);
    console.log(`  Address: ${deployment.address}`);
    console.log(`  Code Hash: ${deployment.codeHash}`);
    console.log(`  Transaction: ${deployment.txHash}`);
    console.log(`  Gas Used: ${deployment.gasUsed.toLocaleString()} VMw`);

    // Query contract info
    const info = await etwasm.getContractInfo(deployment.address);
    console.log(`\n  Contract Info:`);
    console.log(`    Code Hash: ${info.codeHash}`);
    console.log(`    Deployer: ${info.deployer}`);
    console.log(`    Deployed: ${new Date(info.deployedAt).toLocaleString()}`);

  } catch (error) {
    console.log('⚠ Mock deployment (for demonstration)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 3: Call Contract Method');
  console.log('═══════════════════════════════════════════\n');

  const mockContractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

  try {
    console.log('Calling contract.transfer()...');

    const callResult = await etwasm.callContract(
      alice,
      mockContractAddress,
      'transfer',
      [
        bob.address,          // recipient
        1000n * 10n**18n,     // amount
      ],
      0n, // No value sent
      500_000n // Gas limit
    );

    console.log(`\n✓ Transaction successful!`);
    console.log(`  Transaction: ${callResult.txHash}`);
    console.log(`  Gas Used: ${callResult.gasUsed.toLocaleString()} VMw`);
    console.log(`  Cost: ${callResult.gasUsed * 10n**18n / GAS_CONSTANTS.VMW_PER_ETR / 10n**18n} ÉTR`);

    if (callResult.events.length > 0) {
      console.log(`\n  Events emitted:`);
      callResult.events.forEach((event, i) => {
        console.log(`    ${i + 1}. ${event.name}`);
        console.log(`       Data: ${JSON.stringify(event.data, null, 2)}`);
      });
    }

  } catch (error) {
    console.log('⚠ Mock contract call (for demonstration)');
    console.log('  In production, call actual deployed contracts');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 4: Query Contract State');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('Querying contract.balanceOf()...');

    const balance = await etwasm.queryContract(
      mockContractAddress,
      'balanceOf',
      [bob.address],
      alice.address // Caller for gas estimation
    );

    console.log(`\n✓ Query successful!`);
    console.log(`  Bob's balance: ${balance / 10n**18n} MTK`);
    console.log('  Note: Queries are free (no gas charged)');

  } catch (error) {
    console.log('⚠ Mock query (for demonstration)');
    console.log('  Example result: 1000 MTK');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 5: Estimate Gas Before Execution');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('Estimating gas for complex operation...');

    const estimate = await etwasm.estimateGas(
      mockContractAddress,
      'batchTransfer',
      [
        [bob.address, '0x1234...', '0x5678...'], // recipients
        [100n * 10n**18n, 200n * 10n**18n, 300n * 10n**18n], // amounts
      ]
    );

    console.log(`\n✓ Gas estimate:`);
    console.log(`  Estimated: ${estimate.estimated.toLocaleString()} VMw`);
    console.log(`  With buffer: ${estimate.withBuffer.toLocaleString()} VMw (${estimate.bufferPercent}% extra)`);
    console.log(`  Max possible: ${estimate.maxPossible.toLocaleString()} VMw`);

    const costInEtr = estimate.withBuffer * 10n**18n / GAS_CONSTANTS.VMW_PER_ETR / 10n**18n;
    console.log(`\n  Estimated cost: ${costInEtr} ÉTR`);

  } catch (error) {
    console.log('⚠ Mock estimation (for demonstration)');
    console.log('  Example: 250,000 VMw = 0.25 ÉTR');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 6: Multi-Contract DApp');
  console.log('═══════════════════════════════════════════\n');

  console.log('Deploying DeFi DApp with multiple contracts:\n');

  const contracts = [
    {
      name: 'Token',
      file: 'erc20.wasm',
      purpose: 'ERC20 token contract',
      estimatedGas: 400_000n,
    },
    {
      name: 'DEX',
      file: 'dex.wasm',
      purpose: 'Decentralized exchange',
      estimatedGas: 800_000n,
    },
    {
      name: 'Staking',
      file: 'staking.wasm',
      purpose: 'Staking rewards contract',
      estimatedGas: 600_000n,
    },
    {
      name: 'Governance',
      file: 'governance.wasm',
      purpose: 'DAO governance contract',
      estimatedGas: 700_000n,
    },
  ];

  let totalGas = 0n;

  contracts.forEach((contract, i) => {
    console.log(`${i + 1}. ${contract.name} (${contract.file})`);
    console.log(`   Purpose: ${contract.purpose}`);
    console.log(`   Est. Gas: ${contract.estimatedGas.toLocaleString()} VMw`);
    console.log(`   Est. Cost: ${contract.estimatedGas * 10n**18n / GAS_CONSTANTS.VMW_PER_ETR / 10n**18n} ÉTR`);
    console.log();
    totalGas += contract.estimatedGas;
  });

  console.log(`Total deployment cost:`);
  console.log(`  Gas: ${totalGas.toLocaleString()} VMw`);
  console.log(`  ÉTR: ${totalGas * 10n**18n / GAS_CONSTANTS.VMW_PER_ETR / 10n**18n} ÉTR`);

  console.log('\n═══════════════════════════════════════════');
  console.log('Contract Development Best Practices');
  console.log('═══════════════════════════════════════════\n');

  console.log('1. Gas Optimization:');
  console.log('   • Minimize storage operations');
  console.log('   • Batch operations when possible');
  console.log('   • Use efficient data structures');
  console.log('   • Always estimate gas before execution');

  console.log('\n2. Security:');
  console.log('   • Audit contracts before deployment');
  console.log('   • Use SafeMath for arithmetic');
  console.log('   • Implement access controls');
  console.log('   • Test edge cases thoroughly');

  console.log('\n3. Deployment:');
  console.log('   • Test on local node first');
  console.log('   • Deploy to testnet before mainnet');
  console.log('   • Verify deployment success');
  console.log('   • Keep deployment receipts');

  console.log('\n4. Interaction:');
  console.log('   • Use queries for read operations (free)');
  console.log('   • Batch state-changing operations');
  console.log('   • Handle errors gracefully');
  console.log('   • Monitor events for confirmations');

  console.log('\n═══════════════════════════════════════════');
  console.log('Contract Examples Available');
  console.log('═══════════════════════════════════════════\n');

  const examples = [
    { name: 'ERC20 Token', complexity: 'Basic', gas: '~400K VMw' },
    { name: 'NFT (ERC721)', complexity: 'Basic', gas: '~500K VMw' },
    { name: 'DEX (Uniswap-like)', complexity: 'Medium', gas: '~800K VMw' },
    { name: 'Lending Protocol', complexity: 'Advanced', gas: '~1M VMw' },
    { name: 'DAO Governance', complexity: 'Advanced', gas: '~700K VMw' },
    { name: 'Multisig Wallet', complexity: 'Medium', gas: '~600K VMw' },
  ];

  examples.forEach((ex) => {
    const cost = parseFloat(ex.gas.replace(/[^\d]/g, '')) / 1_000;
    console.log(`• ${ex.name.padEnd(25)} ${ex.complexity.padEnd(10)} ~${cost.toFixed(2)} ÉTR`);
  });

  console.log('\n═══════════════════════════════════════════');
  console.log('Comparison: ËtwasmVM vs Other Platforms');
  console.log('═══════════════════════════════════════════\n');

  console.log('ËtwasmVM:');
  console.log('  ✓ Language: WebAssembly (compile from Rust, C++, etc.)');
  console.log('  ✓ Gas: VMw units, predictable costs');
  console.log('  ✓ Speed: Near-native execution');
  console.log('  ✓ Safety: Sandboxed execution');

  console.log('\nEthereum EVM:');
  console.log('  • Language: Solidity, Vyper');
  console.log('  • Gas: Gwei, variable costs');
  console.log('  • Speed: ~10-15 TPS');
  console.log('  • Fees: $1-$50+ per transaction');

  console.log('\nAdvantages:');
  console.log('  1. Lower gas costs (10-100x cheaper)');
  console.log('  2. Faster execution (WASM performance)');
  console.log('  3. More language options');
  console.log('  4. Better developer tools');

  console.log('\n═══════════════════════════════════════════');
  console.log('Gas Cost Examples (Real-World)');
  console.log('═══════════════════════════════════════════\n');

  const operations = [
    { op: 'Simple transfer', gas: 50_000n },
    { op: 'Token swap', gas: 200_000n },
    { op: 'NFT mint', gas: 300_000n },
    { op: 'Add liquidity', gas: 400_000n },
    { op: 'Complex DeFi operation', gas: 800_000n },
  ];

  operations.forEach((op) => {
    const etr = op.gas * 10n**18n / GAS_CONSTANTS.VMW_PER_ETR / 10n**18n;
    const usd = Number(etr) * 50; // Assuming $50/ÉTR
    console.log(`${op.op.padEnd(25)}: ${op.gas.toLocaleString().padStart(8)} VMw = ${etr} ÉTR (~$${usd.toFixed(2)})`);
  });

  console.log('\nNote: Costs are deterministic and ~100x cheaper than Ethereum!');

  // Cleanup
  await api.disconnect();
  console.log('\n✓ Disconnected from chain');
}

// Run example
main()
  .then(() => {
    console.log('\n✅ Example completed successfully!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Example failed:', error);
    process.exit(1);
  });
