#!/usr/bin/env node
/**
 * Check if ËtwasmVM is enabled and ready for contract deployment
 */

const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
  console.log('Checking ËtwasmVM status on mainnet...\n');

  const provider = new WsProvider('ws://100.96.84.69:9944');
  const api = await ApiPromise.create({ provider });

  // Check if etwasmVM pallet exists
  console.log('═══ EtwasmVM Pallet ═══');
  if (api.query.etwasmVM) {
    console.log('✓ etwasmVM pallet: EXISTS');

    const storageKeys = Object.keys(api.query.etwasmVM);
    console.log(`Storage items: ${storageKeys.join(', ')}`);

    if (api.tx.etwasmVM) {
      const txKeys = Object.keys(api.tx.etwasmVM);
      console.log(`Transactions: ${txKeys.join(', ')}`);
    }
  } else {
    console.log('✗ etwasmVM pallet: NOT FOUND');
  }

  // Check if contracts pallet exists (standard ink! contracts)
  console.log('\n═══ Contracts Pallet ═══');
  if (api.query.contracts) {
    console.log('✓ contracts pallet: EXISTS');

    const storageKeys = Object.keys(api.query.contracts);
    console.log(`Storage items: ${storageKeys.join(', ')}`);

    if (api.tx.contracts) {
      const txKeys = Object.keys(api.tx.contracts);
      console.log(`Transactions: ${txKeys.join(', ')}`);
    }
  } else {
    console.log('✗ contracts pallet: NOT FOUND');
  }

  // Check if EVM pallet exists
  console.log('\n═══ EVM Pallet ═══');
  if (api.query.evm) {
    console.log('✓ evm pallet: EXISTS');
  } else {
    console.log('✗ evm pallet: NOT FOUND (EVM is on ETH-PBC)');
  }

  // Check all pallets
  console.log('\n═══ All Available Pallets ═══');
  const pallets = Object.keys(api.query);
  console.log(pallets.join(', '));

  // Summary
  console.log('\n═══ Summary ═══');
  const hasEtwasm = !!api.query.etwasmVM;
  const hasContracts = !!api.query.contracts;
  const hasEvm = !!api.query.evm;

  if (hasEtwasm || hasContracts) {
    console.log('✓ Contract deployment possible via:');
    if (hasEtwasm) console.log('  - ËtwasmVM (custom VM)');
    if (hasContracts) console.log('  - ink! contracts pallet');
  } else if (hasEvm) {
    console.log('✓ EVM available for Solidity contracts');
  } else {
    console.log('✗ No contract execution environment found on Primearc Core');
    console.log('  Options:');
    console.log('  1. Build native pallet-primeswap');
    console.log('  2. Use ETH-PBC for EVM contracts');
    console.log('  3. Add contracts pallet to runtime');
  }

  await api.disconnect();
}

main().catch(console.error);
