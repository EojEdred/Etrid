#!/bin/bash
# Emergency resume script - restarts all relayer services
# Use after emergency pause once issue is resolved

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Configuration
RELAYER_HOSTS=${RELAYER_HOSTS:-"relayer-1.etrid.io relayer-2.etrid.io relayer-3.etrid.io"}
SSH_USER=${SSH_USER:-"ubuntu"}

echo "======================================"
echo "  EMERGENCY RESUME"
echo "======================================"
echo ""
echo -e "${YELLOW}This will restart all relayer services${NC}"
echo ""
read -p "Confirm issue is resolved and safe to resume? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
  echo "Aborted."
  exit 0
fi

echo ""
echo "Resuming relayers..."
echo ""

success_count=0
fail_count=0

for host in $RELAYER_HOSTS; do
  echo "Starting relayer on $host..."

  if ssh "${SSH_USER}@${host}" "pm2 restart relayer-service" 2>/dev/null; then
    echo -e "${GREEN}✓${NC} Started relayer on $host"
    success_count=$((success_count + 1))

    # Wait a bit before starting next one (gradual rollout)
    sleep 5
  else
    echo -e "${RED}✗${NC} Failed to start relayer on $host"
    fail_count=$((fail_count + 1))
  fi
done

echo ""
echo "======================================"
echo "  Summary"
echo "======================================"
echo "Started: $success_count"
echo "Failed: $fail_count"

if [ $fail_count -gt 0 ]; then
  echo -e "${YELLOW}Some relayers failed to start${NC}"
  exit 1
fi

echo ""
echo -e "${GREEN}All relayers have been resumed${NC}"
echo ""
echo "Next steps:"
echo "1. Monitor logs for next 30 minutes: pm2 logs relayer-service"
echo "2. Verify messages are being relayed"
echo "3. Post resolution update to Discord/Twitter"
echo ""
echo "Timestamp: $(date -u '+%Y-%m-%d %H:%M:%S UTC')"

# Log to file
echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Emergency resume executed" >> /tmp/bridge-emergency.log

exit 0
