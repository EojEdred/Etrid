#!/usr/bin/env node

const { ApiPromise, WsProvider } = require('@polkadot/api');

const FLARECHAIN_WS_URL = process.env.FLARECHAIN_WS || 'ws://98.71.91.84:9944';

async function main() {
    console.log('🔍 Checking ETR Lock Status\n');

    const provider = new WsProvider(FLARECHAIN_WS_URL);
    const api = await ApiPromise.create({ provider });

    console.log(`📡 Connected to: ${await api.rpc.system.chain()}`);
    console.log(`   Version: ${await api.rpc.system.version()}\n`);

    // Check lock account
    const lockAccount = await api.query.etrLock.lockAccount();
    console.log('🔐 Lock Account:');
    if (lockAccount.isSome) {
        const address = lockAccount.unwrap().toString();
        console.log(`   Address: ${address}`);

        // Get balance
        const { data: balance } = await api.query.system.account(address);
        console.log(`   Balance: ${formatBalance(balance.free)} ETR`);
    } else {
        console.log('   ❌ Not configured');
    }

    // Check total locked
    const totalLocked = await api.query.etrLock.totalLocked();
    console.log(`\n💰 Total Locked: ${formatBalance(totalLocked)} ETR`);

    // Check per-chain locks
    console.log('\n📊 Locked per Chain:');
    const chainIds = {
        Base: 0,
        Arbitrum: 1,
        Optimism: 2,
        Polygon: 3,
        Ethereum: 10,
        BnbChain: 11,
        Avalanche: 12,
        Solana: 13,
        Bitcoin: 20,
        Cardano: 21,
        Stellar: 22,
        Ripple: 23,
        Dogecoin: 24,
        Tron: 25,
        Chainlink: 26,
        UsdtBridge: 30
    };

    for (const [name, id] of Object.entries(chainIds)) {
        const locked = await api.query.etrLock.lockedForChain(id);
        const amount = locked.toString();
        if (amount !== '0') {
            console.log(`   ${name.padEnd(12)}: ${formatBalance(locked)} ETR`);
        }
    }

    process.exit(0);
}

function formatBalance(balance) {
    const balanceStr = balance.toString();
    if (balanceStr === '0') return '0';
    if (balanceStr.length <= 18) {
        return `0.${balanceStr.padStart(18, '0')}`;
    }
    const whole = balanceStr.slice(0, -18);
    const decimal = balanceStr.slice(-18).replace(/0+$/, '');
    return decimal ? `${whole}.${decimal}` : whole;
}

main().catch(console.error);
