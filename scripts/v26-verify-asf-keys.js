#!/usr/bin/env node
/**
 * V26 ASF Key Verification Script
 * Verifies ASF keys are correctly stored in session pallet
 * Usage: node v26-verify-asf-keys.js [--validator ADDRESS] [--all]
 */

const { ApiPromise, WsProvider } = require('@polkadot/api');
const fs = require('fs');

// Colors for console output
const colors = {
    reset: '\x1b[0m',
    red: '\x1b[31m',
    green: '\x1b[32m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    cyan: '\x1b[36m',
    magenta: '\x1b[35m'
};

function log(message, color = colors.reset) {
    console.log(`${color}${message}${colors.reset}`);
}

function parseArgs() {
    const args = process.argv.slice(2);
    const options = {
        validator: null,
        all: false,
        rpcEndpoint: 'ws://localhost:9944',
        exportJson: false,
        outputFile: null,
        help: false
    };

    for (let i = 0; i < args.length; i++) {
        switch (args[i]) {
            case '--validator':
                options.validator = args[++i];
                break;
            case '--all':
                options.all = true;
                break;
            case '--rpc-endpoint':
                options.rpcEndpoint = args[++i];
                break;
            case '--export-json':
                options.exportJson = true;
                options.outputFile = args[++i] || 'asf-keys-export.json';
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
V26 ASF Key Verification Script

Usage: node v26-verify-asf-keys.js [OPTIONS]

Options:
  --validator ADDRESS    Verify specific validator by address
  --all                  Verify all validators in the current session
  --rpc-endpoint URL     RPC endpoint (default: ws://localhost:9944)
  --export-json [FILE]   Export results to JSON file (default: asf-keys-export.json)
  --help, -h             Show this help message

Examples:
  # Verify specific validator
  node v26-verify-asf-keys.js --validator 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

  # Verify all validators
  node v26-verify-asf-keys.js --all

  # Export to JSON
  node v26-verify-asf-keys.js --all --export-json results.json
`);
}

async function verifyValidator(api, validatorAddress) {
    log(`\nVerifying validator: ${validatorAddress}`, colors.cyan);

    try {
        // Query NextKeys from session pallet
        const nextKeys = await api.query.session.nextKeys(validatorAddress);

        if (nextKeys.isNone) {
            log(`  Status: ${colors.yellow}No session keys registered${colors.reset}`);
            return {
                address: validatorAddress,
                hasKeys: false,
                asfKey: null,
                status: 'no_keys'
            };
        }

        const keysData = nextKeys.unwrap();
        const asfKey = keysData.asf || keysData.toJSON().asf;

        log(`  Status: ${colors.green}Keys registered${colors.reset}`);
        log(`  ASF Key: ${colors.blue}${asfKey}${colors.reset}`);

        // Validate ASF key format
        const asfKeyStr = asfKey.toString();
        const isValidFormat = /^0x[0-9a-fA-F]{64}$/.test(asfKeyStr);

        if (!isValidFormat) {
            log(`  Warning: ${colors.yellow}Invalid ASF key format${colors.reset}`);
            return {
                address: validatorAddress,
                hasKeys: true,
                asfKey: asfKeyStr,
                status: 'invalid_format',
                valid: false
            };
        }

        log(`  Validation: ${colors.green}✓ Valid format${colors.reset}`);

        return {
            address: validatorAddress,
            hasKeys: true,
            asfKey: asfKeyStr,
            status: 'valid',
            valid: true
        };

    } catch (error) {
        log(`  Error: ${colors.red}${error.message}${colors.reset}`);
        return {
            address: validatorAddress,
            hasKeys: false,
            asfKey: null,
            status: 'error',
            error: error.message
        };
    }
}

async function getAllValidators(api) {
    log(`\nQuerying all validators...`, colors.cyan);

    try {
        // Get current session validators
        const validators = await api.query.session.validators();
        const validatorList = validators.toJSON();

        log(`  Found ${validatorList.length} validators in current session`, colors.green);

        return validatorList;
    } catch (error) {
        log(`  Error querying validators: ${error.message}`, colors.red);
        return [];
    }
}

async function verifyAllValidators(api) {
    const validators = await getAllValidators(api);

    if (validators.length === 0) {
        log(`\nNo validators found`, colors.yellow);
        return [];
    }

    log(`\n${'='.repeat(60)}`, colors.blue);
    log(`Verifying ${validators.length} validators...`, colors.blue);
    log(`${'='.repeat(60)}\n`, colors.blue);

    const results = [];

    for (let i = 0; i < validators.length; i++) {
        const validator = validators[i];
        log(`\n[${i + 1}/${validators.length}]`, colors.magenta);

        const result = await verifyValidator(api, validator);
        results.push(result);

        // Small delay to avoid overwhelming the node
        await new Promise(resolve => setTimeout(resolve, 100));
    }

    return results;
}

function printSummary(results) {
    log(`\n${'='.repeat(60)}`, colors.blue);
    log(`Verification Summary`, colors.blue);
    log(`${'='.repeat(60)}\n`, colors.blue);

    const total = results.length;
    const valid = results.filter(r => r.status === 'valid').length;
    const noKeys = results.filter(r => r.status === 'no_keys').length;
    const invalid = results.filter(r => r.status === 'invalid_format').length;
    const errors = results.filter(r => r.status === 'error').length;

    log(`Total Validators:     ${colors.cyan}${total}${colors.reset}`);
    log(`Valid ASF Keys:       ${colors.green}${valid}${colors.reset}`);
    log(`No Keys Registered:   ${colors.yellow}${noKeys}${colors.reset}`);
    log(`Invalid Format:       ${colors.red}${invalid}${colors.reset}`);
    log(`Errors:               ${colors.red}${errors}${colors.reset}`);

    // List validators without keys
    if (noKeys > 0) {
        log(`\n${colors.yellow}Validators without ASF keys:${colors.reset}`);
        results
            .filter(r => r.status === 'no_keys')
            .forEach(r => log(`  ${r.address}`, colors.yellow));
    }

    // List validators with invalid keys
    if (invalid > 0) {
        log(`\n${colors.red}Validators with invalid ASF keys:${colors.reset}`);
        results
            .filter(r => r.status === 'invalid_format')
            .forEach(r => log(`  ${r.address} - ${r.asfKey}`, colors.red));
    }

    // List errors
    if (errors > 0) {
        log(`\n${colors.red}Validators with errors:${colors.reset}`);
        results
            .filter(r => r.status === 'error')
            .forEach(r => log(`  ${r.address} - ${r.error}`, colors.red));
    }

    // Health status
    log(`\nHealth Status:`, colors.cyan);
    const healthPercentage = total > 0 ? ((valid / total) * 100).toFixed(2) : 0;
    const healthColor = healthPercentage === 100 ? colors.green :
                       healthPercentage >= 80 ? colors.yellow :
                       colors.red;
    log(`  ${healthColor}${healthPercentage}% validators ready${colors.reset}`);

    if (healthPercentage === 100) {
        log(`\n${colors.green}✓ All validators have valid ASF keys!${colors.reset}`);
    } else if (healthPercentage >= 80) {
        log(`\n${colors.yellow}⚠ Most validators ready, but some need attention${colors.reset}`);
    } else {
        log(`\n${colors.red}✗ Many validators need ASF key registration${colors.reset}`);
    }
}

function exportToJson(results, filename) {
    const exportData = {
        timestamp: new Date().toISOString(),
        totalValidators: results.length,
        summary: {
            valid: results.filter(r => r.status === 'valid').length,
            noKeys: results.filter(r => r.status === 'no_keys').length,
            invalid: results.filter(r => r.status === 'invalid_format').length,
            errors: results.filter(r => r.status === 'error').length
        },
        validators: results
    };

    fs.writeFileSync(filename, JSON.stringify(exportData, null, 2));
    log(`\nResults exported to: ${colors.blue}${filename}${colors.reset}`);
}

async function verify(options) {
    log('\n========================================', colors.blue);
    log('   V26 ASF Key Verification', colors.blue);
    log('========================================\n', colors.blue);

    try {
        // Connect to node
        log(`Connecting to node...`, colors.cyan);
        log(`  RPC Endpoint: ${options.rpcEndpoint}`);

        const wsProvider = new WsProvider(options.rpcEndpoint);
        const api = await ApiPromise.create({ provider: wsProvider });

        await api.isReady;
        log(`  Connected to chain: ${(await api.rpc.system.chain()).toString()}`, colors.green);
        log(`  Runtime version: ${api.runtimeVersion.specVersion.toString()}`, colors.green);

        let results = [];

        if (options.validator) {
            // Verify single validator
            const result = await verifyValidator(api, options.validator);
            results = [result];
        } else if (options.all) {
            // Verify all validators
            results = await verifyAllValidators(api);
        } else {
            log(`\nError: Must specify --validator or --all`, colors.red);
            printHelp();
            await api.disconnect();
            process.exit(1);
        }

        // Print summary
        if (results.length > 1) {
            printSummary(results);
        }

        // Export to JSON if requested
        if (options.exportJson) {
            exportToJson(results, options.outputFile);
        }

        log(`\n${colors.yellow}Next Steps:${colors.reset}`);
        if (results.some(r => r.status === 'no_keys')) {
            log(`  1. Register missing ASF keys: ${colors.blue}node scripts/v26-register-asf-keys.js${colors.reset}`);
            log(`  2. Or batch register: ${colors.blue}./scripts/v26-batch-register-validators.sh${colors.reset}`);
        }
        log(`  3. Monitor checkpoint signing in validator logs`);
        log(`  4. Verify ASF signatures are being generated`);

        await api.disconnect();

    } catch (error) {
        log(`\n${colors.red}Error: ${error.message}${colors.reset}`);
        log(`\nTroubleshooting:`, colors.yellow);
        log(`  1. Check RPC endpoint is accessible`);
        log(`  2. Verify node is running and synced`);
        log(`  3. Check network connectivity`);
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

    await verify(options);
})();
