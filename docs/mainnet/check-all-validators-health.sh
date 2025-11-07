#!/usr/bin/env bash
# Ëtrid FlareChain - Comprehensive Mainnet Validator Health Check
# Checks all 21 validators across Azure (16 VMs), Oracle (2 VMs), and Azure Sub2 (3 VMs)
# Date: November 7, 2025

set -e

OUTPUT_FILE="validator_health_$(date +%Y%m%d_%H%M%S).txt"
TEMP_LOG="/tmp/validator_health_check.log"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Mainnet Validator Health Check          ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Checking 21 validators across 3 cloud providers..."
echo "Output will be saved to: $OUTPUT_FILE"
echo ""

{
    echo "═══════════════════════════════════════════════════════════════"
    echo "ËTRID FLARECHAIN - MAINNET VALIDATOR HEALTH REPORT"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    echo "Report Time: $(date)"
    echo "Network: Ëtrid FlareChain Mainnet"
    echo "Expected Validators: 21"
    echo ""

    # Test network connectivity first
    echo "─────────────────────────────────────────────────────────────"
    echo "NETWORK CONNECTIVITY TEST"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    REACHABLE_COUNT=0
    TOTAL_VALIDATORS=21

    # Define all validator IPs
    declare -A VALIDATORS=(
        # Oracle Cloud
        ["V1-Gizzi"]="64.181.215.19"
        ["V3-Audit"]="129.80.122.34"

        # Azure Sub 2 (eojedredbitepubkey1)
        ["V0B-EojEdred"]="20.69.26.209"
        ["V1-Governance"]="20.186.91.207"
        ["V2-Security"]="52.252.142.146"

        # Azure Sub 1 (etridfoundation) - West Europe
        ["V6-RuntimePri"]="20.224.104.239"
        ["V7-RuntimeSec"]="108.142.205.177"
        ["V8-CompilerPri"]="4.180.238.67"
        ["V9-CompilerSec"]="4.180.59.25"
        ["V12-Oracle"]="98.71.219.106"

        # Azure Sub 1 - North Europe
        ["V10-Multichain-MONITOR"]="98.71.91.84"
        ["V11-Multichain"]="68.219.230.63"

        # Azure Sub 1 - UK South
        ["V13-EDSC-Pri"]="172.167.8.217"
        ["V14-EDSC-Sec"]="51.142.203.160"
        ["V15-Economics-Pri"]="172.166.164.19"
        ["V16-Economics-Sec"]="172.166.187.180"
        ["V17-Ethics-Pri"]="172.166.210.244"

        # Azure Sub 1 - France Central
        ["V18-Ethics-Sec"]="4.251.115.186"
        ["V19-Docs-Pri"]="52.143.191.232"
        ["V20-Docs-Sec"]="4.211.206.210"
        ["V21-Docs-Ter"]="4.178.181.122"
    )

    echo "Testing RPC endpoints (port 9944)..."
    echo ""

    # Check RPC connectivity
    for name in "${!VALIDATORS[@]}"; do
        ip="${VALIDATORS[$name]}"
        printf "%-25s %s " "$name" "$ip"

        # Test RPC endpoint
        response=$(curl -s -m 3 -X POST \
            -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
            http://$ip:9944 2>/dev/null || echo "")

        if [[ -n "$response" ]] && echo "$response" | grep -q "result"; then
            echo -e "${GREEN}✓ RPC RESPONDING${NC}"
            ((REACHABLE_COUNT++))
        else
            echo -e "${RED}✗ RPC NOT RESPONDING${NC}"
        fi
    done

    echo ""
    echo "RPC Connectivity: $REACHABLE_COUNT/$TOTAL_VALIDATORS responding"
    echo ""

    # If no RPC endpoints responding, blockchain is not running
    if [ $REACHABLE_COUNT -eq 0 ]; then
        echo "═══════════════════════════════════════════════════════════════"
        echo "❌ CRITICAL: NO VALIDATORS RESPONDING"
        echo "═══════════════════════════════════════════════════════════════"
        echo ""
        echo "FlareChain blockchain processes are not running on any validator."
        echo ""
        echo "Possible causes:"
        echo "  1. Validators have not been started yet"
        echo "  2. Systemd services are not running"
        echo "  3. RPC port 9944 is not accessible"
        echo "  4. Firewall blocking external access"
        echo ""
        echo "To start validators, run on each VM:"
        echo "  sudo systemctl start flarechain-validator"
        echo "  sudo systemctl enable flarechain-validator"
        echo ""
        exit 0
    fi

    # Detailed blockchain health check
    echo "─────────────────────────────────────────────────────────────"
    echo "BLOCKCHAIN STATUS DETAILS"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    # Pick first responding validator for detailed checks
    ACTIVE_VALIDATOR=""
    ACTIVE_IP=""

    for name in "${!VALIDATORS[@]}"; do
        ip="${VALIDATORS[$name]}"
        response=$(curl -s -m 3 -X POST \
            -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
            http://$ip:9944 2>/dev/null || echo "")

        if [[ -n "$response" ]] && echo "$response" | grep -q "result"; then
            ACTIVE_VALIDATOR="$name"
            ACTIVE_IP="$ip"
            break
        fi
    done

    if [[ -n "$ACTIVE_VALIDATOR" ]]; then
        echo "Using $ACTIVE_VALIDATOR ($ACTIVE_IP) for blockchain queries..."
        echo ""

        # 1. System Health
        echo "1. NETWORK HEALTH:"
        health=$(curl -s -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
            http://$ACTIVE_IP:9944 2>/dev/null)
        echo "$health" | jq -r '.result | "   Peers: \(.peers)\n   Is Syncing: \(.isSyncing)\n   Should Have Peers: \(.shouldHavePeers)"' 2>/dev/null || echo "$health"
        echo ""

        # 2. Chain Info
        echo "2. CHAIN INFORMATION:"
        chain=$(curl -s -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_chain","params":[],"id":1}' \
            http://$ACTIVE_IP:9944 2>/dev/null)
        echo "   Chain: $(echo $chain | jq -r '.result' 2>/dev/null || echo 'Unknown')"

        properties=$(curl -s -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_properties","params":[],"id":1}' \
            http://$ACTIVE_IP:9944 2>/dev/null)
        echo "   Token: $(echo $properties | jq -r '.result.tokenSymbol[0]' 2>/dev/null || echo 'Unknown')"
        echo "   Decimals: $(echo $properties | jq -r '.result.tokenDecimals[0]' 2>/dev/null || echo 'Unknown')"
        echo ""

        # 3. Block Height
        echo "3. BLOCKCHAIN STATUS:"
        header=$(curl -s -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
            http://$ACTIVE_IP:9944 2>/dev/null)
        block_number=$(echo $header | jq -r '.result.number' 2>/dev/null | xargs printf "%d" 2>/dev/null || echo "Unknown")
        echo "   Current Block: #$block_number"

        finalized=$(curl -s -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"chain_getFinalizedHead","params":[],"id":1}' \
            http://$ACTIVE_IP:9944 2>/dev/null)
        finalized_hash=$(echo $finalized | jq -r '.result' 2>/dev/null || echo "Unknown")
        echo "   Finalized Hash: ${finalized_hash:0:20}..."
        echo ""

        # 4. Validator Set
        echo "4. ACTIVE VALIDATORS:"
        # Try to query validator set
        validators=$(curl -s -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"state_call","params":["AuraApi_authorities","0x"],"id":1}' \
            http://$ACTIVE_IP:9944 2>/dev/null)

        if echo "$validators" | grep -q "result"; then
            echo "   Validator query successful"
            echo "   (Detailed validator list available via RPC)"
        else
            echo "   Validator set: Query unavailable (check runtime API)"
        fi
        echo ""

        # 5. Node Version
        echo "5. NODE INFORMATION:"
        version=$(curl -s -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_version","params":[],"id":1}' \
            http://$ACTIVE_IP:9944 2>/dev/null)
        echo "   Version: $(echo $version | jq -r '.result' 2>/dev/null || echo 'Unknown')"

        name=$(curl -s -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_name","params":[],"id":1}' \
            http://$ACTIVE_IP:9944 2>/dev/null)
        echo "   Implementation: $(echo $name | jq -r '.result' 2>/dev/null || echo 'Unknown')"
        echo ""
    fi

    # Individual validator status
    echo "─────────────────────────────────────────────────────────────"
    echo "INDIVIDUAL VALIDATOR STATUS"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    for name in $(echo "${!VALIDATORS[@]}" | tr ' ' '\n' | sort); do
        ip="${VALIDATORS[$name]}"
        echo "Validator: $name"
        echo "IP: $ip"

        # Check system health via RPC
        health=$(curl -s -m 3 -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
            http://$ip:9944 2>/dev/null || echo "")

        if [[ -n "$health" ]] && echo "$health" | grep -q "result"; then
            peers=$(echo "$health" | jq -r '.result.peers' 2>/dev/null || echo "0")
            syncing=$(echo "$health" | jq -r '.result.isSyncing' 2>/dev/null || echo "unknown")
            echo "  Status: ✓ ONLINE"
            echo "  Peers: $peers"
            echo "  Syncing: $syncing"
        else
            echo "  Status: ✗ OFFLINE or RPC not accessible"
        fi

        echo ""
    done

    echo "═══════════════════════════════════════════════════════════════"
    echo "SUMMARY"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    echo "Total Validators: 21"
    echo "RPC Responding: $REACHABLE_COUNT"
    echo "RPC Not Responding: $((TOTAL_VALIDATORS - REACHABLE_COUNT))"
    echo ""

    if [ $REACHABLE_COUNT -ge 15 ]; then
        echo "Network Status: ✓ HEALTHY (Supermajority online)"
    elif [ $REACHABLE_COUNT -ge 11 ]; then
        echo "Network Status: ⚠ DEGRADED (Simple majority, finality at risk)"
    elif [ $REACHABLE_COUNT -gt 0 ]; then
        echo "Network Status: ❌ CRITICAL (Insufficient validators)"
    else
        echo "Network Status: ❌ OFFLINE (No validators responding)"
    fi
    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "RECOMMENDATIONS:"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    if [ $REACHABLE_COUNT -eq 0 ]; then
        echo "1. Start validator services on all VMs"
        echo "2. Verify systemd services: sudo systemctl status flarechain-validator"
        echo "3. Check firewall rules for port 9944"
    elif [ $REACHABLE_COUNT -lt 15 ]; then
        echo "1. Start remaining $(($TOTAL_VALIDATORS - $REACHABLE_COUNT)) validators"
        echo "2. Check logs on offline validators"
        echo "3. Verify network connectivity between peers"
    else
        echo "Network operating normally. Monitor for stability."
    fi
    echo ""

    echo "═══════════════════════════════════════════════════════════════"
    echo "For detailed logs, SSH into individual VMs and run:"
    echo "  sudo journalctl -u flarechain-validator -f --no-pager"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""

} | tee "$OUTPUT_FILE"

echo ""
echo -e "${GREEN}Health check complete!${NC}"
echo "Report saved to: $OUTPUT_FILE"
echo ""
