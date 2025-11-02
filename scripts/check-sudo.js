#!/usr/bin/env node

const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
    const provider = new WsProvider('ws://163.192.125.23:9944');
    const api = await ApiPromise.create({ provider });

    console.log('🔍 Checking Sudo Account\n');

    const sudoKey = await api.query.sudo.key();

    if (sudoKey.isEmpty) {
        console.log('❌ No sudo key configured');
    } else {
        console.log('✅ Sudo Account:', sudoKey.toString());
    }

    process.exit(0);
}

main().catch(console.error);
