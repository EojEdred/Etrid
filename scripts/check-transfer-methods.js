#!/usr/bin/env node

const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
    const provider = new WsProvider('ws://163.192.125.23:9944');
    const api = await ApiPromise.create({ provider });

    console.log('🔍 Checking available transfer methods\n');

    // Check balances pallet
    if (api.tx.balances) {
        console.log('Balances pallet methods:');
        Object.keys(api.tx.balances).forEach(method => {
            console.log(`  - balances.${method}`);
        });
    }

    // Check other common transfer pallets
    const transferPallets = ['balances', 'currencies', 'tokens', 'assets'];

    console.log('\nSearching for transfer-related methods...');
    for (const pallet of transferPallets) {
        if (api.tx[pallet]) {
            const methods = Object.keys(api.tx[pallet]);
            const transferMethods = methods.filter(m => m.toLowerCase().includes('transfer'));
            if (transferMethods.length > 0) {
                console.log(`\n${pallet}:`);
                transferMethods.forEach(m => console.log(`  ✅ ${pallet}.${m}`));
            }
        }
    }

    process.exit(0);
}

main().catch(console.error);
