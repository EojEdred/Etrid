#!/usr/bin/env node
/**
 * Check governance capabilities for runtime upgrade
 */

const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
  console.log('Connecting to Primearc mainnet...\n');
  const provider = new WsProvider('ws://100.96.84.69:9944');
  const api = await ApiPromise.create({ provider });

  console.log('═══ Checking Runtime Upgrade Paths ═══\n');

  // 1. Check sudo
  console.log('1. SUDO PALLET');
  if (api.query.sudo) {
    const sudoKey = await api.query.sudo.key();
    console.log(`   Sudo key: ${sudoKey.toString()}`);
    console.log('   Status: We do NOT have this key\n');
  }

  // 2. Check governance
  console.log('2. GOVERNANCE PALLET');
  if (api.query.governance) {
    const keys = Object.keys(api.query.governance);
    console.log(`   Storage: ${keys.join(', ')}`);

    // Check if governance can call root
    if (api.tx.governance) {
      const txs = Object.keys(api.tx.governance);
      console.log(`   Transactions: ${txs.join(', ')}`);
    }
  }

  // 3. Check validatorCommittee
  console.log('\n3. VALIDATOR COMMITTEE');
  if (api.query.validatorCommittee) {
    const keys = Object.keys(api.query.validatorCommittee);
    console.log(`   Storage: ${keys.join(', ')}`);

    if (api.tx.validatorCommittee) {
      const txs = Object.keys(api.tx.validatorCommittee);
      console.log(`   Transactions: ${txs.join(', ')}`);
    }

    // Check members
    try {
      if (api.query.validatorCommittee.members) {
        const members = await api.query.validatorCommittee.members();
        console.log(`   Members: ${members.length || members.toHuman()}`);
      }
    } catch (e) {}
  }

  // 4. Check consensusDayProposalSystem
  console.log('\n4. CONSENSUS DAY PROPOSAL SYSTEM');
  if (api.query.consensusDayProposalSystem) {
    const keys = Object.keys(api.query.consensusDayProposalSystem);
    console.log(`   Storage: ${keys.join(', ')}`);

    if (api.tx.consensusDayProposalSystem) {
      const txs = Object.keys(api.tx.consensusDayProposalSystem);
      console.log(`   Transactions: ${txs.join(', ')}`);
    }
  }

  // 5. Check if there's a scheduler
  console.log('\n5. SCHEDULER');
  if (api.query.scheduler) {
    console.log('   Scheduler pallet exists');
    if (api.tx.scheduler) {
      const txs = Object.keys(api.tx.scheduler);
      console.log(`   Transactions: ${txs.join(', ')}`);
    }
  } else {
    console.log('   NOT FOUND');
  }

  // 6. Check system pallet
  console.log('\n6. SYSTEM PALLET (setCode)');
  if (api.tx.system) {
    const txs = Object.keys(api.tx.system);
    console.log(`   Transactions: ${txs.join(', ')}`);

    // Check if setCode exists
    if (api.tx.system.setCode) {
      console.log('   setCode: EXISTS');
      console.log('   Required origin: Root (sudo or governance)');
    }
  }

  // 7. Check council/collective
  console.log('\n7. COLLECTIVE/COUNCIL');
  const collectiveVariants = ['council', 'collective', 'technicalCommittee'];
  for (const name of collectiveVariants) {
    if (api.query[name]) {
      console.log(`   ${name}: EXISTS`);
      if (api.tx[name]) {
        const txs = Object.keys(api.tx[name]);
        console.log(`   Transactions: ${txs.join(', ')}`);
      }
    }
  }

  // 8. Check for democracy
  console.log('\n8. DEMOCRACY');
  if (api.query.democracy) {
    console.log('   Democracy pallet: EXISTS');
    if (api.tx.democracy) {
      const txs = Object.keys(api.tx.democracy);
      console.log(`   Transactions: ${txs.join(', ')}`);
    }
  } else {
    console.log('   NOT FOUND');
  }

  console.log('\n═══ SUMMARY ═══\n');
  console.log('To execute system.setCode() we need Root origin.');
  console.log('Available paths:');
  console.log('  1. Sudo - BLOCKED (key unknown)');
  console.log('  2. Governance - Check if it can dispatch Root calls');
  console.log('  3. ValidatorCommittee - May have privileged access');
  console.log('  4. Democracy - If exists, can propose referenda');
  console.log();

  await api.disconnect();
}

main().catch(console.error);
