#!/usr/bin/env node

const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
    const provider = new WsProvider('ws://163.192.125.23:9944');
    const api = await ApiPromise.create({ provider });

    console.log('🔍 Inspecting EthereumBridge Pallet\n');

    const metadata = await api.rpc.state.getMetadata();
    const pallets = metadata.asLatest.pallets;

    const ethBridge = pallets.find(p => p.name.toString() === 'EthereumBridge');

    if (!ethBridge) {
        console.error('❌ EthereumBridge pallet not found');
        process.exit(1);
    }

    console.log('📦 Storage Items:');
    if (ethBridge.storage.isSome) {
        const storage = ethBridge.storage.unwrap();
        storage.items.forEach(item => {
            console.log(`  - ${item.name.toString()}`);
            console.log(`    Type: ${item.type.toString()}`);
        });
    }

    console.log('\n🔧 Extrinsics (calls):');
    if (ethBridge.calls.isSome) {
        const calls = ethBridge.calls.unwrap();
        calls.forEach(call => {
            console.log(`  - ${call.name.toString()}`);
        });
    }

    console.log('\n📡 Events:');
    if (ethBridge.events.isSome) {
        const events = ethBridge.events.unwrap();
        events.forEach(event => {
            console.log(`  - ${event.name.toString()}`);
        });
    }

    // Try to query if there's a lock account
    console.log('\n🔐 Checking for lock-related storage...');
    try {
        if (api.query.ethereumBridge) {
            const queryKeys = Object.keys(api.query.ethereumBridge);
            console.log('Available queries:', queryKeys);
        }
    } catch (e) {
        console.error('Error:', e.message);
    }

    process.exit(0);
}

main().catch(console.error);
