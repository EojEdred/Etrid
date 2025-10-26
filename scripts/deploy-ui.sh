#!/usr/bin/env bash

################################################################################
# Ëtrid Protocol - UI Applications Deployment Script
################################################################################
# Description: Deploy all UI applications to Vercel
# Usage: ./scripts/deploy-ui.sh [app-name] [--production]
#
# Apps:
#   - wallet-web       : Main wallet web application
#   - validator        : Validator dashboard
#   - watchtower       : Watchtower monitor
#   - all              : Deploy all applications (default)
#
# Options:
#   --production       : Deploy to production (otherwise preview)
#   --help             : Show this help message
################################################################################

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Helper functions to get app paths and names (compatible with bash 3.x)
get_app_path() {
    case $1 in
        wallet-web) echo "$PROJECT_ROOT/apps/wallet-web/etrid-crypto-website" ;;
        validator) echo "$PROJECT_ROOT/apps/validator-dashboard" ;;
        watchtower) echo "$PROJECT_ROOT/apps/watchtower-monitor" ;;
        *) echo "" ;;
    esac
}

get_app_name() {
    case $1 in
        wallet-web) echo "Ëtrid Wallet Web" ;;
        validator) echo "Validator Dashboard" ;;
        watchtower) echo "Watchtower Monitor" ;;
        *) echo "" ;;
    esac
}

################################################################################
# Helper Functions
################################################################################

print_header() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

show_help() {
    cat << EOF
Ëtrid Protocol - UI Applications Deployment Script

Usage: $0 [app-name] [--production]

Apps:
  wallet-web       Deploy the main wallet web application
  validator        Deploy the validator dashboard
  watchtower       Deploy the watchtower monitor
  all              Deploy all applications (default)

Options:
  --production     Deploy to production environment
  --preview        Deploy as preview (default)
  --help           Show this help message

Examples:
  $0                          # Deploy all apps as preview
  $0 wallet-web               # Deploy wallet-web as preview
  $0 validator --production   # Deploy validator to production
  $0 all --production         # Deploy all apps to production

Environment Variables:
  VERCEL_TOKEN                Optional: Vercel authentication token
  VERCEL_ORG_ID               Optional: Vercel organization ID
  VERCEL_PROJECT_ID           Optional: Vercel project ID

EOF
}

################################################################################
# Pre-flight Checks
################################################################################

check_vercel_cli() {
    print_header "Checking Vercel CLI Installation"

    if ! command -v vercel &> /dev/null; then
        print_error "Vercel CLI is not installed"
        echo ""
        echo "To install Vercel CLI, run:"
        echo -e "${GREEN}npm install -g vercel${NC}"
        echo ""
        echo "Or with pnpm:"
        echo -e "${GREEN}pnpm add -g vercel${NC}"
        echo ""
        exit 1
    fi

    print_success "Vercel CLI found: $(vercel --version)"
}

check_authentication() {
    print_header "Checking Vercel Authentication"

    if ! vercel whoami &> /dev/null; then
        print_warning "Not authenticated with Vercel"
        echo ""
        print_info "Running authentication..."
        vercel login
    else
        print_success "Authenticated as: $(vercel whoami)"
    fi
}

check_app_exists() {
    local app=$1
    local path=$(get_app_path "$app")

    if [[ ! -d "$path" ]]; then
        print_error "Application directory not found: $path"
        return 1
    fi

    if [[ ! -f "$path/package.json" ]]; then
        print_error "package.json not found in: $path"
        return 1
    fi

    if [[ ! -f "$path/vercel.json" ]]; then
        print_warning "vercel.json not found in: $path"
    fi

    return 0
}

################################################################################
# Deployment Functions
################################################################################

deploy_app() {
    local app=$1
    local production=$2
    local path=$(get_app_path "$app")
    local name=$(get_app_name "$app")

    print_header "Deploying: $name"

    # Check if app exists
    if ! check_app_exists "$app"; then
        return 1
    fi

    print_info "Application path: $path"

    # Navigate to app directory
    cd "$path"

    # Build deployment command
    local deploy_cmd="vercel"

    if [[ "$production" == "true" ]]; then
        deploy_cmd="$deploy_cmd --prod"
        print_info "Deploying to PRODUCTION"
    else
        print_info "Deploying as PREVIEW"
    fi

    # Add yes flag to skip confirmations in CI
    if [[ -n "${CI:-}" ]]; then
        deploy_cmd="$deploy_cmd --yes"
    fi

    # Execute deployment
    echo ""
    print_info "Running: $deploy_cmd"
    echo ""

    if $deploy_cmd; then
        echo ""
        print_success "Deployment successful for $name"
        return 0
    else
        echo ""
        print_error "Deployment failed for $name"
        return 1
    fi
}

################################################################################
# Main Script
################################################################################

main() {
    local app_to_deploy="all"
    local production="false"

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --production|--prod)
                production="true"
                shift
                ;;
            --preview)
                production="false"
                shift
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            wallet-web|validator|watchtower|all)
                app_to_deploy=$1
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                echo ""
                show_help
                exit 1
                ;;
        esac
    done

    # Run pre-flight checks
    check_vercel_cli
    check_authentication

    echo ""

    # Deploy applications
    local failed_deployments=()

    if [[ "$app_to_deploy" == "all" ]]; then
        print_header "Deploying All Applications"
        echo ""

        for app in wallet-web validator watchtower; do
            if ! deploy_app "$app" "$production"; then
                failed_deployments+=("$app")
            fi
            echo ""
        done
    else
        if ! deploy_app "$app_to_deploy" "$production"; then
            failed_deployments+=("$app_to_deploy")
        fi
    fi

    # Summary
    print_header "Deployment Summary"

    if [[ ${#failed_deployments[@]} -eq 0 ]]; then
        print_success "All deployments completed successfully!"
        exit 0
    else
        print_error "Some deployments failed:"
        for app in "${failed_deployments[@]}"; do
            echo "  - $(get_app_name "$app")"
        done
        exit 1
    fi
}

# Execute main function
main "$@"
