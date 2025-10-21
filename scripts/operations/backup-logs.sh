#!/bin/bash
# Backup logs from all services
# Optionally uploads to S3 if configured

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Configuration
BACKUP_DIR=${BACKUP_DIR:-"/tmp/bridge-logs-backup"}
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
ATTESTER_HOSTS=${ATTESTER_HOSTS:-"attestation-0.etrid.io attestation-1.etrid.io attestation-2.etrid.io attestation-3.etrid.io attestation-4.etrid.io"}
RELAYER_HOSTS=${RELAYER_HOSTS:-"relayer-1.etrid.io relayer-2.etrid.io relayer-3.etrid.io"}
SSH_USER=${SSH_USER:-"ubuntu"}
S3_BUCKET=${S3_BUCKET:-""} # Set to upload to S3

echo "======================================"
echo "  Bridge Log Backup"
echo "======================================"
echo ""
echo "Backing up logs to: $BACKUP_DIR/$TIMESTAMP"
echo ""

# Create backup directory
mkdir -p "$BACKUP_DIR/$TIMESTAMP"

# Backup attestation service logs
echo -e "${YELLOW}Backing up attestation service logs...${NC}"
idx=0
for host in $ATTESTER_HOSTS; do
  echo "  Fetching from $host..."
  ssh "${SSH_USER}@${host}" "pm2 logs attestation-service --lines 1000 --nostream" > "$BACKUP_DIR/$TIMESTAMP/attestation-$idx.log" 2>&1 || true
  idx=$((idx + 1))
done

# Backup relayer service logs
echo -e "${YELLOW}Backing up relayer service logs...${NC}"
idx=1
for host in $RELAYER_HOSTS; do
  echo "  Fetching from $host..."
  ssh "${SSH_USER}@${host}" "pm2 logs relayer-service --lines 1000 --nostream" > "$BACKUP_DIR/$TIMESTAMP/relayer-$idx.log" 2>&1 || true
  idx=$((idx + 1))
done

# Create archive
echo ""
echo "Creating archive..."
cd "$BACKUP_DIR"
tar -czf "bridge-logs-$TIMESTAMP.tar.gz" "$TIMESTAMP"
archive_path="$BACKUP_DIR/bridge-logs-$TIMESTAMP.tar.gz"
echo -e "${GREEN}✓${NC} Created: $archive_path"

# Calculate size
size=$(du -h "$archive_path" | cut -f1)
echo "  Size: $size"

# Upload to S3 if configured
if [ -n "$S3_BUCKET" ]; then
  echo ""
  echo "Uploading to S3..."
  if command -v aws &> /dev/null; then
    aws s3 cp "$archive_path" "s3://$S3_BUCKET/bridge-logs/bridge-logs-$TIMESTAMP.tar.gz"
    echo -e "${GREEN}✓${NC} Uploaded to s3://$S3_BUCKET/bridge-logs/"
  else
    echo -e "${YELLOW}aws CLI not found, skipping S3 upload${NC}"
  fi
fi

# Cleanup old backups (keep last 7 days)
echo ""
echo "Cleaning up old backups..."
find "$BACKUP_DIR" -name "bridge-logs-*.tar.gz" -mtime +7 -delete
echo -e "${GREEN}✓${NC} Removed backups older than 7 days"

# Remove temporary directory
rm -rf "$BACKUP_DIR/$TIMESTAMP"

echo ""
echo "======================================"
echo "  Backup Complete"
echo "======================================"
echo "Archive: $archive_path"
echo ""

exit 0
