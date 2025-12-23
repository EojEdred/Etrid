#!/usr/bin/env node
/**
 * Export Current Chain State for Genesis Migration
 *
 * Exports the current chain state and modifies it to:
 * 1. Preserve all current balances (including ETR seeding)
 * 2. Update sudo key to a known account
 * 3. Generate a new genesis JSON that can be used for chain restart
 *
 * Usage:
 *   node scripts/export-state-genesis.js --network mainnet --output new-genesis.json
 */

const { ApiPromise, WsProvider } = require('@polkadot/api');
const fs = require('fs');
const path = require('path');

const NETWORKS = {
  local: { endpoint: 'ws://127.0.0.1:9944' },
  mainnet: { endpoint: 'ws://100.96.84.69:9944' }
};

// New sudo key (Gizzi Payment - we have the seed phrase)
const NEW_SUDO_KEY = '5HQMqpWrZU1AdN2WumX2Fv8EphJUgiF6fmyMZr94HH31kVQd';

async function main() {
  const args = process.argv.slice(2);
  const networkArg = args.find(a => a.startsWith('--network='))?.split('=')[1]
                  || args[args.indexOf('--network') + 1]
                  || 'mainnet';

  const outputArg = args.find(a => a.startsWith('--output='))?.split('=')[1]
                  || args[args.indexOf('--output') + 1]
                  || 'migrated-genesis.json';

  const network = NETWORKS[networkArg];
  if (!network) {
    console.error(`Unknown network: ${networkArg}`);
    process.exit(1);
  }

  console.log('╔════════════════════════════════════════════════════════════════╗');
  console.log('║          Chain State Export for Genesis Migration             ║');
  console.log('╚════════════════════════════════════════════════════════════════╝\n');

  console.log(`Network: ${networkArg}`);
  console.log(`Endpoint: ${network.endpoint}`);
  console.log(`Output: ${outputArg}\n`);

  // Connect to chain
  console.log('Connecting to Primearc...');
  const provider = new WsProvider(network.endpoint);
  const api = await ApiPromise.create({ provider });

  const chain = await api.rpc.system.chain();
  const [blockHash, blockNumber] = await Promise.all([
    api.rpc.chain.getBlockHash(),
    api.rpc.chain.getHeader().then(h => h.number.toNumber())
  ]);

  console.log(`Connected to: ${chain}`);
  console.log(`Current block: #${blockNumber}`);
  console.log(`Block hash: ${blockHash.toHex()}\n`);

  // Export all account balances
  console.log('═══ Exporting Account Balances ═══\n');

  const balances = [];
  const entries = await api.query.system.account.entries();

  console.log(`Found ${entries.length} accounts`);

  for (const [key, value] of entries) {
    const address = key.args[0].toString();
    const { data: { free, reserved } } = value;
    const total = BigInt(free.toString()) + BigInt(reserved.toString());

    if (total > 0n) {
      balances.push({
        address,
        free: free.toString(),
        reserved: reserved.toString(),
        total: total.toString()
      });
    }
  }

  console.log(`Accounts with balance: ${balances.length}\n`);

  // Show top balances
  const sortedBalances = [...balances].sort((a, b) =>
    BigInt(b.total) > BigInt(a.total) ? 1 : -1
  );

  console.log('Top 10 Balances:');
  for (let i = 0; i < Math.min(10, sortedBalances.length); i++) {
    const b = sortedBalances[i];
    const etr = BigInt(b.total) / 10n**18n;
    console.log(`  ${b.address.slice(0, 20)}... ${etr.toLocaleString()} ETR`);
  }
  console.log();

  // Get current sudo key
  console.log('═══ Sudo Configuration ═══\n');

  let currentSudo = null;
  try {
    const sudoKey = await api.query.sudo.key();
    currentSudo = sudoKey.toString();
    console.log(`Current sudo: ${currentSudo}`);
  } catch (e) {
    console.log('Could not query sudo key');
  }

  console.log(`New sudo:     ${NEW_SUDO_KEY}\n`);

  // Get validator set
  console.log('═══ Validator Configuration ═══\n');

  let validators = [];
  try {
    // Try to get session validators
    const sessionValidators = await api.query.session.validators();
    validators = sessionValidators.map(v => v.toString());
    console.log(`Active validators: ${validators.length}`);
    validators.forEach(v => console.log(`  ${v}`));
  } catch (e) {
    // Try aura authorities
    try {
      const auraAuthorities = await api.query.aura.authorities();
      validators = auraAuthorities.map(v => v.toString());
      console.log(`Aura authorities: ${validators.length}`);
    } catch (e2) {
      console.log('Could not query validators');
    }
  }
  console.log();

  // Build genesis JSON structure
  console.log('═══ Building Genesis JSON ═══\n');

  // Format balances for genesis (address, amount pairs)
  const genesisBalances = balances.map(b => [
    b.address,
    b.total  // Use total (free + reserved) as new free balance
  ]);

  const genesis = {
    name: "Ëtrid Primearc Core Mainnet",
    id: "primearc_core_mainnet",
    chainType: "Live",
    bootNodes: [],
    telemetryEndpoints: [],
    protocolId: "primearc",
    properties: {
      tokenDecimals: 18,
      tokenSymbol: "ETR"
    },
    genesis: {
      runtime: {
        balances: {
          balances: genesisBalances
        },
        sudo: {
          key: NEW_SUDO_KEY
        }
      }
    },
    _metadata: {
      exportedAt: new Date().toISOString(),
      fromBlock: blockNumber,
      fromBlockHash: blockHash.toHex(),
      accountsExported: balances.length,
      sudoChanged: currentSudo !== NEW_SUDO_KEY
    }
  };

  // Save to file
  const outputPath = path.resolve(outputArg);
  fs.writeFileSync(outputPath, JSON.stringify(genesis, null, 2));
  console.log(`Genesis JSON saved to: ${outputPath}\n`);

  // Also save raw balances for reference
  const balancesPath = outputPath.replace('.json', '-balances.json');
  fs.writeFileSync(balancesPath, JSON.stringify({
    exportedAt: new Date().toISOString(),
    blockNumber,
    blockHash: blockHash.toHex(),
    balances: sortedBalances
  }, null, 2));
  console.log(`Balances backup saved to: ${balancesPath}\n`);

  // Summary
  console.log('╔════════════════════════════════════════════════════════════════╗');
  console.log('║                      Export Summary                            ║');
  console.log('╚════════════════════════════════════════════════════════════════╝\n');

  const totalSupply = balances.reduce((sum, b) => sum + BigInt(b.total), 0n);
  console.log(`Total ETR exported: ${(totalSupply / 10n**18n).toLocaleString()} ETR`);
  console.log(`Accounts: ${balances.length}`);
  console.log(`From block: #${blockNumber}`);
  console.log(`Sudo updated: ${currentSudo} → ${NEW_SUDO_KEY}`);

  console.log('\n═══ Next Steps ═══');
  console.log('1. Review the exported genesis JSON');
  console.log('2. Merge with your runtime preset (validators, consensus, etc.)');
  console.log('3. Build chain spec: ./target/release/primearc-core-node build-spec --chain <merged-genesis>');
  console.log('4. Purge chain data on all validators');
  console.log('5. Restart with new chain spec\n');

  await api.disconnect();
}

main().catch(console.error);
