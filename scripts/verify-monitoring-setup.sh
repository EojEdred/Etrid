#!/usr/bin/env bash

# ═══════════════════════════════════════════════════════════════════════════════
# ËTRID MONITORING SETUP VERIFICATION SCRIPT
# ═══════════════════════════════════════════════════════════════════════════════
# This script verifies that all monitoring components are properly configured
# and ready for deployment.
#
# Usage:
#   ./scripts/verify-monitoring-setup.sh
#
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

# Counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
WARNING_CHECKS=0

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
    ((PASSED_CHECKS++))
    ((TOTAL_CHECKS++))
}

print_failure() {
    echo -e "${RED}✗ $1${NC}"
    ((FAILED_CHECKS++))
    ((TOTAL_CHECKS++))
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
    ((WARNING_CHECKS++))
    ((TOTAL_CHECKS++))
}

print_info() {
    echo -e "${CYAN}ℹ $1${NC}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# Verification Checks
# ═══════════════════════════════════════════════════════════════════════════════

print_header "ËTRID MONITORING SETUP VERIFICATION"

cd "$PROJECT_ROOT"

# ───────────────────────────────────────────────────────────────────────────────
# 1. Configuration Files
# ───────────────────────────────────────────────────────────────────────────────

print_section "1. Checking Configuration Files"

# Prometheus configuration
if [ -f "scripts/testnet/prometheus.yml" ]; then
    LINES=$(wc -l < scripts/testnet/prometheus.yml)
    if [ "$LINES" -gt 50 ]; then
        print_success "prometheus.yml exists ($LINES lines)"
    else
        print_failure "prometheus.yml exists but seems incomplete ($LINES lines)"
    fi
else
    print_failure "prometheus.yml not found"
fi

# Alerting rules
if [ -f "scripts/testnet/alerting-rules.yml" ]; then
    LINES=$(wc -l < scripts/testnet/alerting-rules.yml)
    if [ "$LINES" -gt 200 ]; then
        print_success "alerting-rules.yml exists ($LINES lines)"
    else
        print_warning "alerting-rules.yml exists but may be incomplete ($LINES lines)"
    fi
else
    print_failure "alerting-rules.yml not found"
fi

# Grafana dashboard
if [ -f "scripts/testnet/grafana-dashboard.json" ]; then
    LINES=$(wc -l < scripts/testnet/grafana-dashboard.json)
    if [ "$LINES" -gt 400 ]; then
        print_success "grafana-dashboard.json exists ($LINES lines)"
    else
        print_warning "grafana-dashboard.json exists but may be incomplete ($LINES lines)"
    fi
else
    print_failure "grafana-dashboard.json not found"
fi

# ───────────────────────────────────────────────────────────────────────────────
# 2. Docker Configuration
# ───────────────────────────────────────────────────────────────────────────────

print_section "2. Checking Docker Configuration"

if [ -f "docker-compose.yml" ]; then
    print_success "docker-compose.yml exists"

    # Check for Charlie node
    if grep -q "fullnode-charlie" docker-compose.yml; then
        print_success "Charlie (full node) configured in docker-compose.yml"
    else
        print_failure "Charlie node not found in docker-compose.yml"
    fi

    # Check for Prometheus service
    if grep -q "prometheus:" docker-compose.yml; then
        print_success "Prometheus service configured"
    else
        print_failure "Prometheus service not configured"
    fi

    # Check for Grafana service
    if grep -q "grafana:" docker-compose.yml; then
        print_success "Grafana service configured"
    else
        print_failure "Grafana service not configured"
    fi

    # Check for alerting rules mount
    if grep -q "alerting-rules.yml" docker-compose.yml; then
        print_success "Alerting rules mounted in Prometheus"
    else
        print_warning "Alerting rules may not be mounted in Prometheus"
    fi

else
    print_failure "docker-compose.yml not found"
fi

# ───────────────────────────────────────────────────────────────────────────────
# 3. Documentation
# ───────────────────────────────────────────────────────────────────────────────

print_section "3. Checking Documentation"

# Monitoring guide
if [ -f "docs/MONITORING_GUIDE.md" ]; then
    LINES=$(wc -l < docs/MONITORING_GUIDE.md)
    if [ "$LINES" -gt 1000 ]; then
        print_success "MONITORING_GUIDE.md exists ($LINES lines)"
    else
        print_warning "MONITORING_GUIDE.md exists but may be incomplete ($LINES lines)"
    fi
else
    print_failure "MONITORING_GUIDE.md not found"
fi

# Quick start guide
if [ -f "scripts/testnet/MONITORING_QUICK_START.md" ]; then
    LINES=$(wc -l < scripts/testnet/MONITORING_QUICK_START.md)
    if [ "$LINES" -gt 200 ]; then
        print_success "MONITORING_QUICK_START.md exists ($LINES lines)"
    else
        print_warning "MONITORING_QUICK_START.md exists but may be incomplete ($LINES lines)"
    fi
else
    print_failure "MONITORING_QUICK_START.md not found"
fi

# README
if [ -f "scripts/testnet/README_MONITORING.md" ]; then
    LINES=$(wc -l < scripts/testnet/README_MONITORING.md)
    if [ "$LINES" -gt 400 ]; then
        print_success "README_MONITORING.md exists ($LINES lines)"
    else
        print_warning "README_MONITORING.md exists but may be incomplete ($LINES lines)"
    fi
else
    print_failure "README_MONITORING.md not found"
fi

# ───────────────────────────────────────────────────────────────────────────────
# 4. Prometheus Configuration Validation
# ───────────────────────────────────────────────────────────────────────────────

print_section "4. Validating Prometheus Configuration"

if [ -f "scripts/testnet/prometheus.yml" ]; then
    # Check for required scrape configs
    if grep -q "flarechain-alice" scripts/testnet/prometheus.yml; then
        print_success "Alice node configured in Prometheus"
    else
        print_failure "Alice node not configured in Prometheus"
    fi

    if grep -q "flarechain-bob" scripts/testnet/prometheus.yml; then
        print_success "Bob node configured in Prometheus"
    else
        print_failure "Bob node not configured in Prometheus"
    fi

    if grep -q "flarechain-charlie" scripts/testnet/prometheus.yml; then
        print_success "Charlie node configured in Prometheus"
    else
        print_failure "Charlie node not configured in Prometheus"
    fi

    # Check for alerting rules reference
    if grep -q "rule_files:" scripts/testnet/prometheus.yml; then
        print_success "Alerting rules configured in Prometheus"
    else
        print_warning "Alerting rules may not be configured in Prometheus"
    fi
fi

# ───────────────────────────────────────────────────────────────────────────────
# 5. Alerting Rules Validation
# ───────────────────────────────────────────────────────────────────────────────

print_section "5. Validating Alerting Rules"

if [ -f "scripts/testnet/alerting-rules.yml" ]; then
    # Check for critical alerts
    CRITICAL_ALERTS=("NoBlocksProduced" "FinalizationStalled" "NoPeersConnected" "ValidatorOffline" "PPFASealingFailure")

    for alert in "${CRITICAL_ALERTS[@]}"; do
        if grep -q "$alert" scripts/testnet/alerting-rules.yml; then
            print_success "Alert '$alert' configured"
        else
            print_warning "Alert '$alert' not found"
        fi
    done
fi

# ───────────────────────────────────────────────────────────────────────────────
# 6. Grafana Dashboard Validation
# ───────────────────────────────────────────────────────────────────────────────

print_section "6. Validating Grafana Dashboard"

if [ -f "scripts/testnet/grafana-dashboard.json" ]; then
    # Check for essential panels
    PANEL_METRICS=("substrate_block_height" "substrate_finalized_height" "substrate_proposer_number_of_transactions" "substrate_sub_libp2p_peers_count" "process_cpu_seconds_total" "process_resident_memory_bytes")

    for metric in "${PANEL_METRICS[@]}"; do
        if grep -q "$metric" scripts/testnet/grafana-dashboard.json; then
            print_success "Panel for '$metric' configured"
        else
            print_warning "Panel for '$metric' not found"
        fi
    done
fi

# ───────────────────────────────────────────────────────────────────────────────
# 7. Script Availability
# ───────────────────────────────────────────────────────────────────────────────

print_section "7. Checking Helper Scripts"

if [ -f "scripts/setup-monitoring-stack.sh" ]; then
    if [ -x "scripts/setup-monitoring-stack.sh" ]; then
        print_success "setup-monitoring-stack.sh exists and is executable"
    else
        print_warning "setup-monitoring-stack.sh exists but is not executable"
    fi
else
    print_warning "setup-monitoring-stack.sh not found"
fi

if [ -f "scripts/start-testnet.sh" ]; then
    if [ -x "scripts/start-testnet.sh" ]; then
        print_success "start-testnet.sh exists and is executable"
    else
        print_warning "start-testnet.sh exists but is not executable"
    fi
else
    print_warning "start-testnet.sh not found"
fi

# ───────────────────────────────────────────────────────────────────────────────
# 8. Optional: Runtime Checks (if Docker is available)
# ───────────────────────────────────────────────────────────────────────────────

print_section "8. Checking Runtime Environment (Optional)"

if command -v docker &> /dev/null; then
    print_success "Docker is installed"

    if command -v docker-compose &> /dev/null; then
        print_success "Docker Compose is installed"
    else
        print_warning "Docker Compose is not installed (required for easy deployment)"
    fi
else
    print_warning "Docker is not installed (required for containerized deployment)"
fi

if command -v prometheus &> /dev/null; then
    PROM_VERSION=$(prometheus --version 2>&1 | head -1)
    print_success "Prometheus is installed: $PROM_VERSION"
else
    print_info "Prometheus not installed locally (can use Docker)"
fi

if command -v grafana-server &> /dev/null; then
    GRAFANA_VERSION=$(grafana-server -v 2>&1 | head -1)
    print_success "Grafana is installed: $GRAFANA_VERSION"
else
    print_info "Grafana not installed locally (can use Docker)"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Summary
# ═══════════════════════════════════════════════════════════════════════════════

print_header "VERIFICATION SUMMARY"

echo -e "Total Checks:    ${CYAN}$TOTAL_CHECKS${NC}"
echo -e "Passed:          ${GREEN}$PASSED_CHECKS${NC}"
echo -e "Failed:          ${RED}$FAILED_CHECKS${NC}"
echo -e "Warnings:        ${YELLOW}$WARNING_CHECKS${NC}"
echo ""

if [ $FAILED_CHECKS -eq 0 ]; then
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}                 ✓ MONITORING SETUP VERIFICATION PASSED                        ${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "${GREEN}All critical components are properly configured!${NC}"
    echo ""
    echo "Next Steps:"
    echo "  1. Start the monitoring stack:"
    echo "     ${CYAN}docker-compose up -d${NC}"
    echo ""
    echo "  2. Access Grafana:"
    echo "     ${CYAN}http://localhost:3001${NC}"
    echo "     ${YELLOW}Username: admin${NC}"
    echo "     ${YELLOW}Password: etrid2025${NC}"
    echo ""
    echo "  3. Access Prometheus:"
    echo "     ${CYAN}http://localhost:9090${NC}"
    echo ""
    echo "  4. Check monitoring status:"
    echo "     ${CYAN}./scripts/monitoring-status.sh${NC}"
    echo ""

    if [ $WARNING_CHECKS -gt 0 ]; then
        echo -e "${YELLOW}Note: $WARNING_CHECKS warnings were detected. Review the output above.${NC}"
        echo ""
    fi

    exit 0
else
    echo -e "${RED}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${RED}                 ✗ MONITORING SETUP VERIFICATION FAILED                        ${NC}"
    echo -e "${RED}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "${RED}$FAILED_CHECKS critical check(s) failed.${NC}"
    echo ""
    echo "Please review the errors above and ensure all required files are present."
    echo ""
    echo "Required files:"
    echo "  - scripts/testnet/prometheus.yml"
    echo "  - scripts/testnet/alerting-rules.yml"
    echo "  - scripts/testnet/grafana-dashboard.json"
    echo "  - docker-compose.yml (with monitoring services)"
    echo "  - docs/MONITORING_GUIDE.md"
    echo ""
    echo "For more information, see:"
    echo "  ${CYAN}docs/MONITORING_GUIDE.md${NC}"
    echo ""
    exit 1
fi
