#!/usr/bin/env node
/**
 * Resolve DID from on-chain registry and return full DID document
 *
 * Usage:
 *   node scripts/resolve_did.js did:etrid:consensus-dev01
 *   node scripts/resolve_did.js --all
 */

const { ApiPromise, WsProvider } = require('@polkadot/api');
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

// Configuration
const WS_ENDPOINT = process.env.BLOCKCHAIN_WS_URL || 'ws://localhost:9944';
const DIDS_DIR = path.join(__dirname, '../dids');

/**
 * Hash DID identifier (same as on-chain)
 */
function hashDid(didIdentifier) {
    return crypto.createHash('blake2b256').update(didIdentifier).digest('hex');
}

/**
 * Load DID document from local file
 */
function loadDidDocumentLocal(didId) {
    // Extract identity from DID (e.g., "did:etrid:consensus-dev01" → "consensus-dev01")
    const identity = didId.replace('did:etrid:', '');
    const filePath = path.join(DIDS_DIR, `${identity}.json`);

    if (!fs.existsSync(filePath)) {
        return null;
    }

    const data = fs.readFileSync(filePath, 'utf8');
    return JSON.parse(data);
}

/**
 * Resolve DID from blockchain
 */
async function resolveDid(api, didIdentifier) {
    console.log(`\nResolving DID: ${didIdentifier}`);

    // Hash DID for lookup
    const didHash = hashDid(didIdentifier);
    console.log(`  DID Hash: 0x${didHash.substring(0, 16)}...`);

    // Query on-chain registration
    const registration = await api.query.didRegistry.registrations(`0x${didHash}`);

    if (registration.isNone) {
        console.log(`  ❌ DID not found on-chain`);
        return null;
    }

    const regData = registration.unwrap();
    console.log(`  ✅ DID found on-chain`);
    console.log(`     Owner: ${regData.owner.toString()}`);
    console.log(`     Controller: ${regData.controller.toString()}`);
    console.log(`     Document Hash: ${regData.documentHash.toHex()}`);
    console.log(`     Registered at block: ${regData.registeredAt.toString()}`);
    console.log(`     Updated at block: ${regData.updatedAt.toString()}`);
    console.log(`     Revoked: ${regData.revoked.toString()}`);

    if (regData.revoked) {
        console.log(`  ⚠️  DID is revoked!`);
        return null;
    }

    // Load full DID document from local storage
    const didDoc = loadDidDocumentLocal(didIdentifier);

    if (!didDoc) {
        console.log(`  ⚠️  DID document not found locally`);
        return {
            id: didIdentifier,
            onChain: {
                owner: regData.owner.toString(),
                controller: regData.controller.toString(),
                documentHash: regData.documentHash.toHex(),
                registeredAt: regData.registeredAt.toString(),
                updatedAt: regData.updatedAt.toString(),
                revoked: regData.revoked.toString(),
            },
            document: null
        };
    }

    console.log(`  ✅ Full DID document loaded`);

    return {
        id: didIdentifier,
        onChain: {
            owner: regData.owner.toString(),
            controller: regData.controller.toString(),
            documentHash: regData.documentHash.toHex(),
            registeredAt: regData.registeredAt.toString(),
            updatedAt: regData.updatedAt.toString(),
            revoked: regData.revoked.toString(),
        },
        document: didDoc
    };
}

/**
 * List all registered DIDs
 */
async function listAllDids(api) {
    console.log('\nQuerying all registered DIDs...\n');

    const totalDids = await api.query.didRegistry.totalDids();
    console.log(`Total DIDs on-chain: ${totalDids.toString()}\n`);

    // Query all registrations
    const entries = await api.query.didRegistry.registrations.entries();

    console.log(`Found ${entries.length} DID entries:\n`);

    entries.forEach(([key, value]) => {
        const regData = value.unwrap();
        const didHash = key.args[0].toHex();

        console.log(`DID Hash: ${didHash}`);
        console.log(`  DID: ${regData.did.toUtf8()}`);
        console.log(`  Owner: ${regData.owner.toString()}`);
        console.log(`  Controller: ${regData.controller.toString()}`);
        console.log(`  Revoked: ${regData.revoked.toString()}`);
        console.log('');
    });
}

/**
 * Main function
 */
async function main() {
    const args = process.argv.slice(2);

    if (args.length === 0) {
        console.log('Usage:');
        console.log('  node resolve_did.js <did-identifier>');
        console.log('  node resolve_did.js --all\n');
        console.log('Examples:');
        console.log('  node resolve_did.js did:etrid:consensus-dev01');
        console.log('  node resolve_did.js --all');
        process.exit(1);
    }

    console.log('═══════════════════════════════════════════════════════');
    console.log('  Ëtrid AI Devs - DID Resolver');
    console.log('═══════════════════════════════════════════════════════\n');

    // Connect to blockchain
    console.log(`Connecting to FlareChain at ${WS_ENDPOINT}...`);
    const provider = new WsProvider(WS_ENDPOINT);
    const api = await ApiPromise.create({ provider });

    console.log(`✅ Connected to chain: ${(await api.rpc.system.chain()).toString()}`);

    if (args[0] === '--all') {
        await listAllDids(api);
    } else {
        const didIdentifier = args[0];
        const result = await resolveDid(api, didIdentifier);

        if (result) {
            console.log('\n═══════════════════════════════════════════════════════');
            console.log('  Resolved DID Document');
            console.log('═══════════════════════════════════════════════════════\n');
            console.log(JSON.stringify(result, null, 2));
        }
    }

    await api.disconnect();
    console.log('\n✅ Done!');
}

// Run
main()
    .catch((error) => {
        console.error('Fatal error:', error);
        process.exit(1);
    });
