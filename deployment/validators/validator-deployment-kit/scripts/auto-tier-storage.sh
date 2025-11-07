#!/bin/bash
# auto-tier-storage.sh
# Automatically move old blockchain data to attached storage when local storage fills up
# Run this on each validator VM that has attached block storage
# Usage: ./auto-tier-storage.sh

set -e

# Configuration
LOCAL_PATH="/var/lib/etrid"
ARCHIVE_PATH="/mnt/blockchain-archive"
LOCAL_THRESHOLD=75  # Move data when local > 75% full
DAYS_OLD=90  # Move files older than 90 days

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         Ëtrid Storage Auto-Tiering Script                 ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if running on a validator VM
if [ ! -d "$LOCAL_PATH" ]; then
    echo -e "${RED}Error: Validator data directory not found: $LOCAL_PATH${NC}"
    echo "This script must be run on a validator VM"
    exit 1
fi

# Check if we have attached storage
if [ ! -d "$ARCHIVE_PATH" ]; then
    echo -e "${YELLOW}No archive storage attached at $ARCHIVE_PATH${NC}"
    echo ""
    echo "This validator is using local storage only."
    echo "To attach block storage:"
    echo "  1. Create volume via provider console (Hetzner/Vultr/DO)"
    echo "  2. Attach volume to this VM"
    echo "  3. Run: ./attach-block-storage.sh"
    echo ""

    # Show current usage
    echo "Current storage usage:"
    df -h "$LOCAL_PATH"
    exit 0
fi

echo -e "${GREEN}✓ Archive storage found at $ARCHIVE_PATH${NC}"
echo ""

# Check local storage usage
USAGE=$(df -h "$LOCAL_PATH" | tail -1 | awk '{print $5}' | sed 's/%//')
USED=$(df -h "$LOCAL_PATH" | tail -1 | awk '{print $3}')
TOTAL=$(df -h "$LOCAL_PATH" | tail -1 | awk '{print $2}')

echo "Local Storage Status:"
echo "  Path: $LOCAL_PATH"
echo "  Used: $USED / $TOTAL"
echo "  Usage: $USAGE%"
echo "  Threshold: $LOCAL_THRESHOLD%"
echo ""

if [ "$USAGE" -lt "$LOCAL_THRESHOLD" ]; then
    echo -e "${GREEN}✓ Local storage is below threshold (${USAGE}% < ${LOCAL_THRESHOLD}%)${NC}"
    echo "No tiering needed at this time."
    echo ""
    echo "Archive storage status:"
    df -h "$ARCHIVE_PATH"
    exit 0
fi

echo -e "${YELLOW}⚠️  Local storage at ${USAGE}%, exceeds threshold of ${LOCAL_THRESHOLD}%${NC}"
echo "Moving old data to archive storage..."
echo ""

# Safety check - ensure validator is stopped
if systemctl is-active --quiet etrid-validator; then
    echo -e "${RED}Warning: Validator is currently running!${NC}"
    read -p "Stop validator to safely move data? (yes/no): " confirm
    if [ "$confirm" = "yes" ]; then
        echo "Stopping validator..."
        systemctl stop etrid-validator
        echo -e "${GREEN}✓ Validator stopped${NC}"
    else
        echo "Aborted. Please stop the validator manually first."
        exit 1
    fi
fi

# Find and move old data
echo "Finding blockchain data older than $DAYS_OLD days..."

CHAIN_DB="$LOCAL_PATH/chains/mainnet/db"
if [ ! -d "$CHAIN_DB" ]; then
    echo -e "${RED}Error: Chain database not found at $CHAIN_DB${NC}"
    exit 1
fi

# Create archive directory structure
mkdir -p "$ARCHIVE_PATH/db"

# Count files to move
FILE_COUNT=$(find "$CHAIN_DB" -type f -mtime +$DAYS_OLD 2>/dev/null | wc -l)

if [ "$FILE_COUNT" -eq 0 ]; then
    echo "No files older than $DAYS_OLD days found."
    echo "Consider:"
    echo "  - Reducing DAYS_OLD threshold"
    echo "  - Enabling pruning mode"
    echo "  - Attaching larger storage volume"
    exit 0
fi

echo "Found $FILE_COUNT files to archive"
echo ""

# Calculate size to move
SIZE_TO_MOVE=$(find "$CHAIN_DB" -type f -mtime +$DAYS_OLD -exec du -ch {} + 2>/dev/null | grep total | awk '{print $1}')
echo "Size to move: $SIZE_TO_MOVE"
echo ""

read -p "Continue with archiving? (yes/no): " confirm
if [ "$confirm" != "yes" ]; then
    echo "Aborted."
    exit 0
fi

# Move old files to archive
echo "Moving files to $ARCHIVE_PATH..."

moved_count=0
failed_count=0

find "$CHAIN_DB" -type f -mtime +$DAYS_OLD 2>/dev/null | while read -r file; do
    # Get relative path
    rel_path="${file#$CHAIN_DB/}"
    archive_file="$ARCHIVE_PATH/db/$rel_path"

    # Create directory structure
    mkdir -p "$(dirname "$archive_file")"

    # Move file
    if mv "$file" "$archive_file" 2>/dev/null; then
        moved_count=$((moved_count + 1))
        if [ $((moved_count % 100)) -eq 0 ]; then
            echo "  Moved $moved_count files..."
        fi
    else
        failed_count=$((failed_count + 1))
    fi
done

echo ""
echo -e "${GREEN}✓ Archiving complete${NC}"
echo ""

# Show new usage
echo "Updated storage usage:"
echo ""
echo "Local storage:"
df -h "$LOCAL_PATH"
echo ""
echo "Archive storage:"
df -h "$ARCHIVE_PATH"
echo ""

# Restart validator
read -p "Restart validator now? (yes/no): " confirm
if [ "$confirm" = "yes" ]; then
    echo "Starting validator..."
    systemctl start etrid-validator
    sleep 3

    if systemctl is-active --quiet etrid-validator; then
        echo -e "${GREEN}✓ Validator restarted successfully${NC}"
    else
        echo -e "${RED}✗ Validator failed to start${NC}"
        echo "Check logs: journalctl -u etrid-validator -f"
    fi
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Auto-Tiering Complete"
echo ""
echo "To run this automatically, add to crontab:"
echo "  0 3 * * * /path/to/auto-tier-storage.sh > /var/log/etrid-tiering.log 2>&1"
echo ""
echo "This will run daily at 3 AM."
