#!/bin/bash

# Primearc Chain Health Monitor
# Usage: ./monitor-primearc.sh [refresh_seconds]

VALIDATOR_IP="100.93.43.18"
RPC_PORT="9944"
REFRESH_INTERVAL="${1:-5}"

clear

while true; do
  clear
  echo "═══════════════════════════════════════════════════════════"
  echo "         PRIMEARC CORE CHAIN - HEALTH MONITOR"
  echo "═══════════════════════════════════════════════════════════"
  echo ""
  echo "🌐 Validator Endpoint: $VALIDATOR_IP:$RPC_PORT"
  echo "🔄 Auto-refresh: ${REFRESH_INTERVAL}s (Ctrl+C to exit)"
  echo "───────────────────────────────────────────────────────────"
  echo ""

  # Chain Name
  CHAIN_NAME=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_chain"}' \
    http://$VALIDATOR_IP:$RPC_PORT 2>/dev/null | \
    python3 -c "import sys, json; print(json.load(sys.stdin)['result'])" 2>/dev/null)

  echo "📋 Chain: ${CHAIN_NAME:-"N/A"}"
  echo ""

  # System Health
  HEALTH=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
    http://$VALIDATOR_IP:$RPC_PORT 2>/dev/null)

  PEERS=$(echo "$HEALTH" | python3 -c "import sys, json; print(json.load(sys.stdin)['result']['peers'])" 2>/dev/null)
  IS_SYNCING=$(echo "$HEALTH" | python3 -c "import sys, json; print(json.load(sys.stdin)['result']['isSyncing'])" 2>/dev/null)

  echo "👥 Connected Peers: ${PEERS:-"?"}"

  if [ "$IS_SYNCING" = "True" ] || [ "$IS_SYNCING" = "true" ]; then
    echo "🟡 Status: SYNCING"
  elif [ "$IS_SYNCING" = "False" ] || [ "$IS_SYNCING" = "false" ]; then
    echo "🟢 Status: SYNCED"
  else
    echo "🔴 Status: UNKNOWN"
  fi
  echo ""

  # Sync State
  SYNC_STATE=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState"}' \
    http://$VALIDATOR_IP:$RPC_PORT 2>/dev/null)

  CURRENT_BLOCK=$(echo "$SYNC_STATE" | python3 -c "import sys, json; print(json.load(sys.stdin)['result']['currentBlock'])" 2>/dev/null)
  HIGHEST_BLOCK=$(echo "$SYNC_STATE" | python3 -c "import sys, json; print(json.load(sys.stdin)['result']['highestBlock'])" 2>/dev/null)

  if [ -n "$CURRENT_BLOCK" ] && [ -n "$HIGHEST_BLOCK" ]; then
    BLOCKS_BEHIND=$((HIGHEST_BLOCK - CURRENT_BLOCK))
    PROGRESS=$(python3 -c "print(f'{($CURRENT_BLOCK / $HIGHEST_BLOCK * 100):.2f}')" 2>/dev/null)

    echo "📊 Block Progress:"
    echo "   Current:  $(printf '%7s' $CURRENT_BLOCK)"
    echo "   Highest:  $(printf '%7s' $HIGHEST_BLOCK)"
    echo "   Behind:   $(printf '%7s' $BLOCKS_BEHIND) blocks"
    echo "   Progress: ${PROGRESS}%"
  else
    echo "📊 Block Progress: N/A"
  fi
  echo ""

  # Node Info
  NODE_NAME=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_name"}' \
    http://$VALIDATOR_IP:$RPC_PORT 2>/dev/null | \
    python3 -c "import sys, json; print(json.load(sys.stdin)['result'])" 2>/dev/null)

  NODE_VERSION=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_version"}' \
    http://$VALIDATOR_IP:$RPC_PORT 2>/dev/null | \
    python3 -c "import sys, json; print(json.load(sys.stdin)['result'])" 2>/dev/null)

  echo "🖥️  Node Software: ${NODE_NAME:-"N/A"}"
  echo "📦 Version: ${NODE_VERSION:-"N/A"}"
  echo ""

  echo "───────────────────────────────────────────────────────────"
  echo "⏰ Last update: $(date '+%Y-%m-%d %H:%M:%S')"
  echo "═══════════════════════════════════════════════════════════"

  sleep $REFRESH_INTERVAL
done
