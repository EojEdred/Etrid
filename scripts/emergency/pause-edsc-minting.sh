#!/bin/bash
set -e

# Script configuration
SCRIPT_NAME="$(basename "$0")"
CHAIN_ENDPOINT="${CHAIN_ENDPOINT:-ws://localhost:9944}"
LOG_FILE="${LOG_FILE:-/var/log/etrid/pause-edsc-minting.log}"

# Usage function
usage() {
    cat << EOF
Usage: $SCRIPT_NAME [options]

Emergency Pause EDSC Minting - Can be called by any single Director

Options:
  --help                Show this help message
  --endpoint URL        Chain endpoint (default: ws://localhost:9944)
  --signer ACCOUNT      Director account (required)
  --reason TEXT         Reason for pause (required)
  --dry-run             Simulate without executing
  --verbose             Enable verbose logging

Example:
  $SCRIPT_NAME \\
    --signer //Director1 \\
    --reason "Detected unauthorized minting activity"

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
REASON=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --help) usage ;;
        --endpoint) CHAIN_ENDPOINT="$2"; shift 2 ;;
        --signer) SIGNER="$2"; shift 2 ;;
        --reason) REASON="$2"; shift 2 ;;
        --dry-run) DRY_RUN=true; shift ;;
        --verbose) VERBOSE=true; shift ;;
        *) echo "Unknown option: $1"; usage ;;
    esac
done

# Validate inputs
if [[ -z "$SIGNER" ]] || [[ -z "$REASON" ]]; then
    log "ERROR" "Missing required parameters"
    usage
fi

# Main logic
main() {
    log "INFO" "Starting EDSC minting pause operation"
    log "INFO" "Endpoint: $CHAIN_ENDPOINT"
    log "INFO" "Signer: $SIGNER"
    log "INFO" "Reason: $REASON"

    if [[ "$DRY_RUN" == "true" ]]; then
        log "INFO" "DRY RUN MODE - No actual transactions will be submitted"
    fi

    # Create pause transaction
    local pause_script=$(cat << 'EOFJS'
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const process = require('process');

async function pauseEdscMinting() {
    const endpoint = process.env.CHAIN_ENDPOINT;
    const signer = process.env.SIGNER;
    const reason = process.env.REASON;
    const dryRun = process.env.DRY_RUN === 'true';

    console.log(`Connecting to ${endpoint}...`);
    const provider = new WsProvider(endpoint);
    const api = await ApiPromise.create({ provider });

    // Initialize keyring
    const keyring = new Keyring({ type: 'sr25519' });
    const account = keyring.addFromUri(signer);

    console.log(`Pausing EDSC minting with account: ${account.address}`);
    console.log(`Reason: ${reason}`);

    if (dryRun) {
        console.log('DRY RUN: Would call pallet_consensus_day.emergencyPauseMinting()');
        await api.disconnect();
        return;
    }

    // Create and submit pause transaction
    const tx = api.tx.consensusDay.emergencyPauseMinting();

    // Sign and send transaction
    await tx.signAndSend(account, ({ status, events }) => {
        if (status.isInBlock) {
            console.log(`Transaction included in block hash: ${status.asInBlock.toHex()}`);

            events.forEach(({ event }) => {
                if (event.method === 'MintingPaused') {
                    console.log('SUCCESS: EDSC minting has been paused');
                    console.log('IMPORTANT: All Consensus Day distributions are now halted');
                }
                if (event.method === 'ExtrinsicFailed') {
                    console.log('ERROR: Transaction failed');
                    const [dispatchError] = event.data;
                    if (dispatchError.isModule) {
                        const decoded = api.registry.findMetaError(dispatchError.asModule);
                        console.log(`Error: ${decoded.section}.${decoded.name}: ${decoded.docs}`);
                    }
                    process.exit(1);
                }
            });
        } else if (status.isFinalized) {
            console.log(`Transaction finalized at block hash: ${status.asFinalized.toHex()}`);
            console.log('\nNext steps:');
            console.log('1. Investigate the issue');
            console.log('2. To resume, use unpause-edsc-minting.sh (requires 7/9 signatures)');
            api.disconnect();
        }
    });
}

pauseEdscMinting().catch(err => {
    console.error('ERROR:', err);
    process.exit(1);
});
EOFJS
)

    export CHAIN_ENDPOINT SIGNER REASON DRY_RUN

    if command -v node &> /dev/null; then
        echo "$pause_script" | node || {
            log "ERROR" "Failed to pause EDSC minting"
            exit 1
        }
    else
        log "ERROR" "Node.js is not installed"
        exit 1
    fi

    log "INFO" "EDSC minting pause operation complete!"
    log "WARNING" "All Consensus Day distributions are now halted"
    log "INFO" "Documented reason: $REASON"
    log "INFO" "To resume, 7/9 Director signatures are required via unpause-edsc-minting.sh"
}

main "$@"
