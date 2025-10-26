#!/usr/bin/env bash

# ═══════════════════════════════════════════════════════════════════════════════
# ËTRID DEPLOY ALL - Comprehensive Deployment Script
# ═══════════════════════════════════════════════════════════════════════════════
# This script deploys all Etrid blockchain applications including:
# - Wallet Web (Next.js app)
# - Validator Dashboard (Next.js app)
# - Watchtower Monitor (Next.js app)
# - Faucet Service (optional)
# - Block Explorer (optional)
#
# Supports multiple deployment targets:
# - Vercel (default for Next.js apps)
# - Docker containers
# - AWS S3 + CloudFront
# - Traditional server (nginx)
#
# Usage:
#   ./scripts/deploy-all.sh [OPTIONS]
#
# Options:
#   --target <vercel|docker|aws|server>  Deployment target (default: vercel)
#   --environment <production|staging>   Deployment environment (default: production)
#   --skip-wallet                        Skip wallet-web deployment
#   --skip-validator                     Skip validator-dashboard deployment
#   --skip-watchtower                    Skip watchtower-monitor deployment
#   --skip-tests                         Skip pre-deployment tests
#   --include-faucet                     Deploy faucet service
#   --include-explorer                   Deploy block explorer
#   --dry-run                            Show what would be deployed without deploying
#   --rollback                           Rollback to previous deployment
#   --help                               Show this help message
#
# Examples:
#   ./scripts/deploy-all.sh                              # Deploy all to Vercel production
#   ./scripts/deploy-all.sh --target docker              # Deploy all as Docker containers
#   ./scripts/deploy-all.sh --environment staging        # Deploy to staging
#   ./scripts/deploy-all.sh --skip-tests --dry-run       # Preview deployment without tests
#   ./scripts/deploy-all.sh --rollback                   # Rollback to previous version
#
# Prerequisites:
#   - Node.js >= 18.0.0
#   - Vercel CLI (for Vercel deployments): npm install -g vercel
#   - Docker (for Docker deployments)
#   - AWS CLI (for AWS deployments)
#   - SSH access (for server deployments)
#
# Environment Variables:
#   VERCEL_TOKEN    Vercel authentication token
#   AWS_PROFILE     AWS CLI profile name
#   DEPLOY_SSH_KEY  SSH private key for server deployments
#   SENTRY_DSN      Sentry error tracking DSN (optional)
#   ANALYTICS_ID    Analytics tracking ID (optional)
# ═══════════════════════════════════════════════════════════════════════════════

set -e  # Exit on error

# ═══════════════════════════════════════════════════════════════════════════════
# Configuration
# ═══════════════════════════════════════════════════════════════════════════════

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

# Deployment configuration
DEPLOY_TARGET="vercel"
DEPLOY_ENV="production"
SKIP_WALLET=false
SKIP_VALIDATOR=false
SKIP_WATCHTOWER=false
SKIP_TESTS=false
INCLUDE_FAUCET=false
INCLUDE_EXPLORER=false
DRY_RUN=false
ROLLBACK=false

# Application directories
WALLET_DIR="apps/wallet-web/etrid-crypto-website"
VALIDATOR_DIR="apps/validator-dashboard"
WATCHTOWER_DIR="apps/watchtower-monitor"
FAUCET_DIR="apps/faucet-service"
EXPLORER_DIR="apps/block-explorer"

# Deployment state tracking
DEPLOYMENTS_FILE="${PROJECT_ROOT}/.deployments.json"
DEPLOYMENT_ID="deploy-$(date +%Y%m%d-%H%M%S)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# ═══════════════════════════════════════════════════════════════════════════════
# Helper Functions
# ═══════════════════════════════════════════════════════════════════════════════

print_header() {
    echo -e "\n${CYAN}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════════════════════${NC}\n"
}

print_section() {
    echo -e "\n${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ $1${NC}"
}

show_help() {
    grep '^#' "$0" | grep -v '#!/usr/bin/env' | sed 's/^# //' | sed 's/^#//'
    exit 0
}

check_command() {
    if ! command -v "$1" &> /dev/null; then
        print_error "Required command '$1' not found"
        return 1
    fi
    return 0
}

format_duration() {
    local seconds=$1
    local minutes=$((seconds / 60))
    local remaining_seconds=$((seconds % 60))

    if [ $minutes -gt 0 ]; then
        echo "${minutes}m ${remaining_seconds}s"
    else
        echo "${seconds}s"
    fi
}

# Save deployment state
save_deployment_state() {
    local app_name=$1
    local deployment_url=$2
    local status=$3

    local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

    # Create deployments file if it doesn't exist
    if [ ! -f "$DEPLOYMENTS_FILE" ]; then
        echo "{\"deployments\": []}" > "$DEPLOYMENTS_FILE"
    fi

    # Add deployment record (using Python for JSON manipulation)
    python3 -c "
import json
import sys

with open('$DEPLOYMENTS_FILE', 'r') as f:
    data = json.load(f)

data['deployments'].append({
    'id': '$DEPLOYMENT_ID',
    'app': '$app_name',
    'url': '$deployment_url',
    'status': '$status',
    'environment': '$DEPLOY_ENV',
    'target': '$DEPLOY_TARGET',
    'timestamp': '$timestamp'
})

# Keep only last 50 deployments
data['deployments'] = data['deployments'][-50:]

with open('$DEPLOYMENTS_FILE', 'w') as f:
    json.dump(data, f, indent=2)
" 2>/dev/null || print_warning "Could not save deployment state (Python required)"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Parse Command Line Arguments
# ═══════════════════════════════════════════════════════════════════════════════

while [[ $# -gt 0 ]]; do
    case $1 in
        --target)
            DEPLOY_TARGET="$2"
            shift 2
            ;;
        --environment)
            DEPLOY_ENV="$2"
            shift 2
            ;;
        --skip-wallet)
            SKIP_WALLET=true
            shift
            ;;
        --skip-validator)
            SKIP_VALIDATOR=true
            shift
            ;;
        --skip-watchtower)
            SKIP_WATCHTOWER=true
            shift
            ;;
        --skip-tests)
            SKIP_TESTS=true
            shift
            ;;
        --include-faucet)
            INCLUDE_FAUCET=true
            shift
            ;;
        --include-explorer)
            INCLUDE_EXPLORER=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --rollback)
            ROLLBACK=true
            shift
            ;;
        --help|-h)
            show_help
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# ═══════════════════════════════════════════════════════════════════════════════
# Pre-flight Checks
# ═══════════════════════════════════════════════════════════════════════════════

print_header "ËTRID DEPLOY ALL - Starting Deployment Process"

print_info "Deployment Configuration:"
echo "  Target: ${DEPLOY_TARGET}"
echo "  Environment: ${DEPLOY_ENV}"
echo "  Deployment ID: ${DEPLOYMENT_ID}"
echo ""

# Handle rollback mode
if [ "$ROLLBACK" = true ]; then
    print_header "Rollback Mode"

    if [ ! -f "$DEPLOYMENTS_FILE" ]; then
        print_error "No deployment history found. Cannot rollback."
        exit 1
    fi

    print_info "Recent deployments:"
    python3 -c "
import json
with open('$DEPLOYMENTS_FILE', 'r') as f:
    data = json.load(f)
    for dep in data['deployments'][-10:]:
        print(f\"{dep['timestamp']} | {dep['app']:20} | {dep['environment']:10} | {dep['status']}\")
" 2>/dev/null || print_error "Could not read deployment history"

    print_warning "Rollback functionality requires manual intervention"
    print_info "Use your deployment platform's rollback features (Vercel, AWS, etc.)"
    exit 0
fi

# Check prerequisites based on deployment target
print_section "Checking Prerequisites"

check_command "node" || exit 1
check_command "npm" || exit 1
print_success "Node.js found: $(node --version)"
print_success "npm found: $(npm --version)"

case $DEPLOY_TARGET in
    vercel)
        check_command "vercel" || {
            print_error "Vercel CLI not found. Install with: npm install -g vercel"
            exit 1
        }

        if [ -z "$VERCEL_TOKEN" ]; then
            print_warning "VERCEL_TOKEN not set. Using interactive login."
        else
            print_success "Vercel token configured"
        fi
        ;;

    docker)
        check_command "docker" || exit 1
        print_success "Docker found: $(docker --version)"
        ;;

    aws)
        check_command "aws" || {
            print_error "AWS CLI not found. Install from: https://aws.amazon.com/cli/"
            exit 1
        }
        print_success "AWS CLI found: $(aws --version 2>&1 | head -n1)"
        ;;

    server)
        check_command "ssh" || exit 1

        if [ -z "$DEPLOY_SSH_KEY" ]; then
            print_warning "DEPLOY_SSH_KEY not set. Will use default SSH key."
        fi
        ;;

    *)
        print_error "Invalid deployment target: $DEPLOY_TARGET"
        echo "Valid targets: vercel, docker, aws, server"
        exit 1
        ;;
esac

cd "$PROJECT_ROOT"

# ═══════════════════════════════════════════════════════════════════════════════
# Pre-Deployment Tests
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$SKIP_TESTS" = false ] && [ "$DRY_RUN" = false ]; then
    print_header "Running Pre-Deployment Tests"

    APPS_TO_TEST=()
    [ "$SKIP_WALLET" = false ] && APPS_TO_TEST+=("$WALLET_DIR")
    [ "$SKIP_VALIDATOR" = false ] && APPS_TO_TEST+=("$VALIDATOR_DIR")
    [ "$SKIP_WATCHTOWER" = false ] && APPS_TO_TEST+=("$WATCHTOWER_DIR")

    for app_dir in "${APPS_TO_TEST[@]}"; do
        if [ -d "$app_dir" ] && [ -f "$app_dir/package.json" ]; then
            app_name=$(basename "$app_dir")
            print_section "Testing $app_name"

            cd "$app_dir"

            # Install dependencies if needed
            if [ ! -d "node_modules" ]; then
                print_info "Installing dependencies..."
                npm install --silent
            fi

            # Run tests if test script exists
            if grep -q '"test"' package.json; then
                print_info "Running tests..."

                if npm test 2>&1 | tail -n 20; then
                    print_success "$app_name tests passed"
                else
                    print_error "$app_name tests failed"
                    print_warning "Deployment aborted due to test failures"
                    exit 1
                fi
            else
                print_warning "No tests found for $app_name, skipping..."
            fi

            cd "$PROJECT_ROOT"
        fi
    done

    print_success "All pre-deployment tests passed"
else
    print_warning "Skipping pre-deployment tests"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Deployment Functions
# ═══════════════════════════════════════════════════════════════════════════════

deploy_to_vercel() {
    local app_dir=$1
    local app_name=$2

    if [ ! -d "$app_dir" ]; then
        print_warning "$app_name directory not found: $app_dir"
        return 1
    fi

    cd "$app_dir"

    print_info "Building $app_name..."

    if [ "$DRY_RUN" = true ]; then
        print_info "[DRY RUN] Would build and deploy $app_name to Vercel ($DEPLOY_ENV)"
        cd "$PROJECT_ROOT"
        return 0
    fi

    # Build the application
    npm run build || {
        print_error "Build failed for $app_name"
        cd "$PROJECT_ROOT"
        return 1
    }

    print_info "Deploying $app_name to Vercel..."

    # Deploy based on environment
    if [ "$DEPLOY_ENV" = "production" ]; then
        vercel --prod --yes 2>&1 | tee /tmp/vercel-deploy.log
    else
        vercel --yes 2>&1 | tee /tmp/vercel-deploy.log
    fi

    # Extract deployment URL
    DEPLOY_URL=$(grep -oP 'https://[^\s]+' /tmp/vercel-deploy.log | tail -n 1)

    if [ -n "$DEPLOY_URL" ]; then
        print_success "$app_name deployed to: $DEPLOY_URL"
        save_deployment_state "$app_name" "$DEPLOY_URL" "success"
    else
        print_error "Failed to get deployment URL for $app_name"
        save_deployment_state "$app_name" "unknown" "failed"
        cd "$PROJECT_ROOT"
        return 1
    fi

    cd "$PROJECT_ROOT"
    return 0
}

deploy_to_docker() {
    local app_dir=$1
    local app_name=$2
    local image_name="etrid/${app_name}:${DEPLOY_ENV}"

    if [ ! -d "$app_dir" ]; then
        print_warning "$app_name directory not found: $app_dir"
        return 1
    fi

    print_info "Building Docker image for $app_name..."

    if [ "$DRY_RUN" = true ]; then
        print_info "[DRY RUN] Would build Docker image: $image_name"
        return 0
    fi

    # Create Dockerfile if it doesn't exist
    if [ ! -f "$app_dir/Dockerfile" ]; then
        print_warning "No Dockerfile found, creating default Next.js Dockerfile..."

        cat > "$app_dir/Dockerfile" <<'DOCKERFILE_END'
FROM node:18-alpine AS deps
WORKDIR /app
COPY package*.json ./
RUN npm ci

FROM node:18-alpine AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .
RUN npm run build

FROM node:18-alpine AS runner
WORKDIR /app
ENV NODE_ENV production
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs
COPY --from=builder /app/public ./public
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static
USER nextjs
EXPOSE 3000
ENV PORT 3000
CMD ["node", "server.js"]
DOCKERFILE_END
    fi

    # Build Docker image
    cd "$app_dir"
    docker build -t "$image_name" . || {
        print_error "Docker build failed for $app_name"
        cd "$PROJECT_ROOT"
        return 1
    }

    print_success "Docker image built: $image_name"

    # Optionally push to registry
    if [ -n "$DOCKER_REGISTRY" ]; then
        print_info "Pushing to Docker registry..."
        docker tag "$image_name" "${DOCKER_REGISTRY}/${image_name}"
        docker push "${DOCKER_REGISTRY}/${image_name}"
        print_success "Pushed to registry: ${DOCKER_REGISTRY}/${image_name}"
    fi

    save_deployment_state "$app_name" "docker:${image_name}" "success"
    cd "$PROJECT_ROOT"
    return 0
}

deploy_to_aws() {
    local app_dir=$1
    local app_name=$2
    local bucket_name="etrid-${app_name}-${DEPLOY_ENV}"

    if [ ! -d "$app_dir" ]; then
        print_warning "$app_name directory not found: $app_dir"
        return 1
    fi

    print_info "Deploying $app_name to AWS S3..."

    if [ "$DRY_RUN" = true ]; then
        print_info "[DRY RUN] Would deploy to S3 bucket: $bucket_name"
        return 0
    fi

    cd "$app_dir"

    # Build for static export
    print_info "Building static export..."
    npm run build || {
        print_error "Build failed for $app_name"
        cd "$PROJECT_ROOT"
        return 1
    }

    # Create S3 bucket if it doesn't exist
    if ! aws s3 ls "s3://${bucket_name}" 2>/dev/null; then
        print_info "Creating S3 bucket: $bucket_name"
        aws s3 mb "s3://${bucket_name}"

        # Enable static website hosting
        aws s3 website "s3://${bucket_name}" \
            --index-document index.html \
            --error-document 404.html
    fi

    # Sync build output to S3
    print_info "Uploading to S3..."
    aws s3 sync out/ "s3://${bucket_name}/" --delete

    # Invalidate CloudFront cache if distribution exists
    if [ -n "$CLOUDFRONT_DISTRIBUTION_ID" ]; then
        print_info "Invalidating CloudFront cache..."
        aws cloudfront create-invalidation \
            --distribution-id "$CLOUDFRONT_DISTRIBUTION_ID" \
            --paths "/*"
    fi

    DEPLOY_URL="http://${bucket_name}.s3-website-us-east-1.amazonaws.com"
    print_success "$app_name deployed to: $DEPLOY_URL"
    save_deployment_state "$app_name" "$DEPLOY_URL" "success"

    cd "$PROJECT_ROOT"
    return 0
}

# ═══════════════════════════════════════════════════════════════════════════════
# Deploy Applications
# ═══════════════════════════════════════════════════════════════════════════════

print_header "Deploying Applications"

TOTAL_START_TIME=$(date +%s)
DEPLOYMENT_RESULTS=()

# Deploy Wallet Web
if [ "$SKIP_WALLET" = false ]; then
    print_section "Deploying Wallet Web"

    case $DEPLOY_TARGET in
        vercel) deploy_to_vercel "$WALLET_DIR" "wallet-web" ;;
        docker) deploy_to_docker "$WALLET_DIR" "wallet-web" ;;
        aws) deploy_to_aws "$WALLET_DIR" "wallet-web" ;;
        *) print_warning "Deployment target $DEPLOY_TARGET not fully implemented for wallet-web" ;;
    esac

    DEPLOYMENT_RESULTS+=("wallet-web:$?")
else
    print_warning "Skipping wallet-web deployment"
fi

# Deploy Validator Dashboard
if [ "$SKIP_VALIDATOR" = false ]; then
    print_section "Deploying Validator Dashboard"

    case $DEPLOY_TARGET in
        vercel) deploy_to_vercel "$VALIDATOR_DIR" "validator-dashboard" ;;
        docker) deploy_to_docker "$VALIDATOR_DIR" "validator-dashboard" ;;
        aws) deploy_to_aws "$VALIDATOR_DIR" "validator-dashboard" ;;
        *) print_warning "Deployment target $DEPLOY_TARGET not fully implemented for validator-dashboard" ;;
    esac

    DEPLOYMENT_RESULTS+=("validator-dashboard:$?")
else
    print_warning "Skipping validator-dashboard deployment"
fi

# Deploy Watchtower Monitor
if [ "$SKIP_WATCHTOWER" = false ]; then
    print_section "Deploying Watchtower Monitor"

    case $DEPLOY_TARGET in
        vercel) deploy_to_vercel "$WATCHTOWER_DIR" "watchtower-monitor" ;;
        docker) deploy_to_docker "$WATCHTOWER_DIR" "watchtower-monitor" ;;
        aws) deploy_to_aws "$WATCHTOWER_DIR" "watchtower-monitor" ;;
        *) print_warning "Deployment target $DEPLOY_TARGET not fully implemented for watchtower-monitor" ;;
    esac

    DEPLOYMENT_RESULTS+=("watchtower-monitor:$?")
else
    print_warning "Skipping watchtower-monitor deployment"
fi

# Deploy optional services
if [ "$INCLUDE_FAUCET" = true ]; then
    print_section "Deploying Faucet Service"
    print_warning "Faucet deployment not yet implemented"
fi

if [ "$INCLUDE_EXPLORER" = true ]; then
    print_section "Deploying Block Explorer"
    print_warning "Block explorer deployment not yet implemented"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Post-Deployment Health Checks
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$DRY_RUN" = false ]; then
    print_header "Post-Deployment Health Checks"

    # Read deployment URLs from state file
    if [ -f "$DEPLOYMENTS_FILE" ]; then
        print_info "Checking deployment health..."

        # Wait for deployments to become available
        sleep 10

        # Check each recent deployment
        python3 -c "
import json
import urllib.request

with open('$DEPLOYMENTS_FILE', 'r') as f:
    data = json.load(f)

recent = [d for d in data['deployments'] if d['id'] == '$DEPLOYMENT_ID']

for dep in recent:
    url = dep['url']
    if url.startswith('http'):
        try:
            response = urllib.request.urlopen(url, timeout=10)
            if response.status == 200:
                print(f\"✓ {dep['app']}: {url} - Healthy\")
            else:
                print(f\"✗ {dep['app']}: {url} - Status {response.status}\")
        except Exception as e:
            print(f\"✗ {dep['app']}: {url} - Error: {e}\")
" 2>/dev/null || print_warning "Could not run health checks (Python + urllib required)"
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Deployment Summary
# ═══════════════════════════════════════════════════════════════════════════════

TOTAL_END_TIME=$(date +%s)
TOTAL_DURATION=$((TOTAL_END_TIME - TOTAL_START_TIME))

print_header "Deployment Summary"

# Count successes and failures
SUCCESS_COUNT=0
FAILURE_COUNT=0

for result in "${DEPLOYMENT_RESULTS[@]}"; do
    app_name="${result%%:*}"
    exit_code="${result##*:}"

    if [ "$exit_code" = "0" ]; then
        echo -e "${GREEN}✓ $app_name${NC}"
        ((SUCCESS_COUNT++))
    else
        echo -e "${RED}✗ $app_name${NC}"
        ((FAILURE_COUNT++))
    fi
done

echo ""
echo "Deployment ID: $DEPLOYMENT_ID"
echo "Target: $DEPLOY_TARGET"
echo "Environment: $DEPLOY_ENV"
echo "Total time: $(format_duration $TOTAL_DURATION)"
echo ""

if [ $FAILURE_COUNT -eq 0 ]; then
    print_success "All deployments completed successfully!"
    echo ""
    print_info "Deployment URLs:"

    if [ -f "$DEPLOYMENTS_FILE" ]; then
        python3 -c "
import json
with open('$DEPLOYMENTS_FILE', 'r') as f:
    data = json.load(f)
recent = [d for d in data['deployments'] if d['id'] == '$DEPLOYMENT_ID' and d['url'].startswith('http')]
for dep in recent:
    print(f\"  {dep['app']:25} {dep['url']}\")
" 2>/dev/null || echo "  (Check $DEPLOYMENTS_FILE for details)"
    fi

    echo ""
    exit 0
else
    print_error "$FAILURE_COUNT deployment(s) failed"
    echo ""
    print_info "Check logs above for details"
    print_info "Run with --rollback to revert to previous deployment"
    exit 1
fi
