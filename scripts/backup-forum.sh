#!/bin/bash
# Ëtrid Governance Forum Backup Script
# Creates compressed backup of PostgreSQL database and Discourse data
# Usage: ./scripts/backup-forum.sh

set -e

# Configuration
BACKUP_DIR="${BACKUP_DIR:-/backups/forum}"
DATE=$(date +%Y%m%d-%H%M%S)
BACKUP_FILE="etrid-forum-${DATE}.tar.gz"
COMPOSE_FILE="docker-compose.governance-forum.yml"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Ëtrid Governance Forum Backup${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Check if Docker Compose file exists
if [ ! -f "$COMPOSE_FILE" ]; then
    echo -e "${RED}Error: $COMPOSE_FILE not found${NC}"
    exit 1
fi

# Check if containers are running
if ! docker-compose -f $COMPOSE_FILE ps | grep -q "Up"; then
    echo -e "${YELLOW}Warning: Forum containers may not be running${NC}"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Create backup directory
echo "Creating backup directory: ${BACKUP_DIR}"
mkdir -p ${BACKUP_DIR}

# Temporary directory for backup components
TEMP_DIR=$(mktemp -d)
echo "Using temporary directory: ${TEMP_DIR}"

# Backup database
echo ""
echo -e "${GREEN}[1/4] Backing up PostgreSQL database...${NC}"
docker exec etrid-forum-db pg_dump -U discourse discourse > ${TEMP_DIR}/db-${DATE}.sql
DB_SIZE=$(du -h ${TEMP_DIR}/db-${DATE}.sql | cut -f1)
echo "Database backup size: ${DB_SIZE}"

# Backup Discourse data (uploads, backups, etc.)
echo ""
echo -e "${GREEN}[2/4] Backing up Discourse data...${NC}"
docker exec etrid-forum-web tar -czf - /shared 2>/dev/null > ${TEMP_DIR}/discourse-data-${DATE}.tar.gz || true
DATA_SIZE=$(du -h ${TEMP_DIR}/discourse-data-${DATE}.tar.gz | cut -f1)
echo "Discourse data backup size: ${DATA_SIZE}"

# Backup environment configuration
echo ""
echo -e "${GREEN}[3/4] Backing up configuration...${NC}"
if [ -f ".env.forum" ]; then
    cp .env.forum ${TEMP_DIR}/env-forum-${DATE}
    echo "Configuration backed up"
else
    echo -e "${YELLOW}Warning: .env.forum not found, skipping${NC}"
fi

# Create combined archive
echo ""
echo -e "${GREEN}[4/4] Creating combined archive...${NC}"
cd ${TEMP_DIR}
tar -czf ${BACKUP_DIR}/${BACKUP_FILE} * 2>/dev/null
cd - > /dev/null

BACKUP_SIZE=$(du -h ${BACKUP_DIR}/${BACKUP_FILE} | cut -f1)
echo "Final backup size: ${BACKUP_SIZE}"

# Cleanup temporary directory
rm -rf ${TEMP_DIR}

# Upload to S3 (optional)
if [ -n "$BACKUP_S3_BUCKET" ]; then
    echo ""
    echo -e "${GREEN}Uploading to S3...${NC}"

    # Check if AWS CLI is installed
    if command -v aws &> /dev/null; then
        aws s3 cp ${BACKUP_DIR}/${BACKUP_FILE} s3://${BACKUP_S3_BUCKET}/forum-backups/ && \
            echo -e "${GREEN}✓ Uploaded to S3: s3://${BACKUP_S3_BUCKET}/forum-backups/${BACKUP_FILE}${NC}" || \
            echo -e "${YELLOW}Warning: S3 upload failed${NC}"
    else
        echo -e "${YELLOW}Warning: AWS CLI not installed, skipping S3 upload${NC}"
    fi
fi

# Keep only last N days of backups (default: 7)
RETENTION_DAYS="${BACKUP_RETENTION_DAYS:-7}"
echo ""
echo "Cleaning old backups (keeping last ${RETENTION_DAYS} days)..."
find ${BACKUP_DIR} -name "etrid-forum-*.tar.gz" -mtime +${RETENTION_DAYS} -delete
OLD_BACKUPS_COUNT=$(find ${BACKUP_DIR} -name "etrid-forum-*.tar.gz" | wc -l)
echo "Current backups in retention: ${OLD_BACKUPS_COUNT}"

# Summary
echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Backup Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo "Backup file: ${BACKUP_DIR}/${BACKUP_FILE}"
echo "Backup size: ${BACKUP_SIZE}"
echo "Timestamp: ${DATE}"

# Verify backup integrity
echo ""
echo "Verifying backup integrity..."
if tar -tzf ${BACKUP_DIR}/${BACKUP_FILE} > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Backup integrity verified${NC}"
else
    echo -e "${RED}✗ Backup integrity check failed!${NC}"
    exit 1
fi

# Log backup to file
echo "${DATE} - Backup created: ${BACKUP_FILE} (${BACKUP_SIZE})" >> ${BACKUP_DIR}/backup.log

echo ""
echo -e "${GREEN}Backup completed successfully!${NC}"
