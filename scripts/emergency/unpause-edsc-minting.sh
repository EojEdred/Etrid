#!/bin/bash
set -e

# Script configuration
SCRIPT_NAME="$(basename "$0")"
CHAIN_ENDPOINT="${CHAIN_ENDPOINT:-ws://localhost:9944}"
LOG_FILE="${LOG_FILE:-/var/log/etrid/unpause-edsc-minting.log}"

# Usage function
usage() {
    cat << EOF
Usage: $SCRIPT_NAME [options]

Unpause EDSC Minting - Requires 7/9 Director Signatures

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
    log "INFO" "Starting EDSC minting unpause operation"
    log "INFO" "Endpoint: $CHAIN_ENDPOINT"
    log "INFO" "Multisig: $MULTISIG"
    log "INFO" "Required signatures: $THRESHOLD/9"

    if [[ "$DRY_RUN" == "true" ]]; then
        log "INFO" "DRY RUN MODE - No actual transactions will be submitted"
    fi

    # Create unpause proposal
    local unpause_script=$(cat << 'EOFJS'
const { ApiPromise, WsProvider } = require('@polkadot/api');
const process = require('process');

async function createUnpauseProposal() {
    const endpoint = process.env.CHAIN_ENDPOINT;
    const multisig = process.env.MULTISIG;
    const dryRun = process.env.DRY_RUN === 'true';

    console.log(`Connecting to ${endpoint}...`);
    const provider = new WsProvider(endpoint);
    const api = await ApiPromise.create({ provider });

    console.log('Creating unpause proposal...');

    // Create the call
    const call = api.tx.consensusDay.unpauseMinting();
    const callHash = call.method.hash.toHex();
    const callData = call.method.toHex();

    console.log(`Call hash: ${callHash}`);
    console.log(`Call data: ${callData}`);
    console.log(`Multisig account: ${multisig}`);

    if (dryRun) {
        console.log('DRY RUN: Proposal created (not submitted)');
        await api.disconnect();
        return;
    }

    console.log('Next steps:');
    console.log('1. Create multisig proposal with create-multisig-proposal.sh');
    console.log('2. Collect 7 signatures using collect-signatures.sh');
    console.log('3. Execute with execute-multisig.sh');
    console.log('4. Verify minting resumed successfully');

    await api.disconnect();
}

createUnpauseProposal().catch(err => {
    console.error('ERROR:', err);
    process.exit(1);
});
EOFJS
)

    export CHAIN_ENDPOINT MULTISIG DRY_RUN

    if command -v node &> /dev/null; then
        echo "$unpause_script" | node || {
            log "ERROR" "Failed to create unpause proposal"
            exit 1
        }
    else
        log "ERROR" "Node.js is not installed"
        exit 1
    fi

    log "INFO" "Unpause proposal created successfully"
    log "INFO" "Proceed with signature collection"
}

main "$@"
