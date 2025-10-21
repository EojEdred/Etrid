#!/bin/bash
# Health check script for EDSC Bridge services
# Checks all attestation services, relayers, and chain connections

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration (can be overridden by environment variables)
ATTESTATION_SERVICES=${ATTESTATION_SERVICES:-"attestation-0.etrid.io attestation-1.etrid.io attestation-2.etrid.io attestation-3.etrid.io attestation-4.etrid.io"}
RELAYER_SERVICES=${RELAYER_SERVICES:-"relayer-1.etrid.io relayer-2.etrid.io relayer-3.etrid.io"}
ETHEREUM_RPC=${ETHEREUM_RPC:-"https://eth-sepolia.g.alchemy.com/v2/YOUR-KEY"}
SUBSTRATE_RPC=${SUBSTRATE_RPC:-"wss://ember-rpc.etrid.io"}

echo "======================================"
echo "  EDSC Bridge Health Check"
echo "======================================"
echo ""

# Check Attestation Services
echo -e "${YELLOW}Checking Attestation Services...${NC}"
attesters_up=0
attesters_down=0

for service in $ATTESTATION_SERVICES; do
  response=$(curl -s -o /dev/null -w "%{http_code}" "https://$service/health" 2>/dev/null || echo "000")

  if [ "$response" = "200" ]; then
    echo -e "${GREEN}✓${NC} $service - UP"
    attesters_up=$((attesters_up + 1))
  else
    echo -e "${RED}✗${NC} $service - DOWN (HTTP $response)"
    attesters_down=$((attesters_down + 1))
  fi
done

echo ""
echo "Attestation Services: $attesters_up UP / $attesters_down DOWN"

if [ $attesters_up -lt 3 ]; then
  echo -e "${RED}WARNING: Less than 3 attesters are up! Threshold at risk!${NC}"
fi

echo ""

# Check Relayer Services
echo -e "${YELLOW}Checking Relayer Services...${NC}"
relayers_up=0
relayers_down=0

for service in $RELAYER_SERVICES; do
  response=$(curl -s -o /dev/null -w "%{http_code}" "https://$service/health" 2>/dev/null || echo "000")

  if [ "$response" = "200" ]; then
    echo -e "${GREEN}✓${NC} $service - UP"
    relayers_up=$((relayers_up + 1))
  else
    echo -e "${RED}✗${NC} $service - DOWN (HTTP $response)"
    relayers_down=$((relayers_down + 1))
  fi
done

echo ""
echo "Relayer Services: $relayers_up UP / $relayers_down DOWN"

if [ $relayers_up -eq 0 ]; then
  echo -e "${RED}WARNING: No relayers are up! Messages will not be relayed!${NC}"
fi

echo ""

# Check Ethereum RPC
echo -e "${YELLOW}Checking Ethereum RPC...${NC}"
eth_block=$(curl -s -X POST "$ETHEREUM_RPC" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  2>/dev/null | jq -r '.result' || echo "error")

if [ "$eth_block" != "error" ] && [ "$eth_block" != "null" ]; then
  eth_block_decimal=$((16#${eth_block#0x}))
  echo -e "${GREEN}✓${NC} Ethereum RPC - UP (Block: $eth_block_decimal)"
else
  echo -e "${RED}✗${NC} Ethereum RPC - DOWN"
fi

echo ""

# Check Substrate RPC
echo -e "${YELLOW}Checking Substrate RPC...${NC}"
# Note: wscat or websocat required for websocket testing
# Simplified check here
echo "(WebSocket check requires wscat/websocat - skipping detailed check)"
echo -e "${YELLOW}?${NC} Substrate RPC - Check manually with: wscat -c $SUBSTRATE_RPC"

echo ""

# Check Ready Attestations
echo -e "${YELLOW}Checking Ready Attestations...${NC}"
first_attester=$(echo $ATTESTATION_SERVICES | awk '{print $1}')
ready_count=$(curl -s "https://$first_attester/attestations/ready" 2>/dev/null | jq -r '.count' || echo "error")

if [ "$ready_count" != "error" ] && [ "$ready_count" != "null" ]; then
  if [ "$ready_count" -gt 10 ]; then
    echo -e "${RED}WARNING: $ready_count messages ready but not relayed!${NC}"
    echo "This might indicate relayer issues or high gas prices"
  else
    echo -e "${GREEN}✓${NC} $ready_count messages ready to relay"
  fi
else
  echo -e "${YELLOW}?${NC} Could not fetch ready attestations"
fi

echo ""
echo "======================================"
echo "  Summary"
echo "======================================"

total_errors=0

if [ $attesters_up -lt 3 ]; then
  total_errors=$((total_errors + 1))
  echo -e "${RED}✗ Attestation threshold at risk${NC}"
fi

if [ $relayers_up -eq 0 ]; then
  total_errors=$((total_errors + 1))
  echo -e "${RED}✗ No relayers active${NC}"
fi

if [ "$eth_block" = "error" ]; then
  total_errors=$((total_errors + 1))
  echo -e "${RED}✗ Ethereum RPC unavailable${NC}"
fi

if [ "$ready_count" != "error" ] && [ "$ready_count" != "null" ] && [ "$ready_count" -gt 10 ]; then
  total_errors=$((total_errors + 1))
  echo -e "${YELLOW}⚠ Messages queued (may be normal)${NC}"
fi

if [ $total_errors -eq 0 ]; then
  echo -e "${GREEN}✓ All systems operational${NC}"
  exit 0
else
  echo -e "${RED}$total_errors issue(s) detected${NC}"
  exit 1
fi
