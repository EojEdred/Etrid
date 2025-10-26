#!/usr/bin/env node
/**
 * Register all AI Dev DIDs on-chain via OpenDID pallet
 *
 * Prerequisites:
 * - FlareChain node running at ws://localhost:9944
 * - Gizzi account funded with ETR for transaction fees
 * - All DID documents generated in /dids/
 *
 * Usage:
 *   npm install @polkadot/api
 *   node scripts/register_dids.js
 */

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

// Configuration
const WS_ENDPOINT = process.env.BLOCKCHAIN_WS_URL || 'ws://localhost:9944';
const DIDS_DIR = path.join(__dirname, '../dids');
const KEYPAIRS_FILE = path.join(DIDS_DIR, 'keypairs.json');

// AI Dev identities to register
const IDENTITIES = [
    'consensus-dev01',
    'compiler-dev01',
    'governance-dev01',
    'audit-dev01',
    'oracle-dev01',
    'runtime-dev01',
    'economics-dev01',
    'edsc-dev01',
    'security-dev01',
    'multichain-dev01',
    'ethics-dev01',
    'docs-dev01',
    'gizzi',
    'gizzi-claude',
    'gizzi-claudecode',
];

/**
 * Load DID document from file
 */
function loadDidDocument(identity) {
    const filePath = path.join(DIDS_DIR, `${identity}.json`);
    const data = fs.readFileSync(filePath, 'utf8');
    return JSON.parse(data);
}

/**
 * Calculate hash of DID document (for on-chain storage)
 */
function hashDidDocument(didDoc) {
    const docString = JSON.stringify(didDoc);
    return crypto.createHash('sha256').update(docString).digest('hex');
}

/**
 * Register a single DID on-chain
 */
async function registerDid(api, ownerKeyring, didDoc, documentHash) {
    const didIdentifier = didDoc.id; // e.g., "did:etrid:consensus-dev01"
    const controller = didDoc.controller; // e.g., "did:etrid:gizzi"

    console.log(`\nRegistering DID: ${didIdentifier}`);
    console.log(`  Controller: ${controller}`);
    console.log(`  Document Hash: ${documentHash.substring(0, 16)}...`);

    // For now, use owner as controller account
    // TODO: Map DID controller to actual account address
    const controllerAccount = ownerKeyring.address;

    try {
        // Create extrinsic
        const tx = api.tx.didRegistry.registerDid(
            didIdentifier,
            controllerAccount,
            `0x${documentHash}`
        );

        // Sign and send
        const hash = await tx.signAndSend(ownerKeyring);

        console.log(`  ✅ Transaction submitted: ${hash.toHex()}`);
        console.log(`  Waiting for inclusion...`);

        // Wait for finalization
        await new Promise((resolve) => {
            tx.signAndSend(ownerKeyring, ({ status, events }) => {
                if (status.isInBlock) {
                    console.log(`  ✅ Included in block: ${status.asInBlock.toHex()}`);
                }

                if (status.isFinalized) {
                    console.log(`  ✅ Finalized in block: ${status.asFinalized.toHex()}`);

                    // Check events for success
                    events.forEach(({ event }) => {
                        if (api.events.didRegistry.DidRegistered.is(event)) {
                            const [didHash, owner] = event.data;
                            console.log(`  ✅ DID registered successfully!`);
                            console.log(`     DID Hash: ${didHash.toHex()}`);
                            console.log(`     Owner: ${owner.toString()}`);
                        }

                        if (api.events.system.ExtrinsicFailed.is(event)) {
                            console.error(`  ❌ Registration failed`);
                        }
                    });

                    resolve();
                }
            });
        });

        return true;
    } catch (error) {
        console.error(`  ❌ Error registering DID: ${error.message}`);
        return false;
    }
}

/**
 * Main registration function
 */
async function main() {
    console.log('═══════════════════════════════════════════════════════');
    console.log('  Ëtrid AI Devs - DID Registration Script');
    console.log('═══════════════════════════════════════════════════════\n');

    // Connect to blockchain
    console.log(`Connecting to FlareChain at ${WS_ENDPOINT}...`);
    const provider = new WsProvider(WS_ENDPOINT);
    const api = await ApiPromise.create({ provider });

    console.log(`✅ Connected to chain: ${(await api.rpc.system.chain()).toString()}`);
    console.log(`   Runtime version: ${api.runtimeVersion.specVersion.toString()}`);
    console.log(`   Current block: ${(await api.rpc.chain.getBlock()).block.header.number.toString()}\n`);

    // Load keypairs
    console.log('Loading keypairs...');
    const keypairs = JSON.parse(fs.readFileSync(KEYPAIRS_FILE, 'utf8'));
    console.log(`✅ Loaded ${keypairs.length} keypairs\n`);

    // Set up Gizzi's keyring (owner/controller for all DIDs)
    // TODO: Replace with actual Gizzi account
    const keyring = new Keyring({ type: 'sr25519' });

    // For demo, use Alice account (replace with actual funded account)
    const ownerKeyring = keyring.addFromUri('//Alice');
    console.log(`Owner account: ${ownerKeyring.address}`);

    // Check balance
    const { data: balance } = await api.query.system.account(ownerKeyring.address);
    console.log(`Balance: ${balance.free.toString()} units\n`);

    if (balance.free.toBigInt() === 0n) {
        console.error('❌ Owner account has no balance! Fund the account first.');
        process.exit(1);
    }

    // Register all DIDs
    console.log(`Registering ${IDENTITIES.length} DIDs...\n`);

    let successCount = 0;
    let failCount = 0;

    for (const identity of IDENTITIES) {
        try {
            // Load DID document
            const didDoc = loadDidDocument(identity);

            // Calculate document hash
            const documentHash = hashDidDocument(didDoc);

            // Register on-chain
            const success = await registerDid(api, ownerKeyring, didDoc, documentHash);

            if (success) {
                successCount++;
            } else {
                failCount++;
            }

            // Small delay between registrations
            await new Promise(resolve => setTimeout(resolve, 1000));

        } catch (error) {
            console.error(`\n❌ Error processing ${identity}: ${error.message}\n`);
            failCount++;
        }
    }

    // Summary
    console.log('\n═══════════════════════════════════════════════════════');
    console.log('  Registration Summary');
    console.log('═══════════════════════════════════════════════════════');
    console.log(`Total DIDs: ${IDENTITIES.length}`);
    console.log(`✅ Success: ${successCount}`);
    console.log(`❌ Failed: ${failCount}`);
    console.log('═══════════════════════════════════════════════════════\n');

    // Query registered DIDs
    console.log('Querying registered DIDs from chain...\n');
    const totalDids = await api.query.didRegistry.totalDids();
    console.log(`Total DIDs registered on-chain: ${totalDids.toString()}`);

    // Disconnect
    await api.disconnect();
    console.log('\n✅ Done!');
}

// Run
main()
    .catch((error) => {
        console.error('Fatal error:', error);
        process.exit(1);
    });
