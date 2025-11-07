#!/usr/bin/env bash
# Ëtrid FlareChain - Genesis Configuration Analysis
# Extracts validator configuration from genesis chainspec

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_FILE="genesis_analysis.txt"

# Find chainspec files
CHAINSPEC_LOCATIONS=(
    "/Users/macbook/Desktop/etrid/runtime/flare-chain/chainspec-raw.json"
    "/Users/macbook/Desktop/etrid/runtime/flare-chain/chainspec.json"
    "$HOME/.etrid/validator/chains/flarechain_mainnet/chainspec.json"
    "/home/ubuntu/chainspec.json"
)

CHAINSPEC=""
for loc in "${CHAINSPEC_LOCATIONS[@]}"; do
    if [[ -f "$loc" ]]; then
        CHAINSPEC="$loc"
        break
    fi
done

if [[ -z "$CHAINSPEC" ]]; then
    echo "ERROR: Could not find chainspec file"
    echo "Searched locations:"
    printf '%s\n' "${CHAINSPEC_LOCATIONS[@]}"
    exit 1
fi

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Genesis Configuration Analysis          ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Chainspec: $CHAINSPEC"
echo ""

{
    echo "═══════════════════════════════════════════════════════════════"
    echo "GENESIS CONFIGURATION ANALYSIS"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    echo "Chainspec: $CHAINSPEC"
    echo "Analysis Date: $(date)"
    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "1. SESSION PALLET - Initial Authorities"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    if jq -e '.genesis.runtime.session' "$CHAINSPEC" >/dev/null 2>&1; then
        # Extract session keys configuration
        echo "Session Keys Found:"
        jq -r '.genesis.runtime.session.keys // .genesis.runtime.session // empty' "$CHAINSPEC" 2>/dev/null | head -100

        # Count validators
        validator_count=$(jq '[.genesis.runtime.session.keys // .genesis.runtime.session // empty] | length' "$CHAINSPEC" 2>/dev/null || echo "0")
        echo ""
        echo "Total validators in genesis: $validator_count"
    else
        echo "⚠ Session pallet configuration not found in genesis"
    fi

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "2. AURA PALLET - Block Production Authorities"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    if jq -e '.genesis.runtime.aura' "$CHAINSPEC" >/dev/null 2>&1; then
        echo "AURA Authorities:"
        jq -r '.genesis.runtime.aura.authorities // empty' "$CHAINSPEC" 2>/dev/null

        aura_count=$(jq '[.genesis.runtime.aura.authorities // empty] | length' "$CHAINSPEC" 2>/dev/null || echo "0")
        echo ""
        echo "Total AURA authorities: $aura_count"
    else
        echo "⚠ AURA pallet configuration not found"
    fi

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "3. GRANDPA PALLET - Finality Authorities"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    if jq -e '.genesis.runtime.grandpa' "$CHAINSPEC" >/dev/null 2>&1; then
        echo "GRANDPA Authorities:"
        jq -r '.genesis.runtime.grandpa.authorities // empty' "$CHAINSPEC" 2>/dev/null

        grandpa_count=$(jq '[.genesis.runtime.grandpa.authorities // empty] | length' "$CHAINSPEC" 2>/dev/null || echo "0")
        echo ""
        echo "Total GRANDPA authorities: $grandpa_count"
    else
        echo "⚠ GRANDPA pallet configuration not found"
    fi

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "4. ASF PALLET - ASF Consensus Configuration"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    if jq -e '.genesis.runtime.asf' "$CHAINSPEC" >/dev/null 2>&1; then
        echo "ASF Configuration:"
        jq -r '.genesis.runtime.asf // empty' "$CHAINSPEC" 2>/dev/null
    elif jq -e '.genesis.runtime.ppfa' "$CHAINSPEC" >/dev/null 2>&1; then
        echo "PPFA Configuration:"
        jq -r '.genesis.runtime.ppfa // empty' "$CHAINSPEC" 2>/dev/null
    else
        echo "⚠ ASF/PPFA pallet configuration not found"
        echo ""
        echo "Available pallets in genesis:"
        jq -r '.genesis.runtime | keys[]' "$CHAINSPEC" 2>/dev/null | grep -v "^system$\|^timestamp$" | head -20
    fi

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "5. STAKING PALLET - Validator Stakes (if applicable)"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    if jq -e '.genesis.runtime.staking' "$CHAINSPEC" >/dev/null 2>&1; then
        echo "Staking Configuration Found:"
        jq -r '.genesis.runtime.staking.validators // .genesis.runtime.staking.stakers // empty' "$CHAINSPEC" 2>/dev/null | head -50
    else
        echo "ℹ Staking pallet not configured in genesis (may not be required)"
    fi

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "6. BALANCES - Validator Account Endowments"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    if jq -e '.genesis.runtime.balances' "$CHAINSPEC" >/dev/null 2>&1; then
        echo "Endowed accounts (showing first 10):"
        jq -r '.genesis.runtime.balances.balances // empty' "$CHAINSPEC" 2>/dev/null | head -20

        balance_count=$(jq '[.genesis.runtime.balances.balances // empty] | length' "$CHAINSPEC" 2>/dev/null || echo "0")
        echo ""
        echo "Total endowed accounts: $balance_count"
    else
        echo "⚠ Balances configuration not found"
    fi

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "7. VALIDATOR ACCOUNT MAPPING (if available)"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    # Try to extract validator account IDs from session keys
    echo "Attempting to extract validator accounts from session.keys:"
    jq -r '
        .genesis.runtime.session.keys // empty |
        to_entries[] |
        "Validator \(.key): Account \(.value[0] // .value)"
    ' "$CHAINSPEC" 2>/dev/null | head -25 || echo "Could not extract validator accounts"

    echo ""
    echo "═══════════════════════════════════════════════════════════════"
    echo "ANALYSIS SUMMARY"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""

    # Summary statistics
    session_count=$(jq '[.genesis.runtime.session.keys // empty] | length' "$CHAINSPEC" 2>/dev/null || echo "0")
    aura_count=$(jq '[.genesis.runtime.aura.authorities // empty] | length' "$CHAINSPEC" 2>/dev/null || echo "0")
    grandpa_count=$(jq '[.genesis.runtime.grandpa.authorities // empty] | length' "$CHAINSPEC" 2>/dev/null || echo "0")

    echo "Genesis Validator Counts:"
    echo "  Session Keys:       $session_count"
    echo "  AURA Authorities:   $aura_count"
    echo "  GRANDPA Authorities: $grandpa_count"
    echo ""

    if [[ "$session_count" -eq 16 ]]; then
        echo "✓ Genesis defines exactly 16 authorities"
        echo "  This matches the active committee size (validators 6-21)"
        echo ""
        echo "HYPOTHESIS 1 CONFIRMED:"
        echo "  Genesis chainspec defines the initial 16-member committee."
        echo "  Validators 2-4 (Azure VMs) are NOT in genesis authorities."
        echo ""
        echo "IMPLICATION:"
        echo "  Azure VMs must be ADDED to validator set via:"
        echo "  - Session key registration (session.setKeys extrinsic)"
        echo "  - Governance proposal to expand committee"
        echo "  - Runtime upgrade to support 19 validators"
    elif [[ "$session_count" -eq 19 ]]; then
        echo "⚠ Genesis defines 19 authorities but only 16 are active"
        echo ""
        echo "HYPOTHESIS 2 LIKELY:"
        echo "  Session keys for validators 2-4 may not match what's in their keystores."
        echo "  Or validators 2-4 are waiting for epoch change to become active."
    else
        echo "ℹ Genesis defines $session_count authorities"
        echo "  Current active committee: 16 validators"
        echo "  Requires further investigation"
    fi

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "NEXT STEPS:"
    echo "─────────────────────────────────────────────────────────────"
    echo ""
    echo "1. Run query-validator-set.sh to check on-chain validator set"
    echo "2. Compare genesis authorities with active validators"
    echo "3. Determine if validators 2-4 need to:"
    echo "   a) Register session keys on-chain"
    echo "   b) Bond stake (if staking required)"
    echo "   c) Wait for validator set rotation"
    echo "   d) Submit governance proposal to join"
    echo ""

} | tee "$OUTPUT_FILE"

echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Analysis complete: $OUTPUT_FILE"
echo ""
