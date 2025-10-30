#!/bin/bash
# monitor-validator-storage.sh
# Check storage usage on all validators and alert when thresholds are exceeded
# Usage: ./monitor-validator-storage.sh [inventory-file]

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Configuration
THRESHOLD_WARNING=70  # Alert when > 70% full
THRESHOLD_CRITICAL=85  # Critical alert when > 85% full
INVENTORY_FILE="${1:-validator-inventory.txt}"

# Check if inventory file exists
if [ ! -f "$INVENTORY_FILE" ]; then
    echo -e "${RED}Error: Inventory file not found: $INVENTORY_FILE${NC}"
    echo "Usage: $0 [inventory-file]"
    exit 1
fi

echo "╔════════════════════════════════════════════════════════════╗"
echo "║         Ëtrid Validator Storage Monitor                   ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Extract IPs from inventory file
# Skip comment lines and headers
IPS=$(grep -v "^#" "$INVENTORY_FILE" | grep -v "^===" | grep -v "^Name" | awk '{print $2}' | grep -E '^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$')

if [ -z "$IPS" ]; then
    echo -e "${RED}No valid IPs found in inventory file${NC}"
    exit 1
fi

echo "Checking storage on all validators..."
echo ""

total_validators=0
warning_count=0
critical_count=0

# Header
printf "%-20s %-15s %-10s %-10s %-10s %s\n" "Validator" "IP" "Used" "Total" "Usage %" "Status"
printf "%s\n" "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

for IP in $IPS; do
    total_validators=$((total_validators + 1))

    # Get validator name from inventory
    VALIDATOR_NAME=$(grep "$IP" "$INVENTORY_FILE" | awk '{print $1}')

    if [ -z "$VALIDATOR_NAME" ]; then
        VALIDATOR_NAME="unknown-$total_validators"
    fi

    # Get storage usage via SSH
    # Timeout after 5 seconds if unreachable
    STORAGE_INFO=$(timeout 5 ssh -o StrictHostKeyChecking=no -o ConnectTimeout=3 root@$IP \
        "df -h /var/lib/etrid 2>/dev/null | tail -1" 2>/dev/null)

    if [ $? -ne 0 ] || [ -z "$STORAGE_INFO" ]; then
        printf "%-20s %-15s %-10s %-10s %-10s %s\n" \
            "$VALIDATOR_NAME" "$IP" "N/A" "N/A" "N/A" "${RED}✗ Unreachable${NC}"
        continue
    fi

    # Parse storage info
    USED=$(echo "$STORAGE_INFO" | awk '{print $3}')
    TOTAL=$(echo "$STORAGE_INFO" | awk '{print $2}')
    USAGE_PERCENT=$(echo "$STORAGE_INFO" | awk '{print $5}' | sed 's/%//')

    # Determine status
    if [ "$USAGE_PERCENT" -gt "$THRESHOLD_CRITICAL" ]; then
        STATUS="${RED}✗ CRITICAL - Attach storage NOW!${NC}"
        critical_count=$((critical_count + 1))
    elif [ "$USAGE_PERCENT" -gt "$THRESHOLD_WARNING" ]; then
        STATUS="${YELLOW}⚠ WARNING - Consider attaching storage${NC}"
        warning_count=$((warning_count + 1))
    else
        STATUS="${GREEN}✓ OK${NC}"
    fi

    printf "%-20s %-15s %-10s %-10s %-10s %s\n" \
        "$VALIDATOR_NAME" "$IP" "$USED" "$TOTAL" "$USAGE_PERCENT%" "$STATUS"
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Summary:"
echo "  Total validators checked: $total_validators"
echo -e "  ${GREEN}OK: $((total_validators - warning_count - critical_count))${NC}"
echo -e "  ${YELLOW}Warnings: $warning_count${NC}"
echo -e "  ${RED}Critical: $critical_count${NC}"
echo ""

if [ $critical_count -gt 0 ]; then
    echo -e "${RED}⚠️  CRITICAL: $critical_count validator(s) need immediate storage expansion!${NC}"
    echo ""
    echo "To attach block storage, run:"
    echo "  ./attach-block-storage.sh <validator-name> <provider>"
    echo ""
fi

if [ $warning_count -gt 0 ]; then
    echo -e "${YELLOW}⚠️  WARNING: $warning_count validator(s) approaching storage limits${NC}"
    echo "Consider attaching block storage within the next few weeks."
    echo ""
fi

# Storage growth recommendations
echo "Storage Growth Recommendations:"
echo "  ✓ With pruning: ~10-20 GB per month"
echo "  ✓ Attach 500GB volume when usage > 300GB"
echo "  ✓ Cost: Hetzner \$25/mo, Vultr/DO \$50/mo"
echo ""
echo "For automated tiering, see: auto-tier-storage.sh"
