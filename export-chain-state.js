#!/usr/bin/env node

const { ApiPromise, WsProvider } = require('@polkadot/api');
const fs = require('fs');

async function exportChainState() {
  console.log('Connecting to chain...');
  const provider = new WsProvider('ws://100.96.84.69:9944');
  const api = await ApiPromise.create({ provider });

  const chain = await api.rpc.system.chain();
  const blockHash = await api.rpc.chain.getBlockHash();
  const blockNumber = (await api.rpc.chain.getHeader(blockHash)).number.toNumber();

  console.log(`Chain: ${chain}`);
  console.log(`Current block: #${blockNumber}`);
  console.log(`Block hash: ${blockHash}`);
  console.log('');
  console.log('Exporting balances...');

  // Get all accounts with balances
  const entries = await api.query.system.account.entries();
  const balances = [];

  for (const [key, account] of entries) {
    const address = key.args[0].toString();
    const balance = account.data.free.toString();

    if (balance !== '0') {
      balances.push([address, balance]); // Keep as string to preserve large numbers
    }
  }

  console.log(`Found ${balances.length} accounts with balances`);

  // Get current sudo key
  const sudoKey = await api.query.sudo.key();
  console.log(`Current sudo: ${sudoKey.toString()}`);

  // Export the state - convert balances to strings to avoid scientific notation
  const exportedState = {
    _export: {
      note: `Exported from block #${blockNumber} on ${new Date().toISOString().split('T')[0]}`,
      blockHash: blockHash.toString(),
      preservedState: "All balances and state exported from live chain"
    },
    balances: {
      balances: balances.sort((a, b) => {
        const diff = BigInt(b[1]) - BigInt(a[1]);
        return diff > 0n ? 1 : diff < 0n ? -1 : 0;
      }),
      devAccounts: null
    },
    sudo: {
      key: "5HQMqpWrZU1AdN2WumX2Fv8EphJUgiF6fmyMZr94HH31kVQd" // New Gizzi sudo key
    }
  };

  // Save to file
  const outputPath = '/tmp/chain-state-export.json';
  fs.writeFileSync(outputPath, JSON.stringify(exportedState, null, 2));
  console.log(`\n✓ State exported to: ${outputPath}`);
  console.log(`  Total accounts: ${balances.length}`);
  const totalSupply = balances.reduce((sum, [_, bal]) => sum + BigInt(bal), 0n);
  console.log(`  Total supply: ${totalSupply.toString()}`);

  await api.disconnect();
}

exportChainState().catch(console.error);
