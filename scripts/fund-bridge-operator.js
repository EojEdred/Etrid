#!/usr/bin/env node

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

const FLARECHAIN_WS = 'ws://163.192.125.23:9944';
const OPERATOR_ADDRESS = '5GZaEegZ4nUUeg9X6xUe5pdPgSnntdakSuykoNNr2FTsuL3m';
const AMOUNT = '1000'; // ETR

async function main() {
    console.log('💰 Funding Bridge Operator Account\n');

    await cryptoWaitReady();

    const provider = new WsProvider(FLARECHAIN_WS);
    const api = await ApiPromise.create({ provider });

    console.log(`📡 Connected to: ${await api.rpc.system.chain()}\n`);

    const keyring = new Keyring({ type: 'sr25519' });

    // Load Alice (sudo) account
    const alice = keyring.addFromUri('//Alice');
    console.log(`🔑 From: ${alice.address} (Alice - Sudo)`);
    console.log(`🎯 To: ${OPERATOR_ADDRESS} (Bridge Operator)`);
    console.log(`💸 Amount: ${AMOUNT} ETR\n`);

    // Check Alice balance
    const { data: aliceBalance } = await api.query.system.account(alice.address);
    console.log(`💰 Alice Balance: ${formatBalance(aliceBalance.free)} ETR`);

    // Convert amount to planck (18 decimals)
    const amountPlanck = BigInt(AMOUNT) * BigInt(10 ** 18);

    console.log('\n📤 Sending transfer...');

    const transfer = api.tx.balances.transferKeepAlive(OPERATOR_ADDRESS, amountPlanck);

    await new Promise((resolve, reject) => {
        transfer.signAndSend(alice, ({ status, events, dispatchError }) => {
            if (dispatchError) {
                if (dispatchError.isModule) {
                    const decoded = api.registry.findMetaError(dispatchError.asModule);
                    reject(new Error(`${decoded.section}.${decoded.name}: ${decoded.docs}`));
                } else {
                    reject(new Error(dispatchError.toString()));
                }
            }

            if (status.isInBlock) {
                console.log(`✅ Transfer in block: ${status.asInBlock.toHex().slice(0, 10)}...`);
            }

            if (status.isFinalized) {
                console.log(`✅ Transfer finalized: ${status.asFinalized.toHex().slice(0, 10)}...\n`);
                resolve();
            }
        });
    });

    // Check operator balance
    const { data: operatorBalance } = await api.query.system.account(OPERATOR_ADDRESS);
    console.log(`💰 Bridge Operator Balance: ${formatBalance(operatorBalance.free)} ETR`);

    console.log('\n✅ Bridge operator funded successfully!\n');

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

main().catch((error) => {
    console.error('Error:', error);
    process.exit(1);
});
