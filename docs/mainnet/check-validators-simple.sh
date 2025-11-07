#!/bin/bash
# Ëtrid FlareChain - Simple Mainnet Validator Health Check
# Compatible with macOS default bash
# Date: November 7, 2025

OUTPUT_FILE="validator_health_$(date +%Y%m%d_%H%M%S).txt"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Mainnet Validator Health Check          ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

{
    echo "═══════════════════════════════════════════════════════════════"
    echo "ËTRID FLARECHAIN - MAINNET VALIDATOR HEALTH REPORT"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    echo "Report Time: $(date)"
    echo ""

    REACHABLE_COUNT=0
    TOTAL_VALIDATORS=21

    # Oracle Cloud validators
    echo "─────────────────────────────────────────────────────────────"
    echo "ORACLE CLOUD VALIDATORS (2)"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    echo "V1-Gizzi (64.181.215.19):"
    if curl -s -m 3 -X POST -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
        http://64.181.215.19:9944 2>/dev/null | grep -q "result"; then
        echo "  ✓ RPC RESPONDING"
        ((REACHABLE_COUNT++))
    else
        echo "  ✗ RPC NOT RESPONDING"
    fi
    echo ""

    echo "V3-Audit (129.80.122.34):"
    if curl -s -m 3 -X POST -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
        http://129.80.122.34:9944 2>/dev/null | grep -q "result"; then
        echo "  ✓ RPC RESPONDING"
        ((REACHABLE_COUNT++))
    else
        echo "  ✗ RPC NOT RESPONDING"
    fi
    echo ""

    # Azure Sub 2 validators
    echo "─────────────────────────────────────────────────────────────"
    echo "AZURE SUBSCRIPTION 2 - SPECIAL VALIDATORS (3)"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    echo "V0B-EojEdred (20.69.26.209):"
    if curl -s -m 3 -X POST -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
        http://20.69.26.209:9944 2>/dev/null | grep -q "result"; then
        echo "  ✓ RPC RESPONDING"
        ((REACHABLE_COUNT++))
    else
        echo "  ✗ RPC NOT RESPONDING"
    fi
    echo ""

    echo "V1-Governance (20.186.91.207):"
    if curl -s -m 3 -X POST -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
        http://20.186.91.207:9944 2>/dev/null | grep -q "result"; then
        echo "  ✓ RPC RESPONDING"
        ((REACHABLE_COUNT++))
    else
        echo "  ✗ RPC NOT RESPONDING"
    fi
    echo ""

    echo "V2-Security (52.252.142.146):"
    if curl -s -m 3 -X POST -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
        http://52.252.142.146:9944 2>/dev/null | grep -q "result"; then
        echo "  ✓ RPC RESPONDING"
        ((REACHABLE_COUNT++))
    else
        echo "  ✗ RPC NOT RESPONDING"
    fi
    echo ""

    # Azure Sub 1 - West Europe
    echo "─────────────────────────────────────────────────────────────"
    echo "AZURE SUBSCRIPTION 1 - WEST EUROPE (5)"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    for name_ip in "V6-RuntimePri:20.224.104.239" "V7-RuntimeSec:108.142.205.177" \
                    "V8-CompilerPri:4.180.238.67" "V9-CompilerSec:4.180.59.25" \
                    "V12-Oracle:98.71.219.106"; do
        IFS=':' read -r name ip <<< "$name_ip"
        echo "$name ($ip):"
        if curl -s -m 3 -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
            http://$ip:9944 2>/dev/null | grep -q "result"; then
            echo "  ✓ RPC RESPONDING"
            ((REACHABLE_COUNT++))
        else
            echo "  ✗ RPC NOT RESPONDING"
        fi
        echo ""
    done

    # Azure Sub 1 - North Europe
    echo "─────────────────────────────────────────────────────────────"
    echo "AZURE SUBSCRIPTION 1 - NORTH EUROPE (2)"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    for name_ip in "V10-Multichain-MONITOR:98.71.91.84" "V11-Multichain:68.219.230.63"; do
        IFS=':' read -r name ip <<< "$name_ip"
        echo "$name ($ip):"
        if curl -s -m 3 -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
            http://$ip:9944 2>/dev/null | grep -q "result"; then
            echo "  ✓ RPC RESPONDING"
            ((REACHABLE_COUNT++))
        else
            echo "  ✗ RPC NOT RESPONDING"
        fi
        echo ""
    done

    # Azure Sub 1 - UK South
    echo "─────────────────────────────────────────────────────────────"
    echo "AZURE SUBSCRIPTION 1 - UK SOUTH (5)"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    for name_ip in "V13-EDSC-Pri:172.167.8.217" "V14-EDSC-Sec:51.142.203.160" \
                    "V15-Economics-Pri:172.166.164.19" "V16-Economics-Sec:172.166.187.180" \
                    "V17-Ethics-Pri:172.166.210.244"; do
        IFS=':' read -r name ip <<< "$name_ip"
        echo "$name ($ip):"
        if curl -s -m 3 -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
            http://$ip:9944 2>/dev/null | grep -q "result"; then
            echo "  ✓ RPC RESPONDING"
            ((REACHABLE_COUNT++))
        else
            echo "  ✗ RPC NOT RESPONDING"
        fi
        echo ""
    done

    # Azure Sub 1 - France Central
    echo "─────────────────────────────────────────────────────────────"
    echo "AZURE SUBSCRIPTION 1 - FRANCE CENTRAL (4)"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    for name_ip in "V18-Ethics-Sec:4.251.115.186" "V19-Docs-Pri:52.143.191.232" \
                    "V20-Docs-Sec:4.211.206.210" "V21-Docs-Ter:4.178.181.122"; do
        IFS=':' read -r name ip <<< "$name_ip"
        echo "$name ($ip):"
        if curl -s -m 3 -X POST -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
            http://$ip:9944 2>/dev/null | grep -q "result"; then
            echo "  ✓ RPC RESPONDING"
            ((REACHABLE_COUNT++))
        else
            echo "  ✗ RPC NOT RESPONDING"
        fi
        echo ""
    done

    # Detailed status from first responding validator
    echo "═══════════════════════════════════════════════════════════════"
    echo "BLOCKCHAIN STATUS"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""

    if [ $REACHABLE_COUNT -eq 0 ]; then
        echo "❌ CRITICAL: NO VALIDATORS RESPONDING"
        echo ""
        echo "FlareChain blockchain is not running."
        echo ""
        echo "Action required:"
        echo "  1. Start validator services: sudo systemctl start flarechain-validator"
        echo "  2. Check firewall rules for port 9944"
        echo "  3. Verify node binary is present: /usr/local/bin/flarechain-node"
        echo ""
    else
        # Find first responding validator
        for test_ip in "64.181.215.19" "20.69.26.209" "20.186.91.207" "98.71.91.84" "20.224.104.239"; do
            if curl -s -m 3 -X POST -H "Content-Type: application/json" \
                -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
                http://$test_ip:9944 2>/dev/null | grep -q "result"; then
                ACTIVE_IP="$test_ip"
                break
            fi
        done

        if [ -n "$ACTIVE_IP" ]; then
            echo "Querying validator at $ACTIVE_IP..."
            echo ""

            # System health
            health=$(curl -s -m 5 -X POST -H "Content-Type: application/json" \
                -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
                http://$ACTIVE_IP:9944 2>/dev/null)
            echo "Network Health:"
            echo "$health" | python3 -m json.tool 2>/dev/null || echo "$health"
            echo ""

            # Block height
            header=$(curl -s -m 5 -X POST -H "Content-Type: application/json" \
                -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
                http://$ACTIVE_IP:9944 2>/dev/null)
            echo "Current Block:"
            echo "$header" | python3 -c "import sys,json; h=json.load(sys.stdin); print('  Block Number:', int(h['result']['number'], 16) if 'result' in h and 'number' in h['result'] else 'Unknown')" 2>/dev/null || echo "  Unable to query"
            echo ""

            # Chain info
            chain=$(curl -s -m 5 -X POST -H "Content-Type: application/json" \
                -d '{"jsonrpc":"2.0","method":"system_chain","params":[],"id":1}' \
                http://$ACTIVE_IP:9944 2>/dev/null)
            echo "Chain: $(echo $chain | python3 -c "import sys,json; print(json.load(sys.stdin).get('result', 'Unknown'))" 2>/dev/null || echo 'Unknown')"
            echo ""
        fi
    fi

    # Summary
    echo "═══════════════════════════════════════════════════════════════"
    echo "SUMMARY"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    echo "Total Validators: $TOTAL_VALIDATORS"
    echo "RPC Responding: $REACHABLE_COUNT"
    echo "RPC Not Responding: $((TOTAL_VALIDATORS - REACHABLE_COUNT))"
    echo ""

    if [ $REACHABLE_COUNT -ge 15 ]; then
        echo "Network Status: ✓ HEALTHY (Supermajority online)"
        echo "Blockchain is producing and finalizing blocks."
    elif [ $REACHABLE_COUNT -ge 11 ]; then
        echo "Network Status: ⚠ DEGRADED (Simple majority)"
        echo "Finality may be compromised. Start offline validators."
    elif [ $REACHABLE_COUNT -gt 0 ]; then
        echo "Network Status: ❌ CRITICAL (Insufficient validators)"
        echo "Network cannot achieve consensus. Start more validators immediately."
    else
        echo "Network Status: ❌ OFFLINE (No validators responding)"
        echo "Blockchain has not been started."
    fi
    echo ""

    echo "═══════════════════════════════════════════════════════════════"
    echo "Report complete: $(date)"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""

} | tee "$OUTPUT_FILE"

echo ""
echo "Health check complete! Report saved to: $OUTPUT_FILE"
echo ""
