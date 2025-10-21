#!/bin/bash
# Rolling restart of attestation services
# Restarts one attester at a time with delay to maintain threshold

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Configuration
ATTESTER_HOSTS=${ATTESTER_HOSTS:-"attestation-0.etrid.io attestation-1.etrid.io attestation-2.etrid.io attestation-3.etrid.io attestation-4.etrid.io"}
SSH_USER=${SSH_USER:-"ubuntu"}
SOAK_TIME=${SOAK_TIME:-30} # Seconds to wait between restarts

echo "======================================"
echo "  Rolling Attestation Service Restart"
echo "======================================"
echo ""
echo "This will restart attesters one at a time with $SOAK_TIME second intervals"
echo "to maintain the 3-of-5 threshold."
echo ""
read -p "Continue? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
  echo "Aborted."
  exit 0
fi

echo ""

success_count=0
fail_count=0
total=0

for host in $ATTESTER_HOSTS; do
  total=$((total + 1))
  echo "[$total/5] Restarting attester on $host..."

  if ssh "${SSH_USER}@${host}" "pm2 restart attestation-service" 2>/dev/null; then
    echo -e "${GREEN}✓${NC} Restarted $host"
    success_count=$((success_count + 1))

    # Check health
    sleep 5
    response=$(curl -s -o /dev/null -w "%{http_code}" "https://$host/health" 2>/dev/null || echo "000")

    if [ "$response" = "200" ]; then
      echo -e "${GREEN}✓${NC} $host is healthy"
    else
      echo -e "${YELLOW}⚠${NC} $host may not be fully healthy yet"
    fi

    # Wait before next restart (except for last one)
    if [ $total -lt 5 ]; then
      echo "Waiting $SOAK_TIME seconds before next restart..."
      sleep $SOAK_TIME
    fi
  else
    echo -e "${RED}✗${NC} Failed to restart $host"
    fail_count=$((fail_count + 1))
    echo -e "${YELLOW}Continuing with next attester...${NC}"
  fi

  echo ""
done

echo "======================================"
echo "  Summary"
echo "======================================"
echo "Restarted: $success_count/5"
echo "Failed: $fail_count"

if [ $fail_count -gt 2 ]; then
  echo -e "${RED}WARNING: Threshold may be at risk!${NC}"
  exit 1
elif [ $fail_count -gt 0 ]; then
  echo -e "${YELLOW}Some attesters failed but threshold should be maintained${NC}"
  exit 0
else
  echo -e "${GREEN}All attesters restarted successfully${NC}"
  exit 0
fi
