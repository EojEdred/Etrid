#!/bin/bash
# Ëtrid Governance Forum Quick Setup Script
# Interactive setup wizard for forum deployment
# Usage: ./scripts/setup-forum.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

clear
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Ëtrid Governance Forum Setup Wizard${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Check if Docker is installed
echo "Checking prerequisites..."
if ! command -v docker &> /dev/null; then
    echo -e "${RED}✗ Docker is not installed${NC}"
    echo "Please install Docker: https://docs.docker.com/get-docker/"
    exit 1
fi
echo -e "${GREEN}✓ Docker installed${NC}"

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo -e "${RED}✗ Docker Compose is not installed${NC}"
    echo "Please install Docker Compose: https://docs.docker.com/compose/install/"
    exit 1
fi
echo -e "${GREEN}✓ Docker Compose installed${NC}"

# Check if .env.forum already exists
if [ -f ".env.forum" ]; then
    echo ""
    echo -e "${YELLOW}Warning: .env.forum already exists${NC}"
    read -p "Overwrite existing configuration? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Setup cancelled. Using existing configuration."
        exit 0
    fi
fi

# Start configuration wizard
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Step 1: Basic Configuration${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Domain name
read -p "Enter your forum domain (e.g., forum.etrid.network): " DOMAIN
DOMAIN=${DOMAIN:-forum.etrid.network}

# Admin email
read -p "Enter admin email address: " ADMIN_EMAIL
ADMIN_EMAIL=${ADMIN_EMAIL:-admin@etrid.network}

# Database password
echo ""
echo "Generating secure database password..."
DB_PASSWORD=$(openssl rand -base64 32)
echo -e "${GREEN}✓ Database password generated${NC}"

# SMTP Configuration
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Step 2: Email Configuration (SMTP)${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Choose email provider:"
echo "1) SendGrid (Recommended - Free tier: 100 emails/day)"
echo "2) AWS SES"
echo "3) Mailgun"
echo "4) Custom SMTP"
echo "5) Skip (configure later)"
echo ""
read -p "Enter choice (1-5): " EMAIL_CHOICE

case $EMAIL_CHOICE in
    1)
        echo ""
        echo "SendGrid Setup:"
        echo "1. Sign up: https://sendgrid.com/free"
        echo "2. Create API Key: Settings → API Keys"
        echo "3. Choose 'Full Access'"
        echo ""
        read -p "Enter SendGrid API Key: " SMTP_PASSWORD
        SMTP_ADDRESS="smtp.sendgrid.net"
        SMTP_PORT="587"
        SMTP_USER="apikey"
        ;;
    2)
        echo ""
        echo "AWS SES Setup:"
        read -p "Enter AWS region (e.g., us-east-1): " AWS_REGION
        AWS_REGION=${AWS_REGION:-us-east-1}
        read -p "Enter SMTP username: " SMTP_USER
        read -p "Enter SMTP password: " SMTP_PASSWORD
        SMTP_ADDRESS="email-smtp.${AWS_REGION}.amazonaws.com"
        SMTP_PORT="587"
        ;;
    3)
        echo ""
        echo "Mailgun Setup:"
        read -p "Enter Mailgun domain: " MAILGUN_DOMAIN
        read -p "Enter SMTP password: " SMTP_PASSWORD
        SMTP_ADDRESS="smtp.mailgun.org"
        SMTP_PORT="587"
        SMTP_USER="postmaster@${MAILGUN_DOMAIN}"
        ;;
    4)
        echo ""
        echo "Custom SMTP Setup:"
        read -p "SMTP Address: " SMTP_ADDRESS
        read -p "SMTP Port: " SMTP_PORT
        SMTP_PORT=${SMTP_PORT:-587}
        read -p "SMTP Username: " SMTP_USER
        read -p "SMTP Password: " SMTP_PASSWORD
        ;;
    *)
        echo -e "${YELLOW}Skipping email configuration${NC}"
        SMTP_ADDRESS=""
        SMTP_PORT=""
        SMTP_USER=""
        SMTP_PASSWORD=""
        ;;
esac

# Blockchain Integration
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Step 3: Blockchain Integration${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
read -p "Enter Ëtrid RPC URL (default: http://localhost:9933): " RPC_URL
RPC_URL=${RPC_URL:-http://localhost:9933}

# Ports
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Step 4: Port Configuration${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
read -p "Forum HTTP port (default: 8080): " FORUM_PORT
FORUM_PORT=${FORUM_PORT:-8080}

# SSL/HTTPS
echo ""
read -p "Enable HTTPS? (requires SSL certificate) (y/N): " ENABLE_HTTPS
if [[ $ENABLE_HTTPS =~ ^[Yy]$ ]]; then
    FORCE_HTTPS="true"
    HTTP_PORT="80"
    HTTPS_PORT="443"
else
    FORCE_HTTPS="false"
    HTTP_PORT=""
    HTTPS_PORT=""
fi

# Write .env.forum file
echo ""
echo "Writing configuration to .env.forum..."

cat > .env.forum <<EOF
# Ëtrid Governance Forum Configuration
# Generated: $(date)

# Core Settings
DISCOURSE_HOSTNAME=${DOMAIN}
ADMIN_EMAIL=${ADMIN_EMAIL}

# Database
POSTGRES_PASSWORD=${DB_PASSWORD}

# Email (SMTP)
SMTP_ADDRESS=${SMTP_ADDRESS}
SMTP_PORT=${SMTP_PORT}
SMTP_USER=${SMTP_USER}
SMTP_PASSWORD=${SMTP_PASSWORD}

# Ports
FORUM_PORT=${FORUM_PORT}
HTTP_PORT=${HTTP_PORT}
HTTPS_PORT=${HTTPS_PORT}

# HTTPS
FORCE_HTTPS=${FORCE_HTTPS}

# Blockchain Integration
ETRID_RPC_URL=${RPC_URL}

# Optional: S3 & CDN (configure later if needed)
USE_S3=false
S3_BUCKET=
S3_REGION=
S3_ACCESS_KEY=
S3_SECRET_KEY=
CDN_URL=

# Backup
BACKUP_LOCATION=/backups
BACKUP_FREQUENCY=daily
BACKUP_RETENTION_DAYS=7
EOF

echo -e "${GREEN}✓ Configuration saved to .env.forum${NC}"

# Summary
echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Configuration Summary${NC}"
echo -e "${GREEN}========================================${NC}"
echo "Domain: ${DOMAIN}"
echo "Admin Email: ${ADMIN_EMAIL}"
echo "HTTP Port: ${FORUM_PORT}"
echo "HTTPS: ${FORCE_HTTPS}"
echo "RPC URL: ${RPC_URL}"
echo ""

# Ask to start services
read -p "Start forum services now? (y/N): " START_NOW
echo ""

if [[ $START_NOW =~ ^[Yy]$ ]]; then
    echo "Starting forum services..."
    echo ""

    # Start services
    docker-compose -f docker-compose.governance-forum.yml up -d

    echo ""
    echo -e "${GREEN}✓ Services started${NC}"
    echo ""
    echo "Waiting for services to initialize (60 seconds)..."

    # Progress bar
    for i in {1..60}; do
        echo -n "."
        sleep 1
    done
    echo ""

    # Check status
    echo ""
    echo "Service Status:"
    docker-compose -f docker-compose.governance-forum.yml ps

    # Access instructions
    echo ""
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}Forum is Ready!${NC}"
    echo -e "${GREEN}========================================${NC}"
    echo ""
    echo "Access your forum at:"
    if [[ $FORCE_HTTPS == "true" ]]; then
        echo "  https://${DOMAIN}"
    else
        echo "  http://${DOMAIN}:${FORUM_PORT}"
        echo "  or http://$(hostname -I | awk '{print $1}'):${FORUM_PORT}"
    fi
    echo ""
    echo "Next Steps:"
    echo "1. Visit the URL above"
    echo "2. Register admin account using: ${ADMIN_EMAIL}"
    echo "3. Check your email for confirmation link"
    echo "4. Complete the setup wizard"
    echo "5. Configure categories and templates"
    echo ""
    echo "Useful Commands:"
    echo "  View logs: docker-compose -f docker-compose.governance-forum.yml logs -f"
    echo "  Stop forum: docker-compose -f docker-compose.governance-forum.yml down"
    echo "  Restart forum: docker-compose -f docker-compose.governance-forum.yml restart"
    echo "  Backup forum: ./scripts/backup-forum.sh"
    echo ""
    echo "Documentation: ai-devs/GOVERNANCE_FORUM_GUIDE.md"

else
    echo "Setup complete! Configuration saved to .env.forum"
    echo ""
    echo "To start the forum later, run:"
    echo "  docker-compose -f docker-compose.governance-forum.yml up -d"
    echo ""
    echo "Documentation: ai-devs/GOVERNANCE_FORUM_GUIDE.md"
fi

echo ""
echo -e "${GREEN}Setup wizard complete!${NC}"
