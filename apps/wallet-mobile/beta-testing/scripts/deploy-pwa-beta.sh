#!/bin/bash

################################################################################
# PWA Beta Deployment Script
#
# Automates deploying PWA to beta environment (Vercel/Netlify)
#
# Usage:
#   ./deploy-pwa-beta.sh [environment]
#
# Example:
#   ./deploy-pwa-beta.sh beta
#   ./deploy-pwa-beta.sh staging
#
# Prerequisites:
#   - Node.js 18+
#   - Vercel CLI (npm install -g vercel)
#   - Git
################################################################################

set -e  # Exit on error
set -u  # Exit on undefined variable

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_DIR="/home/user/Etrid/apps/wallet-mobile/etrid-wallet"
ENVIRONMENT="${1:-beta}"
BRANCH_NAME=""

# Set branch based on environment
case $ENVIRONMENT in
    beta)
        BRANCH_NAME="beta"
        ;;
    staging)
        BRANCH_NAME="staging"
        ;;
    dev)
        BRANCH_NAME="develop"
        ;;
    *)
        echo "Invalid environment: $ENVIRONMENT"
        echo "Valid options: beta, staging, dev"
        exit 1
        ;;
esac

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    # Check Node.js
    if ! command -v node &> /dev/null; then
        log_error "Node.js not found"
        exit 1
    fi

    NODE_VERSION=$(node --version)
    log_info "Node.js version: $NODE_VERSION"

    # Check npm
    if ! command -v npm &> /dev/null; then
        log_error "npm not found"
        exit 1
    fi

    # Check Vercel CLI
    if ! command -v vercel &> /dev/null; then
        log_warning "Vercel CLI not found. Installing..."
        npm install -g vercel
    fi

    # Check git
    if ! command -v git &> /dev/null; then
        log_error "git not found"
        exit 1
    fi

    log_success "Prerequisites check passed"
}

# Check git status
check_git_status() {
    log_info "Checking git status..."
    cd "$PROJECT_DIR"

    # Check if on correct branch
    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
    if [ "$CURRENT_BRANCH" != "$BRANCH_NAME" ]; then
        log_warning "Not on $BRANCH_NAME branch (currently on $CURRENT_BRANCH)"
        read -p "Switch to $BRANCH_NAME branch? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            git checkout "$BRANCH_NAME"
            log_success "Switched to $BRANCH_NAME branch"
        else
            log_error "Deployment cancelled"
            exit 1
        fi
    fi

    # Check for uncommitted changes
    if [ -n "$(git status --porcelain)" ]; then
        log_warning "Uncommitted changes detected"
        git status --short
        read -p "Commit changes? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            read -p "Commit message: " COMMIT_MSG
            git add .
            git commit -m "$COMMIT_MSG"
            log_success "Changes committed"
        fi
    fi

    # Pull latest changes
    log_info "Pulling latest changes..."
    git pull origin "$BRANCH_NAME" --rebase

    log_success "Git status OK"
}

# Install dependencies
install_dependencies() {
    log_info "Installing dependencies..."
    cd "$PROJECT_DIR"

    npm ci --silent

    log_success "Dependencies installed"
}

# Run linting
run_lint() {
    log_info "Running ESLint..."
    cd "$PROJECT_DIR"

    set +e  # Don't exit on lint warnings
    npm run lint --silent
    LINT_RESULT=$?
    set -e

    if [ $LINT_RESULT -eq 0 ]; then
        log_success "Linting passed"
    else
        log_warning "Linting found issues"
        read -p "Continue anyway? (y/n) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
}

# Run type checking
run_type_check() {
    log_info "Running TypeScript type check..."
    cd "$PROJECT_DIR"

    set +e  # Don't exit on type errors
    npm run type-check --silent
    TYPE_RESULT=$?
    set -e

    if [ $TYPE_RESULT -eq 0 ]; then
        log_success "Type checking passed"
    else
        log_warning "Type checking found errors"
        read -p "Continue anyway? (y/n) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
}

# Run tests
run_tests() {
    log_info "Running tests..."
    cd "$PROJECT_DIR"

    set +e  # Don't exit on test failure
    npm run test:ci --silent
    TEST_RESULT=$?
    set -e

    if [ $TEST_RESULT -eq 0 ]; then
        log_success "Tests passed"
    else
        log_warning "Tests failed"
        read -p "Continue anyway? (y/n) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
}

# Build application
build_app() {
    log_info "Building application for $ENVIRONMENT..."
    cd "$PROJECT_DIR"

    # Set environment variables
    export NEXT_PUBLIC_ENV="$ENVIRONMENT"
    export NEXT_PUBLIC_VERCEL_ENV="$ENVIRONMENT"

    npm run build

    if [ $? -ne 0 ]; then
        log_error "Build failed"
        exit 1
    fi

    log_success "Build completed"
}

# Deploy to Vercel
deploy_to_vercel() {
    log_info "Deploying to Vercel ($ENVIRONMENT)..."
    cd "$PROJECT_DIR"

    # Deploy based on environment
    if [ "$ENVIRONMENT" = "beta" ]; then
        # Production deployment to beta subdomain
        DEPLOY_URL=$(vercel --prod --yes 2>&1 | grep -o 'https://[^ ]*' | tail -1)
    else
        # Preview deployment
        DEPLOY_URL=$(vercel --yes 2>&1 | grep -o 'https://[^ ]*' | tail -1)
    fi

    if [ -z "$DEPLOY_URL" ]; then
        log_error "Deployment failed - no URL returned"
        exit 1
    fi

    log_success "Deployed to: $DEPLOY_URL"
    echo "$DEPLOY_URL" > .last-deploy-url
}

# Run smoke tests
run_smoke_tests() {
    log_info "Running smoke tests..."

    if [ -f ".last-deploy-url" ]; then
        DEPLOY_URL=$(cat .last-deploy-url)

        # Test homepage loads
        HTTP_CODE=$(curl -o /dev/null -s -w "%{http_code}" "$DEPLOY_URL")
        if [ "$HTTP_CODE" = "200" ]; then
            log_success "Homepage loads (HTTP $HTTP_CODE)"
        else
            log_warning "Homepage returned HTTP $HTTP_CODE"
        fi

        # Test critical pages
        PAGES=("/wallet" "/trade" "/nft")
        for PAGE in "${PAGES[@]}"; do
            HTTP_CODE=$(curl -o /dev/null -s -w "%{http_code}" "$DEPLOY_URL$PAGE")
            if [ "$HTTP_CODE" = "200" ] || [ "$HTTP_CODE" = "308" ]; then
                log_success "$PAGE loads (HTTP $HTTP_CODE)"
            else
                log_warning "$PAGE returned HTTP $HTTP_CODE"
            fi
        done
    else
        log_warning "Deploy URL not found, skipping smoke tests"
    fi
}

# Configure feature flags
configure_feature_flags() {
    log_info "Configuring feature flags for $ENVIRONMENT..."

    case $ENVIRONMENT in
        beta)
            log_info "Beta environment - enabling all beta features"
            # Feature flags are set via environment variables in Vercel
            ;;
        staging)
            log_info "Staging environment - stable features only"
            ;;
        dev)
            log_info "Dev environment - all features enabled"
            ;;
    esac

    log_success "Feature flags configured"
}

# Update DNS (if needed)
update_dns() {
    if [ "$ENVIRONMENT" = "beta" ]; then
        log_info "Checking DNS configuration..."

        # Verify beta.wallet.etrid.com points to Vercel
        RESOLVED_IP=$(dig +short beta.wallet.etrid.com | tail -1)
        if [ -n "$RESOLVED_IP" ]; then
            log_success "DNS configured: beta.wallet.etrid.com → $RESOLVED_IP"
        else
            log_warning "DNS not configured for beta.wallet.etrid.com"
            log_info "Add CNAME record: beta.wallet.etrid.com → cname.vercel-dns.com"
        fi
    fi
}

# Generate release notes
generate_release_notes() {
    log_info "Generating release notes..."

    COMMIT_HASH=$(git rev-parse --short HEAD)
    NOTES_FILE="$PROJECT_DIR/PWA_RELEASE_NOTES_$(date +%Y%m%d).md"

    cat > "$NOTES_FILE" <<EOF
# PWA Beta Release

**Environment:** $ENVIRONMENT
**Branch:** $BRANCH_NAME
**Commit:** $COMMIT_HASH
**Date:** $(date +"%Y-%m-%d %H:%M:%S")
**URL:** $(cat .last-deploy-url 2>/dev/null || echo "N/A")

## Changes Since Last Release
$(git log --oneline -n 10)

## What to Test
- [ ] Homepage loads correctly
- [ ] Wallet creation works
- [ ] Transactions functional
- [ ] AU Bloccard accessible
- [ ] Trading features work
- [ ] NFT gallery displays
- [ ] All pages responsive
- [ ] PWA install prompt works
- [ ] Offline mode works
- [ ] No console errors

## Feature Flags Status
- AU Bloccard: $([ "$ENVIRONMENT" = "beta" ] && echo "Enabled" || echo "Check config")
- NFT Marketplace: $([ "$ENVIRONMENT" = "beta" ] && echo "Enabled" || echo "Check config")
- Advanced Trading: $([ "$ENVIRONMENT" = "beta" ] && echo "Enabled" || echo "Check config")

## Feedback
Report issues:
- In-app feedback widget
- Email: beta@etrid.com
- Discord: #pwa-beta

---
Generated by deploy-pwa-beta.sh
EOF

    log_success "Release notes: $NOTES_FILE"
}

# Send notifications
send_notifications() {
    log_info "Sending notifications..."

    DEPLOY_URL=$(cat .last-deploy-url 2>/dev/null || echo "N/A")
    COMMIT_HASH=$(git rev-parse --short HEAD)

    # Slack notification
    if [ -n "${SLACK_WEBHOOK_URL:-}" ]; then
        curl -X POST "$SLACK_WEBHOOK_URL" \
            -H 'Content-Type: application/json' \
            -d "{
                \"text\": \"🌐 PWA Beta Deployed\",
                \"blocks\": [
                    {
                        \"type\": \"section\",
                        \"text\": {
                            \"type\": \"mrkdwn\",
                            \"text\": \"*PWA Deployment Complete*\n\nEnvironment: $ENVIRONMENT\nBranch: $BRANCH_NAME\nCommit: $COMMIT_HASH\"
                        }
                    },
                    {
                        \"type\": \"actions\",
                        \"elements\": [
                            {
                                \"type\": \"button\",
                                \"text\": {
                                    \"type\": \"plain_text\",
                                    \"text\": \"Open App\"
                                },
                                \"url\": \"$DEPLOY_URL\"
                            }
                        ]
                    }
                ]
            }" \
            --silent --output /dev/null

        log_success "Slack notification sent"
    fi

    # Discord notification (optional)
    if [ -n "${DISCORD_WEBHOOK_URL:-}" ]; then
        curl -X POST "$DISCORD_WEBHOOK_URL" \
            -H 'Content-Type: application/json' \
            -d "{
                \"content\": \"🌐 **PWA Beta Deployed** to \`$ENVIRONMENT\`\",
                \"embeds\": [{
                    \"title\": \"Deployment Successful\",
                    \"description\": \"Branch: $BRANCH_NAME\\nCommit: $COMMIT_HASH\",
                    \"url\": \"$DEPLOY_URL\",
                    \"color\": 5814783
                }]
            }" \
            --silent --output /dev/null
    fi
}

# Push to git
push_to_git() {
    log_info "Pushing to git..."
    cd "$PROJECT_DIR"

    git push origin "$BRANCH_NAME"

    log_success "Pushed to $BRANCH_NAME"
}

# Main execution
main() {
    echo ""
    log_info "=========================================="
    log_info "  PWA Beta Deployment"
    log_info "  Environment: $ENVIRONMENT"
    log_info "=========================================="
    echo ""

    check_prerequisites
    echo ""

    check_git_status
    echo ""

    install_dependencies
    echo ""

    run_lint
    echo ""

    run_type_check
    echo ""

    run_tests
    echo ""

    configure_feature_flags
    echo ""

    build_app
    echo ""

    deploy_to_vercel
    echo ""

    update_dns
    echo ""

    run_smoke_tests
    echo ""

    generate_release_notes
    echo ""

    send_notifications
    echo ""

    push_to_git
    echo ""

    log_success "=========================================="
    log_success "  Deployment Complete!"
    log_success "=========================================="
    echo ""
    log_info "Environment: $ENVIRONMENT"
    log_info "URL: $(cat .last-deploy-url 2>/dev/null || echo 'N/A')"
    log_info "Branch: $BRANCH_NAME"
    echo ""
    log_info "Next steps:"
    log_info "1. Test the deployment"
    log_info "2. Monitor error tracking (Sentry)"
    log_info "3. Check analytics"
    log_info "4. Gather beta tester feedback"
    echo ""
}

# Run main function
main

exit 0
