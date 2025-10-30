#!/bin/bash
# backup-to-b2.sh
# Automated backup of validators to Backblaze B2 object storage
# Usage: ./backup-to-b2.sh [inventory-file]

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
INVENTORY_FILE="${1:-validator-inventory.txt}"
BACKUP_BUCKET="etrid-validator-backups"
RETENTION_DAYS=7  # Keep backups for 7 days
LOCAL_BACKUP_DIR="/tmp/etrid-backups"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         Ëtrid Validator Backup to Backblaze B2            ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if b2 CLI is installed
if ! command -v b2 &> /dev/null; then
    echo -e "${RED}Error: Backblaze B2 CLI not installed${NC}"
    echo ""
    echo "Install with:"
    echo "  brew install b2-tools"
    echo ""
    echo "Or: pip install b2"
    exit 1
fi

echo -e "${GREEN}✓ B2 CLI installed${NC}"

# Check if authenticated
if ! b2 account info &> /dev/null; then
    echo -e "${YELLOW}⚠️  Not authenticated with Backblaze B2${NC}"
    echo ""
    echo "To authenticate:"
    echo "  1. Go to: https://secure.backblaze.com/app_keys.htm"
    echo "  2. Create new application key"
    echo "  3. Run: b2 authorize-account <keyID> <applicationKey>"
    echo ""
    exit 1
fi

echo -e "${GREEN}✓ Authenticated with B2${NC}"

# Check if bucket exists
if ! b2 ls "$BACKUP_BUCKET" &> /dev/null; then
    echo -e "${YELLOW}⚠️  Bucket '$BACKUP_BUCKET' not found${NC}"
    echo ""
    read -p "Create bucket? (yes/no): " confirm
    if [ "$confirm" = "yes" ]; then
        echo "Creating bucket..."
        b2 create-bucket "$BACKUP_BUCKET" allPrivate
        echo -e "${GREEN}✓ Bucket created${NC}"
    else
        echo "Aborted."
        exit 1
    fi
fi

echo -e "${GREEN}✓ Bucket '$BACKUP_BUCKET' exists${NC}"
echo ""

# Check inventory file
if [ ! -f "$INVENTORY_FILE" ]; then
    echo -e "${RED}Error: Inventory file not found: $INVENTORY_FILE${NC}"
    exit 1
fi

# Extract IPs
IPS=$(grep -v "^#" "$INVENTORY_FILE" | grep -v "^===" | grep -v "^Name" | awk '{print $2}' | grep -E '^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$')

if [ -z "$IPS" ]; then
    echo -e "${RED}No valid IPs found in inventory${NC}"
    exit 1
fi

VALIDATOR_COUNT=$(echo "$IPS" | wc -l | tr -d ' ')
echo "Found $VALIDATOR_COUNT validators to backup"
echo ""

# Create local backup directory
mkdir -p "$LOCAL_BACKUP_DIR"

# Backup timestamp
TIMESTAMP=$(date +%Y%m%d-%H%M%S)

echo "Starting backup process..."
echo "Timestamp: $TIMESTAMP"
echo "Retention: $RETENTION_DAYS days"
echo ""

successful=0
failed=0

for IP in $IPS; do
    # Get validator name
    VALIDATOR_NAME=$(grep "$IP" "$INVENTORY_FILE" | awk '{print $1}' | head -1)

    if [ -z "$VALIDATOR_NAME" ]; then
        VALIDATOR_NAME="unknown-$(echo $IP | tr '.' '-')"
    fi

    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Backing up: $VALIDATOR_NAME ($IP)"
    echo ""

    BACKUP_FILE="$LOCAL_BACKUP_DIR/${VALIDATOR_NAME}-${TIMESTAMP}.tar.gz"
    B2_PATH="b2://$BACKUP_BUCKET/${VALIDATOR_NAME}-${TIMESTAMP}.tar.gz"

    # Create compressed backup via SSH
    echo "  → Creating backup archive..."

    if timeout 300 ssh -o StrictHostKeyChecking=no -o ConnectTimeout=10 root@$IP \
        "tar czf - /var/lib/etrid/chains/mainnet/db 2>/dev/null" > "$BACKUP_FILE" 2>/dev/null; then

        BACKUP_SIZE=$(du -h "$BACKUP_FILE" | cut -f1)
        echo -e "  ${GREEN}✓ Archive created: $BACKUP_SIZE${NC}"

        # Upload to B2
        echo "  → Uploading to B2..."

        if b2 upload-file --noProgress "$BACKUP_BUCKET" "$BACKUP_FILE" "${VALIDATOR_NAME}-${TIMESTAMP}.tar.gz" 2>/dev/null; then
            echo -e "  ${GREEN}✓ Uploaded to B2${NC}"

            # Remove local backup
            rm "$BACKUP_FILE"

            successful=$((successful + 1))
        else
            echo -e "  ${RED}✗ Upload failed${NC}"
            failed=$((failed + 1))
        fi
    else
        echo -e "  ${RED}✗ Backup creation failed (unreachable or timeout)${NC}"
        failed=$((failed + 1))
    fi

    echo ""
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Backup Summary:"
echo "  ✓ Successful: $successful"
echo "  ✗ Failed: $failed"
echo "  Total: $VALIDATOR_COUNT"
echo ""

# Cleanup old backups (keep last 7 days)
echo "Cleaning up old backups (older than $RETENTION_DAYS days)..."

CUTOFF_DATE=$(date -v-${RETENTION_DAYS}d +%Y%m%d 2>/dev/null || date -d "$RETENTION_DAYS days ago" +%Y%m%d)

b2 ls --long "$BACKUP_BUCKET" | while read -r line; do
    FILENAME=$(echo "$line" | awk '{print $NF}')

    # Extract date from filename (format: validator-XX-YYYYMMDD-HHMMSS.tar.gz)
    FILE_DATE=$(echo "$FILENAME" | grep -oE '[0-9]{8}' | head -1)

    if [ ! -z "$FILE_DATE" ] && [ "$FILE_DATE" -lt "$CUTOFF_DATE" ]; then
        echo "  Deleting old backup: $FILENAME (from $FILE_DATE)"
        b2 delete-file-version "$FILENAME" $(echo "$line" | awk '{print $2}') 2>/dev/null || true
    fi
done

echo ""
echo -e "${GREEN}✓ Backup complete${NC}"
echo ""

# Show bucket info
echo "Backblaze B2 Bucket Status:"
b2 ls --long "$BACKUP_BUCKET" | head -20
echo ""

# Calculate storage costs
TOTAL_FILES=$(b2 ls "$BACKUP_BUCKET" | wc -l | tr -d ' ')
echo "Total backups in bucket: $TOTAL_FILES"
echo ""
echo "Estimated monthly cost:"
echo "  Storage: ~\$5-10/month (depends on total size)"
echo "  Downloads: \$0.01/GB (only when restoring)"
echo ""
echo "To restore a backup:"
echo "  b2 download-file-by-name $BACKUP_BUCKET <filename> <local-path>"
echo ""
echo "To schedule automatic backups, add to crontab:"
echo "  0 2 * * * /path/to/backup-to-b2.sh > /var/log/etrid-backup.log 2>&1"
echo ""
echo "This will run daily at 2 AM."
