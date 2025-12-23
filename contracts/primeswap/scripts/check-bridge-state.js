#!/usr/bin/env node
/**
 * Check bridge pallet states for wrapped tokens
 */

const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
  console.log('Connecting to Primearc mainnet...\n');
  const provider = new WsProvider('ws://100.96.84.69:9944');
  const api = await ApiPromise.create({ provider });

  const bridgePallets = [
    'bitcoinBridge',
    'ethereumBridge',
    'dogeBridge',
    'stellarBridge',
    'xrpBridge',
    'solanaBridge',
    'cardanoBridge',
    'chainlinkBridge',
    'polygonBridge',
    'bnbBridge',
    'tronBridge',
    'usdtBridge'
  ];

  console.log('═══ Bridge Pallet States ═══\n');

  for (const palletName of bridgePallets) {
    const pallet = api.query[palletName];
    if (!pallet) {
      console.log(`${palletName}: NOT FOUND`);
      continue;
    }

    console.log(`\n─── ${palletName} ───`);
    const storageKeys = Object.keys(pallet);
    console.log(`Storage items: ${storageKeys.join(', ')}`);

    // Try to get entries for common storage names
    for (const key of storageKeys) {
      try {
        if (typeof pallet[key].entries === 'function') {
          const entries = await pallet[key].entries();
          if (entries.length > 0) {
            console.log(`  ${key}: ${entries.length} entries`);
            // Show first few entries
            for (let i = 0; i < Math.min(3, entries.length); i++) {
              const [k, v] = entries[i];
              console.log(`    - ${k.args?.map(a => a.toString()).join(', ') || k.toString()}`);
            }
          }
        } else {
          // Single value storage
          const value = await pallet[key]();
          if (value && !value.isEmpty) {
            console.log(`  ${key}: ${JSON.stringify(value.toHuman())}`);
          }
        }
      } catch (e) {
        // Skip errors silently
      }
    }
  }

  // Check custodianRegistry
  console.log('\n\n═══ Custodian Registry ═══');
  if (api.query.custodianRegistry) {
    const keys = Object.keys(api.query.custodianRegistry);
    console.log(`Storage items: ${keys.join(', ')}`);

    for (const key of keys) {
      try {
        if (typeof api.query.custodianRegistry[key].entries === 'function') {
          const entries = await api.query.custodianRegistry[key].entries();
          if (entries.length > 0) {
            console.log(`  ${key}: ${entries.length} entries`);
          }
        }
      } catch (e) {}
    }
  }

  // Check bridgeAttestation
  console.log('\n═══ Bridge Attestation ═══');
  if (api.query.bridgeAttestation) {
    const keys = Object.keys(api.query.bridgeAttestation);
    console.log(`Storage items: ${keys.join(', ')}`);

    for (const key of keys) {
      try {
        if (typeof api.query.bridgeAttestation[key].entries === 'function') {
          const entries = await api.query.bridgeAttestation[key].entries();
          if (entries.length > 0) {
            console.log(`  ${key}: ${entries.length} entries`);
          }
        }
      } catch (e) {}
    }
  }

  // Check reserveVault
  console.log('\n═══ Reserve Vault ═══');
  if (api.query.reserveVault) {
    const keys = Object.keys(api.query.reserveVault);
    console.log(`Storage items: ${keys.join(', ')}`);

    for (const key of keys) {
      try {
        if (typeof api.query.reserveVault[key].entries === 'function') {
          const entries = await api.query.reserveVault[key].entries();
          if (entries.length > 0) {
            console.log(`  ${key}: ${entries.length} entries`);
            for (const [k, v] of entries.slice(0, 3)) {
              console.log(`    - ${k.args?.map(a => a.toString()).join(', ')}: ${JSON.stringify(v.toHuman())}`);
            }
          }
        } else {
          const value = await api.query.reserveVault[key]();
          if (value && !value.isEmpty) {
            console.log(`  ${key}: ${JSON.stringify(value.toHuman())}`);
          }
        }
      } catch (e) {}
    }
  }

  console.log('\n═══ Summary ═══');
  console.log('If bridge pallets have no entries, wrapped tokens have not been created yet.');
  console.log('Genesis migration would only need to preserve balances.');

  await api.disconnect();
}

main().catch(console.error);
