#!/usr/bin/env node

/**
 * FlareChain Runtime Upgrade v106 - GRANDPA Fix Deployment Script
 *
 * This script deploys the runtime upgrade to fix GRANDPA committee formation
 * from 1 to 10 validators.
 *
 * Usage:
 *   npm install @polkadot/api @polkadot/keyring
 *   node deploy-grandpa-fix.js [--endpoint ws://64.181.215.19:9944] [--sudo-uri //Alice]
 */

const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');
const fs = require('fs');
const path = require('path');

// Parse command line arguments
const args = process.argv.slice(2);
const endpointIndex = args.indexOf('--endpoint');
const sudoUriIndex = args.indexOf('--sudo-uri');

const ENDPOINT = endpointIndex !== -1 ? args[endpointIndex + 1] : 'ws://64.181.215.19:9944';
const SUDO_URI = sudoUriIndex !== -1 ? args[sudoUriIndex + 1] : '//Alice';
const WASM_PATH = path.join(__dirname, 'target/release/wbuild/flare-chain-runtime/flare_chain_runtime.compact.compressed.wasm');

console.log('╔════════════════════════════════════════════════════════════════╗');
console.log('║   FlareChain Runtime Upgrade v106 - GRANDPA Committee Fix    ║');
console.log('╚════════════════════════════════════════════════════════════════╝');
console.log('');

async function main() {
    console.log('🔗 Connecting to FlareChain...');
    console.log(`   Endpoint: ${ENDPOINT}`);

    const wsProvider = new WsProvider(ENDPOINT);
    const api = await ApiPromise.create({ provider: wsProvider });

    // Get chain info
    const chain = await api.rpc.system.chain();
    const version = await api.rpc.state.getRuntimeVersion();
    const health = await api.rpc.system.health();

    console.log('');
    console.log('📊 Chain Information:');
    console.log(`   Chain: ${chain}`);
    console.log(`   Current spec_version: ${version.specVersion.toString()}`);
    console.log(`   Peers: ${health.peers.toString()}`);
    console.log(`   Syncing: ${health.isSyncing.toString()}`);
    console.log('');

    // Check current GRANDPA authorities
    const currentAuthorities = await api.query.grandpa.authorities();
    console.log('🔍 Current GRANDPA Authorities:');
    console.log(`   Count: ${currentAuthorities.length}`);
    console.log('');

    // Load WASM blob
    console.log('📦 Loading WASM runtime blob...');
    if (!fs.existsSync(WASM_PATH)) {
        console.error(`❌ ERROR: WASM file not found at: ${WASM_PATH}`);
        console.error('   Please build the runtime first:');
        console.error('   cargo build --release -p flare-chain-runtime');
        process.exit(1);
    }

    const code = fs.readFileSync(WASM_PATH);
    console.log(`   Size: ${(code.length / 1024 / 1024).toFixed(2)} MB (${code.length} bytes)`);
    console.log('');

    // Initialize keyring
    console.log('🔐 Loading sudo key...');
    console.log(`   URI: ${SUDO_URI}`);
    const keyring = new Keyring({ type: 'sr25519' });
    const sudo = keyring.addFromUri(SUDO_URI);
    console.log(`   Address: ${sudo.address}`);
    console.log('');

    // Verify sudo permissions
    const sudoKey = await api.query.sudo.key();
    if (sudoKey.toString() !== sudo.address) {
        console.error(`❌ ERROR: Account ${sudo.address} is not the sudo key!`);
        console.error(`   Current sudo key: ${sudoKey.toString()}`);
        process.exit(1);
    }
    console.log('✅ Sudo key verified');
    console.log('');

    // Create runtime upgrade call
    console.log('🚀 Preparing runtime upgrade transaction...');
    const setCodeCall = api.tx.system.setCode(code);

    // Weight for runtime upgrade (2 seconds ref_time, 1MB proof_size)
    const weight = {
        refTime: 2_000_000_000,  // 2 seconds
        proofSize: 1_048_576     // 1 MB
    };

    const sudoCall = api.tx.sudo.sudoUncheckedWeight(setCodeCall, weight);
    console.log('   Transaction prepared');
    console.log('');

    // Ask for confirmation
    console.log('⚠️  WARNING: This will upgrade the runtime to spec_version 106');
    console.log('   This will update GRANDPA authorities from 1 to 10 validators');
    console.log('');
    console.log('Press Ctrl+C to abort, or wait 5 seconds to continue...');

    await new Promise(resolve => setTimeout(resolve, 5000));

    console.log('');
    console.log('📡 Submitting runtime upgrade transaction...');

    return new Promise((resolve, reject) => {
        sudoCall.signAndSend(sudo, ({ status, events, dispatchError }) => {
            console.log(`   Status: ${status.type}`);

            if (status.isInBlock) {
                console.log('');
                console.log('✅ Transaction included in block:', status.asInBlock.toHex());
                console.log('');
                console.log('📋 Events:');

                events.forEach(({ event }) => {
                    const { section, method, data } = event;
                    console.log(`   • ${section}.${method}`);

                    if (section === 'system' && method === 'ExtrinsicFailed') {
                        const [dispatchError] = data;
                        let errorInfo = dispatchError.toString();

                        if (dispatchError.isModule) {
                            const decoded = api.registry.findMetaError(dispatchError.asModule);
                            errorInfo = `${decoded.section}.${decoded.name}: ${decoded.docs}`;
                        }
                        console.error(`     ❌ Error: ${errorInfo}`);
                    }

                    if (section === 'sudo' && method === 'Sudid') {
                        const [result] = data;
                        if (result.isOk) {
                            console.log('     ✅ Sudo call succeeded');
                        } else {
                            console.error('     ❌ Sudo call failed:', result.asErr.toString());
                        }
                    }

                    if (section === 'system' && method === 'CodeUpdated') {
                        console.log('     ✅ Runtime code updated!');
                    }
                });

                console.log('');
            }

            if (status.isFinalized) {
                console.log('🎉 Transaction finalized in block:', status.asFinalized.toHex());
                console.log('');

                // Verify upgrade
                verifyUpgrade(api).then(() => {
                    resolve();
                    process.exit(0);
                }).catch(reject);
            }

            if (dispatchError) {
                console.error('❌ Transaction failed:', dispatchError.toString());
                reject(dispatchError);
            }
        }).catch(reject);
    });
}

async function verifyUpgrade(api) {
    console.log('🔍 Verifying runtime upgrade...');
    console.log('');

    // Wait a bit for the upgrade to take effect
    await new Promise(resolve => setTimeout(resolve, 3000));

    // Check new spec version
    const version = await api.rpc.state.getRuntimeVersion();
    console.log(`   Spec version: ${version.specVersion.toString()}`);

    if (version.specVersion.toNumber() === 106) {
        console.log('   ✅ Spec version updated to 106');
    } else {
        console.error(`   ❌ ERROR: Expected spec_version 106, got ${version.specVersion.toString()}`);
    }

    // Check GRANDPA authorities
    const authorities = await api.query.grandpa.authorities();
    console.log(`   GRANDPA authorities count: ${authorities.length}`);

    if (authorities.length === 10) {
        console.log('   ✅ GRANDPA committee updated to 10 validators');
    } else {
        console.error(`   ⚠️  WARNING: Expected 10 authorities, got ${authorities.length}`);
    }

    console.log('');
    console.log('╔════════════════════════════════════════════════════════════════╗');
    console.log('║              Runtime Upgrade Complete!                        ║');
    console.log('╚════════════════════════════════════════════════════════════════╝');
    console.log('');
    console.log('Next steps:');
    console.log('1. Monitor validator logs for GRANDPA participation');
    console.log('2. Check finalized block height is increasing');
    console.log('3. Verify all 10 validators are in GRANDPA rounds');
    console.log('');
}

main().catch(error => {
    console.error('');
    console.error('❌ Fatal error:', error);
    process.exit(1);
});
