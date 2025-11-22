#!/usr/bin/env node
/**
 * V26 ASF Key Registration Script
 * Registers ASF public keys to session pallet via setKeys() extrinsic
 * Usage: node v26-register-asf-keys.js --public-key <KEY> --validator-uri <URI>
 */

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');
const fs = require('fs');

// Colors for console output
const colors = {
    reset: '\x1b[0m',
    red: '\x1b[31m',
    green: '\x1b[32m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    cyan: '\x1b[36m'
};

function log(message, color = colors.reset) {
    console.log(`${color}${message}${colors.reset}`);
}

function parseArgs() {
    const args = process.argv.slice(2);
    const options = {
        publicKey: null,
        validatorUri: null,
        rpcEndpoint: 'ws://localhost:9944',
        dryRun: false,
        help: false
    };

    for (let i = 0; i < args.length; i++) {
        switch (args[i]) {
            case '--public-key':
                options.publicKey = args[++i];
                break;
            case '--validator-uri':
                options.validatorUri = args[++i];
                break;
            case '--rpc-endpoint':
                options.rpcEndpoint = args[++i];
                break;
            case '--dry-run':
                options.dryRun = true;
                break;
            case '--help':
            case '-h':
                options.help = true;
                break;
            default:
                log(`Unknown option: ${args[i]}`, colors.red);
                process.exit(1);
        }
    }

    return options;
}

function printHelp() {
    console.log(`
V26 ASF Key Registration Script

Usage: node v26-register-asf-keys.js [OPTIONS]

Options:
  --public-key KEY       ASF sr25519 public key (hex-encoded, with or without 0x prefix)
  --validator-uri URI    Validator account URI (e.g., //Alice or seed phrase)
  --rpc-endpoint URL     RPC endpoint (default: ws://localhost:9944)
  --dry-run              Simulate transaction without submitting
  --help, -h             Show this help message

Examples:
  # Register using validator seed
  node v26-register-asf-keys.js \\
    --public-key 0x1234...abcd \\
    --validator-uri "word1 word2 ... word12"

  # Register using dev account
  node v26-register-asf-keys.js \\
    --public-key 0x1234...abcd \\
    --validator-uri //Alice

  # Dry run to test
  node v26-register-asf-keys.js \\
    --public-key 0x1234...abcd \\
    --validator-uri //Alice \\
    --dry-run
`);
}

function validatePublicKey(publicKey) {
    if (!publicKey) {
        throw new Error('Public key is required');
    }

    // Remove 0x prefix if present
    const cleanKey = publicKey.startsWith('0x') ? publicKey.slice(2) : publicKey;

    // Verify it's a valid hex string
    if (!/^[0-9a-fA-F]{64}$/.test(cleanKey)) {
        throw new Error('Invalid public key format. Expected 64 hex characters (32 bytes)');
    }

    return '0x' + cleanKey;
}

async function checkValidatorBalance(api, validatorAddress) {
    const account = await api.query.system.account(validatorAddress);
    const balance = account.data.free.toBigInt();
    const existentialDeposit = api.consts.balances.existentialDeposit.toBigInt();
    const minRequired = existentialDeposit * 10n; // Require at least 10x existential deposit

    log(`\nValidator Balance Check:`, colors.cyan);
    log(`  Address: ${validatorAddress}`);
    log(`  Free Balance: ${balance.toString()} plancks`);
    log(`  Required: ~${minRequired.toString()} plancks (minimum)`);

    if (balance < minRequired) {
        throw new Error(`Insufficient balance. Need at least ${minRequired} plancks for transaction fees`);
    }

    return true;
}

async function getCurrentSessionKeys(api, validatorAddress) {
    try {
        const keys = await api.query.session.nextKeys(validatorAddress);
        if (keys.isSome) {
            log(`\nCurrent Session Keys Found:`, colors.yellow);
            const keysData = keys.unwrap();
            log(`  ${JSON.stringify(keysData.toHuman(), null, 2)}`);
            return keysData;
        } else {
            log(`\nNo existing session keys found for validator`, colors.yellow);
            return null;
        }
    } catch (error) {
        log(`Note: Could not query existing keys (this is normal for new validators)`, colors.yellow);
        return null;
    }
}

async function registerSessionKeys(options) {
    log('\n========================================', colors.blue);
    log('   V26 ASF Key Registration', colors.blue);
    log('========================================\n', colors.blue);

    try {
        // Validate inputs
        const publicKey = validatePublicKey(options.publicKey);

        if (!options.validatorUri) {
            throw new Error('Validator URI is required (use --validator-uri)');
        }

        log(`Step 1: Validating inputs...`, colors.green);
        log(`  ASF Public Key: ${publicKey}`);
        log(`  RPC Endpoint: ${options.rpcEndpoint}`);
        log(`  Dry Run: ${options.dryRun}`);

        // Initialize crypto
        log(`\nStep 2: Initializing crypto...`, colors.green);
        await cryptoWaitReady();

        // Connect to node
        log(`\nStep 3: Connecting to node...`, colors.green);
        const wsProvider = new WsProvider(options.rpcEndpoint);
        const api = await ApiPromise.create({ provider: wsProvider });

        await api.isReady;
        log(`  Connected to chain: ${(await api.rpc.system.chain()).toString()}`, colors.cyan);
        log(`  Runtime version: ${api.runtimeVersion.specVersion.toString()}`, colors.cyan);

        // Load validator account
        log(`\nStep 4: Loading validator account...`, colors.green);
        const keyring = new Keyring({ type: 'sr25519' });
        const validator = keyring.addFromUri(options.validatorUri);
        log(`  Validator address: ${validator.address}`);

        // Check balance
        await checkValidatorBalance(api, validator.address);

        // Check current keys
        await getCurrentSessionKeys(api, validator.address);

        // Prepare session keys structure
        log(`\nStep 5: Preparing session keys structure...`, colors.green);
        const sessionKeys = {
            asf: publicKey
        };
        log(`  Session Keys: ${JSON.stringify(sessionKeys, null, 2)}`);

        // Empty proof (not required for ASF keys)
        const proof = '0x';

        if (options.dryRun) {
            log(`\n${colors.yellow}DRY RUN MODE - Transaction not submitted${colors.reset}`);
            log(`\nTransaction would be:`, colors.cyan);
            log(`  Extrinsic: session.setKeys`);
            log(`  Keys: ${JSON.stringify(sessionKeys)}`);
            log(`  Proof: ${proof}`);
            log(`  Signer: ${validator.address}`);

            await api.disconnect();
            return;
        }

        // Submit transaction
        log(`\nStep 6: Submitting setKeys transaction...`, colors.green);

        const tx = api.tx.session.setKeys(sessionKeys, proof);

        // Get transaction fee estimate
        const paymentInfo = await tx.paymentInfo(validator);
        log(`  Estimated fee: ${paymentInfo.partialFee.toString()} plancks`, colors.cyan);

        log(`\nSubmitting transaction...`, colors.yellow);

        return new Promise((resolve, reject) => {
            let unsub;

            tx.signAndSend(validator, { nonce: -1 }, (result) => {
                log(`\nTransaction status: ${result.status.type}`, colors.cyan);

                if (result.status.isInBlock) {
                    log(`  Included in block: ${result.status.asInBlock.toString()}`, colors.cyan);
                }

                if (result.status.isFinalized) {
                    log(`  Finalized in block: ${result.status.asFinalized.toString()}`, colors.green);

                    // Check for errors
                    const errors = result.events
                        .filter(({ event }) => api.events.system.ExtrinsicFailed.is(event))
                        .map(({ event: { data: [error, info] } }) => {
                            if (error.isModule) {
                                const decoded = api.registry.findMetaError(error.asModule);
                                return `${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`;
                            }
                            return error.toString();
                        });

                    if (errors.length > 0) {
                        log(`\n${colors.red}Transaction failed with errors:${colors.reset}`);
                        errors.forEach(err => log(`  ${err}`, colors.red));
                        unsub();
                        api.disconnect();
                        reject(new Error(errors.join(', ')));
                        return;
                    }

                    // Success
                    log(`\n========================================`, colors.green);
                    log(`   Registration Successful!`, colors.green);
                    log(`========================================\n`, colors.green);

                    log(`Summary:`, colors.cyan);
                    log(`  Validator: ${validator.address}`);
                    log(`  ASF Public Key: ${publicKey}`);
                    log(`  Block: ${result.status.asFinalized.toString()}`);
                    log(`  Transaction Hash: ${tx.hash.toString()}`);

                    log(`\nNext Steps:`, colors.yellow);
                    log(`  1. Verify registration: node scripts/v26-verify-asf-keys.js --validator ${validator.address}`);
                    log(`  2. Restart validator node to load the key from keystore`);
                    log(`  3. Monitor node logs for ASF checkpoint signing`);

                    unsub();
                    api.disconnect();
                    resolve();
                }

                if (result.isError) {
                    log(`\nTransaction error`, colors.red);
                    unsub();
                    api.disconnect();
                    reject(new Error('Transaction failed'));
                }
            }).then(unsubFn => {
                unsub = unsubFn;
            }).catch(err => {
                log(`\nError submitting transaction: ${err.message}`, colors.red);
                api.disconnect();
                reject(err);
            });
        });

    } catch (error) {
        log(`\n${colors.red}Error: ${error.message}${colors.reset}`);
        log(`\nTroubleshooting:`, colors.yellow);
        log(`  1. Verify the public key is correct (64 hex characters)`);
        log(`  2. Ensure validator account has sufficient balance`);
        log(`  3. Check RPC endpoint is accessible`);
        log(`  4. Try with --dry-run to test parameters`);
        process.exit(1);
    }
}

// Main execution
(async () => {
    const options = parseArgs();

    if (options.help) {
        printHelp();
        process.exit(0);
    }

    await registerSessionKeys(options);
})();
