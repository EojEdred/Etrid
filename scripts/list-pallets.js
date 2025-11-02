#!/usr/bin/env node

const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
    const provider = new WsProvider('ws://163.192.125.23:9944');
    const api = await ApiPromise.create({ provider });

    console.log('Available pallets in runtime:');
    const metadata = await api.rpc.state.getMetadata();
    const pallets = metadata.asLatest.pallets;

    pallets.forEach((pallet) => {
        console.log(`- ${pallet.name.toString()}`);
    });

    console.log('\n🔍 Looking for ETR/Bridge related pallets...');
    pallets.forEach((pallet) => {
        const name = pallet.name.toString().toLowerCase();
        if (name.includes('etr') || name.includes('bridge') || name.includes('lock')) {
            console.log(`✅ Found: ${pallet.name.toString()}`);
        }
    });

    process.exit(0);
}

main().catch(console.error);
