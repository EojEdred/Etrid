#!/usr/bin/env node
/**
 * DID Resolver API Server
 *
 * RESTful API for resolving DIDs from on-chain registry
 *
 * Endpoints:
 *   GET  /api/did/:did_id          - Resolve single DID
 *   GET  /api/dids                 - List all DIDs
 *   GET  /api/health               - Health check
 *
 * Usage:
 *   npm install express cors
 *   node api/did_resolver_api.js
 */

const express = require('express');
const cors = require('cors');
const { ApiPromise, WsProvider } = require('@polkadot/api');
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

// Configuration
const PORT = process.env.DID_API_PORT || 3001;
const WS_ENDPOINT = process.env.BLOCKCHAIN_WS_URL || 'ws://localhost:9944';
const DIDS_DIR = path.join(__dirname, '../dids');

// Create Express app
const app = express();
app.use(cors());
app.use(express.json());

// Global API instance
let api = null;

/**
 * Hash DID identifier
 */
function hashDid(didIdentifier) {
    return crypto.createHash('blake2b256').update(didIdentifier).digest('hex');
}

/**
 * Load DID document from local file
 */
function loadDidDocumentLocal(didId) {
    const identity = didId.replace('did:etrid:', '');
    const filePath = path.join(DIDS_DIR, `${identity}.json`);

    if (!fs.existsSync(filePath)) {
        return null;
    }

    return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

/**
 * GET /api/health
 * Health check endpoint
 */
app.get('/api/health', async (req, res) => {
    try {
        if (!api) {
            return res.status(503).json({
                status: 'unhealthy',
                message: 'Blockchain connection not established'
            });
        }

        const chain = await api.rpc.system.chain();
        const blockNumber = await api.rpc.chain.getBlock();

        res.json({
            status: 'healthy',
            chain: chain.toString(),
            block: blockNumber.block.header.number.toString(),
            endpoint: WS_ENDPOINT,
            timestamp: new Date().toISOString()
        });
    } catch (error) {
        res.status(500).json({
            status: 'unhealthy',
            error: error.message
        });
    }
});

/**
 * GET /api/did/:did_id
 * Resolve a single DID
 */
app.get('/api/did/:did_id', async (req, res) => {
    try {
        const didIdentifier = req.params.did_id.startsWith('did:etrid:')
            ? req.params.did_id
            : `did:etrid:${req.params.did_id}`;

        console.log(`Resolving DID: ${didIdentifier}`);

        // Hash DID for lookup
        const didHash = hashDid(didIdentifier);

        // Query on-chain registration
        const registration = await api.query.didRegistry.registrations(`0x${didHash}`);

        if (registration.isNone) {
            return res.status(404).json({
                error: 'DID not found',
                did: didIdentifier
            });
        }

        const regData = registration.unwrap();

        // Check if revoked
        if (regData.revoked) {
            return res.status(410).json({
                error: 'DID revoked',
                did: didIdentifier,
                revoked: true
            });
        }

        // Load full DID document
        const didDoc = loadDidDocumentLocal(didIdentifier);

        res.json({
            did: didIdentifier,
            onChain: {
                owner: regData.owner.toString(),
                controller: regData.controller.toString(),
                documentHash: regData.documentHash.toHex(),
                registeredAt: regData.registeredAt.toString(),
                updatedAt: regData.updatedAt.toString(),
                revoked: regData.revoked.toString(),
            },
            document: didDoc,
            resolvedAt: new Date().toISOString()
        });

    } catch (error) {
        console.error('Error resolving DID:', error);
        res.status(500).json({
            error: 'Internal server error',
            message: error.message
        });
    }
});

/**
 * GET /api/dids
 * List all registered DIDs
 */
app.get('/api/dids', async (req, res) => {
    try {
        const totalDids = await api.query.didRegistry.totalDids();
        const entries = await api.query.didRegistry.registrations.entries();

        const dids = entries.map(([key, value]) => {
            const regData = value.unwrap();
            return {
                did: regData.did.toUtf8(),
                didHash: key.args[0].toHex(),
                owner: regData.owner.toString(),
                controller: regData.controller.toString(),
                registeredAt: regData.registeredAt.toString(),
                updatedAt: regData.updatedAt.toString(),
                revoked: regData.revoked.toString(),
            };
        });

        res.json({
            total: totalDids.toString(),
            count: dids.length,
            dids: dids
        });

    } catch (error) {
        console.error('Error listing DIDs:', error);
        res.status(500).json({
            error: 'Internal server error',
            message: error.message
        });
    }
});

/**
 * GET /api/stats
 * Get DID registry statistics
 */
app.get('/api/stats', async (req, res) => {
    try {
        const totalDids = await api.query.didRegistry.totalDids();
        const nonce = await api.query.didRegistry.nonce();

        res.json({
            totalDids: totalDids.toString(),
            nonce: nonce.toString(),
            timestamp: new Date().toISOString()
        });

    } catch (error) {
        console.error('Error getting stats:', error);
        res.status(500).json({
            error: 'Internal server error',
            message: error.message
        });
    }
});

/**
 * Initialize blockchain connection
 */
async function initBlockchain() {
    console.log(`Connecting to FlareChain at ${WS_ENDPOINT}...`);

    const provider = new WsProvider(WS_ENDPOINT);
    api = await ApiPromise.create({ provider });

    const chain = await api.rpc.system.chain();
    const version = api.runtimeVersion.specVersion.toString();

    console.log(`✅ Connected to ${chain} (runtime v${version})`);
}

/**
 * Start server
 */
async function startServer() {
    console.log('═══════════════════════════════════════════════════════');
    console.log('  Ëtrid AI Devs - DID Resolver API');
    console.log('═══════════════════════════════════════════════════════\n');

    try {
        // Initialize blockchain connection
        await initBlockchain();

        // Start Express server
        app.listen(PORT, () => {
            console.log(`\n✅ DID Resolver API running on port ${PORT}`);
            console.log(`\nEndpoints:`);
            console.log(`  GET  http://localhost:${PORT}/api/health`);
            console.log(`  GET  http://localhost:${PORT}/api/dids`);
            console.log(`  GET  http://localhost:${PORT}/api/did/:did_id`);
            console.log(`  GET  http://localhost:${PORT}/api/stats`);
            console.log('\nExample:');
            console.log(`  curl http://localhost:${PORT}/api/did/consensus-dev01\n`);
        });

    } catch (error) {
        console.error('❌ Failed to start server:', error);
        process.exit(1);
    }
}

// Handle graceful shutdown
process.on('SIGINT', async () => {
    console.log('\n\nShutting down...');
    if (api) {
        await api.disconnect();
    }
    process.exit(0);
});

// Start
startServer();
