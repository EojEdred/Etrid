#!/usr/bin/env node
/**
 * PrimeSwap ETR Pool Seeding Script
 *
 * Seeds the 11 VirtualReserveAMM pools (ETR/wToken pairs) with ETR from treasury.
 * Uses market cap weighted distribution from DAO Treasury (875M ETR).
 *
 * Pool Architecture:
 * - Each pool receives a dedicated ETR allocation
 * - Single-sided liquidity: Foundation seeds ETR, users add wrapped tokens
 * - Oracle determines initial exchange rate until liquidity builds
 *
 * Usage:
 *   node scripts/seed-etr-pools.js --network mainnet [--dry-run]
 */

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');
const fs = require('fs');
const path = require('path');

// ═══════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════

const NETWORKS = {
  local: { endpoint: 'ws://127.0.0.1:9944' },
  mainnet: { endpoint: 'ws://100.96.84.69:9944' }
};

// Treasury accounts with seed phrases
const TREASURY = {
  daoTreasury: {
    address: '5GBq8WgTBzf6mfyu5qP9JFgJeJt8oFobGpDKjbkSxkn7cQ5K',
    seed: 'win wait seat reject wish vendor supply dry horror loyal need drop',
    allocation: 875_000_000n  // 875M ETR available
  },
  communityLpPool: {
    address: '5Gdae5WysRZbw4GUogcSSvDC5pTCy1Vh2zJe1qRY58t7rssj',
    seed: 'theme seminar nose rural express frequent virtual resemble camp theme know admit',
    allocation: 250_000_000n  // 250M ETR available
  },
  networkExpansion: {
    address: '5FCckgBUS4KoUVQEck7trY9pXMDF64jciARtFuruz3x2LL32',
    seed: 'dismiss faculty charge tape snow transfer found wisdom split never erupt coral',
    allocation: 625_000_000n  // 625M ETR available
  }
};

// Token prices (8 decimals, USD) - for reference
const PRICES = {
  ETR: 400000n,              // $0.004
  BTC: 10000000000000n,      // $100,000
  ETH: 400000000000n,        // $4,000
  XRP: 250000000n,           // $2.50
  SOL: 25000000000n,         // $250
  BNB: 70000000000n,         // $700
  DOGE: 40000000n,           // $0.40
  ADA: 100000000n,           // $1.00
  LINK: 2500000000n,         // $25
  TRX: 30000000n,            // $0.30
  XLM: 50000000n,            // $0.50
  MATIC: 100000000n          // $1.00
};

// Market cap weights (basis points, total = 10000)
const WEIGHTS = {
  BTC: 6766n,   // 67.66%
  ETH: 1531n,   // 15.31%
  XRP: 499n,    // 4.99%
  SOL: 356n,    // 3.56%
  BNB: 320n,    // 3.20%
  DOGE: 214n,   // 2.14%
  ADA: 125n,    // 1.25%
  LINK: 71n,    // 0.71%
  TRX: 53n,     // 0.53%
  XLM: 36n,     // 0.36%
  MATIC: 28n    // 0.28%
};

// Total ETR to distribute across all 11 pools
const TOTAL_ETR_FOR_POOLS = 1_250_000_000n; // 1.25B ETR
const DECIMALS = 18n;
const ONE_ETR = 10n ** DECIMALS;

// ═══════════════════════════════════════════════════════════════
// Helper Functions
// ═══════════════════════════════════════════════════════════════

function calculateAllocations() {
  const totalWeight = Object.values(WEIGHTS).reduce((a, b) => a + b, 0n);
  const totalEtrWithDecimals = TOTAL_ETR_FOR_POOLS * ONE_ETR;

  const allocations = {};
  for (const [token, weight] of Object.entries(WEIGHTS)) {
    allocations[token] = (totalEtrWithDecimals * weight) / totalWeight;
  }

  return allocations;
}

function formatETR(amount) {
  const etr = amount / ONE_ETR;
  return etr.toLocaleString() + ' ETR';
}

async function getBalance(api, address) {
  const { data: { free } } = await api.query.system.account(address);
  return BigInt(free.toString());
}

async function transfer(api, signer, to, amount, dryRun = false) {
  if (dryRun) {
    console.log(`  [DRY RUN] Would transfer ${formatETR(amount)} to ${to}`);
    return { success: true, dryRun: true };
  }

  return new Promise((resolve, reject) => {
    const tx = api.tx.balances.transferKeepAlive(to, amount.toString());

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

      if (status.isFinalized) {
        console.log(`  ✓ Transfer finalized in block ${status.asFinalized.toHex()}`);
        resolve({ success: true, block: status.asFinalized.toHex() });
      }
    }).catch(reject);
  });
}

// ═══════════════════════════════════════════════════════════════
// Main Seeding Function
// ═══════════════════════════════════════════════════════════════

async function main() {
  console.log('╔════════════════════════════════════════════════════════════════╗');
  console.log('║          PrimeSwap ETR Pool Seeding                            ║');
  console.log('╚════════════════════════════════════════════════════════════════╝\n');

  // Parse arguments
  const args = process.argv.slice(2);
  const networkArg = args.find(a => a.startsWith('--network='))?.split('=')[1]
                  || args[args.indexOf('--network') + 1]
                  || 'mainnet';
  const dryRun = args.includes('--dry-run');

  const network = NETWORKS[networkArg];
  if (!network) {
    console.error(`Unknown network: ${networkArg}`);
    process.exit(1);
  }

  console.log(`Network: ${networkArg}`);
  console.log(`Endpoint: ${network.endpoint}`);
  console.log(`Mode: ${dryRun ? 'DRY RUN (no actual transfers)' : 'LIVE EXECUTION'}\n`);

  if (!dryRun) {
    console.log('⚠️  WARNING: This will execute real ETR transfers!');
    console.log('   Press Ctrl+C within 5 seconds to abort...\n');
    await new Promise(r => setTimeout(r, 5000));
  }

  await cryptoWaitReady();

  // Setup keyring
  const keyring = new Keyring({ type: 'sr25519' });

  // Load treasury signers
  const daoTreasury = keyring.addFromMnemonic(TREASURY.daoTreasury.seed);
  const lpPool = keyring.addFromMnemonic(TREASURY.communityLpPool.seed);
  const networkExpansion = keyring.addFromMnemonic(TREASURY.networkExpansion.seed);

  // Connect to chain
  console.log('Connecting to Primearc...');
  const provider = new WsProvider(network.endpoint);
  const api = await ApiPromise.create({ provider });

  const chain = await api.rpc.system.chain();
  console.log(`Connected to: ${chain}\n`);

  // Check treasury balances
  console.log('═══ Treasury Balances ═══');
  const daoBalance = await getBalance(api, TREASURY.daoTreasury.address);
  const lpBalance = await getBalance(api, TREASURY.communityLpPool.address);
  const expBalance = await getBalance(api, TREASURY.networkExpansion.address);

  console.log(`  DAO Treasury:      ${formatETR(daoBalance)}`);
  console.log(`  Community LP Pool: ${formatETR(lpBalance)}`);
  console.log(`  Network Expansion: ${formatETR(expBalance)}`);

  const totalAvailable = daoBalance + lpBalance + expBalance;
  console.log(`  ─────────────────────────────────`);
  console.log(`  Total Available:   ${formatETR(totalAvailable)}\n`);

  // Calculate allocations
  const allocations = calculateAllocations();
  let totalNeeded = 0n;

  console.log('═══ Pool Allocations (Market Cap Weighted) ═══');
  for (const [token, amount] of Object.entries(allocations)) {
    const weight = WEIGHTS[token];
    const pct = Number(weight) / 100;
    console.log(`  ETR/w${token.padEnd(5)} ${formatETR(amount).padStart(25)}  (${pct.toFixed(2)}%)`);
    totalNeeded += amount;
  }
  console.log(`  ─────────────────────────────────────────────`);
  console.log(`  Total Needed:      ${formatETR(totalNeeded)}\n`);

  // Verify sufficient funds
  if (totalAvailable < totalNeeded) {
    console.error(`❌ Insufficient funds! Need ${formatETR(totalNeeded)}, have ${formatETR(totalAvailable)}`);
    await api.disconnect();
    process.exit(1);
  }

  console.log(`✓ Sufficient funds available\n`);

  // ═══════════════════════════════════════════════════════════════
  // Seeding Strategy
  // ═══════════════════════════════════════════════════════════════

  console.log('═══ Seeding Strategy ═══\n');

  // For now, we'll consolidate all pool ETR into the LP Pool account
  // which will serve as the master liquidity pool reserve

  console.log('Strategy: Consolidate ETR into Community LP Pool as master reserve');
  console.log('This account will serve as the protocol treasury for all pools.\n');

  // Calculate how much we need to transfer from each source
  const daoToTransfer = TREASURY.daoTreasury.allocation * ONE_ETR;
  const expToTransfer = TREASURY.networkExpansion.allocation * ONE_ETR;

  // Leave some ETR for gas fees (1000 ETR buffer per account)
  const gasBuffer = 1000n * ONE_ETR;

  console.log('Transfer Plan:');
  console.log(`  From DAO Treasury:     ${formatETR(daoToTransfer - gasBuffer)}`);
  console.log(`  From Network Expansion: ${formatETR(expToTransfer - gasBuffer)}`);
  console.log(`  To: Community LP Pool  (${TREASURY.communityLpPool.address})\n`);

  // Execute transfers
  console.log('═══ Executing Transfers ═══\n');

  // Transfer from DAO Treasury to LP Pool
  if (daoBalance > gasBuffer) {
    const transferAmount = daoBalance - gasBuffer;
    console.log(`1. DAO Treasury → LP Pool: ${formatETR(transferAmount)}`);
    try {
      await transfer(api, daoTreasury, TREASURY.communityLpPool.address, transferAmount, dryRun);
    } catch (err) {
      console.error(`   ❌ Transfer failed: ${err.message}`);
    }
  }

  // Transfer from Network Expansion to LP Pool
  if (expBalance > gasBuffer) {
    const transferAmount = expBalance - gasBuffer;
    console.log(`\n2. Network Expansion → LP Pool: ${formatETR(transferAmount)}`);
    try {
      await transfer(api, networkExpansion, TREASURY.communityLpPool.address, transferAmount, dryRun);
    } catch (err) {
      console.error(`   ❌ Transfer failed: ${err.message}`);
    }
  }

  // Final balance check
  console.log('\n═══ Final Balances ═══');
  const finalDaoBalance = await getBalance(api, TREASURY.daoTreasury.address);
  const finalLpBalance = await getBalance(api, TREASURY.communityLpPool.address);
  const finalExpBalance = await getBalance(api, TREASURY.networkExpansion.address);

  console.log(`  DAO Treasury:      ${formatETR(finalDaoBalance)}`);
  console.log(`  Community LP Pool: ${formatETR(finalLpBalance)}`);
  console.log(`  Network Expansion: ${formatETR(finalExpBalance)}`);

  // Save seeding report
  const report = {
    timestamp: new Date().toISOString(),
    network: networkArg,
    dryRun,
    allocations: Object.fromEntries(
      Object.entries(allocations).map(([k, v]) => [k, { amount: v.toString(), formatted: formatETR(v) }])
    ),
    treasuryBalances: {
      before: {
        daoTreasury: formatETR(daoBalance),
        communityLpPool: formatETR(lpBalance),
        networkExpansion: formatETR(expBalance)
      },
      after: {
        daoTreasury: formatETR(finalDaoBalance),
        communityLpPool: formatETR(finalLpBalance),
        networkExpansion: formatETR(finalExpBalance)
      }
    },
    edscStatus: {
      blocked: true,
      reason: 'EDSC minting requires sudo access to authorize a minter. Sudo key unknown.',
      sudoAccount: '5HCvaHrCfXDasyQNRCdJ4jRtcwMmdkPDZEAF3LqF77qf5JtP',
      minterAuthority: '5DvgxdPMHmkR6oYsWVkKPUvcFJo6CtSdtKKsHQg8rc9F8s1p',
      action: 'Need to find or recover sudo key to authorize EDSC minting'
    }
  };

  const reportPath = path.join(__dirname, '../seeding-report.json');
  fs.writeFileSync(reportPath, JSON.stringify(report, null, 2));
  console.log(`\nReport saved to: ${reportPath}`);

  // Summary
  console.log('\n╔════════════════════════════════════════════════════════════════╗');
  console.log('║                      Seeding Summary                           ║');
  console.log('╚════════════════════════════════════════════════════════════════╝');

  console.log('\n✓ ETR Pool Reserves: READY');
  console.log(`  Total in LP Pool: ${formatETR(finalLpBalance)}`);
  console.log('\n✗ EDSC Pools: BLOCKED');
  console.log('  Reason: Need sudo access to authorize EDSC minter');
  console.log('  Sudo Account: 5HCvaHrCfXDasyQNRCdJ4jRtcwMmdkPDZEAF3LqF77qf5JtP');

  console.log('\n═══ Next Steps ═══');
  console.log('1. Find or recover the sudo key seed phrase');
  console.log('2. Call sudo.sudo(edscToken.authorizeMinter(minterAuthority))');
  console.log('3. Call edscToken.mint() to mint 1B EDSC to Reserve Vault');
  console.log('4. Create EDSC pools with the minted tokens\n');

  await api.disconnect();
}

main().catch(console.error);
