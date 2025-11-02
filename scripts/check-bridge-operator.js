#!/usr/bin/env node

const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
    const provider = new WsProvider('ws://163.192.125.23:9944');
    const api = await ApiPromise.create({ provider });

    console.log('🔍 Checking Bridge Operators\n');

    const bridges = [
        'ethereumBridge',
        'polygonBridge',
        'bnbBridge',
        'bitcoinBridge',
        'solanaBridge',
        'cardanoBridge',
        'stellarBridge',
        'xrpBridge',
        'dogeBridge',
        'tronBridge',
        'chainlinkBridge',
        'usdtBridge'
    ];

    for (const bridge of bridges) {
        try {
            if (api.query[bridge] && api.query[bridge].bridgeOperator) {
                const operator = await api.query[bridge].bridgeOperator();
                const bridgeName = bridge.replace('Bridge', '').toUpperCase();

                if (operator.isEmpty) {
                    console.log(`${bridgeName.padEnd(15)}: ❌ Not configured`);
                } else {
                    const address = operator.toString();
                    console.log(`${bridgeName.padEnd(15)}: ${address}`);
                }
            }
        } catch (e) {
            console.log(`${bridge}: Error - ${e.message}`);
        }
    }

    // Check available extrinsics
    console.log('\n🔧 Available EthereumBridge extrinsics:');
    if (api.tx.ethereumBridge) {
        const calls = Object.keys(api.tx.ethereumBridge);
        calls.forEach(call => console.log(`  - ${call}`));
    }

    process.exit(0);
}

main().catch(console.error);
