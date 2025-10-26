#!/bin/bash
# Ëtrid Governance Forum Restore Script
# Restores forum from backup archive
# Usage: ./scripts/restore-forum.sh <backup-file>

set -e

BACKUP_FILE=$1
COMPOSE_FILE="docker-compose.governance-forum.yml"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Ëtrid Governance Forum Restore${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Check arguments
if [ -z "$BACKUP_FILE" ]; then
    echo -e "${RED}Error: No backup file specified${NC}"
    echo "Usage: ./scripts/restore-forum.sh <backup-file>"
    echo ""
    echo "Available backups:"
    ls -lh /backups/forum/etrid-forum-*.tar.gz 2>/dev/null || echo "No backups found"
    exit 1
fi

# Check if backup file exists
if [ ! -f "$BACKUP_FILE" ]; then
    echo -e "${RED}Error: Backup file not found: $BACKUP_FILE${NC}"
    exit 1
fi

# Verify backup integrity
echo "Verifying backup integrity..."
if tar -tzf ${BACKUP_FILE} > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Backup integrity verified${NC}"
else
    echo -e "${RED}✗ Backup file is corrupted!${NC}"
    exit 1
fi

# Warning about data loss
echo ""
echo -e "${YELLOW}⚠️  WARNING: This will REPLACE all current forum data!${NC}"
echo -e "${YELLOW}   - Current database will be dropped${NC}"
echo -e "${YELLOW}   - All posts, users, and settings will be replaced${NC}"
echo -e "${YELLOW}   - This action cannot be undone${NC}"
echo ""
read -p "Are you sure you want to continue? Type 'yes' to confirm: " -r
echo
if [[ ! $REPLY == "yes" ]]; then
    echo "Restore cancelled"
    exit 0
fi

# Create temporary directory
TEMP_DIR=$(mktemp -d)
echo "Using temporary directory: ${TEMP_DIR}"

# Extract backup
echo ""
echo -e "${GREEN}[1/5] Extracting backup archive...${NC}"
tar -xzf ${BACKUP_FILE} -C ${TEMP_DIR}
echo "✓ Backup extracted"

# Stop services
echo ""
echo -e "${GREEN}[2/5] Stopping forum services...${NC}"
docker-compose -f ${COMPOSE_FILE} down
echo "✓ Services stopped"

# Start only database
echo ""
echo -e "${GREEN}[3/5] Starting database...${NC}"
docker-compose -f ${COMPOSE_FILE} up -d postgres
sleep 10  # Wait for PostgreSQL to be ready
echo "✓ Database started"

# Restore database
echo ""
echo -e "${GREEN}[4/5] Restoring database...${NC}"

# Drop existing database and recreate
docker exec etrid-forum-db psql -U discourse -c "DROP DATABASE IF EXISTS discourse;"
docker exec etrid-forum-db psql -U discourse -c "CREATE DATABASE discourse;"

# Find database backup file
DB_BACKUP=$(find ${TEMP_DIR} -name "db-*.sql" | head -n 1)
if [ -z "$DB_BACKUP" ]; then
    echo -e "${RED}Error: Database backup not found in archive${NC}"
    docker-compose -f ${COMPOSE_FILE} down
    rm -rf ${TEMP_DIR}
    exit 1
fi

# Restore database
cat ${DB_BACKUP} | docker exec -i etrid-forum-db psql -U discourse discourse
echo "✓ Database restored"

# Restore Discourse data
echo ""
echo -e "${GREEN}[5/5] Restoring Discourse data...${NC}"

DATA_BACKUP=$(find ${TEMP_DIR} -name "discourse-data-*.tar.gz" | head -n 1)
if [ -n "$DATA_BACKUP" ]; then
    docker-compose -f ${COMPOSE_FILE} up -d discourse
    sleep 5
    docker exec -i etrid-forum-web tar -xzf - -C / < ${DATA_BACKUP} 2>/dev/null || true
    echo "✓ Discourse data restored"
else
    echo -e "${YELLOW}Warning: Discourse data backup not found, skipping${NC}"
fi

# Restore environment configuration
ENV_BACKUP=$(find ${TEMP_DIR} -name "env-forum-*" | head -n 1)
if [ -n "$ENV_BACKUP" ]; then
    echo ""
    echo "Found environment configuration backup"
    read -p "Restore .env.forum? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cp ${ENV_BACKUP} .env.forum
        echo "✓ Environment configuration restored"
    fi
fi

# Restart all services
echo ""
echo "Restarting all services..."
docker-compose -f ${COMPOSE_FILE} down
docker-compose -f ${COMPOSE_FILE} up -d

# Wait for services to start
echo "Waiting for services to start (30 seconds)..."
sleep 30

# Cleanup
rm -rf ${TEMP_DIR}

# Verify restoration
echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Verifying Restoration${NC}"
echo -e "${GREEN}========================================${NC}"

# Check if containers are running
echo ""
echo "Container status:"
docker-compose -f ${COMPOSE_FILE} ps

# Check database connection
echo ""
echo "Database connection:"
if docker exec etrid-forum-db pg_isready -U discourse > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Database is ready${NC}"
else
    echo -e "${RED}✗ Database connection failed${NC}"
fi

# Check web service
echo ""
echo "Web service:"
if docker exec etrid-forum-web curl -f http://localhost/srv/status > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Web service is responding${NC}"
else
    echo -e "${YELLOW}⚠ Web service may still be starting...${NC}"
fi

# Summary
echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Restore Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo "Restored from: ${BACKUP_FILE}"
echo ""
echo "Next steps:"
echo "1. Visit your forum URL to verify"
echo "2. Log in with admin account"
echo "3. Check Admin → Logs for any errors"
echo "4. Test email delivery"
echo ""
echo -e "${YELLOW}Note: It may take a few minutes for all services to fully initialize${NC}"
