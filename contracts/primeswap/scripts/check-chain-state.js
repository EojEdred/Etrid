#!/usr/bin/env node
/**
 * Check what state exists on-chain that needs to be preserved
 */

const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
  console.log('Connecting to Primearc mainnet...\n');
  const provider = new WsProvider('ws://100.96.84.69:9944');
  const api = await ApiPromise.create({ provider });

  const chain = await api.rpc.system.chain();
  const [blockHash, blockNumber] = await Promise.all([
    api.rpc.chain.getBlockHash(),
    api.rpc.chain.getHeader().then(h => h.number.toNumber())
  ]);

  console.log(`Chain: ${chain}`);
  console.log(`Block: #${blockNumber}`);
  console.log(`Genesis Hash: ${api.genesisHash.toHex()}\n`);

  // Check available pallets
  console.log('═══ Available Pallets ═══');
  const pallets = Object.keys(api.query);
  console.log(pallets.join(', '));
  console.log();

  // Check for contracts pallet
  if (api.query.contracts) {
    console.log('═══ Contracts Pallet ═══');
    try {
      const codeStorage = await api.query.contracts.codeStorage.entries();
      console.log(`Deployed contract codes: ${codeStorage.length}`);

      const contractInfo = await api.query.contracts.contractInfoOf.entries();
      console.log(`Active contracts: ${contractInfo.length}`);

      if (contractInfo.length > 0) {
        console.log('\nContract addresses:');
        for (const [key, value] of contractInfo) {
          console.log(`  ${key.args[0].toString()}`);
        }
      }
    } catch (e) {
      console.log(`Error querying contracts: ${e.message}`);
    }
  } else {
    console.log('═══ Contracts Pallet: NOT FOUND ═══');
  }
  console.log();

  // Check for assets pallet
  if (api.query.assets) {
    console.log('═══ Assets Pallet ═══');
    try {
      const assets = await api.query.assets.asset.entries();
      console.log(`Registered assets: ${assets.length}`);

      if (assets.length > 0) {
        console.log('\nAsset IDs:');
        for (const [key, value] of assets) {
          const assetId = key.args[0].toString();
          const metadata = await api.query.assets.metadata(assetId);
          console.log(`  ID ${assetId}: ${metadata.name.toHuman()} (${metadata.symbol.toHuman()})`);
        }
      }
    } catch (e) {
      console.log(`Error querying assets: ${e.message}`);
    }
  } else {
    console.log('═══ Assets Pallet: NOT FOUND ═══');
  }
  console.log();

  // Check for EDSC token
  if (api.query.edscToken) {
    console.log('═══ EDSC Token Pallet ═══');
    try {
      const totalSupply = await api.query.edscToken.totalSupply();
      console.log(`Total Supply: ${totalSupply.toString()}`);

      if (api.query.edscToken.authorizedMinters) {
        const minters = await api.query.edscToken.authorizedMinters.entries();
        console.log(`Authorized Minters: ${minters.length}`);
      }
    } catch (e) {
      console.log(`Error: ${e.message}`);
    }
  }
  console.log();

  // Check for wrapped tokens / bridge
  const bridgePallets = ['wrappedTokens', 'bridgeTokens', 'pbcBridge', 'crossChain', 'tokenBridge'];
  for (const pallet of bridgePallets) {
    if (api.query[pallet]) {
      console.log(`═══ ${pallet} Pallet ═══`);
      try {
        const entries = await api.query[pallet].entries();
        console.log(`Entries: ${entries.length}`);
      } catch (e) {
        // Try specific queries
        const keys = Object.keys(api.query[pallet]);
        console.log(`Storage items: ${keys.join(', ')}`);
      }
    }
  }

  // Check primeSwap if exists
  if (api.query.primeSwap || api.query.primeswap) {
    const ps = api.query.primeSwap || api.query.primeswap;
    console.log('═══ PrimeSwap Pallet ═══');
    try {
      const pools = await ps.pools?.entries() || [];
      console.log(`Pools: ${pools.length}`);
    } catch (e) {
      console.log(`Storage items: ${Object.keys(ps).join(', ')}`);
    }
  }
  console.log();

  // Summary
  console.log('═══ IMPORTANT ═══');
  console.log('Genesis migration will:');
  console.log('  ✓ Preserve: Account balances');
  console.log('  ✗ Reset: Block height (starts at 0)');
  console.log('  ✗ Change: Genesis hash (new chain identity)');
  console.log('  ? Contract/Asset state: See above');
  console.log();

  await api.disconnect();
}

main().catch(console.error);
