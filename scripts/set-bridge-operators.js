#!/usr/bin/env node

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

const FLARECHAIN_WS = process.env.FLARECHAIN_WS || 'ws://163.192.125.23:9944';
const SUDO_SEED = process.env.SUDO_SEED;
const OPERATOR_TYPE = process.env.OPERATOR_TYPE || 'dedicated'; // 'dedicated' or 'foundation'

const BRIDGES = [
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

async function main() {
    if (!SUDO_SEED) {
        console.error('❌ Error: SUDO_SEED environment variable not set');
        console.error('Usage: SUDO_SEED="your seed phrase" node set-bridge-operators.js');
        process.exit(1);
    }

    console.log('🚀 Setting Bridge Operators for FlareChain\n');

    await cryptoWaitReady();

    const provider = new WsProvider(FLARECHAIN_WS);
    const api = await ApiPromise.create({ provider });

    console.log(`📡 Connected to: ${await api.rpc.system.chain()}`);
    console.log(`   Version: ${await api.rpc.system.version()}\n`);

    const keyring = new Keyring({ type: 'sr25519' });

    // Load sudo account
    const sudo = keyring.addFromUri(SUDO_SEED);
    console.log(`🔑 Sudo Account: ${sudo.address}\n`);

    // Create or use operator account
    let operatorAccount;
    if (OPERATOR_TYPE === 'dedicated') {
        operatorAccount = keyring.addFromUri('//BridgeOperator');
        console.log('🔐 Created dedicated Bridge Operator account');
        console.log(`   Address: ${operatorAccount.address}`);
        console.log(`   ⚠️  SAVE THIS SEED: //BridgeOperator\n`);
    } else {
        operatorAccount = sudo;
        console.log('🔐 Using foundation account as bridge operator\n');
    }

    const operatorAddress = operatorAccount.address;

    // Set operator for each bridge
    console.log('📝 Setting bridge operators...\n');

    for (const bridge of BRIDGES) {
        try {
            if (api.tx[bridge] && api.tx[bridge].setOperator) {
                const bridgeName = bridge.replace('Bridge', '').toUpperCase();

                console.log(`Setting operator for ${bridgeName}...`);

                const tx = api.tx[bridge].setOperator(operatorAddress);
                const sudoTx = api.tx.sudo.sudo(tx);

                await new Promise((resolve, reject) => {
                    sudoTx.signAndSend(sudo, ({ status, events, dispatchError }) => {
                        if (dispatchError) {
                            if (dispatchError.isModule) {
                                const decoded = api.registry.findMetaError(dispatchError.asModule);
                                reject(new Error(`${decoded.section}.${decoded.name}: ${decoded.docs}`));
                            } else {
                                reject(new Error(dispatchError.toString()));
                            }
                        }

                        if (status.isInBlock) {
                            console.log(`  ✅ ${bridgeName} operator set (block: ${status.asInBlock.toHex().slice(0, 10)}...)`);
                            resolve();
                        }
                    });
                });

            } else {
                console.log(`  ⏭️  ${bridge}: setOperator not available`);
            }
        } catch (error) {
            console.error(`  ❌ ${bridge}: ${error.message}`);
        }
    }

    console.log('\n✅ Bridge operator setup complete!\n');

    // Verify
    console.log('🔍 Verifying bridge operators...\n');
    for (const bridge of BRIDGES) {
        try {
            if (api.query[bridge] && api.query[bridge].bridgeOperator) {
                const operator = await api.query[bridge].bridgeOperator();
                const bridgeName = bridge.replace('Bridge', '').toUpperCase();

                if (operator.isEmpty) {
                    console.log(`${bridgeName.padEnd(15)}: ❌ Not configured`);
                } else {
                    const address = operator.toString();
                    const isCorrect = address === operatorAddress;
                    console.log(`${bridgeName.padEnd(15)}: ${isCorrect ? '✅' : '⚠️ '} ${address}`);
                }
            }
        } catch (e) {
            console.log(`${bridge}: Error - ${e.message}`);
        }
    }

    console.log('\n📝 Next Steps:');
    console.log('1. Fund the bridge operator account with ETR for transaction fees');
    console.log(`   Address: ${operatorAddress}`);
    console.log('2. Deploy WrappedETR contracts to EVM chains');
    console.log('3. Configure and start bridge relayer service');
    console.log('4. Test bridge with small amounts\n');

    process.exit(0);
}

main().catch((error) => {
    console.error('Error:', error);
    process.exit(1);
});
