#!/bin/bash
set -e

# Script configuration
SCRIPT_NAME="$(basename "$0")"
CHAIN_ENDPOINT="${CHAIN_ENDPOINT:-ws://localhost:9944}"
LOG_FILE="${LOG_FILE:-/var/log/etrid/manual-distribution.log}"

# Usage function
usage() {
    cat << EOF
Usage: $SCRIPT_NAME [options]

Manual Validator Rewards Distribution - Requires 5/9 Director Signatures

Options:
  --help                Show this help message
  --endpoint URL        Chain endpoint (default: ws://localhost:9944)
  --multisig ACCOUNT    Multisig account address (required)
  --rewards-file FILE   JSON file with validator→amount mapping (required)
  --threshold N         Signature threshold (default: 5)
  --dry-run             Simulate without executing
  --verbose             Enable verbose logging

Rewards File Format (JSON):
{
  "validators": [
    {"address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "amount": "1000"},
    {"address": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", "amount": "1000"}
  ],
  "reason": "Manual distribution for failed Consensus Day 1234",
  "total": "21000"
}

Example:
  $SCRIPT_NAME \\
    --multisig 5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z \\
    --rewards-file /path/to/rewards.json

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
REWARDS_FILE=""
THRESHOLD=5

while [[ $# -gt 0 ]]; do
    case $1 in
        --help) usage ;;
        --endpoint) CHAIN_ENDPOINT="$2"; shift 2 ;;
        --multisig) MULTISIG="$2"; shift 2 ;;
        --rewards-file) REWARDS_FILE="$2"; shift 2 ;;
        --threshold) THRESHOLD="$2"; shift 2 ;;
        --dry-run) DRY_RUN=true; shift ;;
        --verbose) VERBOSE=true; shift ;;
        *) echo "Unknown option: $1"; usage ;;
    esac
done

# Validate inputs
if [[ -z "$MULTISIG" ]] || [[ -z "$REWARDS_FILE" ]]; then
    log "ERROR" "Missing required parameters"
    usage
fi

if [[ ! -f "$REWARDS_FILE" ]]; then
    log "ERROR" "Rewards file not found: $REWARDS_FILE"
    exit 1
fi

# Main logic
main() {
    log "INFO" "Starting manual validator rewards distribution"
    log "INFO" "Endpoint: $CHAIN_ENDPOINT"
    log "INFO" "Multisig: $MULTISIG"
    log "INFO" "Rewards file: $REWARDS_FILE"
    log "INFO" "Required signatures: $THRESHOLD/9"

    if [[ "$DRY_RUN" == "true" ]]; then
        log "INFO" "DRY RUN MODE - No actual transactions will be submitted"
    fi

    # Validate and display rewards file
    log "INFO" "Validating rewards file..."
    if command -v jq &> /dev/null; then
        local validator_count=$(jq -r '.validators | length' "$REWARDS_FILE")
        local total_amount=$(jq -r '.total' "$REWARDS_FILE")
        local reason=$(jq -r '.reason' "$REWARDS_FILE")

        log "INFO" "Validator count: $validator_count"
        log "INFO" "Total amount: $total_amount EDSC"
        log "INFO" "Reason: $reason"
    else
        log "WARNING" "jq not installed, skipping detailed validation"
    fi

    # Create distribution transaction
    local distribution_script=$(cat << 'EOFJS'
const { ApiPromise, WsProvider } = require('@polkadot/api');
const fs = require('fs');
const process = require('process');

async function createDistributionBatch() {
    const endpoint = process.env.CHAIN_ENDPOINT;
    const multisig = process.env.MULTISIG;
    const rewardsFile = process.env.REWARDS_FILE;
    const dryRun = process.env.DRY_RUN === 'true';

    console.log(`Connecting to ${endpoint}...`);
    const provider = new WsProvider(endpoint);
    const api = await ApiPromise.create({ provider });

    // Load rewards data
    const rewardsData = JSON.parse(fs.readFileSync(rewardsFile, 'utf8'));
    console.log(`Loaded ${rewardsData.validators.length} validator rewards`);
    console.log(`Reason: ${rewardsData.reason}`);

    // Create batch transfer transactions
    const transfers = rewardsData.validators.map(validator => {
        const amountInUnits = BigInt(validator.amount) * BigInt(10 ** 12);
        return api.tx.balances.transfer(validator.address, amountInUnits);
    });

    console.log(`Created ${transfers.length} transfer transactions`);

    // Create batch call
    const batchCall = api.tx.utility.batchAll(transfers);
    const callHash = batchCall.method.hash.toHex();
    const callData = batchCall.method.toHex();

    console.log(`Batch call hash: ${callHash}`);
    console.log(`Multisig account: ${multisig}`);

    if (dryRun) {
        console.log('DRY RUN: Batch distribution created (not submitted)');

        // Display summary
        console.log('\nDistribution Summary:');
        rewardsData.validators.forEach((v, i) => {
            console.log(`  ${i + 1}. ${v.address}: ${v.amount} EDSC`);
        });

        await api.disconnect();
        return;
    }

    // Save batch details
    const batchDetails = {
        callHash,
        callData,
        multisig,
        validators: rewardsData.validators,
        reason: rewardsData.reason,
        total: rewardsData.total,
        timestamp: new Date().toISOString()
    };

    console.log('\nBatch distribution proposal created.');
    console.log(JSON.stringify(batchDetails, null, 2));

    console.log('\nNext steps:');
    console.log('1. Create multisig proposal with create-multisig-proposal.sh');
    console.log('2. Collect 5 signatures using collect-signatures.sh');
    console.log('3. Execute with execute-multisig.sh');
    console.log('4. Verify all validators received rewards using verify-recovery.sh');

    await api.disconnect();
}

createDistributionBatch().catch(err => {
    console.error('ERROR:', err);
    process.exit(1);
});
EOFJS
)

    export CHAIN_ENDPOINT MULTISIG REWARDS_FILE DRY_RUN

    if command -v node &> /dev/null; then
        echo "$distribution_script" | node || {
            log "ERROR" "Failed to create distribution batch"
            exit 1
        }
    else
        log "ERROR" "Node.js is not installed"
        exit 1
    fi

    log "INFO" "Manual distribution batch created successfully"
    log "INFO" "Proceed with signature collection"
}

main "$@"
