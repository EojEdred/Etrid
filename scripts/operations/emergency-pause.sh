#!/bin/bash
# Emergency pause script - stops all relayer services
# Use this in case of security incident or critical issue

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
echo "  EMERGENCY PAUSE"
echo "======================================"
echo ""
echo -e "${RED}WARNING: This will stop all relayer services!${NC}"
echo "Messages will not be relayed until services are resumed."
echo ""
read -p "Are you sure you want to pause all relayers? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
  echo "Aborted."
  exit 0
fi

echo ""
echo "Pausing relayers..."
echo ""

success_count=0
fail_count=0

for host in $RELAYER_HOSTS; do
  echo "Stopping relayer on $host..."

  if ssh "${SSH_USER}@${host}" "pm2 stop relayer-service" 2>/dev/null; then
    echo -e "${GREEN}✓${NC} Stopped relayer on $host"
    success_count=$((success_count + 1))
  else
    echo -e "${RED}✗${NC} Failed to stop relayer on $host"
    fail_count=$((fail_count + 1))
  fi
done

echo ""
echo "======================================"
echo "  Summary"
echo "======================================"
echo "Stopped: $success_count"
echo "Failed: $fail_count"

if [ $fail_count -gt 0 ]; then
  echo -e "${YELLOW}Some relayers may still be running${NC}"
  exit 1
fi

echo ""
echo -e "${GREEN}All relayers have been paused${NC}"
echo ""
echo "Next steps:"
echo "1. Investigate the issue that required pausing"
echo "2. Post status update to Discord/Twitter"
echo "3. Resume with: ./scripts/operations/emergency-resume.sh"
echo ""
echo "Timestamp: $(date -u '+%Y-%m-%d %H:%M:%S UTC')"

# Log to file
echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Emergency pause executed" >> /tmp/bridge-emergency.log

exit 0
