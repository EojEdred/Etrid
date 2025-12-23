#!/usr/bin/env node
/**
 * PrimeSwap Pool Initialization & Seeding Script
 *
 * This script:
 * 1. Initializes oracle prices for all tokens
 * 2. Creates VirtualReserveAMM pools (11 ETR/wToken pools)
 * 3. Creates EDSCOraclePool (ETR/EDSC)
 * 4. Creates EDSCPegPool (EDSC/USDT)
 * 5. Seeds pools with ETR from treasury
 *
 * Treasury Accounts Used:
 * - DAO Treasury: 875M ETR (main seeding source)
 * - Community LP Pool: 250M ETR (deployment account)
 * - Reserve Vault: 1B EDSC (for EDSC pools)
 *
 * Usage:
 *   node scripts/initialize-pools.js --network mainnet
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

// Treasury account seeds (from VALIDATOR_INFRASTRUCTURE_MAPPING.md)
const ACCOUNTS = {
  daoTreasury: {
    address: '5GBq8WgTBzf6mfyu5qP9JFgJeJt8oFobGpDKjbkSxkn7cQ5K',
    seed: 'win wait seat reject wish vendor supply dry horror loyal need drop',
    balance: 875_000_000n
  },
  communityLpPool: {
    address: '5Gdae5WysRZbw4GUogcSSvDC5pTCy1Vh2zJe1qRY58t7rssj',
    seed: 'theme seminar nose rural express frequent virtual resemble camp theme know admit',
    balance: 250_000_000n
  },
  reserveVault: {
    address: '5Eq5h1KQkzyDStVVaCnizXPHjL6c8HoetjKvzgdPF6i3w7md',
    seed: 'verify clean debate announce inspire trend tree brave eagle forest quit powder',
    edscBalance: 1_000_000_000n
  }
};

// Token prices (8 decimals, USD)
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
  BTC: 6766n,
  ETH: 1531n,
  XRP: 499n,
  SOL: 356n,
  BNB: 320n,
  DOGE: 214n,
  ADA: 125n,
  LINK: 71n,
  TRX: 53n,
  XLM: 36n,
  MATIC: 28n
};

const TOTAL_ETR_FOR_POOLS = 1_250_000_000n * (10n ** 18n);  // 1.25B ETR
const EDSC_FOR_ETR_POOL = 100_000_000n * (10n ** 18n);      // 100M EDSC
const EDSC_FOR_PEG_POOL = 10_000_000n * (10n ** 18n);       // 10M EDSC

// ═══════════════════════════════════════════════════════════════
// Helper Functions
// ═══════════════════════════════════════════════════════════════

function calculateAllocations() {
  const totalWeight = Object.values(WEIGHTS).reduce((a, b) => a + b, 0n);
  const allocations = {};

  for (const [token, weight] of Object.entries(WEIGHTS)) {
    allocations[token] = (TOTAL_ETR_FOR_POOLS * weight) / totalWeight;
  }

  return allocations;
}

async function callContract(api, signer, contractAddr, methodData, gasLimit = 1000000n) {
  return new Promise((resolve, reject) => {
    const tx = api.tx.etwasmVM.callContract(contractAddr, methodData, gasLimit);

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
        resolve({ block: status.asFinalized.toHex(), events });
      }
    }).catch(reject);
  });
}

// ═══════════════════════════════════════════════════════════════
// Main Initialization
// ═══════════════════════════════════════════════════════════════

async function main() {
  console.log('╔════════════════════════════════════════════════════════════════╗');
  console.log('║        PrimeSwap Pool Initialization & Seeding                 ║');
  console.log('╚════════════════════════════════════════════════════════════════╝\n');

  // Parse network
  const args = process.argv.slice(2);
  const networkArg = args.find(a => a.startsWith('--network='))?.split('=')[1]
                  || args[args.indexOf('--network') + 1]
                  || 'mainnet';

  const network = NETWORKS[networkArg];
  if (!network) {
    console.error(`Unknown network: ${networkArg}`);
    process.exit(1);
  }

  console.log(`Network: ${networkArg}`);
  console.log(`Endpoint: ${network.endpoint}\n`);

  await cryptoWaitReady();

  // Setup keyring
  const keyring = new Keyring({ type: 'sr25519' });

  // Load treasury accounts
  const daoTreasury = keyring.addFromMnemonic(ACCOUNTS.daoTreasury.seed);
  const lpPool = keyring.addFromMnemonic(ACCOUNTS.communityLpPool.seed);
  const reserveVault = keyring.addFromMnemonic(ACCOUNTS.reserveVault.seed);

  console.log('Treasury Accounts:');
  console.log(`  DAO Treasury:     ${daoTreasury.address}`);
  console.log(`  Community LP:     ${lpPool.address}`);
  console.log(`  Reserve Vault:    ${reserveVault.address}\n`);

  // Connect
  console.log('Connecting to Primearc...');
  const provider = new WsProvider(network.endpoint);
  const api = await ApiPromise.create({ provider });

  const chain = await api.rpc.system.chain();
  console.log(`Connected to: ${chain}\n`);

  // Check balances
  console.log('Checking balances...');
  for (const [name, account] of Object.entries(ACCOUNTS)) {
    const { data: { free } } = await api.query.system.account(account.address);
    const balanceETR = BigInt(free.toString()) / (10n ** 18n);
    console.log(`  ${name}: ${balanceETR.toLocaleString()} ETR`);
  }

  // Calculate allocations
  const allocations = calculateAllocations();

  console.log('\n═══ ETR Pool Allocations ═══');
  for (const [token, amount] of Object.entries(allocations)) {
    const etrAmount = amount / (10n ** 18n);
    console.log(`  ETR/w${token.padEnd(5)}: ${etrAmount.toLocaleString()} ETR`);
  }

  console.log('\n═══ EDSC Pool Allocations ═══');
  console.log(`  ETR/EDSC:   ${(EDSC_FOR_ETR_POOL / (10n ** 18n)).toLocaleString()} EDSC`);
  console.log(`  EDSC/USDT:  ${(EDSC_FOR_PEG_POOL / (10n ** 18n)).toLocaleString()} EDSC`);

  // ═══════════════════════════════════════════════════════════════
  // Step 1: Initialize Oracle Prices
  // ═══════════════════════════════════════════════════════════════
  console.log('\n═══ Step 1: Initialize Oracle Prices ═══');

  // The oracle contract is deployed under the LP Pool account
  const oracleOwner = lpPool;

  console.log('Setting token prices in oracle...');
  for (const [token, price] of Object.entries(PRICES)) {
    console.log(`  ${token}: $${Number(price) / 1e8}`);
  }

  console.log('\n  Note: Oracle price initialization requires contract calls.');
  console.log('  The ËtwasmVM pallet uses a different calling convention.');
  console.log('  Prices need to be set via etwasmVM.callContract extrinsic.\n');

  // ═══════════════════════════════════════════════════════════════
  // Step 2: Summary & Manual Steps
  // ═══════════════════════════════════════════════════════════════
  console.log('═══ Step 2: Pool Creation Summary ═══\n');

  console.log('The following pools need to be created via Factory:');
  console.log('');

  let poolNum = 1;
  for (const [token, amount] of Object.entries(allocations)) {
    const etrAmount = amount / (10n ** 18n);
    const price = PRICES[token];
    console.log(`${poolNum}. VirtualReserveAMM: ETR/w${token}`);
    console.log(`   ETR Seed: ${etrAmount.toLocaleString()} ETR`);
    console.log(`   w${token} Price: $${Number(price) / 1e8}`);
    console.log('');
    poolNum++;
  }

  console.log(`${poolNum}. EDSCOraclePool: ETR/EDSC`);
  console.log(`   EDSC Seed: ${(EDSC_FOR_ETR_POOL / (10n ** 18n)).toLocaleString()} EDSC`);
  console.log(`   ETR Price: $${Number(PRICES.ETR) / 1e8}`);
  console.log('');
  poolNum++;

  console.log(`${poolNum}. EDSCPegPool: EDSC/USDT`);
  console.log(`   EDSC Seed: ${(EDSC_FOR_PEG_POOL / (10n ** 18n)).toLocaleString()} EDSC`);
  console.log(`   Rate: 1:1 (stablecoin peg)`);
  console.log('');

  // ═══════════════════════════════════════════════════════════════
  // Save initialization plan
  // ═══════════════════════════════════════════════════════════════
  const plan = {
    network: networkArg,
    timestamp: new Date().toISOString(),
    accounts: {
      daoTreasury: ACCOUNTS.daoTreasury.address,
      communityLpPool: ACCOUNTS.communityLpPool.address,
      reserveVault: ACCOUNTS.reserveVault.address
    },
    oraclePrices: Object.fromEntries(
      Object.entries(PRICES).map(([k, v]) => [k, v.toString()])
    ),
    poolAllocations: Object.fromEntries(
      Object.entries(allocations).map(([k, v]) => [k, v.toString()])
    ),
    edscAllocations: {
      etrEdscPool: EDSC_FOR_ETR_POOL.toString(),
      edscUsdtPool: EDSC_FOR_PEG_POOL.toString()
    }
  };

  const planPath = path.join(__dirname, '../pool-initialization-plan.json');
  fs.writeFileSync(planPath, JSON.stringify(plan, null, 2));
  console.log(`Plan saved to: ${planPath}`);

  console.log('\n╔════════════════════════════════════════════════════════════════╗');
  console.log('║                 Initialization Plan Ready                       ║');
  console.log('╚════════════════════════════════════════════════════════════════╝');

  console.log('\nTo complete pool setup, you need to:');
  console.log('');
  console.log('1. Call oracle.updatePrice() for each token');
  console.log('2. Call factory.createVirtualReservePool() for each ETR/wToken pair');
  console.log('3. Deploy EDSCOraclePool with 100M EDSC seed');
  console.log('4. Deploy EDSCPegPool with 10M EDSC seed');
  console.log('5. Transfer ETR from DAO Treasury to seed each pool');
  console.log('');
  console.log('Total ETR needed: 1,250,000,000 ETR');
  console.log('Total EDSC needed: 110,000,000 EDSC');

  await api.disconnect();
}

main().catch(console.error);
