#!/bin/bash
set -e

# Script configuration
SCRIPT_NAME="$(basename "$0")"
CHAIN_ENDPOINT="${CHAIN_ENDPOINT:-ws://localhost:9944}"
LOG_FILE="${LOG_FILE:-/var/log/etrid/emergency-freeze.log}"

# Usage function
usage() {
    cat << EOF
Usage: $SCRIPT_NAME [options]

Emergency Treasury Freeze - Can be called by any single Director

Options:
  --help                Show this help message
  --endpoint URL        Chain endpoint (default: ws://localhost:9944)
  --signer ACCOUNT      Director account (required)
  --dry-run             Simulate without executing
  --verbose             Enable verbose logging

Environment Variables:
  CHAIN_ENDPOINT        WebSocket endpoint for the chain
  LOG_FILE              Log file path (default: /var/log/etrid/emergency-freeze.log)

Example:
  $SCRIPT_NAME --signer //Director1 --endpoint ws://mainnet.etrid.io:9944

EOF
    exit 1
}

# Logging function
log() {
    local level="$1"
    shift
    local message="$*"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] [$level] $message" | tee -a "$LOG_FILE"
}

# Parse arguments
DRY_RUN=false
VERBOSE=false
SIGNER=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --help) usage ;;
        --endpoint) CHAIN_ENDPOINT="$2"; shift 2 ;;
        --signer) SIGNER="$2"; shift 2 ;;
        --dry-run) DRY_RUN=true; shift ;;
        --verbose) VERBOSE=true; shift ;;
        *) echo "Unknown option: $1"; usage ;;
    esac
done

# Validate inputs
if [[ -z "$SIGNER" ]]; then
    log "ERROR" "Signer account is required"
    usage
fi

# Main logic
main() {
    log "INFO" "Starting treasury freeze operation"
    log "INFO" "Endpoint: $CHAIN_ENDPOINT"
    log "INFO" "Signer: $SIGNER"

    if [[ "$DRY_RUN" == "true" ]]; then
        log "INFO" "DRY RUN MODE - No actual transactions will be submitted"
    fi

    # Create freeze transaction using polkadot-js
    local freeze_script=$(cat << 'EOFJS'
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const process = require('process');

async function freezeTreasury() {
    const endpoint = process.env.CHAIN_ENDPOINT;
    const signer = process.env.SIGNER;
    const dryRun = process.env.DRY_RUN === 'true';

    console.log(`Connecting to ${endpoint}...`);
    const provider = new WsProvider(endpoint);
    const api = await ApiPromise.create({ provider });

    // Initialize keyring
    const keyring = new Keyring({ type: 'sr25519' });
    const account = keyring.addFromUri(signer);

    console.log(`Freezing treasury with account: ${account.address}`);

    if (dryRun) {
        console.log('DRY RUN: Would call pallet_treasury.emergencyFreezeTreasury()');
        await api.disconnect();
        return;
    }

    // Create and submit freeze transaction
    const tx = api.tx.treasury.emergencyFreezeTreasury();

    // Sign and send transaction
    const hash = await tx.signAndSend(account, ({ status, events }) => {
        if (status.isInBlock) {
            console.log(`Transaction included in block hash: ${status.asInBlock.toHex()}`);

            events.forEach(({ event }) => {
                if (event.method === 'TreasuryFrozen') {
                    console.log('SUCCESS: Treasury has been frozen');
                }
                if (event.method === 'ExtrinsicFailed') {
                    console.log('ERROR: Transaction failed');
                    process.exit(1);
                }
            });
        } else if (status.isFinalized) {
            console.log(`Transaction finalized at block hash: ${status.asFinalized.toHex()}`);
            api.disconnect();
        }
    });
}

freezeTreasury().catch(err => {
    console.error('ERROR:', err);
    process.exit(1);
});
EOFJS
)

    # Execute the Node.js script
    export CHAIN_ENDPOINT SIGNER DRY_RUN

    if command -v node &> /dev/null; then
        echo "$freeze_script" | node || {
            log "ERROR" "Failed to freeze treasury"
            exit 1
        }
    else
        log "ERROR" "Node.js is not installed. Please install Node.js and @polkadot/api"
        exit 1
    fi

    log "INFO" "Treasury freeze operation complete!"
    log "INFO" "IMPORTANT: Notify all Directors immediately"
    log "INFO" "To unfreeze, 7/9 Director signatures are required via unfreeze-treasury.sh"
}

main "$@"
