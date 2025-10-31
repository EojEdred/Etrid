#!/bin/bash
set -e

# Script configuration
SCRIPT_NAME="$(basename "$0")"
CHAIN_ENDPOINT="${CHAIN_ENDPOINT:-ws://localhost:9944}"
LOG_FILE="${LOG_FILE:-/var/log/etrid/emergency-unfreeze.log}"

# Usage function
usage() {
    cat << EOF
Usage: $SCRIPT_NAME [options]

Emergency Treasury Unfreeze - Requires 7/9 Director Signatures

Options:
  --help                Show this help message
  --endpoint URL        Chain endpoint (default: ws://localhost:9944)
  --multisig ACCOUNT    Multisig account address (required)
  --threshold N         Signature threshold (default: 7)
  --dry-run             Simulate without executing
  --verbose             Enable verbose logging

Example:
  $SCRIPT_NAME --multisig 5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z

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
MULTISIG=""
THRESHOLD=7

while [[ $# -gt 0 ]]; do
    case $1 in
        --help) usage ;;
        --endpoint) CHAIN_ENDPOINT="$2"; shift 2 ;;
        --multisig) MULTISIG="$2"; shift 2 ;;
        --threshold) THRESHOLD="$2"; shift 2 ;;
        --dry-run) DRY_RUN=true; shift ;;
        --verbose) VERBOSE=true; shift ;;
        *) echo "Unknown option: $1"; usage ;;
    esac
done

# Validate inputs
if [[ -z "$MULTISIG" ]]; then
    log "ERROR" "Multisig account address is required"
    usage
fi

# Main logic
main() {
    log "INFO" "Starting treasury unfreeze operation"
    log "INFO" "Endpoint: $CHAIN_ENDPOINT"
    log "INFO" "Multisig: $MULTISIG"
    log "INFO" "Required signatures: $THRESHOLD/9"

    if [[ "$DRY_RUN" == "true" ]]; then
        log "INFO" "DRY RUN MODE - No actual transactions will be submitted"
    fi

    log "INFO" "Step 1: Create multisig proposal for unfreeze"
    log "INFO" "Step 2: Collect $THRESHOLD Director signatures using collect-signatures.sh"
    log "INFO" "Step 3: Execute multisig transaction using execute-multisig.sh"

    # Create unfreeze proposal
    local unfreeze_script=$(cat << 'EOFJS'
const { ApiPromise, WsProvider } = require('@polkadot/api');
const process = require('process');

async function createUnfreezeProposal() {
    const endpoint = process.env.CHAIN_ENDPOINT;
    const multisig = process.env.MULTISIG;
    const dryRun = process.env.DRY_RUN === 'true';

    console.log(`Connecting to ${endpoint}...`);
    const provider = new WsProvider(endpoint);
    const api = await ApiPromise.create({ provider });

    console.log('Creating unfreeze proposal...');

    // Create the call
    const call = api.tx.treasury.unfreezeEmergency();
    const callHash = call.method.hash.toHex();

    console.log(`Call hash: ${callHash}`);
    console.log(`Multisig account: ${multisig}`);

    if (dryRun) {
        console.log('DRY RUN: Proposal created (not submitted)');
        await api.disconnect();
        return;
    }

    console.log('Next steps:');
    console.log(`1. Share call hash with Directors: ${callHash}`);
    console.log('2. Collect 7 signatures using collect-signatures.sh');
    console.log('3. Execute with execute-multisig.sh');

    await api.disconnect();
}

createUnfreezeProposal().catch(err => {
    console.error('ERROR:', err);
    process.exit(1);
});
EOFJS
)

    export CHAIN_ENDPOINT MULTISIG DRY_RUN

    if command -v node &> /dev/null; then
        echo "$unfreeze_script" | node || {
            log "ERROR" "Failed to create unfreeze proposal"
            exit 1
        }
    else
        log "ERROR" "Node.js is not installed"
        exit 1
    fi

    log "INFO" "Unfreeze proposal created successfully"
    log "INFO" "Proceed with signature collection"
}

main "$@"
