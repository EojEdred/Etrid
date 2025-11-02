#!/usr/bin/env node

const { Keyring } = require('@polkadot/keyring');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

const SUDO_ADDRESS = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

// Common test seeds
const testSeeds = [
    { name: 'Alice', seed: '//Alice' },
    { name: 'Bob', seed: '//Bob' },
    { name: 'Charlie', seed: '//Charlie' },
    { name: 'Dave', seed: '//Dave' },
    { name: 'Eve', seed: '//Eve' },
    { name: 'Ferdie', seed: '//Ferdie' },
    { name: 'Foundation', seed: '//Foundation' },
    { name: 'Sudo', seed: '//Sudo' },
    {
        name: 'EOJ_SESSION',
        seed: 'outer critic holiday path welcome edge awful clap amazing banner slow hurt'
    },
    {
        name: 'EOJ_PAYMENT',
        seed: 'another shove casino erase metal clutch jungle region annual obtain good initial'
    }
];

async function main() {
    await cryptoWaitReady();
    const keyring = new Keyring({ type: 'sr25519' });

    console.log('🔍 Identifying Sudo Account\n');
    console.log(`Target: ${SUDO_ADDRESS}\n`);

    for (const { name, seed } of testSeeds) {
        try {
            const account = keyring.addFromUri(seed);
            const match = account.address === SUDO_ADDRESS;

            if (match) {
                console.log(`✅ MATCH FOUND: ${name}`);
                console.log(`   Seed: ${seed}`);
                console.log(`   Address: ${account.address}\n`);
                process.exit(0);
            } else {
                console.log(`  ${name}: ${account.address}`);
            }
        } catch (e) {
            console.log(`  ${name}: Error - ${e.message}`);
        }
    }

    console.log('\n❌ No match found in test seeds');
    console.log('Try checking the full .env.mainnet file for other seeds\n');
}

main().catch(console.error);
