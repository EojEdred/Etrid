#!/bin/bash
set -e

# Script configuration
SCRIPT_NAME="$(basename "$0")"
CHAIN_ENDPOINT="${CHAIN_ENDPOINT:-ws://localhost:9944}"
LOG_FILE="${LOG_FILE:-/var/log/etrid/emergency-withdrawal.log}"

# Usage function
usage() {
    cat << EOF
Usage: $SCRIPT_NAME [options]

Emergency Withdrawal of Stuck Funds - Requires 7/9 Director Signatures

Options:
  --help                Show this help message
  --endpoint URL        Chain endpoint (default: ws://localhost:9944)
  --multisig ACCOUNT    Multisig account address (required)
  --amount AMOUNT       Amount to withdraw in EDSC (required)
  --destination ADDR    Destination account (required)
  --reason TEXT         Reason for withdrawal (required)
  --threshold N         Signature threshold (default: 7)
  --dry-run             Simulate without executing
  --verbose             Enable verbose logging

Example:
  $SCRIPT_NAME \\
    --multisig 5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z \\
    --amount 1000000 \\
    --destination 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \\
    --reason "Recovery of stuck validator rewards from failed distribution"

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
AMOUNT=""
DESTINATION=""
REASON=""
THRESHOLD=7

while [[ $# -gt 0 ]]; do
    case $1 in
        --help) usage ;;
        --endpoint) CHAIN_ENDPOINT="$2"; shift 2 ;;
        --multisig) MULTISIG="$2"; shift 2 ;;
        --amount) AMOUNT="$2"; shift 2 ;;
        --destination) DESTINATION="$2"; shift 2 ;;
        --reason) REASON="$2"; shift 2 ;;
        --threshold) THRESHOLD="$2"; shift 2 ;;
        --dry-run) DRY_RUN=true; shift ;;
        --verbose) VERBOSE=true; shift ;;
        *) echo "Unknown option: $1"; usage ;;
    esac
done

# Validate inputs
if [[ -z "$MULTISIG" ]] || [[ -z "$AMOUNT" ]] || [[ -z "$DESTINATION" ]] || [[ -z "$REASON" ]]; then
    log "ERROR" "Missing required parameters"
    usage
fi

# Main logic
main() {
    log "INFO" "Starting emergency withdrawal operation"
    log "INFO" "Endpoint: $CHAIN_ENDPOINT"
    log "INFO" "Multisig: $MULTISIG"
    log "INFO" "Amount: $AMOUNT EDSC"
    log "INFO" "Destination: $DESTINATION"
    log "INFO" "Reason: $REASON"
    log "INFO" "Required signatures: $THRESHOLD/9"

    if [[ "$DRY_RUN" == "true" ]]; then
        log "INFO" "DRY RUN MODE - No actual transactions will be submitted"
    fi

    # Create withdrawal proposal
    local withdrawal_script=$(cat << 'EOFJS'
const { ApiPromise, WsProvider } = require('@polkadot/api');
const process = require('process');

async function createWithdrawalProposal() {
    const endpoint = process.env.CHAIN_ENDPOINT;
    const multisig = process.env.MULTISIG;
    const amount = process.env.AMOUNT;
    const destination = process.env.DESTINATION;
    const reason = process.env.REASON;
    const dryRun = process.env.DRY_RUN === 'true';

    console.log(`Connecting to ${endpoint}...`);
    const provider = new WsProvider(endpoint);
    const api = await ApiPromise.create({ provider });

    console.log('Creating emergency withdrawal proposal...');

    // Convert amount to chain units (assuming 12 decimals)
    const amountInUnits = BigInt(amount) * BigInt(10 ** 12);

    // Create the emergency withdrawal call
    const call = api.tx.treasury.emergencyWithdraw(destination, amountInUnits);
    const callHash = call.method.hash.toHex();
    const callData = call.method.toHex();

    console.log(`Call hash: ${callHash}`);
    console.log(`Call data: ${callData}`);
    console.log(`Multisig account: ${multisig}`);
    console.log(`Amount: ${amount} EDSC (${amountInUnits.toString()} units)`);
    console.log(`Destination: ${destination}`);
    console.log(`Reason: ${reason}`);

    if (dryRun) {
        console.log('DRY RUN: Proposal created (not submitted)');
        await api.disconnect();
        return;
    }

    // Save proposal details
    const proposalDetails = {
        callHash,
        callData,
        multisig,
        amount,
        destination,
        reason,
        timestamp: new Date().toISOString()
    };

    console.log('\nProposal details saved.');
    console.log(JSON.stringify(proposalDetails, null, 2));

    console.log('\nNext steps:');
    console.log('1. Create multisig proposal with create-multisig-proposal.sh');
    console.log('2. Collect 7 signatures using collect-signatures.sh');
    console.log('3. Execute with execute-multisig.sh');
    console.log('4. Verify funds transferred to destination');

    await api.disconnect();
}

createWithdrawalProposal().catch(err => {
    console.error('ERROR:', err);
    process.exit(1);
});
EOFJS
)

    export CHAIN_ENDPOINT MULTISIG AMOUNT DESTINATION REASON DRY_RUN

    if command -v node &> /dev/null; then
        echo "$withdrawal_script" | node || {
            log "ERROR" "Failed to create withdrawal proposal"
            exit 1
        }
    else
        log "ERROR" "Node.js is not installed"
        exit 1
    fi

    log "INFO" "Emergency withdrawal proposal created successfully"
    log "WARNING" "This action requires careful review by all Directors"
    log "INFO" "Documented reason: $REASON"
}

main "$@"
