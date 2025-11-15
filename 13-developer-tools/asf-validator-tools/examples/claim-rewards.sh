#!/bin/bash
# Example: Automated reward claiming script
# Run weekly to claim and optionally re-stake rewards

set -e

# Configuration
VALIDATOR_ADDRESS="${1:-$(cat ~/.etrid/validator-address)}"
RPC_ENDPOINT="${2:-http://localhost:9944}"
CONTROLLER_KEY="${3:-${HOME}/.etrid/keys/controller.key}"
AUTO_RESTAKE="${AUTO_RESTAKE:-true}"
MIN_CLAIM_AMOUNT="1000"  # Minimum ETR to claim

echo "════════════════════════════════════════════════════════════"
echo "ËTRID Validator Reward Claiming"
echo "════════════════════════════════════════════════════════════"
echo "Validator: ${VALIDATOR_ADDRESS}"
echo "RPC: ${RPC_ENDPOINT}"
echo ""

# Check pending rewards
echo "Checking pending rewards..."
REWARDS_OUTPUT=$(asf-stake rewards --validator "${VALIDATOR_ADDRESS}" --epochs 30 --rpc "${RPC_ENDPOINT}")
echo "${REWARDS_OUTPUT}"

# Parse pending rewards amount (this would need actual JSON parsing in production)
# For demo, we'll use a placeholder
PENDING_REWARDS=1250  # This would come from parsing the output

echo ""
echo "Pending rewards: ${PENDING_REWARDS} ETR"

# Check if minimum claim amount is met
if [ "${PENDING_REWARDS}" -lt "${MIN_CLAIM_AMOUNT}" ]; then
  echo "Pending rewards (${PENDING_REWARDS} ETR) below minimum claim amount (${MIN_CLAIM_AMOUNT} ETR)"
  echo "Skipping claim."
  exit 0
fi

# Claim rewards
echo ""
echo "Claiming rewards..."
CLAIM_OUTPUT=$(asf-stake claim \
  --validator "${VALIDATOR_ADDRESS}" \
  --keyfile "${CONTROLLER_KEY}" \
  --rpc "${RPC_ENDPOINT}")

echo "${CLAIM_OUTPUT}"

# Re-stake if enabled
if [ "${AUTO_RESTAKE}" = "true" ]; then
  echo ""
  echo "Re-staking claimed rewards..."

  asf-stake bond \
    --amount "${PENDING_REWARDS}" \
    --validator "${VALIDATOR_ADDRESS}" \
    --keyfile "${CONTROLLER_KEY}" \
    --rpc "${RPC_ENDPOINT}"

  echo "✓ Rewards claimed and re-staked!"
else
  echo "✓ Rewards claimed!"
fi

# Log the operation
LOG_FILE="${HOME}/.etrid/logs/rewards.log"
mkdir -p "$(dirname ${LOG_FILE})"
echo "$(date): Claimed ${PENDING_REWARDS} ETR, restaked=${AUTO_RESTAKE}" >> "${LOG_FILE}"

echo ""
echo "════════════════════════════════════════════════════════════"
